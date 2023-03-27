use std::path::PathBuf;

use icalendar::Calendar;
use rocket::{data::ByteUnit, Data, Route};

use crate::mincha_minder::Schedule;

fn schedule() -> Calendar {
    let sch = Schedule::new(40, 43.73, -79.44, 5);

    Calendar::from(sch)
}

#[route(GET, uri = "/<params..>", data = "<input>")]
async fn calendar(params: PathBuf, input: Data<'_>) -> String {
    let data = input.open(ByteUnit::kB).into_string().await;

    println!("PARAMS: {params:?}, DATA: {data:?}");

    schedule().to_string()
}

pub fn routes() -> Vec<Route> {
    routes![calendar]
}
