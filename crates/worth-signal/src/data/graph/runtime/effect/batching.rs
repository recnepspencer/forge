use crate::data::error::SignalError;
use crate::data::trace::HotArtifactWrite;
use crate::logic::evaluation::{EffectComparison, EvaluationEffect};

#[derive(Debug)]
pub(crate) struct ApplyCommitPacket {
    pub(crate) effect: EvaluationEffect,
    pub(crate) comparison: EffectComparison,
    pub(crate) artifact_write: Option<HotArtifactWrite>,
    pub(crate) pending_snapshot: Option<crate::logic::evaluation::PendingDependencySnapshot>,
    pub(crate) defer_snapshot_commit: bool,
}

#[cfg_attr(not(feature = "parallel"), allow(dead_code))]
#[derive(Debug)]
pub(crate) struct SuppressionFreeApplyCommitPacket(pub(super) ApplyCommitPacket);

impl TryFrom<ApplyCommitPacket> for SuppressionFreeApplyCommitPacket {
    type Error = SignalError;

    fn try_from(packet: ApplyCommitPacket) -> Result<Self, Self::Error> {
        if packet.comparison.propagation_suppressed {
            return Err(SignalError::internal(
                "grouped concurrent commit packet unexpectedly required shared suppression",
            ));
        }
        Ok(Self(packet))
    }
}
