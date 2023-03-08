use chrono::{Datelike, Duration, NaiveDate, NaiveDateTime, Utc};
use icalendar::{Calendar, Component, Event, EventLike};
use sunrise::sunrise_sunset;

const DEFAULT_MINCHA_DURATION: i64 = 20;

#[derive(Clone, Copy)]
struct Schedule {
    // Minutes before (negative) or after (positive) sunset
    minutes: i32,
    lat_long: (f64, f64),
    prep_time: u32, // In minutes
}

impl From<Schedule> for Calendar {
    fn from(sch: Schedule) -> Self {
        let mut cal = Calendar::new();

        for evt in (0..90).map(|n| Event::from(MinchaTime::new(sch, n))) {
            cal.push(evt);
        }

        cal
    }
}

#[derive(Clone, Copy, Debug)]
struct Sunset(i64);

impl Sunset {
    fn new((lat, long): (f64, f64), date: NaiveDate) -> Self {
        let (_, sunset) = sunrise_sunset(lat, long, date.year(), date.month(), date.day());

        Self(sunset)
    }
}

impl From<Sunset> for NaiveDateTime {
    fn from(sunset: Sunset) -> Self {
        NaiveDateTime::from_timestamp_opt(sunset.0, 0)
            .expect("Error creating time from sunset value")
    }
}

#[derive(Clone, Copy)]
struct MinchaTime {
    time: NaiveDateTime,
}

impl MinchaTime {
    fn new(sch: Schedule, days_from_today: u32) -> Self {
        let date = Utc::now().date_naive() + Duration::days(days_from_today.into());

        let sunset = Sunset::new(sch.lat_long, date);

        let time = NaiveDateTime::from(sunset) + Duration::minutes(sch.minutes.into())
            - Duration::minutes(sch.prep_time.into());

        Self { time }
    }
}

impl From<MinchaTime> for Event {
    fn from(mt: MinchaTime) -> Self {
        Event::new()
            .summary("Mincha")
            .starts(mt.time)
            .ends(mt.time + Duration::minutes(DEFAULT_MINCHA_DURATION))
            .done()
    }
}

fn main() {
    let sch = Schedule {
        minutes: 40,
        lat_long: (43.76, -79.41),
        prep_time: 5,
    };

    let cal = Calendar::from(sch);

    println!("{}", cal);
}
