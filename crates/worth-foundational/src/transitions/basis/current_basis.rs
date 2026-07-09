use worth_proof::{
    Artifact, AssumptionBasis, AuthorityMarker, AuthorityWitness,
    BoundaryBridgedAuthorityRevalidationRequiredBasis, CurrentValidity, FreshnessScopedBasis,
    NoProofs, TransitionOutcome,
};

use super::canonical::{
    prepare_commit_receipt_for_canonical_basis, prepare_committed_authority_for_canonical_basis,
};
use crate::canonicalization::{
    CanonicalBasisConstructionDenial, CanonicalBasisReadyArtifact, CanonicalizationRuleVersion,
};
use crate::transitions::{
    FoundationalCommitReceiptArtifact, FoundationalCommittedAuthorityArtifact,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CurrentBasisTransitionPhase;
impl worth_proof::PhaseMarker for CurrentBasisTransitionPhase {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalTransitionCurrentBasisAuthority(());

impl FoundationalTransitionCurrentBasisAuthority {
    pub(crate) const fn milestone_5_phase_5() -> Self {
        Self(())
    }
}

impl AuthorityMarker for FoundationalTransitionCurrentBasisAuthority {}

pub fn foundational_transition_current_basis_authority(
) -> AuthorityWitness<FoundationalTransitionCurrentBasisAuthority> {
    AuthorityWitness::from_authority_marker(
        FoundationalTransitionCurrentBasisAuthority::milestone_5_phase_5(),
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalTransitionCurrentBasisReadmissionAuthority(());

impl FoundationalTransitionCurrentBasisReadmissionAuthority {
    pub(crate) const fn milestone_5_phase_5_boundary() -> Self {
        Self(())
    }
}

impl AuthorityMarker for FoundationalTransitionCurrentBasisReadmissionAuthority {}

pub fn foundational_transition_current_basis_readmission_authority(
) -> AuthorityWitness<FoundationalTransitionCurrentBasisReadmissionAuthority> {
    AuthorityWitness::from_authority_marker(
        FoundationalTransitionCurrentBasisReadmissionAuthority::milestone_5_phase_5_boundary(),
    )
}

type CurrentBasisCommittedAuthorityInner<T> = Artifact<
    CurrentBasisTransitionPhase,
    FoundationalCommittedAuthorityArtifact<T>,
    NoProofs,
    FreshnessScopedBasis<CurrentValidity, AssumptionBasis<CanonicalBasisReadyArtifact>>,
>;

type BoundaryBridgedCommittedAuthorityInner<T> = Artifact<
    CurrentBasisTransitionPhase,
    FoundationalCommittedAuthorityArtifact<T>,
    NoProofs,
    BoundaryBridgedAuthorityRevalidationRequiredBasis<CanonicalBasisReadyArtifact>,
>;

type CurrentBasisCommitReceiptInner = Artifact<
    CurrentBasisTransitionPhase,
    FoundationalCommitReceiptArtifact,
    NoProofs,
    FreshnessScopedBasis<CurrentValidity, AssumptionBasis<CanonicalBasisReadyArtifact>>,
>;

type BoundaryBridgedCommitReceiptInner = Artifact<
    CurrentBasisTransitionPhase,
    FoundationalCommitReceiptArtifact,
    NoProofs,
    BoundaryBridgedAuthorityRevalidationRequiredBasis<CanonicalBasisReadyArtifact>,
>;

pub struct CurrentBasisCommittedAuthorityArtifact<T> {
    inner: CurrentBasisCommittedAuthorityInner<T>,
}

impl<T> CurrentBasisCommittedAuthorityArtifact<T> {
    fn new(inner: CurrentBasisCommittedAuthorityInner<T>) -> Self {
        Self { inner }
    }

    pub fn committed(&self) -> &FoundationalCommittedAuthorityArtifact<T> {
        self.inner.payload()
    }

    pub fn strong_basis(&self) -> &CanonicalBasisReadyArtifact {
        self.inner.strong_basis().value()
    }
}

pub struct BoundaryBridgedCurrentBasisCommittedAuthorityArtifact<T> {
    inner: BoundaryBridgedCommittedAuthorityInner<T>,
}

impl<T> BoundaryBridgedCurrentBasisCommittedAuthorityArtifact<T> {
    fn new(inner: BoundaryBridgedCommittedAuthorityInner<T>) -> Self {
        Self { inner }
    }

    pub fn committed(&self) -> &FoundationalCommittedAuthorityArtifact<T> {
        self.inner.payload()
    }
}

pub struct CurrentBasisCommitReceiptArtifact {
    inner: CurrentBasisCommitReceiptInner,
}

impl CurrentBasisCommitReceiptArtifact {
    fn new(inner: CurrentBasisCommitReceiptInner) -> Self {
        Self { inner }
    }

    pub fn receipt(&self) -> &FoundationalCommitReceiptArtifact {
        self.inner.payload()
    }

    pub fn strong_basis(&self) -> &CanonicalBasisReadyArtifact {
        self.inner.strong_basis().value()
    }
}

pub struct BoundaryBridgedCurrentBasisCommitReceiptArtifact {
    inner: BoundaryBridgedCommitReceiptInner,
}

impl BoundaryBridgedCurrentBasisCommitReceiptArtifact {
    fn new(inner: BoundaryBridgedCommitReceiptInner) -> Self {
        Self { inner }
    }

    pub fn receipt(&self) -> &FoundationalCommitReceiptArtifact {
        self.inner.payload()
    }
}

pub fn admit_current_basis_committed_authority<T>(
    version: CanonicalizationRuleVersion,
    committed: FoundationalCommittedAuthorityArtifact<T>,
    authority: AuthorityWitness<FoundationalTransitionCurrentBasisAuthority>,
) -> TransitionOutcome<CurrentBasisCommittedAuthorityArtifact<T>, CanonicalBasisConstructionDenial>
{
    let basis = match prepare_committed_authority_for_canonical_basis(version, &committed) {
        TransitionOutcome::Success(ready) => ready,
        TransitionOutcome::Denied(denial) => return TransitionOutcome::denied(denial),
        TransitionOutcome::Deferred(_)
        | TransitionOutcome::Stale(_)
        | TransitionOutcome::RebindRequired(_)
        | TransitionOutcome::Failed(_) => {
            unreachable!("transition basis preparation uses only denied")
        }
    };

    TransitionOutcome::success(CurrentBasisCommittedAuthorityArtifact::new(
        Artifact::with_current_basis(committed, basis, authority),
    ))
}

pub fn admit_current_basis_commit_receipt(
    version: CanonicalizationRuleVersion,
    receipt: FoundationalCommitReceiptArtifact,
    authority: AuthorityWitness<FoundationalTransitionCurrentBasisAuthority>,
) -> TransitionOutcome<CurrentBasisCommitReceiptArtifact, CanonicalBasisConstructionDenial> {
    let basis = match prepare_commit_receipt_for_canonical_basis(version, &receipt) {
        TransitionOutcome::Success(ready) => ready,
        TransitionOutcome::Denied(denial) => return TransitionOutcome::denied(denial),
        TransitionOutcome::Deferred(_)
        | TransitionOutcome::Stale(_)
        | TransitionOutcome::RebindRequired(_)
        | TransitionOutcome::Failed(_) => {
            unreachable!("transition basis preparation uses only denied")
        }
    };

    TransitionOutcome::success(CurrentBasisCommitReceiptArtifact::new(
        Artifact::with_current_basis(receipt, basis, authority),
    ))
}

pub fn bridge_current_basis_committed_authority_trust_boundary<T>(
    artifact: CurrentBasisCommittedAuthorityArtifact<T>,
) -> BoundaryBridgedCurrentBasisCommittedAuthorityArtifact<T> {
    BoundaryBridgedCurrentBasisCommittedAuthorityArtifact::new(
        artifact.inner.bridge_trust_boundary(),
    )
}

pub fn readmit_current_basis_committed_authority_after_boundary<T>(
    artifact: BoundaryBridgedCurrentBasisCommittedAuthorityArtifact<T>,
    basis: CanonicalBasisReadyArtifact,
    authority: AuthorityWitness<FoundationalTransitionCurrentBasisReadmissionAuthority>,
) -> CurrentBasisCommittedAuthorityArtifact<T> {
    CurrentBasisCommittedAuthorityArtifact::new(
        artifact.inner.readmit_with_authority(basis, authority),
    )
}

pub fn bridge_current_basis_commit_receipt_trust_boundary(
    artifact: CurrentBasisCommitReceiptArtifact,
) -> BoundaryBridgedCurrentBasisCommitReceiptArtifact {
    BoundaryBridgedCurrentBasisCommitReceiptArtifact::new(artifact.inner.bridge_trust_boundary())
}

pub fn readmit_current_basis_commit_receipt_after_boundary(
    artifact: BoundaryBridgedCurrentBasisCommitReceiptArtifact,
    basis: CanonicalBasisReadyArtifact,
    authority: AuthorityWitness<FoundationalTransitionCurrentBasisReadmissionAuthority>,
) -> CurrentBasisCommitReceiptArtifact {
    CurrentBasisCommitReceiptArtifact::new(artifact.inner.readmit_with_authority(basis, authority))
}
