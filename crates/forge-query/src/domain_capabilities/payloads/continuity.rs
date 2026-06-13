use crate::evidence_identity::{
    ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};
use crate::runtime::{
    ForgeQueryContinuityMutationFamily, ForgeQueryContinuityMutationOutcomeClass,
};

use super::common::{
    ForgeQueryDomainCapabilityCategory, ForgeQueryDomainCapabilityPayload,
    ForgeQueryDomainCapabilitySemanticPosture, SealedPayload,
};
use super::continuity_correspondence::ForgeQueryContinuityCorrespondenceSemantics;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryContinuityContributionPosture {
    Preserved,
    Split,
    Replaced,
    CorrespondenceOnly,
}

impl ForgeQueryContinuityContributionPosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Preserved => "preserved",
            Self::Split => "split",
            Self::Replaced => "replaced",
            Self::CorrespondenceOnly => "correspondence-only",
        }
    }

    pub const fn semantic_posture(self) -> ForgeQueryDomainCapabilitySemanticPosture {
        match self {
            Self::Preserved => ForgeQueryDomainCapabilitySemanticPosture::ContinuityPreserved,
            Self::Split => ForgeQueryDomainCapabilitySemanticPosture::ContinuitySplit,
            Self::Replaced => ForgeQueryDomainCapabilitySemanticPosture::ContinuityReplaced,
            Self::CorrespondenceOnly => {
                ForgeQueryDomainCapabilitySemanticPosture::ContinuityCorrespondenceOnly
            }
        }
    }
}

fn compose_continuity_runtime_semantics_identity(
    runtime_semantics: &ForgeQueryContinuityRuntimeSemantics,
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::MutationEvidenceAggregateDigest)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "forge_query_domain_capability_continuity_runtime_semantics_v1",
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("family"),
            runtime_semantics.family().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("outcome_class"),
            runtime_semantics.outcome_class().as_str(),
        )
        .field_identity(
            ForgeQueryEvidenceTag::new("prior_authoritative"),
            runtime_semantics.prior_authoritative_identity(),
        )
        .field_identity_sequence(
            ForgeQueryEvidenceTag::new("successor_authoritative"),
            runtime_semantics.successor_authoritative_identities(),
        )
        .seal()
}

