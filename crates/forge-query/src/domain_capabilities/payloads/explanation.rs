use forge_runtime_bridge::facade::BridgeCausalExplanationEnvelope;

use crate::evidence_identity::{
    ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};
use crate::runtime::{
    CausalEvidenceFamily, CausalEvidenceReferenceSet, CausalInspectionExplanationFamily,
    CausalInspectionMaterializationPolicy, CausalInspectionRedactionPolicy,
    CausalInspectionRichness, CausalInspectionTarget,
};

use super::common::{
    ForgeQueryDomainCapabilityCategory, ForgeQueryDomainCapabilityPayload,
    ForgeQueryDomainCapabilitySemanticPosture, SealedPayload,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryExplanationContributionPosture {
    RequiresContext,
    ExplainsFallback,
    ExplainsAmbiguity,
}

impl ForgeQueryExplanationContributionPosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RequiresContext => "requires-context",
            Self::ExplainsFallback => "explains-fallback",
            Self::ExplainsAmbiguity => "explains-ambiguity",
        }
    }

    pub const fn semantic_posture(self) -> ForgeQueryDomainCapabilitySemanticPosture {
        match self {
            Self::RequiresContext => {
                ForgeQueryDomainCapabilitySemanticPosture::ExplanationRequiresContext
            }
            Self::ExplainsFallback => {
                ForgeQueryDomainCapabilitySemanticPosture::ExplanationExplainsFallback
            }
            Self::ExplainsAmbiguity => {
                ForgeQueryDomainCapabilitySemanticPosture::ExplanationExplainsAmbiguity
            }
        }
    }
}

fn compose_explanation_runtime_semantics_identity(
    reference_set: &CausalEvidenceReferenceSet,
    target: &CausalInspectionTarget,
    explanation_family: CausalInspectionExplanationFamily,
    requested_richness: CausalInspectionRichness,
    requested_evidence_families: &[CausalEvidenceFamily],
    bridge_envelope: Option<&BridgeCausalExplanationEnvelope>,
    redaction_policy: CausalInspectionRedactionPolicy,
    materialization_policy: CausalInspectionMaterializationPolicy,
) -> ForgeQueryEvidenceIdentity {
    let families = requested_evidence_families
        .iter()
        .map(CausalEvidenceFamily::as_str)
        .collect::<Vec<_>>()
        .join("|");
    let mut identity = ForgeQueryEvidenceIdentity::compose(
        ForgeQueryEvidenceScope::MutationEvidenceAggregateDigest,
    )
    .field_shape(
        ForgeQueryEvidenceTag::new("identity_family"),
        "forge_query_domain_capability_explanation_runtime_semantics_v1",
    )
    .field_evidence_identity(
        ForgeQueryEvidenceTag::new("reference_set"),
        reference_set.reference_set_digest().evidence_identity(),
    )
    .field_evidence_identity(
        ForgeQueryEvidenceTag::new("target"),
        target.target_identity().evidence_identity(),
    )
    .field_shape(
        ForgeQueryEvidenceTag::new("family"),
        explanation_family.as_str(),
    )
    .field_shape(
        ForgeQueryEvidenceTag::new("richness"),
        requested_richness.as_str(),
    )
    .field_shape(ForgeQueryEvidenceTag::new("evidence_families"), &families)
    .field_shape(
        ForgeQueryEvidenceTag::new("redaction"),
        redaction_policy.as_str(),
    )
    .field_shape(
        ForgeQueryEvidenceTag::new("materialization"),
        materialization_policy.as_str(),
    );
    identity = match bridge_envelope {
        Some(envelope) => identity.field_bridge_retained_evidence_identity(
            ForgeQueryEvidenceTag::new("bridge_envelope"),
            envelope.envelope_evidence_identity(),
        ),
        None => identity.field_shape(ForgeQueryEvidenceTag::new("bridge_envelope"), "none"),
    };
    identity.seal()
}

