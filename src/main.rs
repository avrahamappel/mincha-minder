use chrono::Utc;
use icalendar::{Calendar, Component, Event, EventLike};

enum Time {
    Fixed(u32),      // Minute of the day
    WithSunset(i32), // Minutes before (negative) or after (positive) sunset
}

struct Schedule {
    tz: TimeZone,
    time: Time,
    prep_time: u32, // In minutes
}

fn make_cal(sch: &Schedule) -> Calendar {
    let events = match sch.time {
        Time::Fixed(n) => {
            let start_time = Utc::today(sch.tz).add_minutes(n);
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
