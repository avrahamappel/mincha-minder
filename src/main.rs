use icalendar::Calendar;

mod mincha_minder;
use mincha_minder::Schedule;

fn schedule() -> Calendar {
    let sch = Schedule::new(40, 43.73, -79.44, 5);

    Calendar::from(sch)
}

fn main() {
    let sch = schedule();
    println!("{sch}");
}
