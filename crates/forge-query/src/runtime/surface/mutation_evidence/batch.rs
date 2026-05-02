use forge_runtime_bridge::facade::BridgeBatchMutationAuthorityBundle;

use super::batch_digest_helpers::{
    batch_continuity_mutation_digest, batch_existing_truth_assertion_digest,
    batch_existing_truth_binding_digest, batch_naming_mutation_digest,
    batch_symbolic_resolution_digest, batch_symbolic_target_reference_digest, batch_target_digest,
};
use super::{
    binding::ForgeQueryExistingTruthBindingEvidence, target::ForgeQueryMutationTargetClass,
    ForgeQueryExistingTruthAssertionEvidence, ForgeQueryMutationTargetEvidence,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryBatchMutationEvidence {
    component_count: usize,
    target_evidence_count: usize,
    existing_truth_assertion_count: usize,
    retained_authoritative_assertion_count: usize,
    backend_verified_assertion_count: usize,
    backend_verified_update_count: usize,
    backend_verified_delete_count: usize,
    existing_truth_binding_count: usize,
    symbolic_target_reference_count: usize,
    symbolic_resolution_count: usize,
    naming_mutation_count: usize,
    continuity_mutation_count: usize,
    resolved_target_count: usize,
    target_collection_count: usize,
    target_entity_count: usize,
    aggregate_existing_truth_assertion_digest: Option<String>,
    aggregate_existing_truth_mode_digest: Option<String>,
    aggregate_existing_truth_binding_digest: Option<String>,
    aggregate_symbolic_target_reference_digest: Option<String>,
    aggregate_symbolic_resolution_digest: Option<String>,
    aggregate_naming_mutation_digest: Option<String>,
    aggregate_continuity_mutation_digest: Option<String>,
    causality_bundle_count: usize,
    provenance_bundle_count: usize,
    outcome_class_count: usize,
    request_digest_count: usize,
    receipt_digest_count: usize,
    aggregate_target_digest: String,
    aggregate_causality_digest: Option<String>,
    aggregate_provenance_digest: Option<String>,
}

impl ForgeQueryBatchMutationEvidence {
    pub(in crate::runtime) fn from_components(
        mutation_families: &[crate::runtime::ForgeQueryMutationFamily],
        components: &[ForgeQueryMutationTargetEvidence],
        existing_truth_assertions: &[Option<ForgeQueryExistingTruthAssertionEvidence>],
        existing_truth_bindings: &[Option<ForgeQueryExistingTruthBindingEvidence>],
        symbolic_target_references: &[Option<
            crate::runtime::ForgeQuerySymbolicTargetReferenceEvidence,
        >],
        symbolic_aspect_resolutions: &[Vec<
            crate::runtime::ForgeQuerySymbolicAspectResolutionEvidence,
        >],
        naming_mutations: &[Option<crate::runtime::ForgeQueryNamingMutationEvidence>],
        continuity_mutations: &[Option<crate::runtime::ForgeQueryContinuityMutationEvidence>],
        aggregate_bridge: Option<&BridgeBatchMutationAuthorityBundle>,
    ) -> Option<Self> {
        if components.is_empty() {
            return None;
        }
        let (
            retained_authoritative_assertion_count,
            backend_verified_assertion_count,
            backend_verified_update_count,
            backend_verified_delete_count,
            aggregate_existing_truth_mode_digest,
        ) = summarize_existing_truth_modes(mutation_families, existing_truth_assertions);

        let resolved_target_count = components
            .iter()
            .filter(|component| component.resolved().entity_identity().is_some())
            .count();
        let target_collection_count = components
            .iter()
            .filter(|component| {
                component.resolved().target_class() == ForgeQueryMutationTargetClass::Collection
                    && component.resolved().collection().is_some()
            })
            .count();
        let target_entity_count = components
            .iter()
            .filter(|component| {
                component.resolved().target_class() == ForgeQueryMutationTargetClass::Entity
            })
            .count();

        Some(Self {
            component_count: components.len(),
            target_evidence_count: components.len(),
            existing_truth_assertion_count: existing_truth_assertions
                .iter()
                .filter(
                    |assertion: &&Option<ForgeQueryExistingTruthAssertionEvidence>| {
                        assertion.is_some()
                    },
                )
                .count(),
            retained_authoritative_assertion_count,
            backend_verified_assertion_count,
            backend_verified_update_count,
            backend_verified_delete_count,
            existing_truth_binding_count: existing_truth_bindings
                .iter()
                .filter(|binding| binding.is_some())
                .count(),
            symbolic_target_reference_count: symbolic_target_references
                .iter()
                .filter(|reference| reference.is_some())
                .count(),
            symbolic_resolution_count: symbolic_target_references
                .iter()
                .filter(|reference| reference.is_some())
                .count()
                + symbolic_aspect_resolutions
                    .iter()
                    .map(Vec::len)
                    .sum::<usize>(),
            naming_mutation_count: naming_mutations
                .iter()
                .filter(|naming| naming.is_some())
                .count(),
            continuity_mutation_count: continuity_mutations
                .iter()
                .filter(|continuity| continuity.is_some())
                .count(),
            resolved_target_count,
            target_collection_count,
            target_entity_count,
            aggregate_existing_truth_assertion_digest: batch_existing_truth_assertion_digest(
                existing_truth_assertions,
            ),
            aggregate_existing_truth_mode_digest,
            aggregate_existing_truth_binding_digest: aggregate_bridge
                .and_then(|bundle| bundle.aggregate_existing_truth_binding_digest())
                .map(str::to_string)
                .or_else(|| batch_existing_truth_binding_digest(existing_truth_bindings)),
            aggregate_symbolic_target_reference_digest: aggregate_bridge
                .and_then(|bundle| bundle.aggregate_symbolic_target_reference_digest())
                .map(str::to_string)
                .or_else(|| batch_symbolic_target_reference_digest(symbolic_target_references)),
            aggregate_symbolic_resolution_digest: batch_symbolic_resolution_digest(
                symbolic_target_references,
                symbolic_aspect_resolutions,
            ),
            aggregate_naming_mutation_digest: aggregate_bridge
                .and_then(|bundle| bundle.aggregate_naming_mutation_digest())
                .map(str::to_string)
                .or_else(|| batch_naming_mutation_digest(naming_mutations)),
            aggregate_continuity_mutation_digest: aggregate_bridge
                .and_then(|bundle| bundle.aggregate_continuity_mutation_digest())
                .map(str::to_string)
                .or_else(|| batch_continuity_mutation_digest(continuity_mutations)),
            causality_bundle_count: aggregate_bridge
                .map_or(0, |bundle| bundle.causality_bundle_count()),
            provenance_bundle_count: aggregate_bridge
                .map_or(0, |bundle| bundle.provenance_bundle_count()),
            outcome_class_count: aggregate_bridge.map_or(0, |bundle| bundle.outcome_class_count()),
            request_digest_count: aggregate_bridge
                .map_or(0, |bundle| bundle.request_digest_count()),
            receipt_digest_count: aggregate_bridge
                .map_or(0, |bundle| bundle.receipt_digest_count()),
            aggregate_target_digest: batch_target_digest(components),
            aggregate_causality_digest: aggregate_bridge
                .map(|bundle| bundle.aggregate_causality_digest().to_string()),
            aggregate_provenance_digest: aggregate_bridge
                .map(|bundle| bundle.aggregate_provenance_digest().to_string()),
        })
    }

    pub fn component_count(&self) -> usize {
        self.component_count
    }

    pub fn target_evidence_count(&self) -> usize {
        self.target_evidence_count
    }

    pub fn existing_truth_assertion_count(&self) -> usize {
        self.existing_truth_assertion_count
    }

    pub fn retained_authoritative_assertion_count(&self) -> usize {
        self.retained_authoritative_assertion_count
    }

    pub fn backend_verified_assertion_count(&self) -> usize {
        self.backend_verified_assertion_count
    }

    pub fn backend_verified_update_count(&self) -> usize {
        self.backend_verified_update_count
    }

    pub fn backend_verified_delete_count(&self) -> usize {
        self.backend_verified_delete_count
    }

    pub fn resolved_target_count(&self) -> usize {
        self.resolved_target_count
    }

    pub fn existing_truth_binding_count(&self) -> usize {
        self.existing_truth_binding_count
    }

    pub fn symbolic_target_reference_count(&self) -> usize {
        self.symbolic_target_reference_count
    }

    pub fn symbolic_resolution_count(&self) -> usize {
        self.symbolic_resolution_count
    }

    pub fn naming_mutation_count(&self) -> usize {
        self.naming_mutation_count
    }

    pub fn continuity_mutation_count(&self) -> usize {
        self.continuity_mutation_count
    }

    pub fn target_collection_count(&self) -> usize {
        self.target_collection_count
    }

    pub fn target_entity_count(&self) -> usize {
        self.target_entity_count
    }

    pub fn aggregate_existing_truth_assertion_digest(&self) -> Option<&str> {
        self.aggregate_existing_truth_assertion_digest.as_deref()
    }

    pub fn aggregate_existing_truth_mode_digest(&self) -> Option<&str> {
        self.aggregate_existing_truth_mode_digest.as_deref()
    }

    pub fn aggregate_existing_truth_binding_digest(&self) -> Option<&str> {
        self.aggregate_existing_truth_binding_digest.as_deref()
    }

    pub fn aggregate_symbolic_target_reference_digest(&self) -> Option<&str> {
        self.aggregate_symbolic_target_reference_digest.as_deref()
    }

    pub fn aggregate_symbolic_resolution_digest(&self) -> Option<&str> {
        self.aggregate_symbolic_resolution_digest.as_deref()
    }

    pub fn aggregate_naming_mutation_digest(&self) -> Option<&str> {
        self.aggregate_naming_mutation_digest.as_deref()
    }

    pub fn aggregate_continuity_mutation_digest(&self) -> Option<&str> {
        self.aggregate_continuity_mutation_digest.as_deref()
    }

    pub fn causality_bundle_count(&self) -> usize {
        self.causality_bundle_count
    }

    pub fn provenance_bundle_count(&self) -> usize {
        self.provenance_bundle_count
    }

    pub fn outcome_class_count(&self) -> usize {
        self.outcome_class_count
    }

    pub fn request_digest_count(&self) -> usize {
        self.request_digest_count
    }

    pub fn receipt_digest_count(&self) -> usize {
        self.receipt_digest_count
    }

    pub fn aggregate_target_digest(&self) -> &str {
        &self.aggregate_target_digest
    }

    pub fn aggregate_causality_digest(&self) -> Option<&str> {
        self.aggregate_causality_digest.as_deref()
    }

    pub fn aggregate_provenance_digest(&self) -> Option<&str> {
        self.aggregate_provenance_digest.as_deref()
    }
}

fn summarize_existing_truth_modes(
    mutation_families: &[crate::runtime::ForgeQueryMutationFamily],
    existing_truth_assertions: &[Option<ForgeQueryExistingTruthAssertionEvidence>],
) -> (usize, usize, usize, usize, Option<String>) {
    let mut retained_authoritative_assertion_count = 0;
    let mut backend_verified_assertion_count = 0;
    let mut backend_verified_update_count = 0;
    let mut backend_verified_delete_count = 0;
    let mode_parts = mutation_families
        .iter()
        .zip(existing_truth_assertions.iter())
        .filter_map(|(family, assertion)| {
            let assertion = assertion.as_ref()?;
            match (family, assertion.mode()) {
                (
                    crate::runtime::ForgeQueryMutationFamily::Assertion,
                    crate::runtime::ForgeQueryExistingTruthAssertionMode::RetainedAuthoritativeAssertion,
                ) => retained_authoritative_assertion_count += 1,
                (
                    crate::runtime::ForgeQueryMutationFamily::Assertion,
                    crate::runtime::ForgeQueryExistingTruthAssertionMode::BackendVerifiedAssertion,
                ) => backend_verified_assertion_count += 1,
                (
                    crate::runtime::ForgeQueryMutationFamily::Update,
                    crate::runtime::ForgeQueryExistingTruthAssertionMode::BackendVerifiedAssertion,
                ) => backend_verified_update_count += 1,
                (
                    crate::runtime::ForgeQueryMutationFamily::Delete,
                    crate::runtime::ForgeQueryExistingTruthAssertionMode::BackendVerifiedAssertion,
                ) => backend_verified_delete_count += 1,
                (
                    family,
                    mode,
                ) => {
                    panic!(
                        "invalid existing-truth assertion mode `{mode}` for mutation family `{family}`"
                    )
                }
            }
            Some(format!(
                "{}:{}:{}",
                family,
                assertion.mode(),
                assertion.verification_digest()
            ))
        })
        .collect::<Vec<_>>();
    let digest = (!mode_parts.is_empty()).then(|| {
        crate::identity::hash_parts(
            &std::iter::once("forge-query-batch-existing-truth-mode-v1".to_string())
                .chain(mode_parts)
                .collect::<Vec<_>>(),
        )
    });
    (
        retained_authoritative_assertion_count,
        backend_verified_assertion_count,
        backend_verified_update_count,
        backend_verified_delete_count,
        digest,
    )
}

#[cfg(test)]
mod batch_tests;
