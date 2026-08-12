use super::{domain_evidence_binding_material, WorthQueryDomainEvidenceBinding};

mod attachment;
pub(in crate::domain_installation::operation_execution) use attachment::{
    admit_direct_completion_content, admit_workflow_completion_content,
    WorthQueryCompletedDomainEvidenceAdmissionDenial,
};

/// Exact completion-bound ordinary evidence.
///
/// Publication owns the admitted descriptive content. Query's ordinary
/// completion owner alone supplies the execution binding and attaches that
/// content to a receipt.
#[derive(Debug, Eq, PartialEq)]
pub struct WorthQueryAdmittedDomainEvidence {
    binding: WorthQueryDomainEvidenceBinding,
    content: worth_query_publication::facade::domain_computation::WorthQueryAdmittedDomainEvidenceContent,
    identity: String,
}

impl WorthQueryAdmittedDomainEvidence {
    fn attach(
        binding: WorthQueryDomainEvidenceBinding,
        content: worth_query_publication::facade::domain_computation::WorthQueryAdmittedDomainEvidenceContent,
    ) -> Self {
        let identity = crate::identity::hash_parts(&[
            "worth_query_admitted_domain_evidence_v2".into(),
            format!("content:{}", content.identity()),
            format!("binding:{}", domain_evidence_binding_material(&binding)),
        ]);
        Self {
            binding,
            content,
            identity,
        }
    }

    pub fn identity(&self) -> &str {
        &self.identity
    }

    pub fn contract_identity(&self) -> &str {
        self.content.contract_identity()
    }

    pub fn binding(&self) -> &WorthQueryDomainEvidenceBinding {
        &self.binding
    }

    pub fn governance(
        &self,
    ) -> &worth_query_publication::facade::domain_computation::WorthQueryDomainEvidenceGovernance
    {
        self.content.governance()
    }

    pub fn core(
        &self,
    ) -> &worth_query_publication::facade::domain_computation::WorthQueryDomainEvidenceCore {
        self.content.core()
    }

    pub fn counter_sidecar(
        &self,
    ) -> &worth_query_publication::facade::domain_computation::WorthQueryAdmittedDomainEvidenceSidecar<
        worth_query_publication::facade::domain_computation::WorthQueryAdmittedStructuralCounter,
    >{
        self.content.counter_sidecar()
    }

    pub fn decision_sidecar(
        &self,
    ) -> &worth_query_publication::facade::domain_computation::WorthQueryAdmittedDomainEvidenceSidecar<
        worth_query_publication::facade::domain_computation::WorthQueryDecisionRecord,
    >{
        self.content.decision_sidecar()
    }

    pub fn candidate_sidecar(
        &self,
    ) -> &worth_query_publication::facade::domain_computation::WorthQueryAdmittedDomainEvidenceSidecar<
        worth_query_publication::facade::domain_computation::WorthQueryCandidateRecord,
    >{
        self.content.candidate_sidecar()
    }

    pub fn transformation_sidecar(
        &self,
    ) -> &worth_query_publication::facade::domain_computation::WorthQueryAdmittedDomainEvidenceSidecar<
        worth_query_publication::facade::domain_computation::WorthQueryTransformationRecord,
    >{
        self.content.transformation_sidecar()
    }

    pub const fn authority_posture(
        &self,
    ) -> worth_query_publication::facade::domain_computation::WorthQueryDomainEvidenceAuthorityPosture
    {
        worth_query_publication::facade::domain_computation::WorthQueryDomainEvidenceAuthorityPosture::DescriptiveOnly
    }

    pub fn replay_meaning(
        &self,
    ) -> worth_query_publication::facade::domain_computation::WorthQueryDomainEvidenceReplayMeaning
    {
        self.content.replay_meaning()
    }
}
