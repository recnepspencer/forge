use std::sync::Arc;

use crate::input::envelope::{TruthBranchIdentity, TruthCommitIdentity, TruthPatchIdentity};
use crate::routing::{
    BridgeInvalidationIdentity, BridgeInvalidationTarget, BridgeRouteContractProof,
    BridgeRouteIdentity, BridgeSubscriptionSlice, BridgeSubscriptionSliceIdentity,
};
use crate::routing::context::BridgeMappingContext;
use crate::routing::BridgeRoutingCounters;
use crate::snapshot::TruthSnapshotIdentity;

use super::contract::BridgeContractDiagnosticsRecord;
use super::lowering::BridgeLoweringDiagnosticsRecord;
use super::route_entry::BridgeRouteRecordEntry;
use super::routing::{BridgeRouteSourceRecord, BridgeRoutingDiagnosticsRecord};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeRouteRecord {
    source: BridgeRouteSourceRecord,
    routing: BridgeRoutingDiagnosticsRecord,
    lowering: BridgeLoweringDiagnosticsRecord,
    contract: BridgeContractDiagnosticsRecord,
}

impl BridgeRouteRecord {
    pub(crate) fn new(
        route_identity: BridgeRouteIdentity,
        invalidation_identity: BridgeInvalidationIdentity,
        source_branch: TruthBranchIdentity,
        source_commit: TruthCommitIdentity,
        source_patch: TruthPatchIdentity,
        source_snapshot: TruthSnapshotIdentity,
        contract_proof: BridgeRouteContractProof,
        subscription_slice_identity: BridgeSubscriptionSliceIdentity,
        entries: Arc<[BridgeRouteRecordEntry]>,
        subscription_slices: Arc<[BridgeSubscriptionSlice]>,
        invalidation_targets: Arc<[BridgeInvalidationTarget]>,
        counters: BridgeRoutingCounters,
    ) -> Self {
        Self {
            source: BridgeRouteSourceRecord::new(
                source_branch,
                source_commit,
                source_patch,
                source_snapshot,
            ),
            routing: BridgeRoutingDiagnosticsRecord::new(route_identity, entries, counters),
            lowering: BridgeLoweringDiagnosticsRecord::new(
                invalidation_identity,
                subscription_slice_identity,
                subscription_slices,
                invalidation_targets,
            ),
            contract: BridgeContractDiagnosticsRecord::new(contract_proof),
        }
    }

    pub fn route_identity(&self) -> &BridgeRouteIdentity {
        self.routing.route_identity()
    }

    pub fn invalidation_identity(&self) -> &BridgeInvalidationIdentity {
        self.lowering.invalidation_identity()
    }

    pub fn source_commit(&self) -> &TruthCommitIdentity {
        self.source.source_commit()
    }

    pub fn source_branch(&self) -> &TruthBranchIdentity {
        self.source.source_branch()
    }

    pub fn source_patch(&self) -> &TruthPatchIdentity {
        self.source.source_patch()
    }

    pub fn source_snapshot(&self) -> &TruthSnapshotIdentity {
        self.source.source_snapshot()
    }

    pub fn contract_proof(&self) -> &BridgeRouteContractProof {
        self.contract.contract_proof()
    }

    pub fn mapping_context(&self) -> &BridgeMappingContext {
        self.contract.mapping_context()
    }

    pub fn source_digest(&self) -> &crate::input::envelope::BridgeCommittedPatchDigest {
        self.contract_proof().source_digest()
    }

    pub fn planning_provenance_digest(&self) -> &str {
        self.contract_proof().planning_provenance_digest()
    }

    pub fn planning_summary_digest(&self) -> &str {
        self.contract_proof().planning_summary_digest()
    }

    pub fn lowering_provenance_digest(&self) -> &str {
        self.contract_proof().lowering_provenance_digest()
    }

    pub fn lowering_summary_digest(&self) -> &str {
        self.contract_proof().lowering_summary_digest()
    }

    pub fn subscription_slice_identity(&self) -> &BridgeSubscriptionSliceIdentity {
        self.lowering.subscription_slice_identity()
    }

    pub fn entries(&self) -> &[BridgeRouteRecordEntry] {
        self.routing.entries()
    }

    pub fn subscription_slices(&self) -> &[BridgeSubscriptionSlice] {
        self.lowering.subscription_slices()
    }

    pub fn invalidation_targets(&self) -> &[BridgeInvalidationTarget] {
        self.lowering.invalidation_targets()
    }

    pub fn counters(&self) -> &BridgeRoutingCounters {
        self.routing.counters()
    }
}
