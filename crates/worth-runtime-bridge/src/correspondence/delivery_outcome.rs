use std::sync::Arc;

use crate::facade::{
    BridgeCommittedRecordChange, BridgeSemanticAspectChange, RelationalBridgeRecordIdentityParts,
    TruthBranchIdentity, TruthCommitIdentity, TruthPatchIdentity, TruthSnapshotIdentity,
};

use super::{
    BridgeAdmittedTruthCommitIdentity, BridgeAdmittedTruthRecordIdentity,
    BridgeAdmittedTruthSnapshotIdentity, BridgeCorrespondenceBasis, BridgeCorrespondenceDenialKind,
    BridgeSemanticDependencyCandidate,
};

/// One authoritative lower-runtime change actually admitted by an installed
/// semantic correspondence. Construction remains Bridge-owned so consumers
/// cannot promote a raw envelope item into delivered-change authority.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeDeliveredCorrespondenceChange {
    inner: BridgeDeliveredCorrespondenceChangeInner,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum BridgeDeliveredCorrespondenceChangeInner {
    SemanticAspect {
        entity_identity: Arc<str>,
        relational_record_identity: Option<RelationalBridgeRecordIdentityParts>,
        change: BridgeSemanticAspectChange,
    },
    StructuralRecord(BridgeCommittedRecordChange),
}

impl BridgeDeliveredCorrespondenceChange {
    pub(crate) fn semantic_aspect(
        entity_identity: Arc<str>,
        relational_record_identity: Option<RelationalBridgeRecordIdentityParts>,
        change: BridgeSemanticAspectChange,
    ) -> Self {
        Self {
            inner: BridgeDeliveredCorrespondenceChangeInner::SemanticAspect {
                entity_identity,
                relational_record_identity,
                change,
            },
        }
    }

    pub(crate) fn structural_record(change: BridgeCommittedRecordChange) -> Self {
        Self {
            inner: BridgeDeliveredCorrespondenceChangeInner::StructuralRecord(change),
        }
    }

    pub fn semantic_change(&self) -> Option<&BridgeSemanticAspectChange> {
        match &self.inner {
            BridgeDeliveredCorrespondenceChangeInner::SemanticAspect { change, .. } => Some(change),
            BridgeDeliveredCorrespondenceChangeInner::StructuralRecord(_) => None,
        }
    }

    pub fn structural_change(&self) -> Option<&BridgeCommittedRecordChange> {
        match &self.inner {
            BridgeDeliveredCorrespondenceChangeInner::SemanticAspect { .. } => None,
            BridgeDeliveredCorrespondenceChangeInner::StructuralRecord(change) => Some(change),
        }
    }

    pub fn entity_identity(&self) -> Option<&str> {
        match &self.inner {
            BridgeDeliveredCorrespondenceChangeInner::SemanticAspect {
                entity_identity, ..
            } => Some(entity_identity),
            BridgeDeliveredCorrespondenceChangeInner::StructuralRecord(_) => None,
        }
    }

    pub fn relational_record_identity(&self) -> Option<RelationalBridgeRecordIdentityParts> {
        match &self.inner {
            BridgeDeliveredCorrespondenceChangeInner::SemanticAspect {
                relational_record_identity,
                ..
            } => *relational_record_identity,
            BridgeDeliveredCorrespondenceChangeInner::StructuralRecord(change) => {
                Some(change.record_identity())
            }
        }
    }

    pub fn admitted_record_identity(&self) -> Option<BridgeAdmittedTruthRecordIdentity> {
        self.relational_record_identity()
            .map(BridgeAdmittedTruthRecordIdentity::admit)
    }
}

/// The exact owner-delivered change set after correspondence admission and
/// Signal mutation. This preserves the Bridge decision instead of asking Query
/// to reinterpret the committed envelope.
#[derive(Debug, Clone)]
pub struct BridgeDeliveredCorrespondenceChangeSet {
    basis: BridgeCorrespondenceBasis,
    dependency: BridgeSemanticDependencyCandidate,
    commit_identity: BridgeAdmittedTruthCommitIdentity,
    patch_identity: TruthPatchIdentity,
    snapshot_identity: BridgeAdmittedTruthSnapshotIdentity,
    branch_identity: TruthBranchIdentity,
    changes: Vec<BridgeDeliveredCorrespondenceChange>,
}

