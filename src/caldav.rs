// use std::convert::Infallible;
// use std::path::PathBuf;

use icalendar::Calendar;
use warp::Filter;

use crate::mincha_minder::Schedule;

fn schedule() -> Calendar {
    let sch = Schedule::new(40, 43.73, -79.44, 5);

    Calendar::from(sch)
}

pub fn routes() -> impl Filter {
    warp::path("caldav")
        .and(warp::filters::any::any())
        .and(warp::filters::path::full())
        .and(warp::filters::method::method())
        .map(|path, method| {
            println!("PATH: {path:?}, METHOD: {method}");

            schedule().to_string()
        })
}
