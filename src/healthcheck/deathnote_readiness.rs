use crate::healthcheck::{
    readiness::{Readiness, ReadinessDetails},
    Status,
};

use anyhow::Result;

use once_cell::sync::Lazy;
use std::sync::Arc;

static STATIC_INSTANCE: Lazy<arc_swap::ArcSwap<DeathNoteReadiness>> =
    Lazy::new(|| arc_swap::ArcSwap::from_pointee(DeathNoteReadiness::default()));

const DEPENDENCY_DB: &str = "db";
const DEPENDENCY_GITHUB: &str = "github";
const DEPENDENCY_STARKNET: &str = "starknet";

#[derive(Default)]
pub struct DeathNoteReadiness {}

pub fn instance() -> Arc<DeathNoteReadiness> {
    STATIC_INSTANCE.load().clone()
}

impl Readiness for DeathNoteReadiness {
    fn is_ready(&self) -> Result<ReadinessDetails> {
        let mut readiness_details = ReadinessDetails::default();
        self.check_dependency(
            DEPENDENCY_DB.to_string(),
            &mut readiness_details,
            Self::healthcheck_db,
        );
        self.check_dependency(
            DEPENDENCY_GITHUB.to_string(),
            &mut readiness_details,
            Self::healthcheck_github,
        );
        self.check_dependency(
            DEPENDENCY_STARKNET.to_string(),
            &mut readiness_details,
            Self::healthcheck_starknet,
        );

        Ok(readiness_details)
    }
}

impl DeathNoteReadiness {
    fn check_dependency(
        &self,
        dependency_name: String,
        readiness_details: &mut ReadinessDetails,
        healthcheck_fn: fn(&Self) -> Status,
    ) {
        let status = healthcheck_fn(self);
        if let Status::Down = status {
            readiness_details.global_status = Status::Down;
        }
        readiness_details
            .dependencies
            .insert(dependency_name, status);
    }

    fn healthcheck_db(&self) -> Status {
        Status::Down
    }
    fn healthcheck_github(&self) -> Status {
        Status::Down
    }
    fn healthcheck_starknet(&self) -> Status {
        Status::Down
    }
}
