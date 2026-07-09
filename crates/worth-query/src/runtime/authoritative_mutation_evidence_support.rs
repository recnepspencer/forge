use crate::identity::hash_parts;

use super::authoritative_mutation_evidence_support_bridge::bridge_backed_verification_support_rows;
pub use super::authoritative_mutation_evidence_support_bridge::{
    WorthQueryBridgeBackedVerificationSupportRow, WorthQueryBridgeBackedVerificationSupportStatus,
};
use super::support::{
    default_graph_composition_extension_hook_support_rows,
    WorthQueryGraphCompositionCapabilitySupportRow,
    WorthQueryGraphCompositionExtensionHookSupportRow,
};
use super::{WorthQueryRuntimeBackendPosture, WorthQueryRuntimeSupportProfile};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryAuthoritativeMutationEvidenceSupport {
    backend_posture: WorthQueryRuntimeBackendPosture,
    declared_resolved_target_model: String,
    existing_truth_binding_families: Vec<String>,
    existing_truth_assertion_modes: Vec<String>,
    existing_truth_probe_modes: Vec<String>,
    existing_truth_verified_mutation_modes: Vec<String>,
    bridge_backed_verification_support_rows: Vec<WorthQueryBridgeBackedVerificationSupportRow>,
    identity_preserving_update_families: Vec<String>,
    symbolic_target_reference_families: Vec<String>,
    symbolic_aspect_reference_families: Vec<String>,
    graph_composition_capability_support_rows: Vec<WorthQueryGraphCompositionCapabilitySupportRow>,
    graph_composition_extension_hook_support_rows:
        Vec<WorthQueryGraphCompositionExtensionHookSupportRow>,
    graph_composition_families: Vec<String>,
    naming_mutation_families: Vec<String>,
    continuity_mutation_families: Vec<String>,
    aggregate_evidence_sections: Vec<String>,
    fail_closed_denial_classes: Vec<String>,
    support_digest: String,
}

impl WorthQueryAuthoritativeMutationEvidenceSupport {
    pub fn derive(support_profile: &WorthQueryRuntimeSupportProfile) -> Self {
        let backend_posture = support_profile.posture();
        let declared_resolved_target_model =
            "declared-resolved-target-evidence-with-touched-fallout".to_string();
        let graph_composition_capability_support_rows = support_profile
            .graph_composition_capability_support_rows()
            .to_vec();
        let graph_composition_extension_hook_support_rows =
            default_graph_composition_extension_hook_support_rows();
        let existing_truth_binding_families = vec![
            "direct_entity_identity".to_string(),
            "direct_relation_identity".to_string(),
        ];
        let existing_truth_assertion_modes = vec![
            "retained_authoritative_assertion".to_string(),
            "backend_verified_assertion".to_string(),
        ];
        let existing_truth_probe_modes = vec!["backend_verified_probe".to_string()];
        let existing_truth_verified_mutation_modes = vec![
            "backend_verified_update".to_string(),
            "backend_verified_delete".to_string(),
        ];
        let bridge_backed_verification_support_rows =
            bridge_backed_verification_support_rows(support_profile);
        let identity_preserving_update_families = vec![
            "direct_entity_identity_update".to_string(),
            "direct_relation_identity_update".to_string(),
        ];
        let symbolic_target_reference_families = vec!["same_batch_declared_target".to_string()];
        let symbolic_aspect_reference_families =
            vec!["same_batch_declared_entity_identity".to_string()];
        let mut graph_composition_families = Vec::new();
        for row in &graph_composition_capability_support_rows {
            let family = row.capability_family().to_string();
            if !graph_composition_families.contains(&family) {
                graph_composition_families.push(family);
            }
        }
        let naming_mutation_families = vec![
            "attach_new_target".to_string(),
            "attach_existing_target".to_string(),
            "rebind_target".to_string(),
            "remove".to_string(),
        ];
        let continuity_mutation_families = vec![
            "rebind_existing_target".to_string(),
            "split_existing_target".to_string(),
        ];
        let aggregate_evidence_sections = vec![
            "batch_mutation_evidence".to_string(),
            "aggregate_existing_truth_binding_digest".to_string(),
            "aggregate_existing_truth_mode_digest".to_string(),
            "aggregate_symbolic_target_reference_digest".to_string(),
            "aggregate_naming_mutation_digest".to_string(),
            "aggregate_continuity_mutation_digest".to_string(),
            "aggregate_causality_digest".to_string(),
            "aggregate_provenance_digest".to_string(),
        ];
        let fail_closed_denial_classes = vec![
            "unsupported-family".to_string(),
            "resolved-target-missing".to_string(),
            "collection-mismatch".to_string(),
            "graph-composition-empty".to_string(),
            "graph-composition-duplicate-symbol".to_string(),
            "graph-composition-unresolved-symbolic-reference".to_string(),
            "graph-composition-symbolic-collection-mismatch".to_string(),
            "graph-composition-existing-target-binding-unsupported".to_string(),
            "graph-composition-existing-target-resolved-target-missing".to_string(),
            "graph-composition-existing-target-collection-mismatch".to_string(),
            "graph-composition-existing-target-retarget-unsupported".to_string(),
            "graph-composition-existing-target-identity-preservation-unavailable".to_string(),
            "graph-composition-existing-target-supersession-unsupported".to_string(),
            "graph-composition-existing-target-backend-verification-unsupported".to_string(),
            "graph-composition-existing-target-clear-assertion-unsupported".to_string(),
            "graph-composition-existing-target-missing-asserted-aspect".to_string(),
            "graph-composition-existing-target-asserted-value-mismatch".to_string(),
            "graph-composition-domain-invariant-denied".to_string(),
            "backend_verification_unsupported".to_string(),
            "missing_asserted_aspect".to_string(),
            "asserted_value_mismatch".to_string(),
            "backend_probe_unsupported".to_string(),
            "resolved_target_unavailable".to_string(),
            "missing_probed_aspect".to_string(),
            "unsupported_symbolic_target_reference".to_string(),
            "requires_same_batch_target_reference".to_string(),
            "requires_existing_truth_binding".to_string(),
            "requires_delete_family".to_string(),
            "requires_update_family".to_string(),
            "requires_authoritative_lane".to_string(),
        ];
        let mut parts = vec![
            "worth_query_authoritative_mutation_evidence_support_v1".to_string(),
            format!("posture:{}", backend_posture.as_str()),
            format!("target-model:{declared_resolved_target_model}"),
        ];
        parts.extend(
            existing_truth_binding_families
                .iter()
                .map(|item| format!("existing-binding:{item}")),
        );
        parts.extend(
            existing_truth_assertion_modes
                .iter()
                .map(|item| format!("existing-assertion:{item}")),
        );
        parts.extend(
            existing_truth_probe_modes
                .iter()
                .map(|item| format!("existing-probe:{item}")),
        );
        parts.extend(
            existing_truth_verified_mutation_modes
                .iter()
                .map(|item| format!("existing-verified-mutation:{item}")),
        );
        parts.extend(
            bridge_backed_verification_support_rows
                .iter()
                .map(|row| format!("bridge-backed-verification:{}", row.row_digest())),
        );
        parts.extend(
            identity_preserving_update_families
                .iter()
                .map(|item| format!("identity-preserving-update:{item}")),
        );
        parts.extend(
            symbolic_target_reference_families
                .iter()
                .map(|item| format!("symbolic-target:{item}")),
        );
        parts.extend(
            symbolic_aspect_reference_families
                .iter()
                .map(|item| format!("symbolic-aspect:{item}")),
        );
        parts.extend(
            graph_composition_families
                .iter()
                .map(|item| format!("graph-composition:{item}")),
        );
        parts.extend(
            graph_composition_capability_support_rows
                .iter()
                .map(|row| format!("graph-composition-row:{}", row.row_digest())),
        );
        parts.extend(
            graph_composition_extension_hook_support_rows
                .iter()
                .map(|row| format!("graph-composition-hook-row:{}", row.row_digest())),
        );
        parts.extend(
            naming_mutation_families
                .iter()
                .map(|item| format!("naming:{item}")),
        );
        parts.extend(
            continuity_mutation_families
                .iter()
                .map(|item| format!("continuity:{item}")),
        );
        parts.extend(
            aggregate_evidence_sections
                .iter()
                .map(|item| format!("aggregate:{item}")),
        );
        parts.extend(
            fail_closed_denial_classes
                .iter()
                .map(|item| format!("fail-closed:{item}")),
        );
        let support_digest = hash_parts(&parts);
        Self {
            backend_posture,
            declared_resolved_target_model,
            existing_truth_binding_families,
            existing_truth_assertion_modes,
            existing_truth_probe_modes,
            existing_truth_verified_mutation_modes,
            bridge_backed_verification_support_rows,
            identity_preserving_update_families,
            symbolic_target_reference_families,
            symbolic_aspect_reference_families,
            graph_composition_capability_support_rows,
            graph_composition_extension_hook_support_rows,
            graph_composition_families,
            naming_mutation_families,
            continuity_mutation_families,
            aggregate_evidence_sections,
            fail_closed_denial_classes,
            support_digest,
        }
    }

