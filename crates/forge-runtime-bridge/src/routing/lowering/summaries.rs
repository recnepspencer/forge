use std::sync::Arc;

use crate::input::envelope::{TruthCommitIdentity, TruthPatchIdentity};
use crate::routing::planning::{BridgeExecutionCounts, BridgePlanningProvenance, BridgeRouteSourceSummary};
use crate::snapshot::TruthSnapshotIdentity;

use crate::routing::BridgeRouteIdentity;
use super::BridgeSubscriptionSliceIdentity;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeLoweringPlanSummary {
    route_identity: BridgeRouteIdentity,
    source: BridgeRouteSourceSummary,
    execution_counts: BridgeExecutionCounts,
    subscription_slice_identity: BridgeSubscriptionSliceIdentity,
}

impl BridgeLoweringPlanSummary {
    pub(crate) fn new(
        route_identity: BridgeRouteIdentity,
        source: BridgeRouteSourceSummary,
        execution_counts: BridgeExecutionCounts,
        subscription_slice_identity: BridgeSubscriptionSliceIdentity,
    ) -> Self {
        Self {
            route_identity,
            source,
            execution_counts,
            subscription_slice_identity,
        }
    }

    pub fn route_identity(&self) -> &BridgeRouteIdentity {
        &self.route_identity
    }

    pub fn source_commit(&self) -> &TruthCommitIdentity {
        self.source.source_commit()
    }

    pub fn source_patch(&self) -> &TruthPatchIdentity {
        self.source.source_patch()
    }

    pub fn source_snapshot(&self) -> &TruthSnapshotIdentity {
        self.source.source_snapshot()
    }

    pub fn invalidation_target_count(&self) -> usize {
        self.execution_counts.invalidation_target_count()
    }

    pub fn subscription_slice_count(&self) -> usize {
        self.execution_counts.subscription_slice_count()
    }

    pub fn subscription_slice_identity(&self) -> &BridgeSubscriptionSliceIdentity {
        &self.subscription_slice_identity
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeLoweringProvenance {
    route_identity: BridgeRouteIdentity,
    planning_provenance: BridgePlanningProvenance,
    digest: Arc<str>,
}

impl BridgeLoweringProvenance {
    pub(crate) fn new(
        route_identity: BridgeRouteIdentity,
        planning_provenance: BridgePlanningProvenance,
        digest: Arc<str>,
    ) -> Self {
        Self {
            route_identity,
            planning_provenance,
            digest,
        }
    }

    pub fn route_identity(&self) -> &BridgeRouteIdentity {
        &self.route_identity
    }

    pub fn planning_provenance(&self) -> &BridgePlanningProvenance {
        &self.planning_provenance
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeLoweringSummary {
    route_identity: BridgeRouteIdentity,
    execution_counts: BridgeExecutionCounts,
    digest: Arc<str>,
}

impl BridgeLoweringSummary {
    pub(crate) fn new(
        route_identity: BridgeRouteIdentity,
        execution_counts: BridgeExecutionCounts,
        digest: Arc<str>,
    ) -> Self {
        Self {
            route_identity,
            execution_counts,
            digest,
        }
    }

    pub fn route_identity(&self) -> &BridgeRouteIdentity {
        &self.route_identity
    }

    pub fn invalidation_target_count(&self) -> usize {
        self.execution_counts.invalidation_target_count()
    }

    pub fn subscription_slice_count(&self) -> usize {
        self.execution_counts.subscription_slice_count()
    }

    pub fn planned_read_count(&self) -> usize {
        self.execution_counts.snapshot_read_count()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
