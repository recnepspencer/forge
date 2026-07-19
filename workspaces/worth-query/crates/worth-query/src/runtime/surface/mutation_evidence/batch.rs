use worth_runtime_bridge::facade::BridgeBatchMutationAuthorityBundle;

use super::batch_digest_helpers::{
    batch_continuity_mutation_digest, batch_existing_truth_assertion_digest,
    batch_existing_truth_binding_digest, batch_naming_mutation_digest,
    batch_symbolic_resolution_digest, batch_symbolic_target_reference_digest, batch_target_digest,
};
use super::{
    binding::WorthQueryExistingTruthBindingEvidence, target::WorthQueryMutationTargetClass,
    WorthQueryExistingTruthAssertionEvidence, WorthQueryMutationTargetEvidence,
};
use crate::runtime::WorthQueryMutationEvidenceDigest;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryBatchMutationEvidence {
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
    aggregate_existing_truth_assertion_digest: Option<WorthQueryMutationEvidenceDigest>,
    aggregate_existing_truth_mode_digest: Option<WorthQueryMutationEvidenceDigest>,
    aggregate_existing_truth_binding_digest: Option<WorthQueryMutationEvidenceDigest>,
    aggregate_symbolic_target_reference_digest: Option<WorthQueryMutationEvidenceDigest>,
    aggregate_symbolic_resolution_digest: Option<WorthQueryMutationEvidenceDigest>,
    aggregate_naming_mutation_digest: Option<WorthQueryMutationEvidenceDigest>,
    aggregate_continuity_mutation_digest: Option<WorthQueryMutationEvidenceDigest>,
    causality_bundle_count: usize,
    provenance_bundle_count: usize,
    outcome_class_count: usize,
    authority_request_count: usize,
    authority_receipt_count: usize,
    aggregate_target_digest: WorthQueryMutationEvidenceDigest,
    aggregate_causality_digest: Option<WorthQueryMutationEvidenceDigest>,
    aggregate_provenance_digest: Option<WorthQueryMutationEvidenceDigest>,
}

