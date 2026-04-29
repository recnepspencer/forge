use forge_runtime_bridge::facade::BridgeBatchMutationAuthorityBundle;

use super::{
    binding::ForgeQueryExistingTruthBindingEvidence, target::ForgeQueryMutationTargetClass,
    ForgeQueryMutationTargetEvidence,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryBatchMutationEvidence {
    component_count: usize,
    target_evidence_count: usize,
    existing_truth_binding_count: usize,
    symbolic_target_reference_count: usize,
    naming_mutation_count: usize,
    continuity_mutation_count: usize,
    resolved_target_count: usize,
    target_collection_count: usize,
    target_entity_count: usize,
    aggregate_existing_truth_binding_digest: Option<String>,
    aggregate_symbolic_target_reference_digest: Option<String>,
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
        components: &[ForgeQueryMutationTargetEvidence],
        existing_truth_bindings: &[Option<ForgeQueryExistingTruthBindingEvidence>],
        symbolic_target_references: &[Option<
            crate::runtime::ForgeQuerySymbolicTargetReferenceEvidence,
        >],
        naming_mutations: &[Option<crate::runtime::ForgeQueryNamingMutationEvidence>],
        continuity_mutations: &[Option<crate::runtime::ForgeQueryContinuityMutationEvidence>],
        aggregate_bridge: Option<&BridgeBatchMutationAuthorityBundle>,
    ) -> Option<Self> {
        if components.is_empty() {
            return None;
        }

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
            existing_truth_binding_count: existing_truth_bindings
                .iter()
                .filter(|binding| binding.is_some())
                .count(),
            symbolic_target_reference_count: symbolic_target_references
                .iter()
                .filter(|reference| reference.is_some())
                .count(),
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
            aggregate_existing_truth_binding_digest: aggregate_bridge
                .and_then(|bundle| bundle.aggregate_existing_truth_binding_digest())
                .map(str::to_string)
                .or_else(|| batch_existing_truth_binding_digest(existing_truth_bindings)),
            aggregate_symbolic_target_reference_digest: aggregate_bridge
                .and_then(|bundle| bundle.aggregate_symbolic_target_reference_digest())
                .map(str::to_string)
                .or_else(|| batch_symbolic_target_reference_digest(symbolic_target_references)),
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

    pub fn resolved_target_count(&self) -> usize {
        self.resolved_target_count
    }

    pub fn existing_truth_binding_count(&self) -> usize {
        self.existing_truth_binding_count
    }

    pub fn symbolic_target_reference_count(&self) -> usize {
        self.symbolic_target_reference_count
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

    pub fn aggregate_existing_truth_binding_digest(&self) -> Option<&str> {
        self.aggregate_existing_truth_binding_digest.as_deref()
    }

    pub fn aggregate_symbolic_target_reference_digest(&self) -> Option<&str> {
        self.aggregate_symbolic_target_reference_digest.as_deref()
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

fn batch_target_digest(components: &[ForgeQueryMutationTargetEvidence]) -> String {
    crate::identity::hash_parts(
        &std::iter::once("forge-query-batch-target-evidence-v1".to_string())
            .chain(components.iter().map(|component| {
                format!(
                    "{}:{}:{}:{}:{}:{}",
                    component.declared().target_class(),
                    component.declared().collection().unwrap_or(""),
                    component.declared().entity_identity().unwrap_or(""),
                    component.resolved().target_class(),
                    component.resolved().collection().unwrap_or(""),
                    component.resolved().entity_identity().unwrap_or("")
                )
            }))
            .collect::<Vec<_>>(),
    )
}

fn batch_existing_truth_binding_digest(
    bindings: &[Option<ForgeQueryExistingTruthBindingEvidence>],
) -> Option<String> {
    let bindings = bindings
        .iter()
        .flatten()
        .map(|binding| binding.binding_digest().to_string())
        .collect::<Vec<_>>();
    if bindings.is_empty() {
        return None;
    }
    Some(crate::identity::hash_parts(
        &std::iter::once("forge-query-batch-existing-truth-binding-v1".to_string())
            .chain(bindings)
            .collect::<Vec<_>>(),
    ))
}

fn batch_symbolic_target_reference_digest(
    references: &[Option<crate::runtime::ForgeQuerySymbolicTargetReferenceEvidence>],
) -> Option<String> {
    let references = references
        .iter()
        .flatten()
        .map(|reference| {
            format!(
                "{}:{}:{}:{}",
                reference.family(),
                reference.symbol(),
                reference.resolved_entity_identity(),
                reference.target_collection().unwrap_or("none")
            )
        })
        .collect::<Vec<_>>();
    if references.is_empty() {
        return None;
    }
    Some(crate::identity::hash_parts(
        &std::iter::once("forge-query-batch-symbolic-target-reference-v1".to_string())
            .chain(references)
            .collect::<Vec<_>>(),
    ))
}

fn batch_continuity_mutation_digest(
    continuities: &[Option<crate::runtime::ForgeQueryContinuityMutationEvidence>],
) -> Option<String> {
    let continuities = continuities
        .iter()
        .flatten()
        .map(|continuity| {
            format!(
                "{:?}:{:?}:{}:{}:{}:{}:{}:{}:{}",
                continuity.family(),
                continuity.outcome_class(),
                continuity.prior_authoritative_identity(),
                if continuity.successor_authoritative_identities().is_empty() {
                    "none".to_string()
                } else {
                    continuity.successor_authoritative_identities().join("|")
                },
                continuity.basis_binding_digest().unwrap_or("none"),
                continuity
                    .resolved_target_entity_identity()
                    .unwrap_or("none"),
                continuity.target_collection().unwrap_or("none"),
                continuity.lineage_digest(),
                continuity.continuity_resolution_digest()
            )
        })
        .collect::<Vec<_>>();
    if continuities.is_empty() {
        return None;
    }
    Some(crate::identity::hash_parts(
        &std::iter::once("forge-query-batch-continuity-mutation-v1".to_string())
            .chain(continuities)
            .collect::<Vec<_>>(),
    ))
}

fn batch_naming_mutation_digest(
    namings: &[Option<crate::runtime::ForgeQueryNamingMutationEvidence>],
) -> Option<String> {
    let namings = namings
        .iter()
        .flatten()
        .map(|naming| {
            format!(
                "{:?}:{:?}:{}:{}:{}:{}:{}",
                naming.family(),
                naming.outcome(),
                naming.attachment_identity(),
                naming.prior_authoritative_identity().unwrap_or("none"),
                naming.target_authoritative_identity().unwrap_or("none"),
                naming.resolved_target_entity_identity().unwrap_or("none"),
                naming.target_collection().unwrap_or("none")
            )
        })
        .collect::<Vec<_>>();
    if namings.is_empty() {
        return None;
    }
    Some(crate::identity::hash_parts(
        &std::iter::once("forge-query-batch-naming-mutation-v1".to_string())
            .chain(namings)
            .collect::<Vec<_>>(),
    ))
}

#[cfg(test)]
mod batch_tests;
