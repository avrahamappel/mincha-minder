#[macro_use]
extern crate rocket;

mod caldav;
mod mincha_minder;

#[launch]
fn rocket_main() -> _ {
    rocket::build().mount("/caldav", caldav::routes())
}