impl WorthQueryBatchMutationEvidence {
    pub(in crate::runtime) fn from_components(
        mutation_families: &[crate::runtime::WorthQueryMutationFamily],
        components: &[WorthQueryMutationTargetEvidence],
        existing_truth_assertions: &[Option<WorthQueryExistingTruthAssertionEvidence>],
        existing_truth_bindings: &[Option<WorthQueryExistingTruthBindingEvidence>],
        symbolic_target_references: &[Option<
            crate::runtime::WorthQuerySymbolicTargetReferenceEvidence,
        >],
        symbolic_aspect_resolutions: &[Vec<
            crate::runtime::WorthQuerySymbolicAspectResolutionEvidence,
        >],
        naming_mutations: &[Option<crate::runtime::WorthQueryNamingMutationEvidence>],
        continuity_mutations: &[Option<crate::runtime::WorthQueryContinuityMutationEvidence>],
        causality_evidence: &[Option<crate::runtime::WorthQueryMutationCausalityEvidence>],
        provenance_evidence: &[Option<crate::runtime::WorthQueryMutationProvenanceEvidence>],
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
                component.resolved().target_class() == WorthQueryMutationTargetClass::Collection
                    && component.resolved().collection().is_some()
            })
            .count();
        let target_entity_count = components
            .iter()
            .filter(|component| {
                component.resolved().target_class() == WorthQueryMutationTargetClass::Entity
            })
            .count();

        Some(Self {
            component_count: components.len(),
            target_evidence_count: components.len(),
            existing_truth_assertion_count: existing_truth_assertions
                .iter()
                .filter(
                    |assertion: &&Option<WorthQueryExistingTruthAssertionEvidence>| {
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
            aggregate_existing_truth_binding_digest: batch_existing_truth_binding_digest(
                existing_truth_bindings,
            ),
            aggregate_symbolic_target_reference_digest: batch_symbolic_target_reference_digest(
                symbolic_target_references,
            ),
            aggregate_symbolic_resolution_digest: batch_symbolic_resolution_digest(
                symbolic_target_references,
                symbolic_aspect_resolutions,
            ),
            aggregate_naming_mutation_digest: batch_naming_mutation_digest(naming_mutations),
            aggregate_continuity_mutation_digest: batch_continuity_mutation_digest(
                continuity_mutations,
            ),
            causality_bundle_count: aggregate_bridge
                .map_or(0, |bundle| bundle.causality_bundle_count()),
            provenance_bundle_count: aggregate_bridge
                .map_or(0, |bundle| bundle.provenance_bundle_count()),
            outcome_class_count: aggregate_bridge.map_or(0, |bundle| bundle.outcome_class_count()),
            authority_request_count: aggregate_bridge
                .map_or(0, |bundle| bundle.authority_request_count()),
            authority_receipt_count: aggregate_bridge
                .map_or(0, |bundle| bundle.authority_receipt_count()),
            aggregate_target_digest: batch_target_digest(components),
            aggregate_causality_digest: causality_evidence
                .iter()
                .find_map(Option::as_ref)
                .map(|evidence| evidence.causality_digest().clone()),
            aggregate_provenance_digest: provenance_evidence
                .iter()
                .find_map(Option::as_ref)
                .map(|evidence| evidence.execution_record_digest().clone()),
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

    pub fn aggregate_existing_truth_assertion_digest(
        &self,
    ) -> Option<&WorthQueryMutationEvidenceDigest> {
        self.aggregate_existing_truth_assertion_digest.as_ref()
    }

    pub fn aggregate_existing_truth_mode_digest(
        &self,
    ) -> Option<&WorthQueryMutationEvidenceDigest> {
        self.aggregate_existing_truth_mode_digest.as_ref()
    }

    pub fn aggregate_existing_truth_binding_digest(
        &self,
    ) -> Option<&WorthQueryMutationEvidenceDigest> {
        self.aggregate_existing_truth_binding_digest.as_ref()
    }

    pub fn aggregate_symbolic_target_reference_digest(
        &self,
    ) -> Option<&WorthQueryMutationEvidenceDigest> {
        self.aggregate_symbolic_target_reference_digest.as_ref()
    }

    pub fn aggregate_symbolic_resolution_digest(
        &self,
    ) -> Option<&WorthQueryMutationEvidenceDigest> {
        self.aggregate_symbolic_resolution_digest.as_ref()
    }

    pub fn aggregate_naming_mutation_digest(&self) -> Option<&WorthQueryMutationEvidenceDigest> {
        self.aggregate_naming_mutation_digest.as_ref()
    }

    pub fn aggregate_continuity_mutation_digest(
        &self,
    ) -> Option<&WorthQueryMutationEvidenceDigest> {
        self.aggregate_continuity_mutation_digest.as_ref()
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

    pub fn authority_request_count(&self) -> usize {
        self.authority_request_count
    }

    pub fn authority_receipt_count(&self) -> usize {
        self.authority_receipt_count
    }

    pub fn aggregate_target_digest(&self) -> &WorthQueryMutationEvidenceDigest {
        &self.aggregate_target_digest
    }

    pub fn aggregate_causality_digest(&self) -> Option<&WorthQueryMutationEvidenceDigest> {
        self.aggregate_causality_digest.as_ref()
    }

    pub fn aggregate_provenance_digest(&self) -> Option<&WorthQueryMutationEvidenceDigest> {
        self.aggregate_provenance_digest.as_ref()
    }
}

fn summarize_existing_truth_modes(
    mutation_families: &[crate::runtime::WorthQueryMutationFamily],
    existing_truth_assertions: &[Option<WorthQueryExistingTruthAssertionEvidence>],
) -> (
    usize,
    usize,
    usize,
    usize,
    Option<WorthQueryMutationEvidenceDigest>,
) {
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
                    crate::runtime::WorthQueryMutationFamily::Assertion,
                    crate::runtime::WorthQueryExistingTruthAssertionMode::RetainedAuthoritativeAssertion,
                ) => retained_authoritative_assertion_count += 1,
                (
                    crate::runtime::WorthQueryMutationFamily::Assertion,
                    crate::runtime::WorthQueryExistingTruthAssertionMode::BackendVerifiedAssertion,
                ) => backend_verified_assertion_count += 1,
                (
                    crate::runtime::WorthQueryMutationFamily::Update,
                    crate::runtime::WorthQueryExistingTruthAssertionMode::BackendVerifiedAssertion,
                ) => backend_verified_update_count += 1,
                (
                    crate::runtime::WorthQueryMutationFamily::Delete,
                    crate::runtime::WorthQueryExistingTruthAssertionMode::BackendVerifiedAssertion,
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
            Some(
                crate::evidence_identity::worth_query_evidence_identity(
                    crate::evidence_identity::WorthQueryEvidenceScope::MutationEvidenceAggregateDigest,
                )
                .field_shape(
                    crate::evidence_identity::WorthQueryEvidenceTag::new("role"),
                    "batch-existing-truth-mode-entry",
                )
                .field_shape(
                    crate::evidence_identity::WorthQueryEvidenceTag::new("family"),
                    family.as_str(),
                )
                .field_shape(
                    crate::evidence_identity::WorthQueryEvidenceTag::new("mode"),
                    assertion.mode().as_str(),
                )
                .field_evidence_identity(
                    crate::evidence_identity::WorthQueryEvidenceTag::new("verification"),
                    assertion.verification_evidence_identity(),
                )
                .seal(),
            )
        })
        .collect::<Vec<_>>();
    let digest = (!mode_parts.is_empty()).then(|| {
        let identity = crate::evidence_identity::worth_query_evidence_identity(
            crate::evidence_identity::WorthQueryEvidenceScope::MutationEvidenceAggregateDigest,
        )
        .field_shape(
            crate::evidence_identity::WorthQueryEvidenceTag::new("role"),
            "batch-existing-truth-mode",
        )
        .field_evidence_identity_sequence(
            crate::evidence_identity::WorthQueryEvidenceTag::new("entry"),
            mode_parts.iter(),
        )
        .seal();
        WorthQueryMutationEvidenceDigest::aggregate("batch-existing-truth-mode", identity)
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
