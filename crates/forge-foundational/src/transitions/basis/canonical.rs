#[path = "canonical_branch.rs"]
mod canonical_branch;
#[path = "canonical_commit.rs"]
mod canonical_commit;
#[path = "canonical_merge.rs"]
mod canonical_merge;
#[path = "canonical_shared.rs"]
mod canonical_shared;

use forge_proof::TransitionOutcome;

use crate::canonicalization::{
    prepare_canonical_basis_sequence, CanonicalBasisConstructionDenial, CanonicalBasisDomain,
    CanonicalBasisEntry, CanonicalBasisReadyArtifact, CanonicalizationRuleVersion,
};
use crate::transitions::{
    FoundationalBranchCandidateArtifact, FoundationalCommitReceiptArtifact,
    FoundationalCommittedAuthorityArtifact, FoundationalMergeVerdict,
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
