use std::sync::Arc;

use super::{
    authority::BridgeMutationAuthorityBundle,
    batch_authority_labels::{
        continuity_family_label, continuity_outcome_label, naming_family_label,
        naming_outcome_label, symbolic_target_family_label, writeback_failure_label,
        writeback_outcome_label,
    },
    digest::{
        batch_continuity_mutation_digest, batch_existing_truth_binding_digest,
        batch_mutation_causality_digest, batch_mutation_provenance_digest,
        batch_naming_mutation_digest, batch_symbolic_target_reference_digest,
    },
};
use crate::writeback::BridgeContinuityMutationBundle;

/// One bridge-authored aggregate packet for an ordered authoritative mutation session.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeBatchMutationAuthorityBundle {
    component_count: usize,
    causality_bundle_count: usize,
    provenance_bundle_count: usize,
    existing_truth_binding_count: usize,
    symbolic_target_reference_count: usize,
    naming_mutation_count: usize,
    continuity_mutation_count: usize,
    outcome_class_count: usize,
    authority_request_count: usize,
    authority_receipt_count: usize,
    aggregate_existing_truth_binding_digest: Option<Arc<str>>,
    aggregate_symbolic_target_reference_digest: Option<Arc<str>>,
    aggregate_naming_mutation_digest: Option<Arc<str>>,
    aggregate_continuity_mutation_digest: Option<Arc<str>>,
    aggregate_causality_digest: Arc<str>,
    aggregate_provenance_digest: Arc<str>,
}

impl BridgeBatchMutationAuthorityBundle {
    pub fn from_components(components: &[BridgeMutationAuthorityBundle]) -> Option<Self> {
        if components.is_empty() {
            return None;
        }
        let counts = BatchAuthorityCounts::from_components(components);
        let digests = BatchAuthorityDigests::from_components(components);
        Some(Self {
            component_count: components.len(),
            causality_bundle_count: components.len(),
            provenance_bundle_count: components.len(),
            existing_truth_binding_count: counts.existing_truth_binding_count,
            symbolic_target_reference_count: counts.symbolic_target_reference_count,
            naming_mutation_count: counts.naming_mutation_count,
            continuity_mutation_count: counts.continuity_mutation_count,
            outcome_class_count: counts.outcome_class_count,
            authority_request_count: counts.authority_request_count,
            authority_receipt_count: counts.authority_receipt_count,
            aggregate_existing_truth_binding_digest: digests.existing_truth_binding,
            aggregate_symbolic_target_reference_digest: digests.symbolic_target_reference,
            aggregate_naming_mutation_digest: digests.naming_mutation,
            aggregate_continuity_mutation_digest: digests.continuity_mutation,
            aggregate_causality_digest: digests.causality,
            aggregate_provenance_digest: digests.provenance,
        })
    }

    pub fn component_count(&self) -> usize {
        self.component_count
    }

    pub fn causality_bundle_count(&self) -> usize {
        self.causality_bundle_count
    }

