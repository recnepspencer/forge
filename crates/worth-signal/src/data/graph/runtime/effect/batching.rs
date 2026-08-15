use crate::data::trace::HotArtifactWrite;
use crate::logic::evaluation::{EffectComparison, EvaluationEffect};

use super::super::graph::PreparedDirectCauseAdmission;

#[derive(Debug)]
pub(crate) struct ApplyCommitPacket {
    pub(crate) effect: EvaluationEffect,
    pub(crate) comparison: EffectComparison,
    pub(crate) artifact_write: Option<HotArtifactWrite>,
    pub(crate) pending_snapshot: Option<crate::logic::evaluation::PendingDependencySnapshot>,
    pub(crate) defer_snapshot_commit: bool,
}

#[derive(Debug)]
pub(crate) struct OutputCommitPacket {
    pub(crate) apply: ApplyCommitPacket,
    pub(crate) prepared_direct:
        Option<crate::data::proof::invalidation::progression::PreparedDirectInvalidation>,
    pub(crate) direct_causes: Option<PreparedDirectCauseAdmission>,
}

#[cfg_attr(not(feature = "parallel"), allow(dead_code))]
#[derive(Debug)]
pub(crate) struct PreparedParallelApplyCommitPacket(pub(super) ApplyCommitPacket);

impl From<ApplyCommitPacket> for PreparedParallelApplyCommitPacket {
    fn from(packet: ApplyCommitPacket) -> Self {
        Self(packet)
    }
}
