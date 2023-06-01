// use std::path::PathBuf;
// use std::convert::Infallible;

// use icalendar::Calendar;
// use warp::Filter;

// use crate::mincha_minder::Schedule;

// fn schedule() -> Calendar {
//     let sch = Schedule::new(40, 43.73, -79.44, 5);

//     Calendar::from(sch)
// }

// // #[route(GET, uri = "/<params..>", data = "<input>")]
// // async fn calendar(params: PathBuf, input: Data<'_>) -> String {
// //     let data = input.open(ByteUnit::kB).into_string().await;

// //     println!("PARAMS: {params:?}, DATA: {data:?}");

// //     schedule().to_string()
// // }

// pub fn routes() -> impl Filter {
//     warp::path!("/caldav")
//         .and(warp::filters::path::full())
//         .and(warp::filters::method::method())
//         .map(|path, method| {
//             println!("PATH: {path:?}, METHOD: {method}");

//             schedule().to_string()
//         })
// }

#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::http::{ContentType, Method};
use rocket::request::{FromRequest, Outcome};
use rocket::response::content::{Content, Plain};
use rocket::response::status::Custom;
use rocket::{Data, Request, State};

#[get("/")]
fn propfind() -> Content<String> {
    Content(ContentType::XML, "<propfind>...</propfind>".to_string())
}

#[options("/")]
fn options() -> &'static str {
    "OPTIONS handler"
}

#[get("/calendars/<id>")]
fn get_calendar(id: String) -> &'static str {
    // Retrieve calendar data and return it
    "GET calendar handler"
}

#[put("/calendars/<id>", data = "<calendar>")]
fn put_calendar(id: String, calendar: Data<'_, Plain>) -> Result<&'static str, Custom<String>> {
    // Handle the calendar data and store it
    Ok("PUT calendar handler")
}

#[delete("/calendars/<id>")]
fn delete_calendar(id: String) -> Result<&'static str, Custom<String>> {
    // Delete the specified calendar
    Ok("DELETE calendar handler")
}

#[report("/calendars/<id>", data = "<report>")]
fn report_calendar(id: String, report: Data<'_, Plain>) -> Result<Content<String>, Custom<String>> {
    // Process the report data and generate a response
    let response_body = "<report>...</report>".to_string();
    let content_type = ContentType::XML;
    Ok(Content(content_type, response_body))
}

#[catch(405)]
fn method_not_allowed(request: &Request) -> Content<String> {
    match request.method {
        "PROPFIND" => propfind(request),
        "REPORT" => report_calendar(request),
        _ => Content(
            ContentType::XML,
            "<error>Method not allowed</error>".to_string(),
        ),
    }
}

#[catch(404)]
fn not_found(_: &Request) -> Content<String> {
    Content(ContentType::XML, "<error>Not found</error>".to_string())
}

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![
                propfind,
                options,
                get_calendar,
                put_calendar,
                delete_calendar,
                report_calendar
            ],
        )
        .register(catchers![method_not_allowed, not_found])
        .launch();
}