impl BridgeDeliveredCorrespondenceChangeSet {
    pub(crate) fn new(
        basis: BridgeCorrespondenceBasis,
        dependency: BridgeSemanticDependencyCandidate,
        envelope: &crate::input::envelope::BridgeCommittedPatchEnvelope,
        changes: Vec<BridgeDeliveredCorrespondenceChange>,
    ) -> Self {
        Self {
            basis,
            dependency,
            commit_identity: BridgeAdmittedTruthCommitIdentity::admit(
                envelope.commit_identity().clone(),
            ),
            patch_identity: envelope.patch_identity().clone(),
            snapshot_identity: BridgeAdmittedTruthSnapshotIdentity::admit(
                envelope.snapshot_identity().clone(),
            ),
            branch_identity: envelope.branch_identity().clone(),
            changes,
        }
    }

    pub fn basis(&self) -> &BridgeCorrespondenceBasis {
        &self.basis
    }

    pub fn dependency(&self) -> &BridgeSemanticDependencyCandidate {
        &self.dependency
    }

    pub fn commit_identity(&self) -> &TruthCommitIdentity {
        self.commit_identity.projection()
    }

    pub fn admitted_commit_identity(&self) -> &BridgeAdmittedTruthCommitIdentity {
        &self.commit_identity
    }

    pub fn patch_identity(&self) -> &TruthPatchIdentity {
        &self.patch_identity
    }

    pub fn snapshot_identity(&self) -> &TruthSnapshotIdentity {
        self.snapshot_identity.projection()
    }

    pub fn admitted_snapshot_identity(&self) -> &BridgeAdmittedTruthSnapshotIdentity {
        &self.snapshot_identity
    }

    pub fn retains_delivery_identity(
        &self,
        commit_identity: &TruthCommitIdentity,
        snapshot_identity: &TruthSnapshotIdentity,
    ) -> bool {
        self.commit_identity.projection() == commit_identity
            && self.snapshot_identity.projection() == snapshot_identity
    }

    pub fn branch_identity(&self) -> &TruthBranchIdentity {
        &self.branch_identity
    }

    pub fn changes(&self) -> &[BridgeDeliveredCorrespondenceChange] {
        &self.changes
    }
}

#[derive(Debug, Clone)]
pub struct BridgeCorrespondenceDeliveryReceipt {
    counters: CorrespondenceDeliveryCounters,
    change_set: BridgeDeliveredCorrespondenceChangeSet,
}

impl BridgeCorrespondenceDeliveryReceipt {
    pub(crate) const fn new(
        counters: CorrespondenceDeliveryCounters,
        change_set: BridgeDeliveredCorrespondenceChangeSet,
    ) -> Self {
        Self {
            counters,
            change_set,
        }
    }

    pub const fn counters(&self) -> CorrespondenceDeliveryCounters {
        self.counters
    }

    pub const fn change_set(&self) -> &BridgeDeliveredCorrespondenceChangeSet {
        &self.change_set
    }

    pub const fn truth_targets_admitted(&self) -> usize {
        self.counters.truth_targets_admitted()
    }

    pub const fn source_load_attempts(&self) -> usize {
        self.counters.source_load_attempts()
    }

    pub const fn source_envelopes_loaded(&self) -> usize {
        self.counters.source_envelopes_loaded()
    }

    pub const fn allocation_registry_lock_attempts(&self) -> usize {
        self.counters.allocation_registry_lock_attempts()
    }

    pub const fn signal_capability_admissions(&self) -> usize {
        self.counters.signal_capability_admissions()
    }

    pub const fn failed_deliveries(&self) -> usize {
        self.counters.failed_deliveries()
    }

    pub const fn correspondence_lookups(&self) -> usize {
        self.counters.correspondence_lookups()
    }

