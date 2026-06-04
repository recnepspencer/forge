use crate::identities::CanonicalDigestId;
use crate::transitions::FoundationalBranchId;

use super::scope_evidence_validation::{
    sorted_unique_no_ops, validate_no_op_loci, validate_scope_evidence_loci,
    validate_selected_loci_have_one_outcome, validate_selected_scope_has_complete_outcomes,
    validate_skipped_evidence,
};
use super::scoped::{
    sorted_unique_aspects, sorted_unique_nodes, FoundationalMergeScope,
    FoundationalSelectedAspectRequestEntry, FoundationalSelectedNodeLocus,
};
use super::vocabulary::FoundationalMergeConstructionDenial;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoundationalScopeAdmissionBasis {
    DirectSourceIdentity,
    IdentityCorresponded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalSelectedScopeNoOpCause {
    UnchangedSourceTruth,
    EquivalentTargetTruth,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalSelectedScopeLocus {
    Node(FoundationalSelectedNodeLocus),
    Aspect(FoundationalSelectedAspectRequestEntry),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FoundationalSelectedScopeNoOpEvidence {
    locus: FoundationalSelectedScopeLocus,
    cause: FoundationalSelectedScopeNoOpCause,
}

impl FoundationalSelectedScopeNoOpEvidence {
    pub fn new(
        locus: FoundationalSelectedScopeLocus,
        cause: FoundationalSelectedScopeNoOpCause,
    ) -> Self {
        Self { locus, cause }
    }

    pub fn locus(&self) -> &FoundationalSelectedScopeLocus {
        &self.locus
    }

    pub const fn cause(&self) -> FoundationalSelectedScopeNoOpCause {
        self.cause
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalSkippedOutOfScopeEvidence {
    skipped_candidate_count: u64,
    skipped_digest: Option<CanonicalDigestId>,
}

impl FoundationalSkippedOutOfScopeEvidence {
    pub const fn new(
        skipped_candidate_count: u64,
        skipped_digest: Option<CanonicalDigestId>,
    ) -> Self {
        Self {
            skipped_candidate_count,
            skipped_digest,
        }
    }

    pub const fn skipped_candidate_count(&self) -> u64 {
        self.skipped_candidate_count
    }

    pub const fn skipped_digest(&self) -> Option<CanonicalDigestId> {
        self.skipped_digest
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalScopeBreadthSummary {
    requested_locus_count: u64,
    admitted_locus_count: u64,
    no_op_locus_count: u64,
    skipped_candidate_count: u64,
    conflict_check_width: u64,
}

impl FoundationalScopeBreadthSummary {
    pub const fn new(
        requested_locus_count: u64,
        admitted_locus_count: u64,
        no_op_locus_count: u64,
        skipped_candidate_count: u64,
        conflict_check_width: u64,
    ) -> Self {
        Self {
            requested_locus_count,
            admitted_locus_count,
            no_op_locus_count,
            skipped_candidate_count,
            conflict_check_width,
        }
    }

    pub const fn requested_locus_count(&self) -> u64 {
        self.requested_locus_count
    }

    pub const fn admitted_locus_count(&self) -> u64 {
        self.admitted_locus_count
    }

    pub const fn no_op_locus_count(&self) -> u64 {
        self.no_op_locus_count
    }

    pub const fn skipped_candidate_count(&self) -> u64 {
        self.skipped_candidate_count
    }

    pub const fn conflict_check_width(&self) -> u64 {
        self.conflict_check_width
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalAdmittedMergeScopeEvidence {
    source_branch: FoundationalBranchId,
    target_branch: FoundationalBranchId,
    requested_scope: FoundationalMergeScope,
    admission_basis: FoundationalScopeAdmissionBasis,
    admitted_nodes: Vec<FoundationalSelectedNodeLocus>,
    admitted_aspects: Vec<FoundationalSelectedAspectRequestEntry>,
    selected_no_ops: Vec<FoundationalSelectedScopeNoOpEvidence>,
    skipped: FoundationalSkippedOutOfScopeEvidence,
    breadth: FoundationalScopeBreadthSummary,
}

impl FoundationalAdmittedMergeScopeEvidence {
    pub fn new(
        source_branch: FoundationalBranchId,
        target_branch: FoundationalBranchId,
        requested_scope: FoundationalMergeScope,
        admission_basis: FoundationalScopeAdmissionBasis,
        admitted_nodes: impl IntoIterator<Item = FoundationalSelectedNodeLocus>,
        admitted_aspects: impl IntoIterator<Item = FoundationalSelectedAspectRequestEntry>,
        selected_no_ops: impl IntoIterator<Item = FoundationalSelectedScopeNoOpEvidence>,
        skipped: FoundationalSkippedOutOfScopeEvidence,
        conflict_check_width: u64,
    ) -> Result<Self, FoundationalMergeConstructionDenial> {
        let admitted_nodes = sorted_unique_nodes(admitted_nodes)?;
        let admitted_aspects = sorted_unique_aspects(admitted_aspects)?;
        let selected_no_ops = sorted_unique_no_ops(selected_no_ops)?;
        validate_scope_evidence_loci(&requested_scope, &admitted_nodes, &admitted_aspects)?;
        validate_no_op_loci(&requested_scope, &selected_no_ops)?;
        validate_selected_loci_have_one_outcome(
            &admitted_nodes,
            &admitted_aspects,
            &selected_no_ops,
        )?;
        validate_selected_scope_has_complete_outcomes(
            &requested_scope,
            &admitted_nodes,
            &admitted_aspects,
            &selected_no_ops,
        )?;
        validate_skipped_evidence(&requested_scope, skipped)?;
        let breadth = FoundationalScopeBreadthSummary::new(
            requested_scope.requested_locus_count(),
            admitted_nodes.len() as u64 + admitted_aspects.len() as u64,
            selected_no_ops.len() as u64,
            skipped.skipped_candidate_count(),
            conflict_check_width,
        );
        Ok(Self {
            source_branch,
            target_branch,
            requested_scope,
            admission_basis,
            admitted_nodes,
            admitted_aspects,
            selected_no_ops,
            skipped,
            breadth,
        })
    }

    pub fn admit_all_requested(
        source_branch: FoundationalBranchId,
        target_branch: FoundationalBranchId,
        requested_scope: &FoundationalMergeScope,
        admission_basis: FoundationalScopeAdmissionBasis,
        conflict_check_width: u64,
    ) -> Self {
        Self {
            source_branch,
            target_branch,
            requested_scope: requested_scope.clone(),
            admission_basis,
            admitted_nodes: requested_scope.selected_nodes_loci().to_vec(),
            admitted_aspects: requested_scope.selected_aspect_loci().to_vec(),
            selected_no_ops: Vec::new(),
            skipped: FoundationalSkippedOutOfScopeEvidence::new(0, None),
            breadth: FoundationalScopeBreadthSummary::new(
                requested_scope.requested_locus_count(),
                requested_scope.requested_locus_count(),
                0,
                0,
                conflict_check_width,
            ),
        }
    }

    pub fn source_branch(&self) -> &FoundationalBranchId {
        &self.source_branch
    }

    pub fn target_branch(&self) -> &FoundationalBranchId {
        &self.target_branch
    }

    pub fn requested_scope(&self) -> &FoundationalMergeScope {
        &self.requested_scope
    }

    pub const fn admission_basis(&self) -> FoundationalScopeAdmissionBasis {
        self.admission_basis
    }

    pub fn admitted_nodes(&self) -> &[FoundationalSelectedNodeLocus] {
        &self.admitted_nodes
    }

    pub fn admitted_aspects(&self) -> &[FoundationalSelectedAspectRequestEntry] {
        &self.admitted_aspects
    }

    pub fn selected_no_ops(&self) -> &[FoundationalSelectedScopeNoOpEvidence] {
        &self.selected_no_ops
    }

    pub const fn skipped(&self) -> FoundationalSkippedOutOfScopeEvidence {
        self.skipped
    }

    pub const fn breadth(&self) -> FoundationalScopeBreadthSummary {
        self.breadth
    }
}
