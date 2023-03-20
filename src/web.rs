#[get("/")]
fn landing() {
    Template::render(
        "landing",
        context! {
            register_url: "/register",
            login_url: "/login",
        },
    );
}

get /schedule/new

post /schedule/new

get /schedule/<id>

put /schedule/<id>
