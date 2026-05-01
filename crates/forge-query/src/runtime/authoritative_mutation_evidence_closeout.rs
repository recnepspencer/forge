use crate::identity::hash_parts;
use forge_runtime_bridge::facade::{
    BridgeAuthoritativeMutationEvidenceCloseout, BridgeAuthoritativeMutationEvidenceSupport,
};

use super::authoritative_mutation_evidence_bridge_compat::assert_bridge_support_compatibility;
use super::{
    ForgeQueryMutationApiCompatibilityReport, ForgeQueryRuntimeBackendPosture,
    ForgeQueryRuntimePublicApiNamingContract, ForgeQueryRuntimePublicSupportMatrix,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryAuthoritativeMutationEvidenceSupport {
    backend_posture: ForgeQueryRuntimeBackendPosture,
    declared_resolved_target_model: String,
    existing_truth_binding_families: Vec<String>,
    existing_truth_assertion_modes: Vec<String>,
    existing_truth_probe_modes: Vec<String>,
    existing_truth_verified_mutation_modes: Vec<String>,
    symbolic_target_reference_families: Vec<String>,
    symbolic_aspect_reference_families: Vec<String>,
    graph_composition_families: Vec<String>,
    naming_mutation_families: Vec<String>,
    continuity_mutation_families: Vec<String>,
    aggregate_evidence_sections: Vec<String>,
    fail_closed_denial_classes: Vec<String>,
    support_digest: String,
}

impl ForgeQueryAuthoritativeMutationEvidenceSupport {
    pub fn derive(backend_posture: ForgeQueryRuntimeBackendPosture) -> Self {
        let declared_resolved_target_model =
            "declared-resolved-target-evidence-with-touched-fallout".to_string();
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
        let symbolic_target_reference_families = vec!["same_batch_declared_target".to_string()];
        let symbolic_aspect_reference_families =
            vec!["same_batch_declared_entity_identity".to_string()];
        let graph_composition_families = vec![
            "same_batch_entity_relation_identity_edges".to_string(),
            "mixed_existing_and_symbolic_entity_identity_edges".to_string(),
            "same_batch_symbolic_relation_followup_mutation".to_string(),
        ];
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
            "forge_query_authoritative_mutation_evidence_support_v1".to_string(),
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
            symbolic_target_reference_families,
            symbolic_aspect_reference_families,
            graph_composition_families,
            naming_mutation_families,
            continuity_mutation_families,
            aggregate_evidence_sections,
            fail_closed_denial_classes,
            support_digest,
        }
    }

    pub fn backend_posture(&self) -> ForgeQueryRuntimeBackendPosture {
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

    pub fn existing_truth_assertion_modes(&self) -> &[String] {
        &self.existing_truth_assertion_modes
    }

    pub fn existing_truth_probe_modes(&self) -> &[String] {
        &self.existing_truth_probe_modes
    }

    pub fn existing_truth_verified_mutation_modes(&self) -> &[String] {
        &self.existing_truth_verified_mutation_modes
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryAuthoritativeMutationEvidenceCloseout {
    backend_posture: ForgeQueryRuntimeBackendPosture,
    support_matrix_digest: String,
    mutation_compatibility_digest: String,
    naming_contract_digest: String,
    query_support_digest: String,
    bridge_support_digest: String,
    bridge_closeout_digest: String,
    safe_to_build_now: Vec<String>,
    must_not_assume_yet: Vec<String>,
    migration_guidance: Vec<String>,
    required_verification_commands: Vec<String>,
    closeout_digest: String,
}

impl ForgeQueryAuthoritativeMutationEvidenceCloseout {
    pub fn derive(
        backend_posture: ForgeQueryRuntimeBackendPosture,
        support_matrix: &ForgeQueryRuntimePublicSupportMatrix,
        mutation_compatibility: &ForgeQueryMutationApiCompatibilityReport,
        naming_contract: &ForgeQueryRuntimePublicApiNamingContract,
        query_support: &ForgeQueryAuthoritativeMutationEvidenceSupport,
        bridge_support: &BridgeAuthoritativeMutationEvidenceSupport,
        bridge_closeout: &BridgeAuthoritativeMutationEvidenceCloseout,
    ) -> Self {
        assert_bridge_support_compatibility(query_support, bridge_support, bridge_closeout);
        let bridge_support_digest = bridge_support.support_digest().to_string();
        let bridge_closeout_digest = bridge_closeout.closeout_digest().to_string();
        let safe_to_build_now = vec![
            "workspace.insert/update/delete/batch receipts preserve declared-versus-resolved target evidence together with touched-aspect fallout".to_string(),
            "existing-truth binding, same-batch symbolic target reference, same-batch symbolic aspect reference, naming mutation, and continuity mutation evidence are part of the ordinary public receipt and inspection story".to_string(),
            "existing-truth assertions now distinguish retained authoritative assertions from backend-verified assertions on the public receipt and inspection surface".to_string(),
            "mixed existing-truth authority sessions now preserve aggregate mode evidence that distinguishes retained assertions, backend-verified assertions, verified updates, and verified deletes without reconstructing that story from component receipts".to_string(),
            "existing-truth probes now expose a typed backend-verified probe lane for current authoritative values without smuggling that truth through mutation receipts".to_string(),
            "existing-truth verified updates now expose a typed backend-verified update lane that proves current authoritative values before applying update-family mutation receipts".to_string(),
            "existing-truth verified deletes now expose a typed backend-verified delete lane that proves current authoritative values before applying delete-family mutation receipts".to_string(),
            "existing-truth batch receipts, scalar inspection, and probe surfaces keep retained assertions, backend-verified assertions, backend-verified probes, verified updates, and verified deletes semantically distinct under mixed authority sessions".to_string(),
            "batch and import-style authority sessions preserve aggregate existing-binding, symbolic-target, symbolic-aspect, naming, continuity, causality, and provenance digests".to_string(),
            "downstream domains may rely on Query receipts and inspection instead of rebuilding target-recovery, naming, or continuity explanation glue locally".to_string(),
            "downstream domains may rely on `verify_existing(...)` only when the active backend actually supports backend verification; unsupported backends remain typed and fail-closed".to_string(),
            "downstream domains may rely on `update_existing_verified(...)` only when the active backend actually supports backend verification; unsupported backends remain typed and fail-closed".to_string(),
            "downstream domains may rely on `delete_existing_verified(...)` only when the active backend actually supports backend verification; unsupported backends remain typed and fail-closed".to_string(),
        ];
        let must_not_assume_yet = vec![
            "authority-mutation evidence closes durable restart, temporal, async, or store-backed mutation semantics".to_string(),
            "unsupported identity-binding, naming, or continuity families remain fail-closed until explicitly admitted".to_string(),
            "unsupported existing-truth binding, assertion, verified-mutation, and probe neighbors remain typed and fail-closed rather than degrading into best-effort compatibility".to_string(),
            "downstream code may bypass Query receipts and inspect raw bridge/runtime provenance bags directly".to_string(),
        ];
        let migration_guidance = vec![
            "move authoritative mutation onto workspace.insert/update/delete/batch and consume receipts plus inspect output as the domain explanation contract".to_string(),
            "use `workspace.assert_existing(...)` for retained assertion receipts and `workspace.verify_existing(...)` when the backend must prove current stored truth before returning an assertion receipt".to_string(),
            "use `workspace.probe_existing(...)` when the domain needs current authoritative aspect values as input rather than a retained assertion receipt".to_string(),
            "use `workspace.update_existing_verified(...)` when the backend must prove current stored truth immediately before an existing-target update-family mutation".to_string(),
            "use `workspace.delete_existing_verified(...)` when the backend must prove current stored truth immediately before an existing-target delete-family mutation".to_string(),
            "delete local existing-target rebinding, naming outcome reconstruction, and continuity breadcrumb glue once equivalent Query evidence is available".to_string(),
            "treat unsupported mutation-evidence neighbors as fail-closed support gates rather than compatibility seams".to_string(),
        ];
        let required_verification_commands = vec![
            "cargo fmt -p forge-query".to_string(),
            "cargo check -p forge-query --tests".to_string(),
            "cargo test --manifest-path crates/forge-query/Cargo.toml --test phase_boundaries_compile_fail".to_string(),
            "cargo test -p forge-query".to_string(),
            "cargo fmt -p forge-runtime-bridge".to_string(),
            "cargo check -p forge-runtime-bridge --tests".to_string(),
            "cargo test -p forge-runtime-bridge".to_string(),
            "cargo test --manifest-path crates/forge-runtime-bridge/Cargo.toml --test phase_boundaries_compile_fail".to_string(),
            "git diff --check".to_string(),
        ];
        let mut parts = vec![
            "forge_query_authoritative_mutation_evidence_closeout_v1".to_string(),
            format!("posture:{}", backend_posture.as_str()),
            format!("matrix:{}", support_matrix.matrix_digest()),
            format!("mutation:{}", mutation_compatibility.report_digest()),
            format!("naming:{}", naming_contract.contract_digest()),
            format!("query-support:{}", query_support.support_digest()),
            format!("bridge-support:{bridge_support_digest}"),
            format!("bridge-closeout:{bridge_closeout_digest}"),
        ];
        parts.extend(safe_to_build_now.iter().map(|item| format!("safe:{item}")));
        parts.extend(
            must_not_assume_yet
                .iter()
                .map(|item| format!("deferred:{item}")),
        );
        parts.extend(
            migration_guidance
                .iter()
                .map(|item| format!("migration:{item}")),
        );
        parts.extend(
            required_verification_commands
                .iter()
                .map(|item| format!("verify:{item}")),
        );
        let closeout_digest = hash_parts(&parts);
        Self {
            backend_posture,
            support_matrix_digest: support_matrix.matrix_digest().to_string(),
            mutation_compatibility_digest: mutation_compatibility.report_digest().to_string(),
            naming_contract_digest: naming_contract.contract_digest().to_string(),
            query_support_digest: query_support.support_digest().to_string(),
            bridge_support_digest,
            bridge_closeout_digest,
            safe_to_build_now,
            must_not_assume_yet,
            migration_guidance,
            required_verification_commands,
            closeout_digest,
        }
    }

    pub fn backend_posture(&self) -> ForgeQueryRuntimeBackendPosture {
        self.backend_posture
    }

    pub fn support_matrix_digest(&self) -> &str {
        &self.support_matrix_digest
    }

    pub fn mutation_compatibility_digest(&self) -> &str {
        &self.mutation_compatibility_digest
    }

    pub fn naming_contract_digest(&self) -> &str {
        &self.naming_contract_digest
    }

    pub fn query_support_digest(&self) -> &str {
        &self.query_support_digest
    }

    pub fn bridge_support_digest(&self) -> &str {
        &self.bridge_support_digest
    }

    pub fn bridge_closeout_digest(&self) -> &str {
        &self.bridge_closeout_digest
    }

    pub fn safe_to_build_now(&self) -> &[String] {
        &self.safe_to_build_now
    }

    pub fn must_not_assume_yet(&self) -> &[String] {
        &self.must_not_assume_yet
    }

    pub fn migration_guidance(&self) -> &[String] {
        &self.migration_guidance
    }

    pub fn required_verification_commands(&self) -> &[String] {
        &self.required_verification_commands
    }

    pub fn closeout_digest(&self) -> &str {
        &self.closeout_digest
    }
}
