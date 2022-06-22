use crate::healthcheck::Status;
use anyhow::Result;
use rocket::serde::{Deserialize, Serialize};

pub trait Liveness {
    fn is_alive(&self) -> Result<LivenessDetails>;
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct LivenessDetails {
    pub status: Status,
}

impl LivenessDetails {
    pub fn new(status: Status) -> Self {
        Self { status }
    }
}