fn compose_explanation_payload_identity(
    posture: ForgeQueryExplanationContributionPosture,
    semantic_code: &str,
    detail: &str,
    runtime_semantics: Option<&ForgeQueryExplanationRuntimeSemantics>,
) -> ForgeQueryEvidenceIdentity {
    let mut identity = ForgeQueryEvidenceIdentity::compose(
        ForgeQueryEvidenceScope::MutationEvidenceAggregateDigest,
    )
    .field_shape(
        ForgeQueryEvidenceTag::new("identity_family"),
        "forge_query_domain_capability_payload_v2",
    )
    .field_shape(
        ForgeQueryEvidenceTag::new("category"),
        ForgeQueryDomainCapabilityCategory::ExplanationInspection.as_str(),
    )
    .field_shape(ForgeQueryEvidenceTag::new("posture"), posture.as_str())
    .field_shape(ForgeQueryEvidenceTag::new("semantic_code"), semantic_code)
    .field_shape(ForgeQueryEvidenceTag::new("detail"), detail);
    identity = match runtime_semantics {
        Some(semantics) => identity.field_evidence_identity(
            ForgeQueryEvidenceTag::new("runtime_semantics"),
            semantics.semantics_identity(),
        ),
        None => identity.field_shape(ForgeQueryEvidenceTag::new("runtime_semantics"), "none"),
    };
    identity.seal()
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryExplanationRuntimeSemantics {
    reference_set: CausalEvidenceReferenceSet,
    target: CausalInspectionTarget,
    explanation_family: CausalInspectionExplanationFamily,
    requested_richness: CausalInspectionRichness,
    requested_evidence_families: Vec<CausalEvidenceFamily>,
    bridge_envelope: Option<BridgeCausalExplanationEnvelope>,
    redaction_policy: CausalInspectionRedactionPolicy,
    materialization_policy: CausalInspectionMaterializationPolicy,
    semantics_identity: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryExplanationRuntimeSemantics {
    pub fn new(
        reference_set: CausalEvidenceReferenceSet,
        target: CausalInspectionTarget,
        explanation_family: CausalInspectionExplanationFamily,
        requested_richness: CausalInspectionRichness,
        requested_evidence_families: Vec<CausalEvidenceFamily>,
        bridge_envelope: Option<BridgeCausalExplanationEnvelope>,
        redaction_policy: CausalInspectionRedactionPolicy,
        materialization_policy: CausalInspectionMaterializationPolicy,
    ) -> Self {
        let semantics_identity = compose_explanation_runtime_semantics_identity(
            &reference_set,
            &target,
            explanation_family,
            requested_richness,
            &requested_evidence_families,
            bridge_envelope.as_ref(),
            redaction_policy,
            materialization_policy,
        );
        Self {
            reference_set,
            target,
            explanation_family,
            requested_richness,
            requested_evidence_families,
            bridge_envelope,
            redaction_policy,
            materialization_policy,
            semantics_identity,
        }
    }

    pub fn reference_set(&self) -> &CausalEvidenceReferenceSet {
        &self.reference_set
    }

    pub fn target(&self) -> &CausalInspectionTarget {
        &self.target
    }

    pub fn explanation_family(&self) -> CausalInspectionExplanationFamily {
        self.explanation_family
    }

    pub fn requested_richness(&self) -> CausalInspectionRichness {
        self.requested_richness
    }

    pub fn requested_evidence_families(&self) -> &[CausalEvidenceFamily] {
        &self.requested_evidence_families
    }

    pub fn bridge_envelope(&self) -> Option<&BridgeCausalExplanationEnvelope> {
        self.bridge_envelope.as_ref()
    }

    pub fn redaction_policy(&self) -> CausalInspectionRedactionPolicy {
        self.redaction_policy
    }

    pub fn materialization_policy(&self) -> CausalInspectionMaterializationPolicy {
        self.materialization_policy
    }

    pub(in crate::domain_capabilities) fn semantics_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.semantics_identity
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryExplanationContributionPayload {
    posture: ForgeQueryExplanationContributionPosture,
    semantic_code: String,
    detail: String,
    runtime_semantics: Option<ForgeQueryExplanationRuntimeSemantics>,
    payload_identity: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryExplanationContributionPayload {
    pub fn new(
        posture: ForgeQueryExplanationContributionPosture,
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self {
        Self::with_runtime_semantics(posture, semantic_code, detail, None)
    }

    pub fn with_runtime_semantics(
        posture: ForgeQueryExplanationContributionPosture,
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
        runtime_semantics: Option<ForgeQueryExplanationRuntimeSemantics>,
    ) -> Self {
        let semantic_code = semantic_code.into();
        let detail = detail.into();
        let payload_identity = compose_explanation_payload_identity(
            posture,
            &semantic_code,
            &detail,
            runtime_semantics.as_ref(),
        );
        Self {
            posture,
            semantic_code,
            detail,
            runtime_semantics,
            payload_identity,
        }
    }

    pub fn category(&self) -> ForgeQueryDomainCapabilityCategory {
        ForgeQueryDomainCapabilityCategory::ExplanationInspection
    }

    pub fn posture(&self) -> ForgeQueryExplanationContributionPosture {
        self.posture
    }

    pub fn semantic_code(&self) -> &str {
        &self.semantic_code
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }

    pub fn runtime_semantics(&self) -> Option<&ForgeQueryExplanationRuntimeSemantics> {
        self.runtime_semantics.as_ref()
    }

    pub fn payload_digest(&self) -> &str {
        self.payload_identity.as_str()
    }
}

impl SealedPayload for ForgeQueryExplanationContributionPayload {}

impl ForgeQueryDomainCapabilityPayload for ForgeQueryExplanationContributionPayload {
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
