use crate::healthcheck::Status;
use crate::healthcheck::{
    liveness::{Liveness, LivenessDetails},
    readiness::{Readiness, ReadinessDetails},
};
use anyhow::Result;

pub struct AlwaysUpLiveness {}

impl Liveness for AlwaysUpLiveness {
    fn is_alive(&self) -> Result<LivenessDetails> {
        Ok(LivenessDetails::new(Status::Up))
    }
}

pub struct AlwaysUpReadiness {}

impl Readiness for AlwaysUpReadiness {
    fn is_ready(&self) -> Result<ReadinessDetails> {
        Ok(ReadinessDetails::new(Status::Up))
    }
}
