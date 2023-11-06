mod mincha_minder;
use mincha_minder::Schedule;

fn main() {
    let cal = Schedule::new(40, 43.73, -79.44, 5).to_ical();
    println!("{cal}");
}
