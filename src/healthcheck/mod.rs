use crate::healthcheck::{
    liveness::LivenessDetails,
    readiness::{Readiness, ReadinessDetails},
};
use rocket::get;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};

pub mod deathnote_readiness;
pub mod liveness;
pub mod mock;
pub mod readiness;

#[get("/liveness")]
pub fn get_liveness() -> Json<LivenessDetails> {
    Json(LivenessDetails::new(Status::Up))
}

#[get("/readiness")]
pub fn get_readiness() -> Json<ReadinessDetails> {
    let readiness_service = deathnote_readiness::instance();
    let readiness_result = readiness_service.is_ready();
    Json(readiness_result.unwrap())
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub enum Status {
    Up,
    Down,
}

impl Default for Status {
    fn default() -> Self {
        Status::Up
    }
}