    pub fn provenance_bundle_count(&self) -> usize {
        self.provenance_bundle_count
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

    pub fn outcome_class_count(&self) -> usize {
        self.outcome_class_count
    }

    pub fn authority_request_count(&self) -> usize {
        self.authority_request_count
    }

    pub fn authority_receipt_count(&self) -> usize {
        self.authority_receipt_count
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

    pub fn aggregate_causality_digest(&self) -> &str {
        self.aggregate_causality_digest.as_ref()
    }

    pub fn aggregate_provenance_digest(&self) -> &str {
        self.aggregate_provenance_digest.as_ref()
    }
}

struct BatchAuthorityCounts {
    existing_truth_binding_count: usize,
    symbolic_target_reference_count: usize,
    naming_mutation_count: usize,
    continuity_mutation_count: usize,
    outcome_class_count: usize,
    authority_request_count: usize,
    authority_receipt_count: usize,
}

impl BatchAuthorityCounts {
    fn from_components(components: &[BridgeMutationAuthorityBundle]) -> Self {
        Self {
            existing_truth_binding_count: count_components(components, |component| {
                component.existing_truth_binding().is_some()
            }),
            symbolic_target_reference_count: count_components(components, |component| {
                component.symbolic_target_reference().is_some()
            }),
            naming_mutation_count: count_components(components, |component| {
                component.naming_mutation().is_some()
            }),
            continuity_mutation_count: count_components(components, |component| {
                component.continuity_mutation().is_some()
            }),
            outcome_class_count: count_components(components, |component| {
                component.provenance().outcome_class().is_some()
            }),
            authority_request_count: count_components(components, |component| {
                component.provenance().authority_request().is_some()
            }),
            authority_receipt_count: count_components(components, |component| {
                component.provenance().authority_receipt().is_some()
            }),
        }
    }
}

struct BatchAuthorityDigests {
    existing_truth_binding: Option<Arc<str>>,
    symbolic_target_reference: Option<Arc<str>>,
    naming_mutation: Option<Arc<str>>,
    continuity_mutation: Option<Arc<str>>,
    causality: Arc<str>,
    provenance: Arc<str>,
}

impl BatchAuthorityDigests {
    fn from_components(components: &[BridgeMutationAuthorityBundle]) -> Self {
        Self {
            existing_truth_binding: aggregate_existing_truth_bindings(components),
            symbolic_target_reference: aggregate_symbolic_target_references(components),
            naming_mutation: aggregate_naming_mutations(components),
            continuity_mutation: aggregate_continuity_mutations(components),
            causality: aggregate_causality(components),
            provenance: aggregate_provenance(components),
        }
    }
}

fn count_components(
    components: &[BridgeMutationAuthorityBundle],
    predicate: impl Fn(&BridgeMutationAuthorityBundle) -> bool,
) -> usize {
    components
        .iter()
        .filter(|component| predicate(component))
        .count()
}

fn aggregate_existing_truth_bindings(
    components: &[BridgeMutationAuthorityBundle],
) -> Option<Arc<str>> {
    batch_existing_truth_binding_digest(components.iter().filter_map(|component| {
        component
            .existing_truth_binding()
            .map(|binding| binding.binding_digest().to_string())
    }))
}

fn aggregate_symbolic_target_references(
    components: &[BridgeMutationAuthorityBundle],
) -> Option<Arc<str>> {
    batch_symbolic_target_reference_digest(components.iter().filter_map(|component| {
        component.symbolic_target_reference().map(|reference| {
            format!(
                "{}:{}:{}:{}",
                symbolic_target_family_label(reference.family()),
                reference.symbol(),
                reference.resolved_entity_identity(),
                reference.target_collection().unwrap_or("none")
            )
        })
    }))
}

fn aggregate_naming_mutations(components: &[BridgeMutationAuthorityBundle]) -> Option<Arc<str>> {
    batch_naming_mutation_digest(components.iter().filter_map(|component| {
        component.naming_mutation().map(|naming| {
            format!(
                "{}:{}:{}:{}:{}:{}:{}",
                naming_family_label(naming.family()),
                naming_outcome_label(naming.outcome()),
                naming.attachment_identity(),
                naming.prior_authoritative_identity().unwrap_or("none"),
                naming.target_authoritative_identity().unwrap_or("none"),
                naming.resolved_target_entity_identity().unwrap_or("none"),
                naming.target_collection().unwrap_or("none")
            )
        })
    }))
}

fn aggregate_continuity_mutations(
    components: &[BridgeMutationAuthorityBundle],
) -> Option<Arc<str>> {
    batch_continuity_mutation_digest(components.iter().filter_map(|component| {
        component.continuity_mutation().map(|continuity| {
            format!(
                "{}:{}:{}:{}:{}:{}:{}:{}:{}",
                continuity_family_label(continuity.family()),
                continuity_outcome_label(continuity.outcome_class()),
                continuity.prior_authoritative_identity(),
                format_continuity_successor_identities(continuity),
                continuity.basis_binding_digest().unwrap_or("none"),
                continuity
                    .resolved_target_entity_identity()
                    .unwrap_or("none"),
                continuity.target_collection().unwrap_or("none"),
                continuity.lineage_digest(),
                continuity.continuity_resolution_digest()
            )
        })
    }))
}

fn aggregate_causality(components: &[BridgeMutationAuthorityBundle]) -> Arc<str> {
    batch_mutation_causality_digest(components.iter().flat_map(|component| {
        [
            format!("causality:{}", component.causality().causality_digest()),
            format!(
                "truth-trigger:{}",
                component.causality().truth_trigger_digest()
            ),
            format!("route:{}", component.causality().route_digest()),
            format!(
                "evaluation:{}",
                component.causality().evaluation_surface_digest()
            ),
            format!("truth-view:{}", component.causality().truth_view_digest()),
        ]
    }))
}

fn aggregate_provenance(components: &[BridgeMutationAuthorityBundle]) -> Arc<str> {
    batch_mutation_provenance_digest(
        components
            .iter()
            .flat_map(mutation_provenance_digest_entries),
    )
}

fn mutation_provenance_digest_entries(component: &BridgeMutationAuthorityBundle) -> [String; 13] {
    let provenance = component.provenance();
    [
        format!("contract:{}", provenance.contract_digest()),
        format!(
            "writeback-effect-artifact:{}",
            provenance.writeback_effect_artifact_digest()
        ),
        format!("effect-intent:{}", provenance.effect_intent_digest()),
        format!(
            "effect-intent-basis:{}",
            provenance.effect_intent_patch_canonical_basis()
        ),
        format!(
            "feedback-provenance:{}",
            provenance.feedback_provenance_digest()
        ),
        format!("causality:{}", provenance.causality_digest()),
        format!(
            "strategy-basis:{}:{}",
            provenance.strategy_descriptor_basis().canonical_basis(),
            provenance.strategy_descriptor_basis().digest()
        ),
        format!("execution:{}", provenance.execution_record_digest()),
        format!(
            "outcome:{}",
            provenance
                .outcome_class()
                .map_or("none", writeback_outcome_label)
        ),
        format!(
            "authority-artifact-proof:{}",
            provenance.authoritative_artifact_digest().unwrap_or("none")
        ),
        format!("request:{}", provenance.request_digest().unwrap_or("none")),
        format!("receipt:{}", provenance.receipt_digest().unwrap_or("none")),
        format!(
            "failure:{}",
            provenance
                .failure_class()
                .map_or("none", writeback_failure_label)
        ),
    ]
}

fn format_continuity_successor_identities(continuity: &BridgeContinuityMutationBundle) -> String {
    if continuity.successor_authoritative_identities().is_empty() {
        return "none".to_string();
    }
    continuity
        .successor_authoritative_identities()
        .iter()
        .map(|identity| identity.as_str())
        .collect::<Vec<_>>()
        .join("|")
}
