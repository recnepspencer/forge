mod identity;

use crate::domain_installation::{
    WorthQueryAdmittedDomainEvidence, WorthQueryAdmittedDomainEvidenceSidecar,
    WorthQueryAdmittedStructuralCounter, WorthQueryCandidateRecord, WorthQueryDecisionRecord,
    WorthQueryDomainEvidenceAuthorityPosture, WorthQueryDomainEvidenceBinding,
    WorthQueryDomainEvidenceCore, WorthQueryDomainEvidenceGovernance,
    WorthQueryTransformationRecord,
};

use super::CausalInspectionRedactionPolicy;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryDomainEvidenceInspectionSidecar<T> {
    NotApplicable,
    Omitted,
    DigestOnly { digest: String },
    Materialized { digest: String, records: Vec<T> },
}

impl<T> WorthQueryDomainEvidenceInspectionSidecar<T> {
    pub fn digest(&self) -> Option<&str> {
        match self {
            Self::NotApplicable | Self::Omitted => None,
            Self::DigestOnly { digest } | Self::Materialized { digest, .. } => Some(digest),
        }
    }

    pub fn records(&self) -> Option<&[T]> {
        match self {
            Self::Materialized { records, .. } => Some(records),
            Self::NotApplicable | Self::Omitted | Self::DigestOnly { .. } => None,
        }
    }

    pub const fn is_omitted(&self) -> bool {
        matches!(self, Self::Omitted)
    }

    pub const fn is_not_applicable(&self) -> bool {
        matches!(self, Self::NotApplicable)
    }
}

/// A governance-narrowing diagnostic copy of admitted domain evidence. It
/// explains an execution but cannot authorize operation, admission, repair,
/// artifact production, or publication.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryDomainEvidenceInspectionCopy {
    source_evidence_identity: String,
    contract_identity: String,
    binding: WorthQueryDomainEvidenceBinding,
    governance: WorthQueryDomainEvidenceGovernance,
    core: WorthQueryDomainEvidenceCore,
    counter_sidecar: WorthQueryDomainEvidenceInspectionSidecar<WorthQueryAdmittedStructuralCounter>,
    decision_sidecar: WorthQueryDomainEvidenceInspectionSidecar<WorthQueryDecisionRecord>,
    candidate_sidecar: WorthQueryDomainEvidenceInspectionSidecar<WorthQueryCandidateRecord>,
    transformation_sidecar:
        WorthQueryDomainEvidenceInspectionSidecar<WorthQueryTransformationRecord>,
    redaction_policy: CausalInspectionRedactionPolicy,
    identity: String,
}

impl WorthQueryDomainEvidenceInspectionCopy {
    pub fn derive(
        source: &WorthQueryAdmittedDomainEvidence,
        redaction_policy: CausalInspectionRedactionPolicy,
    ) -> Self {
        let counter_sidecar = narrow_sidecar(source.counter_sidecar(), redaction_policy);
        let decision_sidecar = narrow_sidecar(source.decision_sidecar(), redaction_policy);
        let candidate_sidecar = narrow_sidecar(source.candidate_sidecar(), redaction_policy);
        let transformation_sidecar =
            narrow_sidecar(source.transformation_sidecar(), redaction_policy);
        let identity = identity::inspection_copy_identity(
            source,
            &counter_sidecar,
            &decision_sidecar,
            &candidate_sidecar,
            &transformation_sidecar,
            redaction_policy,
        );
        Self {
            source_evidence_identity: source.identity().to_owned(),
            contract_identity: source.contract_identity().to_owned(),
            binding: source.binding().clone(),
            governance: source.governance().clone(),
            core: source.core().clone(),
            counter_sidecar,
            decision_sidecar,
            candidate_sidecar,
            transformation_sidecar,
            redaction_policy,
            identity,
        }
    }

    pub fn identity(&self) -> &str {
        &self.identity
    }

    pub fn source_evidence_identity(&self) -> &str {
        &self.source_evidence_identity
    }

    pub fn contract_identity(&self) -> &str {
        &self.contract_identity
    }

    pub fn binding(&self) -> &WorthQueryDomainEvidenceBinding {
        &self.binding
    }

    pub fn governance(&self) -> &WorthQueryDomainEvidenceGovernance {
        &self.governance
    }

    pub fn core(&self) -> &WorthQueryDomainEvidenceCore {
        &self.core
    }

    pub fn counter_sidecar(
        &self,
    ) -> &WorthQueryDomainEvidenceInspectionSidecar<WorthQueryAdmittedStructuralCounter> {
        &self.counter_sidecar
    }

    pub fn decision_sidecar(
        &self,
    ) -> &WorthQueryDomainEvidenceInspectionSidecar<WorthQueryDecisionRecord> {
        &self.decision_sidecar
    }

    pub fn candidate_sidecar(
        &self,
    ) -> &WorthQueryDomainEvidenceInspectionSidecar<WorthQueryCandidateRecord> {
        &self.candidate_sidecar
    }

    pub fn transformation_sidecar(
        &self,
    ) -> &WorthQueryDomainEvidenceInspectionSidecar<WorthQueryTransformationRecord> {
        &self.transformation_sidecar
    }

    pub const fn redaction_policy(&self) -> CausalInspectionRedactionPolicy {
        self.redaction_policy
    }

    pub const fn authority_posture(&self) -> WorthQueryDomainEvidenceAuthorityPosture {
        WorthQueryDomainEvidenceAuthorityPosture::DescriptiveOnly
    }
}

fn narrow_sidecar<T>(
    source: &WorthQueryAdmittedDomainEvidenceSidecar<T>,
    _redaction_policy: CausalInspectionRedactionPolicy,
) -> WorthQueryDomainEvidenceInspectionSidecar<T> {
    match source {
        WorthQueryAdmittedDomainEvidenceSidecar::NotApplicable => {
            WorthQueryDomainEvidenceInspectionSidecar::NotApplicable
        }
        WorthQueryAdmittedDomainEvidenceSidecar::Omitted => {
            WorthQueryDomainEvidenceInspectionSidecar::Omitted
        }
        WorthQueryAdmittedDomainEvidenceSidecar::DigestOnly { digest } => {
            WorthQueryDomainEvidenceInspectionSidecar::DigestOnly {
                digest: digest.clone(),
            }
        }
        WorthQueryAdmittedDomainEvidenceSidecar::Materialized { digest, .. } => {
            WorthQueryDomainEvidenceInspectionSidecar::DigestOnly {
                digest: digest.clone(),
            }
        }
        WorthQueryAdmittedDomainEvidenceSidecar::PartiallyMaterialized { digest, .. } => {
            WorthQueryDomainEvidenceInspectionSidecar::DigestOnly {
                digest: digest.clone(),
            }
        }
    }
}
