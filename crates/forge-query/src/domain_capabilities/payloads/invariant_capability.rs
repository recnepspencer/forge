use crate::identity::hash_parts;
use crate::runtime::ForgeQueryGraphCompositionCapabilityClass;
use forge_relational::facade::runtime::{InvariantCatalog, InvariantRegistration};

use super::common::{
    ForgeQueryDomainCapabilityCategory, ForgeQueryDomainCapabilityPayload,
    ForgeQueryDomainCapabilitySemanticPosture, SealedPayload,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryInvariantCapabilityContributionPosture {
    CapabilityGap,
    InvariantDenial,
    SupportSummary,
    InvariantRegistration,
}

impl ForgeQueryInvariantCapabilityContributionPosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CapabilityGap => "capability-gap",
            Self::InvariantDenial => "invariant-denial",
            Self::SupportSummary => "support-summary",
            Self::InvariantRegistration => "invariant-registration",
        }
    }

    pub const fn semantic_posture(self) -> ForgeQueryDomainCapabilitySemanticPosture {
        match self {
            Self::CapabilityGap => {
                ForgeQueryDomainCapabilitySemanticPosture::InvariantCapabilityGap
            }
            Self::InvariantDenial => ForgeQueryDomainCapabilitySemanticPosture::InvariantDenial,
            Self::SupportSummary => {
                ForgeQueryDomainCapabilitySemanticPosture::InvariantSupportSummary
            }
            Self::InvariantRegistration => {
                ForgeQueryDomainCapabilitySemanticPosture::InvariantRegistration
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphCapabilityRuntimeSemantics {
    capability_family: String,
    capability_class: ForgeQueryGraphCompositionCapabilityClass,
}

impl ForgeQueryGraphCapabilityRuntimeSemantics {
    pub fn new(
        capability_family: impl Into<String>,
        capability_class: ForgeQueryGraphCompositionCapabilityClass,
    ) -> Self {
        Self {
            capability_family: capability_family.into(),
            capability_class,
        }
    }

    pub fn capability_family(&self) -> &str {
        &self.capability_family
    }

    pub fn capability_class(&self) -> ForgeQueryGraphCompositionCapabilityClass {
        self.capability_class
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphInvariantDenialRuntimeSemantics {
    invariant_family: String,
    declared_collections: Vec<String>,
    declared_symbols: Vec<String>,
    target_combination_families: Vec<String>,
    lifecycle_families: Vec<String>,
    program_digest: String,
    breadth_digest: String,
    counter_snapshot: String,
}

impl ForgeQueryGraphInvariantDenialRuntimeSemantics {
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
        Self {
            invariant_family: invariant_family.into(),
            declared_collections: declared_collections.into_iter().map(Into::into).collect(),
            declared_symbols: declared_symbols.into_iter().map(Into::into).collect(),
            target_combination_families: target_combination_families
                .into_iter()
                .map(Into::into)
                .collect(),
            lifecycle_families: lifecycle_families.into_iter().map(Into::into).collect(),
            program_digest: program_digest.into(),
            breadth_digest: breadth_digest.into(),
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
        &self.program_digest
    }

    pub fn breadth_digest(&self) -> &str {
        &self.breadth_digest
    }

    pub fn counter_snapshot(&self) -> &str {
        &self.counter_snapshot
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryInvariantRegistrationRuntimeSemantics {
    invariant_catalog: InvariantCatalog,
}

impl ForgeQueryInvariantRegistrationRuntimeSemantics {
    pub fn new(invariant_catalog: InvariantCatalog) -> Self {
        Self { invariant_catalog }
    }

    pub fn from_registration(registration: InvariantRegistration) -> Self {
        Self::new(InvariantCatalog {
            registrations: vec![registration],
        })
    }

    pub fn invariant_catalog(&self) -> &InvariantCatalog {
        &self.invariant_catalog
    }

    pub fn canonical_invariant_catalog(&self) -> InvariantCatalog {
        self.invariant_catalog.canonicalized()
    }

    pub fn registration_digest(&self) -> String {
        self.invariant_catalog.canonical_registration_digest()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryInvariantCapabilityContributionPayload {
    posture: ForgeQueryInvariantCapabilityContributionPosture,
    semantic_code: String,
    detail: String,
    graph_capability: Option<ForgeQueryGraphCapabilityRuntimeSemantics>,
    graph_invariant_denial: Option<ForgeQueryGraphInvariantDenialRuntimeSemantics>,
    invariant_registration: Option<ForgeQueryInvariantRegistrationRuntimeSemantics>,
    payload_digest: String,
}

impl ForgeQueryInvariantCapabilityContributionPayload {
    pub fn new(
        posture: ForgeQueryInvariantCapabilityContributionPosture,
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self {
        Self::with_graph_capability(posture, semantic_code, detail, None)
    }

    pub fn with_graph_capability(
        posture: ForgeQueryInvariantCapabilityContributionPosture,
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
        graph_capability: Option<ForgeQueryGraphCapabilityRuntimeSemantics>,
    ) -> Self {
        Self::with_runtime_semantics(posture, semantic_code, detail, graph_capability, None, None)
    }

    pub fn with_graph_invariant_denial(
        posture: ForgeQueryInvariantCapabilityContributionPosture,
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
        graph_invariant_denial: Option<ForgeQueryGraphInvariantDenialRuntimeSemantics>,
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
        posture: ForgeQueryInvariantCapabilityContributionPosture,
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
        invariant_registration: Option<ForgeQueryInvariantRegistrationRuntimeSemantics>,
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
        posture: ForgeQueryInvariantCapabilityContributionPosture,
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
        graph_capability: Option<ForgeQueryGraphCapabilityRuntimeSemantics>,
        graph_invariant_denial: Option<ForgeQueryGraphInvariantDenialRuntimeSemantics>,
        invariant_registration: Option<ForgeQueryInvariantRegistrationRuntimeSemantics>,
    ) -> Self {
        let semantic_code = semantic_code.into();
        let detail = detail.into();
        let graph_capability_digest = graph_capability.as_ref().map_or_else(
            || "none".to_string(),
            |graph_capability| {
                format!(
                    "{}:{}",
                    graph_capability.capability_family(),
                    graph_capability.capability_class().as_str()
                )
            },
        );
        let graph_invariant_denial_digest = graph_invariant_denial.as_ref().map_or_else(
            || "none".to_string(),
            |graph_invariant_denial| {
                format!(
                    "{}:{}:{}:{}:{}:{}:{}",
                    graph_invariant_denial.invariant_family(),
                    graph_invariant_denial.declared_collections().join("|"),
                    graph_invariant_denial.declared_symbols().join("|"),
                    graph_invariant_denial
                        .target_combination_families()
                        .join("|"),
                    graph_invariant_denial.lifecycle_families().join("|"),
                    graph_invariant_denial.program_digest(),
                    graph_invariant_denial.breadth_digest(),
                )
            },
        );
        let invariant_registration_digest = invariant_registration.as_ref().map_or_else(
            || "none".to_string(),
            ForgeQueryInvariantRegistrationRuntimeSemantics::registration_digest,
        );
        let payload_digest = hash_parts(&[
            "forge_query_domain_capability_payload_v2".to_string(),
            format!(
                "category:{}",
                ForgeQueryDomainCapabilityCategory::InvariantCapability.as_str()
            ),
            format!("posture:{}", posture.as_str()),
            format!("semantic_code:{semantic_code}"),
            format!("detail:{detail}"),
            format!("graph_capability:{graph_capability_digest}"),
            format!("graph_invariant_denial:{graph_invariant_denial_digest}"),
            format!("invariant_registration:{invariant_registration_digest}"),
        ]);
        Self {
            posture,
            semantic_code,
            detail,
            graph_capability,
            graph_invariant_denial,
            invariant_registration,
            payload_digest,
        }
    }

    pub fn category(&self) -> ForgeQueryDomainCapabilityCategory {
        ForgeQueryDomainCapabilityCategory::InvariantCapability
    }

    pub fn posture(&self) -> ForgeQueryInvariantCapabilityContributionPosture {
        self.posture
    }

    pub fn semantic_code(&self) -> &str {
        &self.semantic_code
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }

    pub fn graph_capability(&self) -> Option<&ForgeQueryGraphCapabilityRuntimeSemantics> {
        self.graph_capability.as_ref()
    }

    pub fn graph_invariant_denial(
        &self,
    ) -> Option<&ForgeQueryGraphInvariantDenialRuntimeSemantics> {
        self.graph_invariant_denial.as_ref()
    }

    pub fn invariant_registration(
        &self,
    ) -> Option<&ForgeQueryInvariantRegistrationRuntimeSemantics> {
        self.invariant_registration.as_ref()
    }

    pub fn payload_digest(&self) -> &str {
        &self.payload_digest
    }
}

impl SealedPayload for ForgeQueryInvariantCapabilityContributionPayload {}

impl ForgeQueryDomainCapabilityPayload for ForgeQueryInvariantCapabilityContributionPayload {
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
