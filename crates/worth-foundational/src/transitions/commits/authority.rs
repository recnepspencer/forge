use worth_proof::{
    Artifact, AssumptionBasis, AuthorityMarker, AuthorityProves, AuthorityWitness, CurrentValidity,
    FreshnessScopedBasis, Proof, ProofSet,
};

use super::super::{
    FoundationalMergeVerdict, FoundationalMergeVerdictKind, FoundationalStrategyBasis,
    FoundationalTransitionStrategyContractBasis, FoundationalTransitionStrategyDescriptorDigest,
    FoundationalTransitionStrategyIdentity,
};
use super::vocabulary::{
    FoundationalAuthorityTransitionClass, FoundationalAuthorityTransitionDenial,
    FoundationalAuthorityTransitionOutcomeKind, FoundationalCommitDeltaSummary,
    FoundationalCommitParentBasis, FoundationalCommitParentage,
    FoundationalCommittedAuthorityInput, FoundationalMergeAncestryBasis, FoundationalNoOpCause,
};
use crate::transitions::commits::FoundationalCommittedAuthorityAdmitted;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalCommittedAuthorityPhase;
impl worth_proof::PhaseMarker for FoundationalCommittedAuthorityPhase {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalCommittedAuthorityAdmissionBasis {
    outcome_kind: FoundationalAuthorityTransitionOutcomeKind,
}

impl FoundationalCommittedAuthorityAdmissionBasis {
    pub const fn new(outcome_kind: FoundationalAuthorityTransitionOutcomeKind) -> Self {
        Self { outcome_kind }
    }

