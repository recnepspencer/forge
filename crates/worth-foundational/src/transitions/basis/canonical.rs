#[path = "canonical_branch.rs"]
mod canonical_branch;
#[path = "canonical_commit.rs"]
mod canonical_commit;
#[path = "canonical_merge.rs"]
mod canonical_merge;
#[path = "canonical_scope.rs"]
mod canonical_scope;
#[path = "canonical_shared.rs"]
mod canonical_shared;

use worth_proof::TransitionOutcome;

use crate::canonicalization::{
    prepare_canonical_basis_sequence, CanonicalBasisConstructionDenial, CanonicalBasisDomain,
    CanonicalBasisEntry, CanonicalBasisReadyArtifact, CanonicalizationRuleVersion,
};
use crate::transitions::{
    FoundationalAdmittedMergeScopeEvidence, FoundationalBranchCandidateArtifact,
    FoundationalCommitReceiptArtifact, FoundationalCommittedAuthorityArtifact,
    FoundationalMergeScope, FoundationalMergeVerdict, FoundationalScopedMergeDenialEvidence,
    FoundationalScopedMergeUnavailablePosture,
};

pub fn prepare_branch_candidate_for_canonical_basis<T>(
    version: CanonicalizationRuleVersion,
    candidate: &FoundationalBranchCandidateArtifact<T>,
) -> TransitionOutcome<CanonicalBasisReadyArtifact, CanonicalBasisConstructionDenial> {
    prepare_canonical_basis_sequence(
        version,
        CanonicalBasisDomain::Transition,
        canonical_branch::candidate_entries(candidate),
    )
}

pub fn prepare_staged_branch_for_canonical_basis<T>(
    version: CanonicalizationRuleVersion,
    staged: &crate::transitions::FoundationalStagedBranchArtifact<T>,
) -> TransitionOutcome<CanonicalBasisReadyArtifact, CanonicalBasisConstructionDenial> {
    prepare_canonical_basis_sequence(
        version,
        CanonicalBasisDomain::Transition,
        canonical_branch::staged_entries(staged),
    )
}

pub fn prepare_merge_verdict_for_canonical_basis<T>(
    version: CanonicalizationRuleVersion,
    verdict: &FoundationalMergeVerdict<T>,
) -> TransitionOutcome<CanonicalBasisReadyArtifact, CanonicalBasisConstructionDenial> {
    prepare_canonical_basis_sequence(
        version,
        CanonicalBasisDomain::Transition,
        canonical_merge::merge_verdict_entries(verdict),
    )
}

pub fn prepare_merge_scope_for_canonical_basis(
    version: CanonicalizationRuleVersion,
    scope: &FoundationalMergeScope,
) -> TransitionOutcome<CanonicalBasisReadyArtifact, CanonicalBasisConstructionDenial> {
    prepare_canonical_basis_sequence(
        version,
        CanonicalBasisDomain::Transition,
        canonical_scope::merge_scope_entries("merge.scope", scope),
    )
}

pub fn prepare_admitted_merge_scope_for_canonical_basis(
    version: CanonicalizationRuleVersion,
    evidence: &FoundationalAdmittedMergeScopeEvidence,
) -> TransitionOutcome<CanonicalBasisReadyArtifact, CanonicalBasisConstructionDenial> {
    prepare_canonical_basis_sequence(
        version,
        CanonicalBasisDomain::Transition,
        canonical_scope::admitted_scope_entries("merge.scope_evidence", evidence),
    )
}

pub fn prepare_scoped_merge_denial_for_canonical_basis(
    version: CanonicalizationRuleVersion,
    evidence: &FoundationalScopedMergeDenialEvidence,
) -> TransitionOutcome<CanonicalBasisReadyArtifact, CanonicalBasisConstructionDenial> {
    prepare_canonical_basis_sequence(
        version,
        CanonicalBasisDomain::Transition,
        canonical_scope::scoped_denial_entries("merge.scope_denial", evidence),
    )
}

pub fn prepare_scoped_merge_unavailable_for_canonical_basis(
    version: CanonicalizationRuleVersion,
    posture: &FoundationalScopedMergeUnavailablePosture,
) -> TransitionOutcome<CanonicalBasisReadyArtifact, CanonicalBasisConstructionDenial> {
    prepare_canonical_basis_sequence(
        version,
        CanonicalBasisDomain::Transition,
        canonical_scope::scoped_unavailable_entries("merge.scope_unavailable", posture),
    )
}

pub fn prepare_committed_authority_for_canonical_basis<T>(
    version: CanonicalizationRuleVersion,
    committed: &FoundationalCommittedAuthorityArtifact<T>,
) -> TransitionOutcome<CanonicalBasisReadyArtifact, CanonicalBasisConstructionDenial> {
    prepare_canonical_basis_sequence(
        version,
        CanonicalBasisDomain::Transition,
        canonical_commit::committed_authority_entries(committed),
    )
}

pub fn prepare_commit_receipt_for_canonical_basis(
    version: CanonicalizationRuleVersion,
    receipt: &FoundationalCommitReceiptArtifact,
) -> TransitionOutcome<CanonicalBasisReadyArtifact, CanonicalBasisConstructionDenial> {
    prepare_canonical_basis_sequence(
        version,
        CanonicalBasisDomain::Transition,
        canonical_commit::receipt_entries(receipt),
    )
}

pub fn foundational_transition_canonical_basis_entries(
    ready: &CanonicalBasisReadyArtifact,
) -> &[CanonicalBasisEntry] {
    ready.payload().entries()
}
