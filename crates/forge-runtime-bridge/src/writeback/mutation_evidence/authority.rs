use std::sync::Arc;

use super::{
    digest::{aggregate_digest, aggregate_optional_digest},
    existing_truth::BridgeExistingTruthBindingBundle,
    provenance::{BridgeMutationCausalityBundle, BridgeMutationProvenanceBundle},
};
use crate::writeback::{
    BridgeContinuityMutationBundle, BridgeDerivedWritebackEffect, BridgeNamingMutationBundle,
    BridgeSymbolicTargetReferenceBundle, BridgeWritebackAuthorityOutcome,
    BridgeWritebackCausalityBasis, BridgeWritebackExecutionRecord,
    BridgeWritebackFeedbackProvenance,
};

/// One bridge-authored carry-forward packet suitable for Query receipt lowering.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeMutationAuthorityBundle {
    causality: BridgeMutationCausalityBundle,
    provenance: BridgeMutationProvenanceBundle,
    existing_truth_binding: Option<BridgeExistingTruthBindingBundle>,
    symbolic_target_reference: Option<BridgeSymbolicTargetReferenceBundle>,
    naming_mutation: Option<BridgeNamingMutationBundle>,
    continuity_mutation: Option<BridgeContinuityMutationBundle>,
}

impl BridgeMutationAuthorityBundle {
    pub fn from_writeback_artifacts(
        causality: &BridgeWritebackCausalityBasis,
        effect: &BridgeDerivedWritebackEffect,
        feedback: &BridgeWritebackFeedbackProvenance,
        execution_record: &BridgeWritebackExecutionRecord,
        outcome: Option<&BridgeWritebackAuthorityOutcome>,
    ) -> Self {
        Self {
            causality: BridgeMutationCausalityBundle::from_writeback_causality(causality),
            provenance: BridgeMutationProvenanceBundle::from_writeback_artifacts(
                effect,
                feedback,
                execution_record,
                outcome,
            ),
            existing_truth_binding: None,
            symbolic_target_reference: None,
            naming_mutation: None,
            continuity_mutation: None,
        }
    }

    pub fn causality(&self) -> &BridgeMutationCausalityBundle {
        &self.causality
    }

    pub fn provenance(&self) -> &BridgeMutationProvenanceBundle {
        &self.provenance
    }

    pub fn existing_truth_binding(&self) -> Option<&BridgeExistingTruthBindingBundle> {
        self.existing_truth_binding.as_ref()
    }

    pub fn with_existing_truth_binding(
        mut self,
        binding: BridgeExistingTruthBindingBundle,
    ) -> Self {
        self.existing_truth_binding = Some(binding);
        self
    }

    pub fn symbolic_target_reference(&self) -> Option<&BridgeSymbolicTargetReferenceBundle> {
        self.symbolic_target_reference.as_ref()
    }

    pub fn with_symbolic_target_reference(
        mut self,
        reference: BridgeSymbolicTargetReferenceBundle,
    ) -> Self {
        self.symbolic_target_reference = Some(reference);
        self
    }

    pub fn naming_mutation(&self) -> Option<&BridgeNamingMutationBundle> {
        self.naming_mutation.as_ref()
    }

    pub fn with_naming_mutation(mut self, naming: BridgeNamingMutationBundle) -> Self {
        self.naming_mutation = Some(naming);
        self
    }

    pub fn continuity_mutation(&self) -> Option<&BridgeContinuityMutationBundle> {
        self.continuity_mutation.as_ref()
    }