    pub const fn outcome_kind(&self) -> FoundationalAuthorityTransitionOutcomeKind {
        self.outcome_kind
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalCommittedAuthorityAdmission(());

impl FoundationalCommittedAuthorityAdmission {
    pub(crate) const fn milestone_5_phase_3() -> Self {
        Self(())
    }
}

impl AuthorityMarker for FoundationalCommittedAuthorityAdmission {}
impl AuthorityProves<FoundationalCommittedAuthorityAdmitted>
    for FoundationalCommittedAuthorityAdmission
{
}

pub fn foundational_committed_authority_admission(
) -> AuthorityWitness<FoundationalCommittedAuthorityAdmission> {
    AuthorityWitness::from_authority_marker(
        FoundationalCommittedAuthorityAdmission::milestone_5_phase_3(),
    )
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct FoundationalCommittedAuthorityTransition<T> {
    verdict: FoundationalMergeVerdict<T>,
    input: FoundationalCommittedAuthorityInput,
}

impl<T> FoundationalCommittedAuthorityTransition<T> {
    fn new(
        verdict: FoundationalMergeVerdict<T>,
        input: FoundationalCommittedAuthorityInput,
    ) -> Self {
        Self { verdict, input }
    }
}

type FoundationalCommittedAuthorityInner<T> = Artifact<
    FoundationalCommittedAuthorityPhase,
    FoundationalCommittedAuthorityTransition<T>,
    Proof<FoundationalCommittedAuthorityAdmitted, FoundationalCommittedAuthorityAdmission>,
    FreshnessScopedBasis<
        CurrentValidity,
        AssumptionBasis<FoundationalCommittedAuthorityAdmissionBasis>,
    >,
>;

pub struct FoundationalCommittedAuthorityArtifact<T> {
    inner: FoundationalCommittedAuthorityInner<T>,
}

impl<T> FoundationalCommittedAuthorityArtifact<T> {
    fn new(inner: FoundationalCommittedAuthorityInner<T>) -> Self {
        Self { inner }
    }

    pub fn merge_verdict(&self) -> &FoundationalMergeVerdict<T> {
        &self.inner.payload().verdict
    }

    pub fn source_branch(&self) -> &crate::transitions::FoundationalBranchId {
        self.merge_verdict().source_branch()
    }

    pub fn target_branch(&self) -> &crate::transitions::FoundationalBranchId {
        self.merge_verdict().target_branch()
    }

    pub fn fork_basis(&self) -> &crate::transitions::FoundationalBranchCandidateForkBasis {
        self.merge_verdict().fork_basis()
    }

    pub fn observation_basis(
        &self,
    ) -> crate::transitions::FoundationalBranchCandidateObservationBasis {
        self.merge_verdict().observation_basis()
    }

    pub fn comparison_basis(
        &self,
    ) -> Option<&crate::transitions::FoundationalBranchCandidateComparisonBasis> {
        self.merge_verdict().comparison_basis()
    }

    pub fn transition_class(&self) -> FoundationalAuthorityTransitionClass {
        self.inner.payload().input.transition_class()
    }

    pub fn transition_outcome_kind(&self) -> FoundationalAuthorityTransitionOutcomeKind {
        self.transition_class().outcome_kind()
    }

    pub fn no_op_cause(&self) -> Option<FoundationalNoOpCause> {
        self.inner.payload().input.no_op_cause()
    }

    pub fn parent_basis(&self) -> FoundationalCommitParentBasis {
        self.inner.payload().input.parent_basis()
    }

    pub fn parentage(&self) -> &FoundationalCommitParentage {
        self.inner.payload().input.parentage()
    }

    pub fn merge_ancestry_basis(&self) -> Option<FoundationalMergeAncestryBasis> {
        self.inner.payload().input.merge_ancestry_basis()
    }

    pub fn committed_delta_summary(&self) -> &FoundationalCommitDeltaSummary {
        self.inner.payload().input.committed_delta_summary()
    }

    pub fn strategy_identity(&self) -> &FoundationalTransitionStrategyIdentity {
        self.merge_verdict().strategy_identity()
    }

    pub fn strategy_descriptor_digest(&self) -> FoundationalTransitionStrategyDescriptorDigest {
        self.merge_verdict().strategy_descriptor_digest()
    }

    pub fn strategy_contract_basis(&self) -> FoundationalTransitionStrategyContractBasis {
        self.merge_verdict().strategy_contract_basis()
    }

    pub fn strategy_basis(&self) -> FoundationalStrategyBasis {
        self.merge_verdict().strategy_basis()
    }

    pub fn merge_basis(&self) -> &crate::transitions::FoundationalMergeBasis {
        self.merge_verdict().merge_basis()
    }

    pub fn transition_basis_identity(
        &self,
    ) -> crate::transitions::FoundationalTransitionBasisIdentity {
        self.merge_verdict().merge_basis().identity()
    }

    pub fn payload(&self) -> &T {
        self.merge_verdict().payload()
    }

    pub fn proofs(
        &self,
    ) -> &Proof<FoundationalCommittedAuthorityAdmitted, FoundationalCommittedAuthorityAdmission>
    where
        Proof<FoundationalCommittedAuthorityAdmitted, FoundationalCommittedAuthorityAdmission>:
            ProofSet,
    {
        self.inner.proofs()
    }

    pub fn admission_basis(&self) -> &FoundationalCommittedAuthorityAdmissionBasis {
        self.inner.strong_basis().value()
    }
}

impl<T> FoundationalMergeVerdict<T> {
    pub fn commit_with(
        self,
        input: FoundationalCommittedAuthorityInput,
        authority: AuthorityWitness<FoundationalCommittedAuthorityAdmission>,
    ) -> Result<FoundationalCommittedAuthorityArtifact<T>, FoundationalAuthorityTransitionDenial>
    {
        match self.kind() {
            FoundationalMergeVerdictKind::Accepted | FoundationalMergeVerdictKind::Advisory => {}
            verdict_kind => {
                return Err(
                    FoundationalAuthorityTransitionDenial::MergeVerdictNotCommitEligible {
                        verdict_kind,
                    },
                );
            }
        }

        let proof = Proof::from_authority_witness(&authority);
        let basis = FoundationalCommittedAuthorityAdmissionBasis::new(
            input.transition_class().outcome_kind(),
        );

        Ok(FoundationalCommittedAuthorityArtifact::new(
            Artifact::with_proofs_and_current_basis(
                FoundationalCommittedAuthorityTransition::new(self, input),
                proof,
                basis,
                authority,
            ),
        ))
    }
}
