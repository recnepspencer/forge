use std::sync::Arc;

use crate::input::envelope::{TruthCommitIdentity, TruthPatchIdentity};
use crate::routing::{BridgeRouteIdentity, BridgeRoutingCounters};
use crate::snapshot::TruthSnapshotIdentity;

use super::route_entry::BridgeRouteRecordEntry;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeRouteSourceRecord {
    source_commit: TruthCommitIdentity,
    source_patch: TruthPatchIdentity,
    source_snapshot: TruthSnapshotIdentity,
}

impl BridgeRouteSourceRecord {
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
pub struct BridgeRoutingDiagnosticsRecord {
    route_identity: BridgeRouteIdentity,
    entries: Arc<[BridgeRouteRecordEntry]>,
    counters: BridgeRoutingCounters,
}

impl BridgeRoutingDiagnosticsRecord {
    pub(crate) fn new(
        route_identity: BridgeRouteIdentity,
        entries: Arc<[BridgeRouteRecordEntry]>,
        counters: BridgeRoutingCounters,
    ) -> Self {
        Self {
            route_identity,
            entries,
            counters,
        }
    }

    pub fn route_identity(&self) -> &BridgeRouteIdentity {
        &self.route_identity
    }

    pub fn entries(&self) -> &[BridgeRouteRecordEntry] {
        &self.entries
    }

    pub fn counters(&self) -> &BridgeRoutingCounters {
        &self.counters
    }
}
