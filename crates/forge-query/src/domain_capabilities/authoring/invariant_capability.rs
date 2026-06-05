use crate::runtime::{
    ForgeQueryIntentDeclaration, ForgeQueryLowerRuntimeBoundaryEnvelope,
    ForgeQueryLowerRuntimeBoundaryEnvelopeSource,
};
use forge_relational::facade::runtime::{InvariantCatalog, InvariantRegistration};

use super::bind_requested;
use crate::domain_capabilities::payloads::{
    ForgeQueryGraphCapabilityRuntimeSemantics, ForgeQueryGraphInvariantDenialRuntimeSemantics,
    ForgeQueryInvariantCapabilityContributionPayload,
    ForgeQueryInvariantCapabilityContributionPosture,
    ForgeQueryInvariantRegistrationRuntimeSemantics,
};
use crate::domain_capabilities::proof_integration::ForgeQueryRequestedInvariantCapabilityContribution;
use crate::domain_capabilities::targets::{
    ForgeQueryDeclarationBoundContributionTarget,
    ForgeQueryLowerRuntimeBoundaryBoundContributionTarget,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryInvariantCapabilityContributionAuthoring {
    payload: ForgeQueryInvariantCapabilityContributionPayload,
}

impl ForgeQueryInvariantCapabilityContributionAuthoring {
    pub fn capability_gap(semantic_code: impl Into<String>, detail: impl Into<String>) -> Self {
        Self::new(
            ForgeQueryInvariantCapabilityContributionPosture::CapabilityGap,
            semantic_code,
            detail,
        )
    }

    pub fn graph_capability_gap(
        capability_family: impl Into<String>,
        capability_class: crate::runtime::ForgeQueryGraphCompositionCapabilityClass,
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self {
        Self::with_graph_capability(
            ForgeQueryInvariantCapabilityContributionPosture::CapabilityGap,
            semantic_code,
            detail,
            ForgeQueryGraphCapabilityRuntimeSemantics::new(capability_family, capability_class),
        )
    }

    pub fn invariant_denial(semantic_code: impl Into<String>, detail: impl Into<String>) -> Self {
        Self::new(
            ForgeQueryInvariantCapabilityContributionPosture::InvariantDenial,
            semantic_code,
            detail,
        )
    }

    pub fn graph_invariant_denial(
        invariant_family: impl Into<String>,
        declared_collections: impl IntoIterator<Item = impl Into<String>>,
        declared_symbols: impl IntoIterator<Item = impl Into<String>>,
        target_combination_families: impl IntoIterator<Item = impl Into<String>>,
        lifecycle_families: impl IntoIterator<Item = impl Into<String>>,
        program_digest: impl Into<String>,
        breadth_digest: impl Into<String>,
        counter_snapshot: impl Into<String>,
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self {
        Self::with_graph_invariant_denial(
            ForgeQueryInvariantCapabilityContributionPosture::InvariantDenial,
            semantic_code,
            detail,
            ForgeQueryGraphInvariantDenialRuntimeSemantics::new(
                invariant_family,
                declared_collections,
                declared_symbols,
                target_combination_families,
                lifecycle_families,
                program_digest,
                breadth_digest,
                counter_snapshot,
            ),
        )
    }

    pub fn support_summary(semantic_code: impl Into<String>, detail: impl Into<String>) -> Self {
        Self::new(
            ForgeQueryInvariantCapabilityContributionPosture::SupportSummary,
            semantic_code,
            detail,
        )
    }

    pub fn graph_support_summary(
        capability_family: impl Into<String>,
        capability_class: crate::runtime::ForgeQueryGraphCompositionCapabilityClass,
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self {
        Self::with_graph_capability(
            ForgeQueryInvariantCapabilityContributionPosture::SupportSummary,
            semantic_code,
            detail,
            ForgeQueryGraphCapabilityRuntimeSemantics::new(capability_family, capability_class),
        )
    }

    pub fn invariant_registration(
        invariant_catalog: InvariantCatalog,
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self {
        Self::with_invariant_registration(
            ForgeQueryInvariantCapabilityContributionPosture::InvariantRegistration,
            semantic_code,
            detail,
            ForgeQueryInvariantRegistrationRuntimeSemantics::new(invariant_catalog),
        )
    }

    pub fn invariant_rule_registration(
        registration: InvariantRegistration,
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self {
        Self::with_invariant_registration(
            ForgeQueryInvariantCapabilityContributionPosture::InvariantRegistration,
            semantic_code,
            detail,
            ForgeQueryInvariantRegistrationRuntimeSemantics::from_registration(registration),
        )
    }

    pub fn for_intent_declaration(
        self,
        declaration: &ForgeQueryIntentDeclaration,
    ) -> ForgeQueryRequestedInvariantCapabilityContribution<
        ForgeQueryDeclarationBoundContributionTarget,
    > {
        self.bind_to_declaration_target(
            ForgeQueryDeclarationBoundContributionTarget::for_intent_declaration(declaration),
        )
    }

    pub fn for_lower_runtime_boundary_envelope(
        self,
        envelope: &ForgeQueryLowerRuntimeBoundaryEnvelope,
    ) -> ForgeQueryRequestedInvariantCapabilityContribution<
        ForgeQueryLowerRuntimeBoundaryBoundContributionTarget,
    > {
        self.bind_to_lower_runtime_boundary_target(
            ForgeQueryLowerRuntimeBoundaryBoundContributionTarget::for_lower_runtime_boundary_envelope(
                envelope,
            ),
        )
    }

    pub fn for_lower_runtime_boundary_source<S>(
        self,
        source: &S,
    ) -> ForgeQueryRequestedInvariantCapabilityContribution<
        ForgeQueryLowerRuntimeBoundaryBoundContributionTarget,
    >
    where
        S: ForgeQueryLowerRuntimeBoundaryEnvelopeSource + ?Sized,
    {
        self.for_lower_runtime_boundary_envelope(source.lower_runtime_boundary_envelope())
    }

    pub fn bind_to_declaration_target(
        self,
        target: ForgeQueryDeclarationBoundContributionTarget,
    ) -> ForgeQueryRequestedInvariantCapabilityContribution<
        ForgeQueryDeclarationBoundContributionTarget,
    > {
        bind_requested(self.payload, target)
    }

    pub fn bind_to_lower_runtime_boundary_target(
        self,
        target: ForgeQueryLowerRuntimeBoundaryBoundContributionTarget,
    ) -> ForgeQueryRequestedInvariantCapabilityContribution<
        ForgeQueryLowerRuntimeBoundaryBoundContributionTarget,
    > {
        bind_requested(self.payload, target)
    }

    fn new(
        posture: ForgeQueryInvariantCapabilityContributionPosture,
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            payload: ForgeQueryInvariantCapabilityContributionPayload::new(
                posture,
                semantic_code,
                detail,
            ),
        }
    }

    fn with_graph_capability(
        posture: ForgeQueryInvariantCapabilityContributionPosture,
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
        graph_capability: ForgeQueryGraphCapabilityRuntimeSemantics,
    ) -> Self {
        Self {
            payload: ForgeQueryInvariantCapabilityContributionPayload::with_graph_capability(
                posture,
                semantic_code,
                detail,
                Some(graph_capability),
            ),
        }
    }

    fn with_graph_invariant_denial(
        posture: ForgeQueryInvariantCapabilityContributionPosture,
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
        graph_invariant_denial: ForgeQueryGraphInvariantDenialRuntimeSemantics,
    ) -> Self {
        Self {
            payload: ForgeQueryInvariantCapabilityContributionPayload::with_graph_invariant_denial(
                posture,
                semantic_code,
                detail,
                Some(graph_invariant_denial),
            ),
        }
    }

    fn with_invariant_registration(
        posture: ForgeQueryInvariantCapabilityContributionPosture,
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
        invariant_registration: ForgeQueryInvariantRegistrationRuntimeSemantics,
    ) -> Self {
        Self {
            payload: ForgeQueryInvariantCapabilityContributionPayload::with_invariant_registration(
                posture,
                semantic_code,
                detail,
                Some(invariant_registration),
            ),
        }
    }
}
