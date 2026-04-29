use crate::identity::hash_parts;
use forge_runtime_bridge::facade::{
    BridgeAuthoritativeMutationEvidenceCloseout, BridgeAuthoritativeMutationEvidenceSupport,
};

use super::{
    ForgeQueryMutationApiCompatibilityReport, ForgeQueryRuntimeBackendPosture,
    ForgeQueryRuntimePublicApiNamingContract, ForgeQueryRuntimePublicSupportMatrix,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryAuthoritativeMutationEvidenceSupport {
    backend_posture: ForgeQueryRuntimeBackendPosture,
    declared_resolved_target_model: String,
    existing_truth_binding_families: Vec<String>,
    symbolic_target_reference_families: Vec<String>,
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
        let existing_truth_binding_families = vec!["direct_entity_identity".to_string()];
        let symbolic_target_reference_families = vec!["same_batch_declared_target".to_string()];
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
            "aggregate_symbolic_target_reference_digest".to_string(),
            "aggregate_naming_mutation_digest".to_string(),
            "aggregate_continuity_mutation_digest".to_string(),
            "aggregate_causality_digest".to_string(),
            "aggregate_provenance_digest".to_string(),
        ];
        let fail_closed_denial_classes = vec![
            "unresolved_existing_truth_binding".to_string(),
            "mismatched_target_class_binding".to_string(),
            "unsupported_symbolic_target_reference".to_string(),
            "unsupported_naming_mutation_family".to_string(),
            "unsupported_continuity_mutation_family".to_string(),
            "preview_continuity_requires_authoritative_lane".to_string(),
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
            symbolic_target_reference_families
                .iter()
                .map(|item| format!("symbolic-target:{item}")),
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
            symbolic_target_reference_families,
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
            "existing-truth binding, same-batch symbolic target reference, naming mutation, and continuity mutation evidence are part of the ordinary public receipt and inspection story".to_string(),
            "batch and import-style authority sessions preserve aggregate existing-binding, symbolic-target, naming, continuity, causality, and provenance digests".to_string(),
            "downstream domains may rely on Query receipts and inspection instead of rebuilding target-recovery, naming, or continuity explanation glue locally".to_string(),
        ];
        let must_not_assume_yet = vec![
            "authority-mutation evidence closes durable restart, temporal, async, or store-backed mutation semantics".to_string(),
            "unsupported identity-binding, naming, or continuity families remain fail-closed until explicitly admitted".to_string(),
            "downstream code may bypass Query receipts and inspect raw bridge/runtime provenance bags directly".to_string(),
        ];
        let migration_guidance = vec![
            "move authoritative mutation onto workspace.insert/update/delete/batch and consume receipts plus inspect output as the domain explanation contract".to_string(),
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

fn assert_bridge_support_compatibility(
    query_support: &ForgeQueryAuthoritativeMutationEvidenceSupport,
    bridge_support: &BridgeAuthoritativeMutationEvidenceSupport,
    bridge_closeout: &BridgeAuthoritativeMutationEvidenceCloseout,
) {
    let mut failures = Vec::new();

    for section in [
        "declared-resolved-target-evidence",
        "batch-session-causality-provenance",
        "existing-truth-binding",
        "same-batch-symbolic-target-reference",
        "naming-mutation-evidence",
        "continuity-mutation-evidence",
        "replay-safe-request-receipt-digests",
    ] {
        if !bridge_support
            .carry_forward_sections()
            .iter()
            .any(|bridge_section| bridge_section == section)
        {
            failures.push(format!("missing carry-forward section `{section}`"));
        }
    }

    for family in query_support.existing_truth_binding_families() {
        if !bridge_support
            .existing_truth_binding_families()
            .iter()
            .any(|bridge_family| bridge_family == family)
        {
            failures.push(format!("missing existing-truth binding family `{family}`"));
        }
    }
    for family in query_support.symbolic_target_reference_families() {
        if !bridge_support
            .symbolic_target_reference_families()
            .iter()
            .any(|bridge_family| bridge_family == family)
        {
            failures.push(format!("missing symbolic target family `{family}`"));
        }
    }
    for family in query_support.naming_mutation_families() {
        if !bridge_support
            .naming_mutation_families()
            .iter()
            .any(|bridge_family| bridge_family == family)
        {
            failures.push(format!("missing naming family `{family}`"));
        }
    }
    for family in query_support.continuity_mutation_families() {
        if !bridge_support
            .continuity_mutation_families()
            .iter()
            .any(|bridge_family| bridge_family == family)
        {
            failures.push(format!("missing continuity family `{family}`"));
        }
    }

    for section in [
        "aggregate_existing_truth_binding_digest",
        "aggregate_symbolic_target_reference_digest",
        "aggregate_naming_mutation_digest",
        "aggregate_continuity_mutation_digest",
        "aggregate_causality_digest",
        "aggregate_provenance_digest",
    ] {
        if !bridge_support
            .aggregate_evidence_sections()
            .iter()
            .any(|bridge_section| bridge_section == section)
        {
            failures.push(format!("missing aggregate evidence section `{section}`"));
        }
    }

    if !bridge_closeout
        .must_not_assume_yet()
        .iter()
        .any(|line| line.contains("existing-truth binding") && line.contains("fail-closed"))
    {
        failures.push(
            "bridge closeout does not fail-close unsupported existing-truth binding families"
                .to_string(),
        );
    }

    assert!(
        failures.is_empty(),
        "bridge/query authoritative mutation evidence drifted: {}",
        failures.join(", ")
    );
}