fn compose_continuity_payload_identity(
    posture: ForgeQueryContinuityContributionPosture,
    semantic_code: &str,
    detail: &str,
    runtime_semantics: Option<&ForgeQueryContinuityRuntimeSemantics>,
    correspondence_semantics: Option<&ForgeQueryContinuityCorrespondenceSemantics>,
) -> ForgeQueryEvidenceIdentity {
    let mut identity = ForgeQueryEvidenceIdentity::compose(
        ForgeQueryEvidenceScope::MutationEvidenceAggregateDigest,
    )
    .field_shape(
        ForgeQueryEvidenceTag::new("identity_family"),
        "forge_query_domain_capability_payload_v3",
    )
    .field_shape(
        ForgeQueryEvidenceTag::new("category"),
        ForgeQueryDomainCapabilityCategory::ContinuityLineage.as_str(),
    )
    .field_shape(ForgeQueryEvidenceTag::new("posture"), posture.as_str())
    .field_shape(ForgeQueryEvidenceTag::new("semantic_code"), semantic_code)
    .field_shape(ForgeQueryEvidenceTag::new("detail"), detail);
    identity = match runtime_semantics {
        Some(runtime) => identity.field_evidence_identity(
            ForgeQueryEvidenceTag::new("runtime"),
            &compose_continuity_runtime_semantics_identity(runtime),
        ),
        None => identity.field_shape(ForgeQueryEvidenceTag::new("runtime"), "none"),
    };
    identity = match correspondence_semantics {
        Some(correspondence) => identity.field_shape(
            ForgeQueryEvidenceTag::new("correspondence"),
            correspondence.digest_fragment(),
        ),
        None => identity.field_shape(ForgeQueryEvidenceTag::new("correspondence"), "none"),
    };
    identity.seal()
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryContinuityRuntimeSemantics {
    family: ForgeQueryContinuityMutationFamily,
    outcome_class: ForgeQueryContinuityMutationOutcomeClass,
    prior_authoritative_identity: String,
    successor_authoritative_identities: Vec<String>,
}

impl ForgeQueryContinuityRuntimeSemantics {
    pub fn new<I, S>(
        family: ForgeQueryContinuityMutationFamily,
        outcome_class: ForgeQueryContinuityMutationOutcomeClass,
        prior_authoritative_identity: impl Into<String>,
        successor_authoritative_identities: I,
    ) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self {
            family,
            outcome_class,
            prior_authoritative_identity: prior_authoritative_identity.into(),
            successor_authoritative_identities: successor_authoritative_identities
                .into_iter()
                .map(Into::into)
                .collect(),
        }
    }

    pub fn family(&self) -> ForgeQueryContinuityMutationFamily {
        self.family
    }

    pub fn outcome_class(&self) -> ForgeQueryContinuityMutationOutcomeClass {
        self.outcome_class
    }

    pub fn prior_authoritative_identity(&self) -> &str {
        &self.prior_authoritative_identity
    }

    pub fn successor_authoritative_identities(&self) -> &[String] {
        &self.successor_authoritative_identities
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryContinuityContributionPayload {
    posture: ForgeQueryContinuityContributionPosture,
    semantic_code: String,
    detail: String,
    runtime_semantics: Option<ForgeQueryContinuityRuntimeSemantics>,
    correspondence_semantics: Option<ForgeQueryContinuityCorrespondenceSemantics>,
    payload_identity: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryContinuityContributionPayload {
    pub fn new(
        posture: ForgeQueryContinuityContributionPosture,
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self {
        Self::with_runtime_semantics(posture, semantic_code, detail, None)
    }

    pub fn with_runtime_semantics(
        posture: ForgeQueryContinuityContributionPosture,
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
        runtime_semantics: Option<ForgeQueryContinuityRuntimeSemantics>,
    ) -> Self {
        Self::with_all_semantics(posture, semantic_code, detail, runtime_semantics, None)
    }

    pub fn with_correspondence_semantics(
        posture: ForgeQueryContinuityContributionPosture,
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
        correspondence_semantics: Option<ForgeQueryContinuityCorrespondenceSemantics>,
    ) -> Self {
        Self::with_all_semantics(
            posture,
            semantic_code,
            detail,
            None,
            correspondence_semantics,
        )
    }

    pub fn with_all_semantics(
        posture: ForgeQueryContinuityContributionPosture,
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
        runtime_semantics: Option<ForgeQueryContinuityRuntimeSemantics>,
        correspondence_semantics: Option<ForgeQueryContinuityCorrespondenceSemantics>,
    ) -> Self {
        let semantic_code = semantic_code.into();
        let detail = detail.into();
        let payload_identity = compose_continuity_payload_identity(
            posture,
            &semantic_code,
            &detail,
            runtime_semantics.as_ref(),
            correspondence_semantics.as_ref(),
        );
        Self {
            posture,
            semantic_code,
            detail,
            runtime_semantics,
            correspondence_semantics,
            payload_identity,
        }
    }

    pub fn category(&self) -> ForgeQueryDomainCapabilityCategory {
        ForgeQueryDomainCapabilityCategory::ContinuityLineage
    }

    pub fn posture(&self) -> ForgeQueryContinuityContributionPosture {
        self.posture
    }

    pub fn semantic_code(&self) -> &str {
        &self.semantic_code
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }

    pub fn runtime_semantics(&self) -> Option<&ForgeQueryContinuityRuntimeSemantics> {
        self.runtime_semantics.as_ref()
    }

    pub fn correspondence_semantics(&self) -> Option<&ForgeQueryContinuityCorrespondenceSemantics> {
        self.correspondence_semantics.as_ref()
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

impl SealedPayload for ForgeQueryContinuityContributionPayload {}

impl ForgeQueryDomainCapabilityPayload for ForgeQueryContinuityContributionPayload {
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
}
