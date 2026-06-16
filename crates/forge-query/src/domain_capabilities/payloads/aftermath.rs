use crate::evidence_identity::{
    ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};
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

fn aftermath_source_identity(source: &ProjectionConsumptionSource) -> ForgeQueryEvidenceIdentity {
    let mut identity = ForgeQueryEvidenceIdentity::compose(
        ForgeQueryEvidenceScope::MutationEvidenceAggregateDigest,
    )
    .field_shape(
        ForgeQueryEvidenceTag::new("identity_family"),
        "forge_query_aftermath_source_v1",
    )
    .field_shape(
        ForgeQueryEvidenceTag::new("source_family"),
        source.family().as_str(),
    );
    identity = match source.source_identity_handle().evidence_identity() {
        Some(source_identity) => {
            identity.field_evidence_identity(ForgeQueryEvidenceTag::new("source"), source_identity)
        }
        None => identity.field_shape(
            ForgeQueryEvidenceTag::new("source_label"),
            source.source_identity(),
        ),
    };
    identity.seal()
}

fn aftermath_binding_identity(
    binding: &ProjectionConsumptionBindingContext,
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::MutationEvidenceAggregateDigest)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "forge_query_aftermath_binding_v1",
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("result_shape"),
            binding.result_shape_digest(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("authorized_projection_query"),
            binding.authorized_projection_query_digest(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("authorized_projection_result_shape"),
            binding.authorized_projection_result_shape_digest(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("authorized_projection"),
            binding.authorized_projection_identity(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("narrowed_result_shape"),
            binding.narrowed_result_shape_digest(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("policy"),
            binding.policy_digest(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("tenant_schema_basis"),
            binding.tenant_schema_basis_digest(),
        )
        .seal()
}

fn compose_aftermath_runtime_semantics_identity(
    runtime_semantics: &ForgeQueryAftermathRuntimeSemantics,
) -> ForgeQueryEvidenceIdentity {
    let requested = runtime_semantics
        .requested_facts()
        .requested()
        .map(|request| match request.field_key() {
            Some(field) => format!("{}:{field}", request.kind().as_str()),
            None => request.kind().as_str().to_string(),
        })
        .collect::<Vec<_>>();
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::MutationEvidenceAggregateDigest)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "forge_query_domain_capability_aftermath_runtime_semantics_v2",
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("source"),
            &aftermath_source_identity(runtime_semantics.source()),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("binding"),
            &aftermath_binding_identity(runtime_semantics.binding()),
        )
        .field_value_sequence(ForgeQueryEvidenceTag::new("requested"), &requested)
        .seal()
}

fn compose_aftermath_payload_identity(
    posture: ForgeQueryAftermathContributionPosture,
    semantic_code: &str,
    detail: &str,
    runtime_semantics: Option<&ForgeQueryAftermathRuntimeSemantics>,
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
        ForgeQueryDomainCapabilityCategory::ConsequenceAftermath.as_str(),
    )
    .field_shape(ForgeQueryEvidenceTag::new("posture"), posture.as_str())
    .field_shape(ForgeQueryEvidenceTag::new("semantic_code"), semantic_code)
    .field_shape(ForgeQueryEvidenceTag::new("detail"), detail);
    identity = match runtime_semantics {
        Some(runtime) => {
            let runtime_identity = runtime.semantics_identity();
            identity
                .field_evidence_identity(ForgeQueryEvidenceTag::new("runtime"), &runtime_identity)
        }
        None => identity.field_shape(ForgeQueryEvidenceTag::new("runtime"), "none"),
    };
    identity.seal()
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

    pub(in crate::domain_capabilities) fn semantics_identity(&self) -> ForgeQueryEvidenceIdentity {
        compose_aftermath_runtime_semantics_identity(self)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryAftermathContributionPayload {
    posture: ForgeQueryAftermathContributionPosture,
    semantic_code: String,
    detail: String,
    runtime_semantics: Option<ForgeQueryAftermathRuntimeSemantics>,
    payload_identity: ForgeQueryEvidenceIdentity,
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
        let payload_identity = compose_aftermath_payload_identity(
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
        self.payload_identity.as_str()
    }

    pub fn payload_for_reporting(&self) -> &str {
        self.payload_identity.as_str()
    }

    pub fn payload_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.payload_identity
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

    fn payload_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.payload_identity
    }
}
