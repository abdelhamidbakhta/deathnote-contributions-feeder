use crate::healthcheck::Status;
use anyhow::Result;
use rocket::serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(crate = "rocket::serde")]
pub struct ReadinessDetails {
    pub global_status: Status,
    pub dependencies: HashMap<String, Status>,
}

pub trait Readiness {
    fn is_ready(&self) -> Result<ReadinessDetails>;
}

impl ReadinessDetails {
    pub fn new(global_status: Status) -> Self {
        Self {
            global_status,
            dependencies: HashMap::new(),
        }
    }
}
