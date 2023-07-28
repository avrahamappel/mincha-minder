use std::net::SocketAddr;
use std::str::FromStr;

use warp::Filter;

// mod caldav;
// mod mincha_minder;

#[tokio::main]
async fn main() {
    // Cheap debugging
    let log = warp::log::custom(|info| {
        eprintln!("{} {} {}", info.method(), info.path(), info.status(),);
    });

    warp::serve(
        warp::get()
            .and(warp::path::param())
            .and(warp::header("user-agent"))
            .map(|param: String, user_agent: String| format!("Hello {param} and {user_agent}"))
            .with(log),
    )
    .run(SocketAddr::from_str("127.0.0.1:8000").unwrap())
    .await
}
