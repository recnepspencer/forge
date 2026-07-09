use worth_proof::{
    Artifact, AssumptionBasis, AuthorityMarker, AuthorityProves, AuthorityWitness, CurrentValidity,
    FreshnessScopedBasis, Proof, ProofSet,
};

use super::bundle::FoundationalTransitionBundleBuilder;
use super::vocabulary::{
    build_receipt_claim, FoundationalCommitId, FoundationalCommitReceiptIdentity,
    FoundationalCommitReceiptIssuanceDenial, FoundationalTransitionIssuanceCause,
    FoundationalTransitionProvenanceRow,
};
use crate::boundary_artifacts::{
    FoundationalBoundaryReceiptSurface, FoundationalReceiptEvidenceBoundaryClaim,
};
use crate::transitions::{
    FoundationalAuthorityTransitionClass, FoundationalAuthorityTransitionOutcomeKind,
    FoundationalCommittedAuthorityArtifact, FoundationalMergeBasis, FoundationalNoOpCause,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalCommitReceiptPhase;
impl worth_proof::PhaseMarker for FoundationalCommitReceiptPhase {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalCommitReceiptIssued;
impl worth_proof::ProofMarker for FoundationalCommitReceiptIssued {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalCommitReceiptIssuanceBasis {
    outcome_kind: FoundationalAuthorityTransitionOutcomeKind,
    issuance_cause: FoundationalTransitionIssuanceCause,
}

impl FoundationalCommitReceiptIssuanceBasis {
    pub const fn new(
        outcome_kind: FoundationalAuthorityTransitionOutcomeKind,
        issuance_cause: FoundationalTransitionIssuanceCause,
    ) -> Self {
        Self {
            outcome_kind,
            issuance_cause,
        }
    }

    pub const fn outcome_kind(&self) -> FoundationalAuthorityTransitionOutcomeKind {
        self.outcome_kind
    }

    pub const fn issuance_cause(&self) -> FoundationalTransitionIssuanceCause {
        self.issuance_cause
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalCommitReceiptIssuance(());

impl FoundationalCommitReceiptIssuance {
    pub(crate) const fn milestone_5_phase_4() -> Self {
        Self(())
    }
}

impl AuthorityMarker for FoundationalCommitReceiptIssuance {}
impl AuthorityProves<FoundationalCommitReceiptIssued> for FoundationalCommitReceiptIssuance {}

pub fn foundational_commit_receipt_issuance() -> AuthorityWitness<FoundationalCommitReceiptIssuance>
{
    AuthorityWitness::from_authority_marker(FoundationalCommitReceiptIssuance::milestone_5_phase_4())
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct FoundationalCommitReceiptPayload {
    receipt_identity: FoundationalCommitReceiptIdentity,
    commit_id: FoundationalCommitId,
    branch_id: crate::transitions::FoundationalBranchId,
    parent_basis: crate::transitions::FoundationalCommitParentBasis,
    parentage: crate::transitions::FoundationalCommitParentage,
    merge_ancestry_basis: Option<crate::transitions::FoundationalMergeAncestryBasis>,
    transition_class: FoundationalAuthorityTransitionClass,
    no_op_cause: Option<FoundationalNoOpCause>,
    committed_delta_summary: crate::transitions::FoundationalCommitDeltaSummary,
    merge_basis: FoundationalMergeBasis,
    receipt_claim: FoundationalReceiptEvidenceBoundaryClaim<FoundationalBoundaryReceiptSurface>,
    provenance_rows: Vec<FoundationalTransitionProvenanceRow>,
}

type FoundationalCommitReceiptInner = Artifact<
    FoundationalCommitReceiptPhase,
    FoundationalCommitReceiptPayload,
    Proof<FoundationalCommitReceiptIssued, FoundationalCommitReceiptIssuance>,
    FreshnessScopedBasis<CurrentValidity, AssumptionBasis<FoundationalCommitReceiptIssuanceBasis>>,
>;

pub struct FoundationalCommitReceiptArtifact {
    inner: FoundationalCommitReceiptInner,
}

impl FoundationalCommitReceiptArtifact {
    fn new(inner: FoundationalCommitReceiptInner) -> Self {
        Self { inner }
    }

    pub fn receipt_identity(&self) -> FoundationalCommitReceiptIdentity {
        self.inner.payload().receipt_identity
    }

    pub fn commit_id(&self) -> FoundationalCommitId {
        self.inner.payload().commit_id
    }

    pub fn branch_id(&self) -> &crate::transitions::FoundationalBranchId {
        &self.inner.payload().branch_id
    }

    pub fn parent_basis(&self) -> crate::transitions::FoundationalCommitParentBasis {
        self.inner.payload().parent_basis
    }

    pub fn parentage(&self) -> &crate::transitions::FoundationalCommitParentage {
        &self.inner.payload().parentage
    }

    pub fn merge_ancestry_basis(
        &self,
    ) -> Option<crate::transitions::FoundationalMergeAncestryBasis> {
        self.inner.payload().merge_ancestry_basis
    }

    pub fn transition_class(&self) -> FoundationalAuthorityTransitionClass {
        self.inner.payload().transition_class
    }

    pub fn no_op_cause(&self) -> Option<FoundationalNoOpCause> {
        self.inner.payload().no_op_cause
    }

    pub fn delta_evidence(&self) -> &crate::transitions::FoundationalCommitDeltaSummary {
        &self.inner.payload().committed_delta_summary
    }

    pub fn strategy_identity(&self) -> &crate::transitions::FoundationalTransitionStrategyIdentity {
        self.inner.payload().provenance_rows[0].strategy_identity()
    }

    pub fn strategy_descriptor_digest(
        &self,
    ) -> crate::transitions::FoundationalTransitionStrategyDescriptorDigest {
        self.inner.payload().provenance_rows[0].strategy_descriptor_digest()
    }

    pub fn transition_basis_identity(
        &self,
    ) -> crate::transitions::FoundationalTransitionBasisIdentity {
        self.inner.payload().merge_basis.identity()
    }

    pub fn receipt_claim(
        &self,
    ) -> &FoundationalReceiptEvidenceBoundaryClaim<FoundationalBoundaryReceiptSurface> {
        &self.inner.payload().receipt_claim
    }

    pub fn transition_provenance_rows(&self) -> &[FoundationalTransitionProvenanceRow] {
        &self.inner.payload().provenance_rows
    }

    pub fn proofs(
        &self,
    ) -> &Proof<FoundationalCommitReceiptIssued, FoundationalCommitReceiptIssuance>
    where
        Proof<FoundationalCommitReceiptIssued, FoundationalCommitReceiptIssuance>: ProofSet,
    {
        self.inner.proofs()
    }

    pub fn issuance_basis(&self) -> &FoundationalCommitReceiptIssuanceBasis {
        self.inner.strong_basis().value()
    }
}

pub(crate) fn derive_transition_provenance_row<T>(
    committed: &FoundationalCommittedAuthorityArtifact<T>,
    receipt_identity: FoundationalCommitReceiptIdentity,
    commit_id: FoundationalCommitId,
) -> FoundationalTransitionProvenanceRow {
    let verdict = committed.merge_verdict();
    let issuance_cause =
        FoundationalTransitionIssuanceCause::for_transition_class(committed.transition_class());

    FoundationalTransitionProvenanceRow::new(
        verdict.source_branch().clone(),
        verdict.target_branch().clone(),
        committed.parent_basis(),
        verdict.merge_basis().clone(),
        committed.transition_class(),
        committed.no_op_cause(),
        verdict.strategy_identity().clone(),
        verdict.strategy_descriptor_digest(),
        verdict.observation_basis(),
        verdict.comparison_basis().cloned(),
        verdict.correspondence_basis(),
        verdict.remap_basis(),
        Some(issuance_cause),
        Some(commit_id),
        Some(receipt_identity),
    )
}

pub(crate) fn derive_transition_report_row<T>(
    committed: &FoundationalCommittedAuthorityArtifact<T>,
) -> FoundationalTransitionProvenanceRow {
    let verdict = committed.merge_verdict();

    FoundationalTransitionProvenanceRow::new(
        verdict.source_branch().clone(),
        verdict.target_branch().clone(),
        committed.parent_basis(),
        verdict.merge_basis().clone(),
        committed.transition_class(),
        committed.no_op_cause(),
        verdict.strategy_identity().clone(),
        verdict.strategy_descriptor_digest(),
        verdict.observation_basis(),
        verdict.comparison_basis().cloned(),
        verdict.correspondence_basis(),
        verdict.remap_basis(),
        None,
        None,
        None,
    )
}

pub(crate) fn issue_transition_receipt_from_committed<T>(
    committed: &FoundationalCommittedAuthorityArtifact<T>,
    receipt_identity: FoundationalCommitReceiptIdentity,
    commit_id: FoundationalCommitId,
    authority: AuthorityWitness<FoundationalCommitReceiptIssuance>,
) -> Result<FoundationalCommitReceiptArtifact, FoundationalCommitReceiptIssuanceDenial> {
    let row = derive_transition_provenance_row(committed, receipt_identity, commit_id);
    let claim = build_receipt_claim(
        committed.target_branch(),
        commit_id,
        receipt_identity,
        committed.transition_class(),
        committed.committed_delta_summary(),
    )?;
    let issuance_cause =
        FoundationalTransitionIssuanceCause::for_transition_class(committed.transition_class());
    let payload = FoundationalCommitReceiptPayload {
        receipt_identity,
        commit_id,
        branch_id: committed.target_branch().clone(),
        parent_basis: committed.parent_basis(),
        parentage: committed.parentage().clone(),
        merge_ancestry_basis: committed.merge_ancestry_basis(),
        transition_class: committed.transition_class(),
        no_op_cause: committed.no_op_cause(),
        committed_delta_summary: committed.committed_delta_summary().clone(),
        merge_basis: committed.merge_basis().clone(),
        receipt_claim: claim,
        provenance_rows: vec![row],
    };
    let proof = Proof::from_authority_witness(&authority);
    let basis = FoundationalCommitReceiptIssuanceBasis::new(
        committed.transition_outcome_kind(),
        issuance_cause,
    );

    Ok(FoundationalCommitReceiptArtifact::new(
        Artifact::with_proofs_and_current_basis(payload, proof, basis, authority),
    ))
}

impl<T> FoundationalCommittedAuthorityArtifact<T> {
    pub fn issue_receipt(
        &self,
        receipt_identity: FoundationalCommitReceiptIdentity,
        commit_id: FoundationalCommitId,
        authority: AuthorityWitness<FoundationalCommitReceiptIssuance>,
    ) -> Result<FoundationalCommitReceiptArtifact, FoundationalCommitReceiptIssuanceDenial> {
        issue_transition_receipt_from_committed(self, receipt_identity, commit_id, authority)
    }

    pub fn emit_transition_bundle(self) -> FoundationalTransitionBundleBuilder<T> {
        FoundationalTransitionBundleBuilder::new(self)
    }
}
