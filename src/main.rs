use chrono::{DateTime, Datelike, Duration, NaiveDate, NaiveDateTime, TimeZone, Utc};
use icalendar::{Alarm, Calendar, Component, Event, EventLike, Trigger};
use sunrise::sunrise_sunset;
use tzf_rs::DefaultFinder;
use tzfile::Tz;

const DEFAULT_MINCHA_DURATION: i64 = 20;

struct LatLong {
    lat: f64,
    long: f64,
    tz: Tz,
}

impl LatLong {
    fn new(lat: f64, long: f64) -> Self {
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

struct MinchaTime<'sch> {
    time: DateTime<Utc>,
    sch: &'sch Schedule,
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

impl<'sch> MinchaTime<'sch> {
    fn new(sch: &'sch Schedule, days_from_today: u32) -> Self {
        let tz = sch.lat_long.as_tz();

        let date =
            current_time().with_timezone(&tz).date_naive() + Duration::days(days_from_today.into());

        let sunset = Sunset::new(&sch.lat_long, date);

        let time = (tz.from_utc_datetime(&sunset.into()) + Duration::minutes(sch.minutes.into()))
            .with_timezone(&Utc);

        Self { time, sch }
    }

    fn start(&self) -> DateTime<Utc> {
        self.time
    }

    fn end(&self) -> DateTime<Utc> {
        self.time + Duration::minutes(DEFAULT_MINCHA_DURATION)
    }

    fn alarm(&self) -> Duration {
        Duration::minutes(self.sch.prep_time.into())
    }
}

impl From<MinchaTime<'_>> for Event {
    fn from(mt: MinchaTime) -> Self {
        Event::new()
            .summary("Mincha")
            .starts(mt.start())
            .ends(mt.end())
            .alarm(Alarm::display(
                "Time for Mincha",
                Trigger::before_start(mt.alarm()),
            ))
            .done()
    }
}

struct Schedule {
    // Minutes before (negative) or after (positive) sunset
    minutes: i32,
    lat_long: LatLong,
    prep_time: u32, // In minutes
}

impl From<Schedule> for Calendar {
    fn from(sch: Schedule) -> Self {
        let evts_iter = (0..90)
            .map(|n| MinchaTime::new(&sch, n))
            // @todo exclude Friday and Shabbos
            // .filter(|mt| ![Friday, Saturday].contains(mt.time.day))
            .map(Event::from);

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

    /// Helper function to build a date quickly
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
        let sch = Schedule {
            lat_long: LatLong::new(43.73, -79.44), // North York, Ontario
            minutes: 40,                           // 40 minutes after shkiyah in Bobov
            prep_time: 5,
        };
        let tz = Tz::named("America/Toronto").unwrap();

        let cal = Calendar::from(sch);
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

fn main() {
    let sch = Schedule {
        minutes: 40,
        lat_long: LatLong::new(43.73, -79.44),
        prep_time: 5,
    };

    let cal = Calendar::from(sch);

    println!("{cal}");
}
// @TODO replace expcts with result bubbling
