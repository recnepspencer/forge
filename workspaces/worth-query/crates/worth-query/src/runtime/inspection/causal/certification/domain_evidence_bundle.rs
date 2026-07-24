mod identity;

use crate::domain_installation::{
    WorthQueryDomainEvidenceAuthorityPosture, WorthQueryDomainEvidenceBinding,
    WorthQueryDomainEvidenceCore, WorthQueryDomainEvidenceGovernance,
};

use super::super::{
    CausalInspectionRedactionPolicy, WorthQueryDomainEvidenceInspectionCopy,
    WorthQueryDomainEvidenceInspectionSidecar,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryDomainEvidenceCertificationSidecar {
    NotApplicable,
    Omitted,
    Digest { digest: String },
}

impl WorthQueryDomainEvidenceCertificationSidecar {
    pub fn digest(&self) -> Option<&str> {
        match self {
            Self::NotApplicable | Self::Omitted => None,
            Self::Digest { digest } => Some(digest),
        }
    }

    pub const fn is_omitted(&self) -> bool {
        matches!(self, Self::Omitted)
    }

    pub const fn is_not_applicable(&self) -> bool {
        matches!(self, Self::NotApplicable)
    }
}

/// Certification-safe descriptive evidence. Mandatory meaning and governance
/// are retained, while every applicable sidecar is represented only by digest.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryDomainEvidenceCertificationBundle {
    source_inspection_identity: String,
    source_evidence_identity: String,
    contract_identity: String,
    binding: WorthQueryDomainEvidenceBinding,
    governance: WorthQueryDomainEvidenceGovernance,
    core: WorthQueryDomainEvidenceCore,
    decision_sidecar: WorthQueryDomainEvidenceCertificationSidecar,
    candidate_sidecar: WorthQueryDomainEvidenceCertificationSidecar,
    transformation_sidecar: WorthQueryDomainEvidenceCertificationSidecar,
    source_redaction_policy: CausalInspectionRedactionPolicy,
    identity: String,
}

impl WorthQueryDomainEvidenceCertificationBundle {
    pub fn derive(source: &WorthQueryDomainEvidenceInspectionCopy) -> Self {
        let decision_sidecar = digest_sidecar(source.decision_sidecar());
        let candidate_sidecar = digest_sidecar(source.candidate_sidecar());
        let transformation_sidecar = digest_sidecar(source.transformation_sidecar());
        let identity = identity::certification_bundle_identity(
            source,
            &decision_sidecar,
            &candidate_sidecar,
            &transformation_sidecar,
        );
        Self {
            source_inspection_identity: source.identity().to_owned(),
            source_evidence_identity: source.source_evidence_identity().to_owned(),
            contract_identity: source.contract_identity().to_owned(),
            binding: source.binding().clone(),
            governance: source.governance().clone(),
            core: source.core().clone(),
            decision_sidecar,
            candidate_sidecar,
            transformation_sidecar,
            source_redaction_policy: source.redaction_policy(),
            identity,
        }
    }

    pub fn identity(&self) -> &str {
        &self.identity
    }

    pub fn source_inspection_identity(&self) -> &str {
        &self.source_inspection_identity
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

    pub const fn decision_sidecar(&self) -> &WorthQueryDomainEvidenceCertificationSidecar {
        &self.decision_sidecar
    }

    pub const fn candidate_sidecar(&self) -> &WorthQueryDomainEvidenceCertificationSidecar {
        &self.candidate_sidecar
    }

    pub const fn transformation_sidecar(&self) -> &WorthQueryDomainEvidenceCertificationSidecar {
        &self.transformation_sidecar
    }

    pub const fn source_redaction_policy(&self) -> CausalInspectionRedactionPolicy {
        self.source_redaction_policy
    }

    pub const fn authority_posture(&self) -> WorthQueryDomainEvidenceAuthorityPosture {
        WorthQueryDomainEvidenceAuthorityPosture::DescriptiveOnly
    }
}

fn digest_sidecar<T>(
    sidecar: &WorthQueryDomainEvidenceInspectionSidecar<T>,
) -> WorthQueryDomainEvidenceCertificationSidecar {
    match sidecar {
        WorthQueryDomainEvidenceInspectionSidecar::NotApplicable => {
            WorthQueryDomainEvidenceCertificationSidecar::NotApplicable
        }
        WorthQueryDomainEvidenceInspectionSidecar::Omitted => {
            WorthQueryDomainEvidenceCertificationSidecar::Omitted
        }
        WorthQueryDomainEvidenceInspectionSidecar::DigestOnly { digest }
        | WorthQueryDomainEvidenceInspectionSidecar::Materialized { digest, .. } => {
            WorthQueryDomainEvidenceCertificationSidecar::Digest {
                digest: digest.clone(),
            }
        }
    }
}
