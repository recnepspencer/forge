use crate::evidence_identity::{
    ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};

use super::common::{
    ForgeQueryDomainCapabilityCategory, ForgeQueryDomainCapabilityPayload,
    ForgeQueryDomainCapabilitySemanticPosture, SealedPayload,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryAdmissionContributionPosture {
    Advisory,
    Violation,
    SupportOnly,
}

impl ForgeQueryAdmissionContributionPosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Advisory => "advisory",
            Self::Violation => "violation",
            Self::SupportOnly => "support-only",
        }
    }

    pub const fn semantic_posture(self) -> ForgeQueryDomainCapabilitySemanticPosture {
        match self {
            Self::Advisory => ForgeQueryDomainCapabilitySemanticPosture::AdmissionAdvisory,
            Self::Violation => ForgeQueryDomainCapabilitySemanticPosture::AdmissionViolation,
            Self::SupportOnly => ForgeQueryDomainCapabilitySemanticPosture::AdmissionSupportOnly,
        }
    }

    pub const fn default_decision_stage(self) -> &'static str {
        match self {
            Self::Advisory => "domain_capability_advisory",
            Self::Violation => "domain_capability_violation",
            Self::SupportOnly => "domain_capability_support_only",
        }
    }
}

fn compose_admission_payload_identity(
    posture: ForgeQueryAdmissionContributionPosture,
    semantic_code: &str,
    detail: &str,
    decision_stage: &str,
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::MutationEvidenceAggregateDigest)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "forge_query_domain_capability_payload_v3",
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("category"),
            ForgeQueryDomainCapabilityCategory::Admission.as_str(),
        )
        .field_shape(ForgeQueryEvidenceTag::new("posture"), posture.as_str())
        .field_shape(ForgeQueryEvidenceTag::new("semantic_code"), semantic_code)
        .field_shape(ForgeQueryEvidenceTag::new("detail"), detail)
        .field_shape(ForgeQueryEvidenceTag::new("decision_stage"), decision_stage)
        .seal()
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryAdmissionContributionPayload {
    posture: ForgeQueryAdmissionContributionPosture,
    semantic_code: String,
    detail: String,
    decision_stage: &'static str,
    payload_identity: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryAdmissionContributionPayload {
    pub fn new(
        posture: ForgeQueryAdmissionContributionPosture,
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self {
        Self::with_decision_stage(
            posture,
            semantic_code,
            detail,
            posture.default_decision_stage(),
        )
    }

    pub fn with_decision_stage(
        posture: ForgeQueryAdmissionContributionPosture,
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
        decision_stage: &'static str,
    ) -> Self {
        let semantic_code = semantic_code.into();
        let detail = detail.into();
        let payload_identity =
            compose_admission_payload_identity(posture, &semantic_code, &detail, decision_stage);
        Self {
            posture,
            semantic_code,
            detail,
            decision_stage,
            payload_identity,
        }
    }

    pub fn category(&self) -> ForgeQueryDomainCapabilityCategory {
        ForgeQueryDomainCapabilityCategory::Admission
    }

    pub fn posture(&self) -> ForgeQueryAdmissionContributionPosture {
        self.posture
    }

    pub fn semantic_code(&self) -> &str {
        &self.semantic_code
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }

    pub fn decision_stage(&self) -> &'static str {
        self.decision_stage
    }

    pub fn payload_digest(&self) -> &str {
        self.payload_identity.as_str()
    }

    pub fn payload_for_reporting(&self) -> &str {
        self.payload_identity.as_str()
    }

    pub fn payload_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.payload_identity
    }
}

impl SealedPayload for ForgeQueryAdmissionContributionPayload {}

impl ForgeQueryDomainCapabilityPayload for ForgeQueryAdmissionContributionPayload {
    fn category(&self) -> ForgeQueryDomainCapabilityCategory {
        self.category()
    }

    fn posture_label(&self) -> &'static str {
        self.posture().as_str()
    }

    fn semantic_posture(&self) -> ForgeQueryDomainCapabilitySemanticPosture {
        self.posture().semantic_posture()
    }

    fn semantic_code(&self) -> &str {
        self.semantic_code()
    }

    fn detail(&self) -> &str {
        self.detail()
    }

    fn payload_digest(&self) -> &str {
        self.payload_digest()
    }

    fn payload_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.payload_identity
    }
}
