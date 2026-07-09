use crate::domain_capabilities::identity::domain_capability_scope_encoder;
use crate::evidence_identity::{WorthQueryEvidenceIdentity, WorthQueryEvidenceTag};
use crate::runtime::WorthQueryGraphCompositionCapabilityClass;
use worth_relational::facade::runtime::{InvariantCatalog, InvariantRegistration};

use super::common::{
    SealedPayload, WorthQueryDomainCapabilityCategory, WorthQueryDomainCapabilityPayload,
    WorthQueryDomainCapabilitySemanticPosture,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryInvariantCapabilityContributionPosture {
    CapabilityGap,
    InvariantDenial,
    SupportSummary,
    InvariantRegistration,
}

impl WorthQueryInvariantCapabilityContributionPosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CapabilityGap => "capability-gap",
            Self::InvariantDenial => "invariant-denial",
            Self::SupportSummary => "support-summary",
            Self::InvariantRegistration => "invariant-registration",
        }
    }

    pub const fn semantic_posture(self) -> WorthQueryDomainCapabilitySemanticPosture {
        match self {
            Self::CapabilityGap => {
                WorthQueryDomainCapabilitySemanticPosture::InvariantCapabilityGap
            }
            Self::InvariantDenial => WorthQueryDomainCapabilitySemanticPosture::InvariantDenial,
            Self::SupportSummary => {
                WorthQueryDomainCapabilitySemanticPosture::InvariantSupportSummary
            }
            Self::InvariantRegistration => {
                WorthQueryDomainCapabilitySemanticPosture::InvariantRegistration
            }
        }
    }
}

fn graph_invariant_semantics_identity(
    role: &'static str,
    digest_label: &str,
) -> WorthQueryEvidenceIdentity {
    domain_capability_scope_encoder("worth_query_graph_invariant_semantics_digest_v1")
        .field_shape(WorthQueryEvidenceTag::new("role"), role)
        .field_shape(WorthQueryEvidenceTag::new("digest_label"), digest_label)
        .seal()
}

fn compose_graph_capability_identity(
    graph_capability: &WorthQueryGraphCapabilityRuntimeSemantics,
) -> WorthQueryEvidenceIdentity {
    domain_capability_scope_encoder("worth_query_graph_capability_runtime_semantics_v1")
        .field_shape(
            WorthQueryEvidenceTag::new("capability_family"),
            graph_capability.capability_family(),
        )
        .field_shape(
            WorthQueryEvidenceTag::new("capability_class"),
            graph_capability.capability_class().as_str(),
        )
        .seal()
}

fn compose_graph_invariant_denial_identity(
    graph_invariant_denial: &WorthQueryGraphInvariantDenialRuntimeSemantics,
) -> WorthQueryEvidenceIdentity {
    domain_capability_scope_encoder("worth_query_graph_invariant_denial_runtime_semantics_v1")
        .field_shape(
            WorthQueryEvidenceTag::new("invariant_family"),
            graph_invariant_denial.invariant_family(),
        )
        .field_value_sequence(
            WorthQueryEvidenceTag::new("declared_collections"),
            graph_invariant_denial.declared_collections(),
        )
        .field_value_sequence(
            WorthQueryEvidenceTag::new("declared_symbols"),
            graph_invariant_denial.declared_symbols(),
        )
        .field_value_sequence(
            WorthQueryEvidenceTag::new("target_combination_families"),
            graph_invariant_denial.target_combination_families(),
        )
        .field_value_sequence(
            WorthQueryEvidenceTag::new("lifecycle_families"),
            graph_invariant_denial.lifecycle_families(),
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("program"),
            graph_invariant_denial.program_identity(),
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("breadth"),
            graph_invariant_denial.breadth_identity(),
        )
        .field_shape(
            WorthQueryEvidenceTag::new("counter_snapshot"),
            graph_invariant_denial.counter_snapshot(),
        )
        .seal()
}

pub(crate) fn compose_invariant_registration_identity(
    invariant_registration: &WorthQueryInvariantRegistrationRuntimeSemantics,
) -> WorthQueryEvidenceIdentity {
    invariant_registration.registration_identity()
}

