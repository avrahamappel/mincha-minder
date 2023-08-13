use std::net::SocketAddr;
use std::str::FromStr;

use icalendar::Calendar;
use warp::Filter;

// mod caldav;
mod mincha_minder;
use mincha_minder::Schedule;

fn schedule() -> Calendar {
    let sch = Schedule::new(40, 43.73, -79.44, 5);

    Calendar::from(sch)
}

#[tokio::main]
async fn main() {
    // Cheap debugging
    let log = warp::log::custom(|info| {
        eprintln!("{} {} {}", info.method(), info.path(), info.status(),);
    });

    warp::serve(
        warp::path("caldav")
            .and(warp::filters::any::any())
            .and(warp::filters::path::full())
            .and(warp::filters::method::method())
            .map(|path, method| {
                println!("PATH: {path:?}, METHOD: {method}");

                schedule().to_string()
            })
            .with(log),
    )
    .run(SocketAddr::from_str("127.0.0.1:8000").unwrap())
    .await
}
