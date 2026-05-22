use forge_runtime_bridge::facade::BridgeCausalExplanationEnvelope;

use crate::identity::hash_parts;
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
        Self {
            reference_set,
            target,
            explanation_family,
            requested_richness,
            requested_evidence_families,
            bridge_envelope,
            redaction_policy,
            materialization_policy,
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

    fn digest_fragment(&self) -> String {
        let families = self
            .requested_evidence_families
            .iter()
            .map(CausalEvidenceFamily::as_str)
            .collect::<Vec<_>>()
            .join("|");
        hash_parts(&[
            "forge_query_domain_capability_explanation_runtime_semantics_v1".to_string(),
            format!(
                "reference-set:{}",
                self.reference_set.reference_set_digest().as_str()
            ),
            format!("target:{}", self.target.target_digest()),
            format!("family:{}", self.explanation_family.as_str()),
            format!("richness:{}", self.requested_richness.as_str()),
            format!("evidence-families:{families}"),
            format!(
                "bridge-envelope:{}",
                self.bridge_envelope
                    .as_ref()
                    .map(|envelope| envelope.envelope_digest())
                    .unwrap_or("none")
            ),
            format!("redaction:{}", self.redaction_policy.as_str()),
            format!("materialization:{}", self.materialization_policy.as_str()),
        ])
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryExplanationContributionPayload {
    posture: ForgeQueryExplanationContributionPosture,
    semantic_code: String,
    detail: String,
    runtime_semantics: Option<ForgeQueryExplanationRuntimeSemantics>,
    payload_digest: String,
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
        let runtime_digest = runtime_semantics.as_ref().map_or_else(
            || "none".to_string(),
            ForgeQueryExplanationRuntimeSemantics::digest_fragment,
        );
        let payload_digest = hash_parts(&[
            "forge_query_domain_capability_payload_v2".to_string(),
            format!(
                "category:{}",
                ForgeQueryDomainCapabilityCategory::ExplanationInspection.as_str()
            ),
            format!("posture:{}", posture.as_str()),
            format!("semantic_code:{semantic_code}"),
            format!("detail:{detail}"),
            format!("runtime:{runtime_digest}"),
        ]);
        Self {
            posture,
            semantic_code,
            detail,
            runtime_semantics,
            payload_digest,
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
        &self.payload_digest
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
}
