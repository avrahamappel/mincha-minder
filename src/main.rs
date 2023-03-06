use chrono::{FixedOffset, Local, NaiveDate, NaiveTime, TimeZone, Utc};
use icalendar::{Calendar, Component, Event, EventLike};

const DEFAULT_MINCHA_DURATION: u32 = 20;

enum Time {
    Fixed(u32),      // Minute of the day
    WithSunset(i32), // Minutes before (negative) or after (positive) sunset
}

struct Schedule {
    tz: i32,
    time: Time,
    prep_time: u32, // In minutes
}

fn make_cal(sch: &Schedule) -> Calendar {
    let events = match sch.time {
        Time::Fixed(n) => {
            let start_time = Local::now()
                .date_naive()
                .and_time(
                    NaiveTime::from_num_seconds_from_midnight_opt(n * 60, 0)
                        .expect("Bad number of seconds since midnight"),
                )
                .and_local_timezone(
                    FixedOffset::east_opt(sch.tz * 100).expect("Invalid tz offset"),
                );

            // Extract common event params, like summary
            let evt = Event::new()
                .summary("Mincha")
                .starts(start_time)
                .ends(start_time.add_minutes(DEFAULT_MINCHA_DURATION))
                .repeats(weekly, 1..=5)
                .done();
            vec![evt]
        }
        Time::WithSunset(n) => (0..90).map(|i| todo!()).collect(),
    };

    let mut cal = Calendar::new();

    for event in events {
        cal.push(event);
    }

    cal
}

fn main() {
    println!("Hello, world!");
}