    pub fn with_continuity_mutation(mut self, continuity: BridgeContinuityMutationBundle) -> Self {
        self.continuity_mutation = Some(continuity);
        self
    }
}

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
    request_digest_count: usize,
    receipt_digest_count: usize,
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

        let outcome_class_count = components
            .iter()
            .filter(|component| component.provenance().outcome_class().is_some())
            .count();
        let request_digest_count = components
            .iter()
            .filter(|component| component.provenance().request_digest().is_some())
            .count();
        let receipt_digest_count = components
            .iter()
            .filter(|component| component.provenance().receipt_digest().is_some())
            .count();
        let existing_truth_bindings = components
            .iter()
            .filter_map(|component| component.existing_truth_binding());
        let symbolic_target_references = components
            .iter()
            .filter_map(|component| component.symbolic_target_reference());
        let naming_mutations = components
            .iter()
            .filter_map(|component| component.naming_mutation());
        let continuity_mutations = components
            .iter()
            .filter_map(|component| component.continuity_mutation());

        let aggregate_existing_truth_binding_digest = aggregate_optional_digest(
            "bridge-batch-existing-truth-binding",
            existing_truth_bindings.map(|binding| binding.binding_digest().to_string()),
        );
        let aggregate_symbolic_target_reference_digest = aggregate_optional_digest(
            "bridge-batch-symbolic-target-reference",
            symbolic_target_references.map(|reference| {
                format!(
                    "{:?}:{}:{}:{}",
                    reference.family(),
                    reference.symbol(),
                    reference.resolved_entity_identity(),
                    reference.target_collection().unwrap_or("none")
                )
            }),
        );
        let aggregate_naming_mutation_digest = aggregate_optional_digest(
            "bridge-batch-naming-mutation",
            naming_mutations.map(|naming| {
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
            }),
        );
        let aggregate_continuity_mutation_digest = aggregate_optional_digest(
            "bridge-batch-continuity-mutation",
            continuity_mutations.map(|continuity| {
                format!(
                    "{:?}:{:?}:{}:{}:{}:{}:{}:{}:{}",
                    continuity.family(),
                    continuity.outcome_class(),
                    continuity.prior_authoritative_identity(),
                    if continuity.successor_authoritative_identities().is_empty() {
                        "none".to_string()
                    } else {
                        continuity
                            .successor_authoritative_identities()
                            .iter()
                            .map(|value| value.as_ref())
                            .collect::<Vec<_>>()
                            .join("|")
                    },
                    continuity.basis_binding_digest().unwrap_or("none"),
                    continuity
                        .resolved_target_entity_identity()
                        .unwrap_or("none"),
                    continuity.target_collection().unwrap_or("none"),
                    continuity.lineage_digest(),
                    continuity.continuity_resolution_digest()
                )
            }),
        );

        let aggregate_causality_digest = aggregate_digest(
            "bridge-batch-mutation-causality",
            components.iter().flat_map(|component| {
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
            }),
        );
        let aggregate_provenance_digest = aggregate_digest(
            "bridge-batch-mutation-provenance",
            components.iter().flat_map(|component| {
                let provenance = component.provenance();
                [
                    format!("contract:{}", provenance.contract_digest()),
                    format!("derived-effect:{}", provenance.derived_effect_digest()),
                    format!("proposed-effect:{}", provenance.proposed_effect_digest()),
                    format!(
                        "feedback-provenance:{}",
                        provenance.feedback_provenance_digest()
                    ),
                    format!("causality:{}", provenance.causality_digest()),
                    format!("strategy:{}", provenance.strategy_descriptor_digest()),
                    format!("execution:{}", provenance.execution_record_digest()),
                    format!(
                        "outcome:{}",
                        provenance
                            .outcome_class()
                            .map(|value| format!("{value:?}"))
                            .unwrap_or_else(|| "none".to_string())
                    ),
                    format!(
                        "authoritative-artifact:{}",
                        provenance.authoritative_artifact_digest().unwrap_or("none")
                    ),
                    format!("request:{}", provenance.request_digest().unwrap_or("none")),
                    format!("receipt:{}", provenance.receipt_digest().unwrap_or("none")),
                    format!(
                        "failure:{}",
                        provenance
                            .failure_class()
                            .map(|value| format!("{value:?}"))
                            .unwrap_or_else(|| "none".to_string())
                    ),
                ]
            }),
        );

        Some(Self {
            component_count: components.len(),
            causality_bundle_count: components.len(),
            provenance_bundle_count: components.len(),
            existing_truth_binding_count: components
                .iter()
                .filter(|component| component.existing_truth_binding().is_some())
                .count(),
            symbolic_target_reference_count: components
                .iter()
                .filter(|component| component.symbolic_target_reference().is_some())
                .count(),
            naming_mutation_count: components
                .iter()
                .filter(|component| component.naming_mutation().is_some())
                .count(),
            continuity_mutation_count: components
                .iter()
                .filter(|component| component.continuity_mutation().is_some())
                .count(),
            outcome_class_count,
            request_digest_count,
            receipt_digest_count,
            aggregate_existing_truth_binding_digest,
            aggregate_symbolic_target_reference_digest,
            aggregate_naming_mutation_digest,
            aggregate_continuity_mutation_digest,
            aggregate_causality_digest,
            aggregate_provenance_digest,
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

    pub fn request_digest_count(&self) -> usize {
        self.request_digest_count
    }

    pub fn receipt_digest_count(&self) -> usize {
        self.receipt_digest_count
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
