use std::sync::Arc;

use worth_signal::facade::{ClockTick, TemporalWakeId};
use sha2::{Digest, Sha256};

use crate::subscription::BridgeSubscriptionCounters;

use super::BridgeTemporalRoutingLaneKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeTemporalWakeRoutingRejectionKind {
    DuplicateWakeSubmission,
    StaleWakeSubmission,
    RoutingLaneMismatch,
    TruthPatchBranchIdentityMismatch,
    TruthPatchSnapshotIdentityMismatch,
}

impl BridgeTemporalWakeRoutingRejectionKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DuplicateWakeSubmission => "duplicate_wake_submission",
            Self::StaleWakeSubmission => "stale_wake_submission",
            Self::RoutingLaneMismatch => "routing_lane_mismatch",
            Self::TruthPatchBranchIdentityMismatch => "truth_patch_branch_identity_mismatch",
            Self::TruthPatchSnapshotIdentityMismatch => "truth_patch_snapshot_identity_mismatch",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeTemporalWakeRoutingRejection {
    rejection_kind: BridgeTemporalWakeRoutingRejectionKind,
    routing_lane_kind: BridgeTemporalRoutingLaneKind,
    wake_id: TemporalWakeId,
    wake_tick: ClockTick,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeTemporalWakeRoutingRejection {
    pub(crate) fn new(
        rejection_kind: BridgeTemporalWakeRoutingRejectionKind,
        routing_lane_kind: BridgeTemporalRoutingLaneKind,
        activation_lane_identity: &str,
        temporal_basis_identity: &str,
        wake_id: TemporalWakeId,
        wake_tick: ClockTick,
        truth_patch_identity: Option<&str>,
    ) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-temporal-wake-routing-rejection|kind={}|lane={}|activation={}|temporal-basis={}|wake-id={}|wake-tick={}|truth-patch={}",
            rejection_kind.as_str(),
            routing_lane_kind.as_str(),
            activation_lane_identity,
            temporal_basis_identity,
            wake_id.get(),
            wake_tick.get(),
            truth_patch_identity.unwrap_or("none"),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            rejection_kind,
            routing_lane_kind,
            wake_id,
            wake_tick,
            counters: match rejection_kind {
                BridgeTemporalWakeRoutingRejectionKind::DuplicateWakeSubmission => {
                    BridgeSubscriptionCounters::from_temporal_duplicate_clock_rejection()
                }
                BridgeTemporalWakeRoutingRejectionKind::StaleWakeSubmission => {
                    BridgeSubscriptionCounters::from_temporal_stale_clock_rejection()
                }
                _ => BridgeSubscriptionCounters::from_temporal_subscription_rejection(),
            },
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-temporal-wake-routing-rejection:sha256:{digest:x}"
            )),
        }
    }

    pub fn rejection_kind(&self) -> BridgeTemporalWakeRoutingRejectionKind {
        self.rejection_kind
    }

    pub fn routing_lane_kind(&self) -> BridgeTemporalRoutingLaneKind {
        self.routing_lane_kind
    }

    pub const fn wake_id(&self) -> TemporalWakeId {
        self.wake_id
    }

    pub const fn wake_tick(&self) -> ClockTick {
        self.wake_tick
    }

    pub fn counters(&self) -> &BridgeSubscriptionCounters {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
