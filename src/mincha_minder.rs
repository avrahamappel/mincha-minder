use chrono::{DateTime, Datelike, Duration, NaiveDate, NaiveDateTime, TimeZone, Utc};
use icalendar::{Alarm, Calendar, Component, Event, EventLike, Trigger};
use sunrise::sunrise_sunset;
use tzf_rs::DefaultFinder;
use tzfile::Tz;

const DEFAULT_DURATION: i32 = 15;

pub struct LatLong {
    lat: f64,
    long: f64,
    tz: Tz,
}

impl LatLong {
    pub fn new(lat: f64, long: f64) -> Self {
        let finder = DefaultFinder::new();
        let tz_name = finder.get_tz_name(long, lat);
        let tz = Tz::named(tz_name).expect("Couldn't convert tz name into time zone");

        Self { lat, long, tz }
    }

    fn as_tz(&self) -> &Tz {
        &self.tz
    }
}

#[derive(Clone, Copy, Debug)]
struct Sunset(i64);

impl Sunset {
    fn new(LatLong { lat, long, .. }: &LatLong, date: NaiveDate) -> Self {
        let (_, sunset) = sunrise_sunset(*lat, *long, date.year(), date.month(), date.day());

        Self(sunset)
    }
}

impl From<Sunset> for NaiveDateTime {
    fn from(sunset: Sunset) -> Self {
        NaiveDateTime::from_timestamp_opt(sunset.0, 0)
            .expect("Error creating time from sunset value")
    }
}

fn current_time() -> DateTime<Utc> {
    // Hack for test
    // @TODO instead of doing this, should be a date field in Schedule
    // Might be able to iterate over duration when creating events
    // Can then pass arbitrary dates for tests
    if cfg!(test) {
        return Utc
            .with_ymd_and_hms(2023, 3, 10, 12, 0, 0)
            .single()
            .unwrap();
    }

    Utc::now()
}

struct ShaahZemanis(f64);

impl ShaahZemanis {
    fn new(hours: f64) -> Self {
        Self(hours.min(12.0))
    }
}

pub struct Time {
    // Hours after daylight (should never be more than 12)
    hours: ShaahZemanis,
    // Clock minutes before (negative) or after (positive)
    offset: i32,
}

pub struct Zeman {
    time: Time,
    name: &'static str,
    prep_time: u32, // In minutes
}

impl Zeman {
    fn time(&self) -> DateTime<Utc> {
        // pass the current date in here somehow
        // find duration between sunrise and sunset
        // divide into 12
        // multiply by time.hours
        // add offset
        todo!()
    }

    fn alarm_time(&self) -> Duration {
        Duration::minutes(self.prep_time.into())
    }

    fn to_evt_for_date(self, date: DateTime<Utc>) -> Event {
        Event::new()
            .summary(self.name)
            .starts(self.time())
            .ends(self.time() + DEFAULT_DURATION)
            .alarm(Alarm::display(
                self.name,
                Trigger::before_start(self.alarm_time()),
            ))
            .done()
    }
}

pub struct Schedule {
    events: Vec<Zeman>,
    lat_long: LatLong,
    date: DateTime<Utc>,
    // Number of days to generate events for
    days: u32,
}

impl Schedule {
    pub fn new(lat: f64, long: f64) -> Self {
        let lat_long = LatLong::new(lat, long);
        Self {
            lat_long,
            events: vec![],
            days: 90,
            date: Utc::now().with_timezone(lat_long.tz),
        }
    }

    pub fn with_date(mut self, date: DateTime<Utc>) -> Self {
        self.date = date;
        self
    }

    pub fn with_zemanim(mut self, events: impl IntoIterator<Item = Zeman>) -> Self {
        self.events = events.into_iter().collect();
        self
    }

    pub fn with_days(mut self, days: u32) -> Self {
        self.days = days;
        self
    }

    fn events_for_day(&self, day: u32) -> Vec<Event> {}

    pub fn to_ical(&self) -> String {
        Calendar::from(self).to_string()
    }
}

impl From<&Schedule> for Calendar {
    fn from(sch: &Schedule) -> Self {
        let evts_iter = (0..sch.days).flat_map(|n| sch.events_for_day(n));

        let mut cal = Calendar::new();

        for evt in evts_iter {
            cal.push(evt);
        }

        cal
    }
}

#[cfg(test)]
mod tests {
    use icalendar::{CalendarComponent, DatePerhapsTime};

    use super::*;

    // Helper function to build a date quickly
    #[allow(clippy::needless_pass_by_value)]
    fn date2utc(
        tz: impl TimeZone,
        y: i32,
        mo: u32,
        d: u32,
        h: u32,
        min: u32,
        s: u32,
    ) -> DatePerhapsTime {
        DatePerhapsTime::from(
            tz.with_ymd_and_hms(y, mo, d, h, min, s)
                .unwrap()
                .with_timezone(&Utc),
        )
    }

    #[test]
    fn generates_events_for_correct_times() {
        let sch = Schedule::new(43.73, -79.44) // North York, Ontario
            .with_zemanim(vec![Zeman {
                name: "Mincha",
                time: Time {
                    hours: ShaahZemanis::new(12.0), // Shkiyah
                    offset: 40,                     // 40 minutes after shkiyah in Bobov
                },
                prep_time: 5,
            }]);
        let tz = Tz::named("America/Toronto").unwrap();

        let cal = Calendar::from(&sch);
        let first_event = cal.iter().find_map(CalendarComponent::as_event).unwrap();
        let last_event = cal
            .iter()
            .filter_map(CalendarComponent::as_event)
            .last()
            .unwrap();

        assert_eq!("Mincha", first_event.get_summary().unwrap());
        assert_eq!(
            date2utc(&tz, 2023, 3, 10, 18, 56, 56),
            first_event.get_start().unwrap()
        );
        assert_eq!(
            date2utc(&tz, 2023, 3, 10, 19, 16, 56),
            first_event.get_end().unwrap()
        );
        assert_eq!(
            date2utc(&tz, 2023, 6, 7, 21, 36, 46),
            last_event.get_start().unwrap()
        );
        assert_eq!(
            date2utc(&tz, 2023, 6, 7, 21, 56, 46),
            last_event.get_end().unwrap()
        );
    }
}

// @TODO replace expcts with result bubbling