fn compose_invariant_capability_payload_identity(
    posture: WorthQueryInvariantCapabilityContributionPosture,
    semantic_code: &str,
    detail: &str,
    graph_capability: Option<&WorthQueryGraphCapabilityRuntimeSemantics>,
    graph_invariant_denial: Option<&WorthQueryGraphInvariantDenialRuntimeSemantics>,
    invariant_registration: Option<&WorthQueryInvariantRegistrationRuntimeSemantics>,
) -> WorthQueryEvidenceIdentity {
    let mut identity = domain_capability_scope_encoder("worth_query_domain_capability_payload_v3")
        .field_shape(
            WorthQueryEvidenceTag::new("category"),
            WorthQueryDomainCapabilityCategory::InvariantCapability.as_str(),
        )
        .field_shape(WorthQueryEvidenceTag::new("posture"), posture.as_str())
        .field_shape(WorthQueryEvidenceTag::new("semantic_code"), semantic_code)
        .field_shape(WorthQueryEvidenceTag::new("detail"), detail);
    identity = match graph_capability {
        Some(semantics) => identity.field_evidence_identity(
            WorthQueryEvidenceTag::new("graph_capability"),
            &compose_graph_capability_identity(semantics),
        ),
        None => identity.field_shape(WorthQueryEvidenceTag::new("graph_capability"), "none"),
    };
    identity = match graph_invariant_denial {
        Some(semantics) => identity.field_evidence_identity(
            WorthQueryEvidenceTag::new("graph_invariant_denial"),
            &compose_graph_invariant_denial_identity(semantics),
        ),
        None => identity.field_shape(WorthQueryEvidenceTag::new("graph_invariant_denial"), "none"),
    };
    identity = match invariant_registration {
        Some(semantics) => identity.field_evidence_identity(
            WorthQueryEvidenceTag::new("invariant_registration"),
            &compose_invariant_registration_identity(semantics),
        ),
        None => identity.field_shape(WorthQueryEvidenceTag::new("invariant_registration"), "none"),
    };
    identity.seal()
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphCapabilityRuntimeSemantics {
    capability_family: String,
    capability_class: WorthQueryGraphCompositionCapabilityClass,
}

impl WorthQueryGraphCapabilityRuntimeSemantics {
    pub fn new(
        capability_family: impl Into<String>,
        capability_class: WorthQueryGraphCompositionCapabilityClass,
    ) -> Self {
        Self {
            capability_family: capability_family.into(),
            capability_class,
        }
    }

    pub fn capability_family(&self) -> &str {
        &self.capability_family
    }

    pub fn capability_class(&self) -> WorthQueryGraphCompositionCapabilityClass {
        self.capability_class
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphInvariantDenialRuntimeSemantics {
    invariant_family: String,
    declared_collections: Vec<String>,
    declared_symbols: Vec<String>,
    target_combination_families: Vec<String>,
    lifecycle_families: Vec<String>,
    program_identity: WorthQueryEvidenceIdentity,
    breadth_identity: WorthQueryEvidenceIdentity,
    counter_snapshot: String,
}

impl WorthQueryGraphInvariantDenialRuntimeSemantics {
    pub fn new(
        invariant_family: impl Into<String>,
        declared_collections: impl IntoIterator<Item = impl Into<String>>,
        declared_symbols: impl IntoIterator<Item = impl Into<String>>,
        target_combination_families: impl IntoIterator<Item = impl Into<String>>,
        lifecycle_families: impl IntoIterator<Item = impl Into<String>>,
        program_digest: impl Into<String>,
        breadth_digest: impl Into<String>,
        counter_snapshot: impl Into<String>,
    ) -> Self {
        let program_digest = program_digest.into();
        let breadth_digest = breadth_digest.into();
        Self {
            invariant_family: invariant_family.into(),
            declared_collections: declared_collections.into_iter().map(Into::into).collect(),
            declared_symbols: declared_symbols.into_iter().map(Into::into).collect(),
            target_combination_families: target_combination_families
                .into_iter()
                .map(Into::into)
                .collect(),
            lifecycle_families: lifecycle_families.into_iter().map(Into::into).collect(),
            program_identity: graph_invariant_semantics_identity("program", &program_digest),
            breadth_identity: graph_invariant_semantics_identity("breadth", &breadth_digest),
            counter_snapshot: counter_snapshot.into(),
        }
    }

    pub fn invariant_family(&self) -> &str {
        &self.invariant_family
    }

    pub fn declared_collections(&self) -> &[String] {
        &self.declared_collections
    }

    pub fn declared_symbols(&self) -> &[String] {
        &self.declared_symbols
    }

    pub fn target_combination_families(&self) -> &[String] {
        &self.target_combination_families
    }

    pub fn lifecycle_families(&self) -> &[String] {
        &self.lifecycle_families
    }

    pub fn program_digest(&self) -> &str {
        self.program_identity.as_str()
    }

    pub fn program_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.program_identity
    }

    pub fn breadth_digest(&self) -> &str {
        self.breadth_identity.as_str()
    }

    pub fn breadth_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.breadth_identity
    }

    pub fn counter_snapshot(&self) -> &str {
        &self.counter_snapshot
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryInvariantRegistrationRuntimeSemantics {
    invariant_catalog: InvariantCatalog,
    registration_identity: WorthQueryEvidenceIdentity,
}

impl WorthQueryInvariantRegistrationRuntimeSemantics {
    pub fn new(invariant_catalog: InvariantCatalog) -> Self {
        let registration_identity = Self::registration_identity_for_catalog(&invariant_catalog);
        Self {
            invariant_catalog,
            registration_identity,
        }
    }

    pub fn from_registration(registration: InvariantRegistration) -> Self {
        Self::new(InvariantCatalog {
            registrations: vec![registration],
        })
    }

    fn registration_identity_for_catalog(
        invariant_catalog: &InvariantCatalog,
    ) -> WorthQueryEvidenceIdentity {
        domain_capability_scope_encoder("worth_query_invariant_registration_runtime_semantics_v1")
            .field_shape(
                WorthQueryEvidenceTag::new("registration_label"),
                invariant_catalog.canonical_registration_digest(),
            )
            .seal()
    }

    pub fn invariant_catalog(&self) -> &InvariantCatalog {
        &self.invariant_catalog
    }

    pub fn canonical_invariant_catalog(&self) -> InvariantCatalog {
        self.invariant_catalog.canonicalized()
    }

    pub fn registration_digest(&self) -> String {
        self.registration_identity.as_str().to_string()
    }

    pub fn registration_identity(&self) -> WorthQueryEvidenceIdentity {
        self.registration_identity.clone()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryInvariantCapabilityContributionPayload {
    posture: WorthQueryInvariantCapabilityContributionPosture,
    semantic_code: String,
    detail: String,
    graph_capability: Option<WorthQueryGraphCapabilityRuntimeSemantics>,
    graph_invariant_denial: Option<WorthQueryGraphInvariantDenialRuntimeSemantics>,
    invariant_registration: Option<WorthQueryInvariantRegistrationRuntimeSemantics>,
    payload_identity: WorthQueryEvidenceIdentity,
}

impl WorthQueryInvariantCapabilityContributionPayload {
    pub fn new(
        posture: WorthQueryInvariantCapabilityContributionPosture,
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self {
        Self::with_graph_capability(posture, semantic_code, detail, None)
    }

    pub fn with_graph_capability(
        posture: WorthQueryInvariantCapabilityContributionPosture,
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
        graph_capability: Option<WorthQueryGraphCapabilityRuntimeSemantics>,
    ) -> Self {
        Self::with_runtime_semantics(posture, semantic_code, detail, graph_capability, None, None)
    }

    pub fn with_graph_invariant_denial(
        posture: WorthQueryInvariantCapabilityContributionPosture,
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
        graph_invariant_denial: Option<WorthQueryGraphInvariantDenialRuntimeSemantics>,
    ) -> Self {
        Self::with_runtime_semantics(
            posture,
            semantic_code,
            detail,
            None,
            graph_invariant_denial,
            None,
        )
    }

    pub fn with_invariant_registration(
        posture: WorthQueryInvariantCapabilityContributionPosture,
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
        invariant_registration: Option<WorthQueryInvariantRegistrationRuntimeSemantics>,
    ) -> Self {
        Self::with_runtime_semantics(
            posture,
            semantic_code,
            detail,
            None,
            None,
            invariant_registration,
        )
    }

    pub fn with_runtime_semantics(
        posture: WorthQueryInvariantCapabilityContributionPosture,
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
        graph_capability: Option<WorthQueryGraphCapabilityRuntimeSemantics>,
        graph_invariant_denial: Option<WorthQueryGraphInvariantDenialRuntimeSemantics>,
        invariant_registration: Option<WorthQueryInvariantRegistrationRuntimeSemantics>,
    ) -> Self {
        let semantic_code = semantic_code.into();
        let detail = detail.into();
        let payload_identity = compose_invariant_capability_payload_identity(
            posture,
            &semantic_code,
            &detail,
            graph_capability.as_ref(),
            graph_invariant_denial.as_ref(),
            invariant_registration.as_ref(),
        );
        Self {
            posture,
            semantic_code,
            detail,
            graph_capability,
            graph_invariant_denial,
            invariant_registration,
            payload_identity,
        }
    }

    pub fn category(&self) -> WorthQueryDomainCapabilityCategory {
        WorthQueryDomainCapabilityCategory::InvariantCapability
    }

    pub fn posture(&self) -> WorthQueryInvariantCapabilityContributionPosture {
        self.posture
    }

    pub fn semantic_code(&self) -> &str {
        &self.semantic_code
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }

    pub fn graph_capability(&self) -> Option<&WorthQueryGraphCapabilityRuntimeSemantics> {
        self.graph_capability.as_ref()
    }

    pub fn graph_invariant_denial(
        &self,
    ) -> Option<&WorthQueryGraphInvariantDenialRuntimeSemantics> {
        self.graph_invariant_denial.as_ref()
    }

    pub fn invariant_registration(
        &self,
    ) -> Option<&WorthQueryInvariantRegistrationRuntimeSemantics> {
        self.invariant_registration.as_ref()
    }

    pub fn payload_digest(&self) -> &str {
        self.payload_identity.as_str()
    }

    pub fn payload_for_reporting(&self) -> &str {
        self.payload_identity.as_str()
    }

    pub fn payload_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.payload_identity
    }
}

impl SealedPayload for WorthQueryInvariantCapabilityContributionPayload {}

impl WorthQueryDomainCapabilityPayload for WorthQueryInvariantCapabilityContributionPayload {
    fn category(&self) -> WorthQueryDomainCapabilityCategory {
        self.category()
    }

    fn posture_label(&self) -> &'static str {
        self.posture().as_str()
    }

    fn semantic_posture(&self) -> WorthQueryDomainCapabilitySemanticPosture {
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

    fn payload_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.payload_identity
    }
}
