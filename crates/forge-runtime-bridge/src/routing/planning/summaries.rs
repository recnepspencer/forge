use std::sync::Arc;

use crate::input::envelope::{
    BridgeCommittedPatchDigest, BridgeProducerMetadata, TruthCommitIdentity, TruthPatchIdentity,
};
use crate::snapshot::TruthSnapshotIdentity;

use super::BridgeRouteIdentity;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeRouteSourceSummary {
    source_commit: TruthCommitIdentity,
    source_patch: TruthPatchIdentity,
    source_snapshot: TruthSnapshotIdentity,
}

impl BridgeRouteSourceSummary {
    pub(crate) fn new(
        source_commit: TruthCommitIdentity,
        source_patch: TruthPatchIdentity,
        source_snapshot: TruthSnapshotIdentity,
    ) -> Self {
        Self {
            source_commit,
            source_patch,
            source_snapshot,
        }
    }

    pub fn source_commit(&self) -> &TruthCommitIdentity {
        &self.source_commit
    }

    pub fn source_patch(&self) -> &TruthPatchIdentity {
        &self.source_patch
    }

    pub fn source_snapshot(&self) -> &TruthSnapshotIdentity {
        &self.source_snapshot
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeExecutionCounts {
    invalidation_target_count: usize,
    subscription_slice_count: usize,
    snapshot_read_count: usize,
}

impl BridgeExecutionCounts {
    pub(crate) fn new(
        invalidation_target_count: usize,
        subscription_slice_count: usize,
        snapshot_read_count: usize,
    ) -> Self {
        Self {
            invalidation_target_count,
            subscription_slice_count,
            snapshot_read_count,
        }
    }

    pub fn invalidation_target_count(&self) -> usize {
        self.invalidation_target_count
    }

    pub fn subscription_slice_count(&self) -> usize {
        self.subscription_slice_count
    }

    pub fn snapshot_read_count(&self) -> usize {
        self.snapshot_read_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgePlanningProvenance {
    route_identity: BridgeRouteIdentity,
    source_digest: BridgeCommittedPatchDigest,
    digest: Arc<str>,
}

impl BridgePlanningProvenance {
    pub(crate) fn new(
        route_identity: BridgeRouteIdentity,
        source_digest: BridgeCommittedPatchDigest,
        digest: Arc<str>,
    ) -> Self {
        Self {
            route_identity,
            source_digest,
            digest,
        }
    }

    pub fn route_identity(&self) -> &BridgeRouteIdentity {
        &self.route_identity
    }

    pub fn source_digest(&self) -> &BridgeCommittedPatchDigest {
        &self.source_digest
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgePlanningSummary {
    route_identity: BridgeRouteIdentity,
    routing_entry_count: usize,
    execution_counts: BridgeExecutionCounts,
    digest: Arc<str>,
}

impl BridgePlanningSummary {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        route_identity: BridgeRouteIdentity,
        routing_entry_count: usize,
        execution_counts: BridgeExecutionCounts,
        digest: Arc<str>,
    ) -> Self {
        Self {
            route_identity,
            routing_entry_count,
            execution_counts,
            digest,
        }
    }

    pub fn route_identity(&self) -> &BridgeRouteIdentity {
        &self.route_identity
    }

    pub fn routing_entry_count(&self) -> usize {
        self.routing_entry_count
    }

    pub fn invalidation_target_count(&self) -> usize {
        self.execution_counts.invalidation_target_count()
    }

    pub fn subscription_slice_count(&self) -> usize {
        self.execution_counts.subscription_slice_count()
    }

    pub fn snapshot_read_count(&self) -> usize {
        self.execution_counts.snapshot_read_count()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeRoutingSummary {
    route_identity: BridgeRouteIdentity,
    source: BridgeRouteSourceSummary,
    producer_metadata: BridgeProducerMetadata,
    routing_entry_count: usize,
    invalidation_target_count: usize,
}

impl BridgeRoutingSummary {
    pub(crate) fn new(
        route_identity: BridgeRouteIdentity,
        source: BridgeRouteSourceSummary,
        producer_metadata: BridgeProducerMetadata,
        routing_entry_count: usize,
        invalidation_target_count: usize,
    ) -> Self {
        Self {
            route_identity,
            source,
            producer_metadata,
            routing_entry_count,
            invalidation_target_count,
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

    pub fn producer_metadata(&self) -> &BridgeProducerMetadata {
        &self.producer_metadata
    }

    pub fn routing_entry_count(&self) -> usize {
        self.routing_entry_count
    }

    pub fn invalidation_target_count(&self) -> usize {
        self.invalidation_target_count
    }
}
