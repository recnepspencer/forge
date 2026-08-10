use super::super::failure::{
    SupportTrustFailure, SupportTrustFailureKind, SupportTrustRecoveryPosture,
};
use super::certification_validation::require_non_empty;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportCertificationLaneDigestSet {
    control_lane_digest: String,
    hostile_lane_digest: String,
    rebuild_or_replay_lane_digest: String,
}

impl SupportCertificationLaneDigestSet {
    pub fn new(
        control_lane_digest: impl Into<String>,
        hostile_lane_digest: impl Into<String>,
        rebuild_or_replay_lane_digest: impl Into<String>,
    ) -> Result<Self, SupportTrustFailure> {
        let control_lane_digest = require_non_empty("control lane digest", control_lane_digest)?;
        let hostile_lane_digest = require_non_empty("hostile lane digest", hostile_lane_digest)?;
        let rebuild_or_replay_lane_digest = require_non_empty(
            "rebuild or replay lane digest",
            rebuild_or_replay_lane_digest,
        )?;
        if control_lane_digest == hostile_lane_digest
            || control_lane_digest == rebuild_or_replay_lane_digest
            || hostile_lane_digest == rebuild_or_replay_lane_digest
        {
            return Err(SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustCoverageMissing,
                SupportTrustRecoveryPosture::RerunCertification,
                "support trust certification rows cannot compare a lane to itself",
            ));
        }
        Ok(Self {
            control_lane_digest,
            hostile_lane_digest,
            rebuild_or_replay_lane_digest,
        })
    }

    pub fn control_lane_digest(&self) -> &str {
        &self.control_lane_digest
    }

    pub fn hostile_lane_digest(&self) -> &str {
        &self.hostile_lane_digest
    }

    pub fn rebuild_or_replay_lane_digest(&self) -> &str {
        &self.rebuild_or_replay_lane_digest
    }
}
