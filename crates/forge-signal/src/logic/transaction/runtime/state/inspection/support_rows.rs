use serde::Serialize;

use crate::logic::transaction::runtime::state::merge::{
    BranchMergeRequestScopeFamily, ScopedMergeCandidateBreadthSummary,
    SignalMergeCompatibilityFactInventory, SignalMergeCompatibilityPostureKind,
    SignalMergeCompatibilityWitness, SignalMergeStrategyWitness,
};
use crate::logic::transaction::runtime::state::{
    SignalBranchBasis, SignalBranchHeadPosture, SignalBranchRestorePosture,
};
use crate::state::{SignalBranchId, SignalSnapshotId};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SignalBranchBasisInspectionRow {
    branch_id: SignalBranchId,
    snapshot_id: Option<SignalSnapshotId>,
    basis_digest: String,
    head_posture: SignalBranchHeadPosture,
    restore_posture: SignalBranchRestorePosture,
}

impl SignalBranchBasisInspectionRow {
    pub(crate) fn from_branch_basis(branch_basis: &SignalBranchBasis) -> Self {
        Self {
            branch_id: branch_basis.branch_id(),
            snapshot_id: branch_basis.snapshot_id(),
            basis_digest: branch_basis.basis_digest().to_owned(),
            head_posture: branch_basis.head_posture().clone(),
            restore_posture: branch_basis.restore_posture().clone(),
        }
    }

    pub fn branch_id(&self) -> SignalBranchId {
        self.branch_id
    }

    pub fn snapshot_id(&self) -> Option<SignalSnapshotId> {
        self.snapshot_id
    }

    pub fn basis_digest(&self) -> &str {
        &self.basis_digest
    }

    pub fn head_posture(&self) -> &SignalBranchHeadPosture {
        &self.head_posture
    }

    pub fn restore_posture(&self) -> &SignalBranchRestorePosture {
        &self.restore_posture
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SignalScopedMergeInspectionRow {
    scope_family: BranchMergeRequestScopeFamily,
    declaration_digest: String,
    admitted_scope_digest: String,
    skipped_scope_digest: Option<String>,
    no_op_scope_digest: Option<String>,
    breadth_summary: ScopedMergeCandidateBreadthSummary,
}

impl SignalScopedMergeInspectionRow {
    pub(crate) fn from_compatibility_facts(
        compatibility: &SignalMergeCompatibilityFactInventory,
    ) -> Self {
        Self {
            scope_family: compatibility.scope_family(),
            declaration_digest: compatibility.declaration_digest().to_owned(),
            admitted_scope_digest: compatibility.admitted_scope_digest().to_owned(),
            skipped_scope_digest: compatibility.skipped_scope_digest().map(ToOwned::to_owned),
            no_op_scope_digest: compatibility.no_op_scope_digest().map(ToOwned::to_owned),
            breadth_summary: compatibility.breadth_summary().clone(),
        }
    }

    pub fn scope_family(&self) -> BranchMergeRequestScopeFamily {
        self.scope_family
    }

    pub fn declaration_digest(&self) -> &str {
        &self.declaration_digest
    }

    pub fn admitted_scope_digest(&self) -> &str {
        &self.admitted_scope_digest
    }

    pub fn skipped_scope_digest(&self) -> Option<&str> {
        self.skipped_scope_digest.as_deref()
    }

    pub fn no_op_scope_digest(&self) -> Option<&str> {
        self.no_op_scope_digest.as_deref()
    }

    pub fn breadth_summary(&self) -> &ScopedMergeCandidateBreadthSummary {
        &self.breadth_summary
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SignalStrategyInspectionRow {
    witness_digest: String,
    merge_strategy_digest: String,
    invalidation_strategy_digest: String,
    delivery_strategy_digest: String,
}

impl SignalStrategyInspectionRow {
    pub(crate) fn from_strategy_witness(strategy_witness: &SignalMergeStrategyWitness) -> Self {
        Self {
            witness_digest: strategy_witness.witness_digest().to_owned(),
            merge_strategy_digest: strategy_witness.merge_strategy_digest().to_owned(),
            invalidation_strategy_digest: strategy_witness
                .invalidation_strategy_digest()
                .to_owned(),
            delivery_strategy_digest: strategy_witness.delivery_strategy_digest().to_owned(),
        }
    }

    pub fn witness_digest(&self) -> &str {
        &self.witness_digest
    }

    pub fn merge_strategy_digest(&self) -> &str {
        &self.merge_strategy_digest
    }

    pub fn invalidation_strategy_digest(&self) -> &str {
        &self.invalidation_strategy_digest
    }

    pub fn delivery_strategy_digest(&self) -> &str {
        &self.delivery_strategy_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SignalCompatibilityInspectionRow {
    compatibility_digest: String,
    branch_basis_digest: String,
    declaration_digest: String,
    admitted_scope_digest: String,
    strategy_witness_digest: String,
    posture_kind: SignalMergeCompatibilityPostureKind,
}

impl SignalCompatibilityInspectionRow {
    pub(crate) fn from_witness(
        compatibility_witness: &SignalMergeCompatibilityWitness,
        posture_kind: SignalMergeCompatibilityPostureKind,
    ) -> Self {
        Self {
            compatibility_digest: compatibility_witness.compatibility_digest().to_owned(),
            branch_basis_digest: compatibility_witness
                .fact_inventory()
                .branch_basis_digest()
                .to_owned(),
            declaration_digest: compatibility_witness
                .fact_inventory()
                .declaration_digest()
                .to_owned(),
            admitted_scope_digest: compatibility_witness
                .fact_inventory()
                .admitted_scope_digest()
                .to_owned(),
            strategy_witness_digest: compatibility_witness
                .fact_inventory()
                .strategy_witness_digest()
                .to_owned(),
            posture_kind,
        }
    }

    pub fn compatibility_digest(&self) -> &str {
        &self.compatibility_digest
    }

    pub fn branch_basis_digest(&self) -> &str {
        &self.branch_basis_digest
    }

    pub fn declaration_digest(&self) -> &str {
        &self.declaration_digest
    }

    pub fn admitted_scope_digest(&self) -> &str {
        &self.admitted_scope_digest
    }

    pub fn strategy_witness_digest(&self) -> &str {
        &self.strategy_witness_digest
    }

    pub fn posture_kind(&self) -> SignalMergeCompatibilityPostureKind {
        self.posture_kind
    }
}