    pub fn backend_posture(&self) -> WorthQueryRuntimeBackendPosture {
        self.backend_posture
    }

    pub fn declared_resolved_target_model(&self) -> &str {
        &self.declared_resolved_target_model
    }

    pub fn existing_truth_binding_families(&self) -> &[String] {
        &self.existing_truth_binding_families
    }

    pub fn symbolic_target_reference_families(&self) -> &[String] {
        &self.symbolic_target_reference_families
    }

    pub fn symbolic_aspect_reference_families(&self) -> &[String] {
        &self.symbolic_aspect_reference_families
    }

    pub fn graph_composition_families(&self) -> &[String] {
        &self.graph_composition_families
    }

    pub fn graph_composition_capability_support_rows(
        &self,
    ) -> &[WorthQueryGraphCompositionCapabilitySupportRow] {
        &self.graph_composition_capability_support_rows
    }

    pub fn graph_composition_extension_hook_support_rows(
        &self,
    ) -> &[WorthQueryGraphCompositionExtensionHookSupportRow] {
        &self.graph_composition_extension_hook_support_rows
    }

    pub fn existing_truth_assertion_modes(&self) -> &[String] {
        &self.existing_truth_assertion_modes
    }

    pub fn existing_truth_probe_modes(&self) -> &[String] {
        &self.existing_truth_probe_modes
    }

    pub fn existing_truth_verified_mutation_modes(&self) -> &[String] {
        &self.existing_truth_verified_mutation_modes
    }

    pub fn bridge_backed_verification_support_rows(
        &self,
    ) -> &[WorthQueryBridgeBackedVerificationSupportRow] {
        &self.bridge_backed_verification_support_rows
    }

    pub fn identity_preserving_update_families(&self) -> &[String] {
        &self.identity_preserving_update_families
    }

    pub fn naming_mutation_families(&self) -> &[String] {
        &self.naming_mutation_families
    }

    pub fn continuity_mutation_families(&self) -> &[String] {
        &self.continuity_mutation_families
    }

    pub fn aggregate_evidence_sections(&self) -> &[String] {
        &self.aggregate_evidence_sections
    }

    pub fn fail_closed_denial_classes(&self) -> &[String] {
        &self.fail_closed_denial_classes
    }

    pub fn support_digest(&self) -> &str {
        &self.support_digest
    }
}
