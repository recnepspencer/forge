use crate::identity::hash_parts;
use crate::projection_consumption::{
    ProjectMaterializedFacts, ProjectionConsumptionBindingContext, ProjectionConsumptionSource,
};

use super::common::{
    ForgeQueryDomainCapabilityCategory, ForgeQueryDomainCapabilityPayload,
    ForgeQueryDomainCapabilitySemanticPosture, SealedPayload,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryAftermathContributionPosture {
    EstablishesFact,
    ConsumesFact,
    DeclaresResidue,
}

impl ForgeQueryAftermathContributionPosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::EstablishesFact => "establishes-fact",
            Self::ConsumesFact => "consumes-fact",
            Self::DeclaresResidue => "declares-residue",
        }
    }

    pub const fn semantic_posture(self) -> ForgeQueryDomainCapabilitySemanticPosture {
        match self {
            Self::EstablishesFact => {
                ForgeQueryDomainCapabilitySemanticPosture::AftermathEstablishesFact
            }
            Self::ConsumesFact => ForgeQueryDomainCapabilitySemanticPosture::AftermathConsumesFact,
            Self::DeclaresResidue => {
                ForgeQueryDomainCapabilitySemanticPosture::AftermathDeclaresResidue
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryAftermathRuntimeSemantics {
    source: ProjectionConsumptionSource,
    binding: ProjectionConsumptionBindingContext,
    requested_facts: ProjectMaterializedFacts,
}

impl ForgeQueryAftermathRuntimeSemantics {
    pub fn new(
        source: ProjectionConsumptionSource,
        binding: ProjectionConsumptionBindingContext,
        requested_facts: ProjectMaterializedFacts,
    ) -> Self {
        Self {
            source,
            binding,
            requested_facts,
        }
    }

    pub fn source(&self) -> &ProjectionConsumptionSource {
        &self.source
    }

    pub fn binding(&self) -> &ProjectionConsumptionBindingContext {
        &self.binding
    }

    pub fn requested_facts(&self) -> &ProjectMaterializedFacts {
        &self.requested_facts
    }

    fn digest_fragment(&self) -> String {
        let requested = self
            .requested_facts
            .requested()
            .map(|request| match request.field_key() {
                Some(field) => format!("{}:{field}", request.kind().as_str()),
                None => request.kind().as_str().to_string(),
            })
            .collect::<Vec<_>>()
            .join("|");
        hash_parts(&[
            "forge_query_domain_capability_aftermath_runtime_semantics_v1".to_string(),
            format!("source-family:{}", self.source.family().as_str()),
            format!("source-identity:{}", self.source.source_identity()),
            format!("result-shape:{}", self.binding.result_shape_digest()),
            format!(
                "authorized-projection:{}",
                self.binding.authorized_projection_identity()
            ),
            format!("policy:{}", self.binding.policy_digest()),
            format!("requested:{requested}"),
        ])
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryAftermathContributionPayload {
    posture: ForgeQueryAftermathContributionPosture,
    semantic_code: String,
    detail: String,
    runtime_semantics: Option<ForgeQueryAftermathRuntimeSemantics>,
    payload_digest: String,
}

impl ForgeQueryAftermathContributionPayload {
    pub fn new(
        posture: ForgeQueryAftermathContributionPosture,
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self {
        Self::with_runtime_semantics(posture, semantic_code, detail, None)
    }

    pub fn with_runtime_semantics(
        posture: ForgeQueryAftermathContributionPosture,
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
        runtime_semantics: Option<ForgeQueryAftermathRuntimeSemantics>,
    ) -> Self {
        let semantic_code = semantic_code.into();
        let detail = detail.into();
        let runtime_digest = runtime_semantics.as_ref().map_or_else(
            || "none".to_string(),
            ForgeQueryAftermathRuntimeSemantics::digest_fragment,
        );
        let payload_digest = hash_parts(&[
            "forge_query_domain_capability_payload_v2".to_string(),
            format!(
                "category:{}",
                ForgeQueryDomainCapabilityCategory::ConsequenceAftermath.as_str()
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
        ForgeQueryDomainCapabilityCategory::ConsequenceAftermath
    }

    pub fn posture(&self) -> ForgeQueryAftermathContributionPosture {
        self.posture
    }

    pub fn semantic_code(&self) -> &str {
        &self.semantic_code
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }

    pub fn runtime_semantics(&self) -> Option<&ForgeQueryAftermathRuntimeSemantics> {
        self.runtime_semantics.as_ref()
    }

    pub fn payload_digest(&self) -> &str {
        &self.payload_digest
    }
}

impl SealedPayload for ForgeQueryAftermathContributionPayload {}

impl ForgeQueryDomainCapabilityPayload for ForgeQueryAftermathContributionPayload {
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
