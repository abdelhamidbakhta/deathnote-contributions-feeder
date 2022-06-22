#[macro_use]
extern crate rocket;
use deathnote_contributions_feeder::healthcheck;

#[get("/")]
fn index() -> &'static str {
    "Death Note Contributions Feeder REST API is running."
}

#[launch]
fn rocket() -> _ {
    env_logger::init();
    rocket::build().mount(
        "/",
        routes![index, healthcheck::get_liveness, healthcheck::get_readiness],
    )
}