    pub const fn signal_seeds_emitted(&self) -> usize {
        self.counters.signal_seeds_emitted()
    }

    pub const fn node_fan_out(&self) -> usize {
        self.counters.node_fan_out()
    }

    pub const fn slots_touched(&self) -> usize {
        self.counters.slots_touched()
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CorrespondenceDeliveryCounters {
    pub(crate) source_load_attempts: usize,
    pub(crate) source_envelopes_loaded: usize,
    pub(crate) allocation_registry_lock_attempts: usize,
    pub(crate) allocation_source_set_checks: usize,
    pub(crate) signal_basis_target_checks: usize,
    pub(crate) signal_capability_admissions: usize,
    pub(crate) failed_deliveries: usize,
    pub(crate) truth_targets_admitted: usize,
    pub(crate) correspondence_lookups: usize,
    pub(crate) semantic_match_checks: usize,
    pub(crate) relevant_change_checks: usize,
    pub(crate) projection_paths_inspected: usize,
    pub(crate) source_widening_target_checks: usize,
    pub(crate) signal_seeds_emitted: usize,
    pub(crate) node_fan_out: usize,
    pub(crate) slots_touched: usize,
}

impl CorrespondenceDeliveryCounters {
    pub const fn zero() -> Self {
        Self {
            source_load_attempts: 0,
            source_envelopes_loaded: 0,
            allocation_registry_lock_attempts: 0,
            allocation_source_set_checks: 0,
            signal_basis_target_checks: 0,
            signal_capability_admissions: 0,
            failed_deliveries: 0,
            truth_targets_admitted: 0,
            correspondence_lookups: 0,
            semantic_match_checks: 0,
            relevant_change_checks: 0,
            projection_paths_inspected: 0,
            source_widening_target_checks: 0,
            signal_seeds_emitted: 0,
            node_fan_out: 0,
            slots_touched: 0,
        }
    }

    pub const fn truth_targets_admitted(self) -> usize {
        self.truth_targets_admitted
    }

    pub const fn source_load_attempts(self) -> usize {
        self.source_load_attempts
    }

    pub const fn source_envelopes_loaded(self) -> usize {
        self.source_envelopes_loaded
    }

    pub const fn allocation_registry_lock_attempts(self) -> usize {
        self.allocation_registry_lock_attempts
    }

    pub const fn allocation_source_set_checks(self) -> usize {
        self.allocation_source_set_checks
    }

    pub const fn signal_basis_target_checks(self) -> usize {
        self.signal_basis_target_checks
    }

    pub const fn signal_capability_admissions(self) -> usize {
        self.signal_capability_admissions
    }

    pub const fn failed_deliveries(self) -> usize {
        self.failed_deliveries
    }

    pub const fn correspondence_lookups(self) -> usize {
        self.correspondence_lookups
    }

    pub const fn semantic_match_checks(self) -> usize {
        self.semantic_match_checks
    }

    pub const fn relevant_change_checks(self) -> usize {
        self.relevant_change_checks
    }

    pub const fn projection_paths_inspected(self) -> usize {
        self.projection_paths_inspected
    }

    pub const fn source_widening_target_checks(self) -> usize {
        self.source_widening_target_checks
    }

    pub const fn signal_seeds_emitted(self) -> usize {
        self.signal_seeds_emitted
    }

    pub const fn node_fan_out(self) -> usize {
        self.node_fan_out
    }

    pub const fn slots_touched(self) -> usize {
        self.slots_touched
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BridgeCorrespondenceDeliveryDenial {
    kind: BridgeCorrespondenceDenialKind,
    counters: CorrespondenceDeliveryCounters,
}

impl BridgeCorrespondenceDeliveryDenial {
    pub(crate) const fn new(
        kind: BridgeCorrespondenceDenialKind,
        counters: CorrespondenceDeliveryCounters,
    ) -> Self {
        Self { kind, counters }
    }

    pub const fn kind(self) -> BridgeCorrespondenceDenialKind {
        self.kind
    }

    pub const fn counters(self) -> CorrespondenceDeliveryCounters {
        self.counters
    }
}
