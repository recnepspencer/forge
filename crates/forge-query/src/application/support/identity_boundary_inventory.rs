//! Single source of truth for Milestone 9.6 identity-boundary covered inventory,
//! folklore residue scanning, and session-entrypoint audits.

pub const EVIDENCE_IDENTITY_COVERED_SURFACES: &[&str] = &[
    "runtime_public_support_matrix_row",
    "runtime_public_support_matrix",
    "runtime_public_api_family_contract",
    "runtime_public_api_contract",
    "runtime_public_api_transcript_evidence",
    "runtime_public_api_naming_row",
    "runtime_public_api_naming_contract",
    "runtime_state_snapshot",
    "basis_admission_evidence_row",
    "preview_basis_admission",
    "branch_basis_admission",
    "preview_intent_admission",
    "preview_intent_receipt",
    "branch_intent_admission",
    "branch_intent_receipt",
    "intent_denial_evidence",
    "preview_closeout_evidence",
    "preview_promotion_denial_evidence",
    "preview_execution_evidence",
    "preview_promotion_rebinding",
    "graph_composition_domain_invariant_denial",
    "read_domain_invariant_denial",
    "application_support_report",
];

pub const STOP_CLASS_COVERED_CONTRACTS: &[&str] = &[
    "missing_runtime_component",
    "existing_truth_assertion_denied",
    "existing_truth_probe_denied",
    "mutation_binding_denied",
    "mutation_continuity_denied",
    "graph_composition_denied",
    "graph_composition_domain_invariant_denied",
    "mutation_naming_denied",
    "mutation_target_reference_denied",
    "read_composition_denied",
    "read_composition_domain_invariant_denied",
    "workspace",
    "program",
    "runtime_lookup_failed",
    "missing_runtime_artifact",
    "shared_read_stale_basis",
    "runtime_declaration_failed",
    "session_label_collision",
    "unsupported_authority",
    "intent_commit_denied",
    "intent_execution_routing_failed",
    "effect_policy_denied",
    "preview_promotion_denied",
    "family_admission_denied",
];

pub const SESSION_LABEL_ORDINARY_ENTRYPOINTS: &[&str] = &[
    "runtime.preview",
    "runtime.branch",
    "runtime.try_preview",
    "runtime.try_branch",
    "workspace.preview",
    "workspace.branch",
];

pub const EXACT_ZERO_FORMAT_DIGEST_PATHS: &[&str] = &[
    "application/support/report.rs",
    "runtime/support_matrix.rs",
    "runtime/state_snapshot.rs",
    "runtime/public_api_transcript.rs",
    "runtime/public_api.rs",
    "runtime/support/profile.rs",
    "runtime/public_api_naming.rs",
    "runtime/intent/preview.rs",
    "runtime/intent/denial.rs",
    "runtime/intent/branch.rs",
    "runtime/support/authority_artifacts.rs",
    "runtime/preview/evidence/closeout.rs",
    "runtime/preview/evidence/promotion.rs",
    "runtime/preview/evidence/execution.rs",
    "runtime/preview/workflow_ops.rs",
    "runtime/mutation/graph_composition/domain_invariant_denial.rs",
    "runtime/mutation/graph_composition/hooks.rs",
    "runtime/read_composition_hooks.rs",
    "runtime/surface/read_domain_invariant_denial.rs",
];

pub const EXACT_ZERO_STRING_MATCHING_PATHS: &[&str] =
    &["runtime/tests/stop_class/consumer_support/routing.rs"];

pub const EXACT_ZERO_RAW_SESSION_ADMISSION_PATHS: &[&str] =
    &["runtime/runtime_sessions.rs", "runtime/workspace.rs"];

/// Paths that retain pre-9.6 joined-string digest folklore by explicit milestone scope.
pub const EXCLUDED_FOLKLORE_PATHS: &[&str] = &[
    "subscription/",
    "projection_consumption/",
    "workflow/",
    "domain_capabilities/",
    "harness/milestone_nine_five_",
    "runtime/intent/receipt.rs",
    "runtime/intent/effect_triggered.rs",
    "runtime/intent/failure.rs",
    "runtime/intent/declaration.rs",
    "runtime/intent/provenance.rs",
];

const FORBIDDEN_DIGEST_FOLKLORE_PATTERNS: &[&str] = &[
    "hash_parts(",
    "digest_owned_parts(",
    ".join(\"|\")",
    "format!(\"{}|",
    "format!(\"{:?}\"",
    "format!(\"{:?}|",
    "format!(\"{:#?}\"",
    "format!(\"{:#?}|",
];

const REQUIRED_TYPED_SESSION_LABEL_SIGNATURES: &[&str] = &[
    "pub fn preview<'a>(\n        &'a mut self,\n        label: ForgeQuerySessionLabel,",
    "pub fn branch<'a>(\n        &'a mut self,\n        label: ForgeQuerySessionLabel,",
    "pub fn preview_with_options<'a>(\n        &'a mut self,\n        label: ForgeQuerySessionLabel,",
    "pub fn branch_with_options<'a>(\n        &'a mut self,\n        label: ForgeQuerySessionLabel,",
    "pub fn try_preview<'a>(\n        &'a mut self,\n        label: ForgeQuerySessionLabel,",
    "pub fn try_branch<'a>(\n        &'a mut self,\n        label: ForgeQuerySessionLabel,",
    "pub fn try_preview_with_options<'a>(\n        &'a mut self,\n        label: ForgeQuerySessionLabel,",
    "pub fn try_branch_with_options<'a>(\n        &'a mut self,\n        label: ForgeQuerySessionLabel,",
];

const FORBIDDEN_RAW_SESSION_LABEL_SIGNATURES: &[&str] = &[
    "pub fn preview<'a>(\n        &'a mut self,\n        label: impl Into<String>,",
    "pub fn branch<'a>(\n        &'a mut self,\n        label: impl Into<String>,",
    "pub fn preview_with_options<'a>(\n        &'a mut self,\n        label: impl Into<String>,",
    "pub fn branch_with_options<'a>(\n        &'a mut self,\n        label: impl Into<String>,",
    "pub fn try_preview<'a>(\n        &'a mut self,\n        label: impl Into<String>,",
    "pub fn try_branch<'a>(\n        &'a mut self,\n        label: impl Into<String>,",
    "pub fn try_preview_with_options<'a>(\n        &'a mut self,\n        label: impl Into<String>,",
    "pub fn try_branch_with_options<'a>(\n        &'a mut self,\n        label: impl Into<String>,",
];

pub fn normalize_source_text(source: &str) -> String {
    source.replace("\r\n", "\n")
}

pub fn source_for_format_digest_path(path: &str) -> Option<&'static str> {
    match path {
        "application/support/report.rs" => Some(include_str!("report.rs")),
        "runtime/support_matrix.rs" => Some(include_str!("../../runtime/support_matrix.rs")),
        "runtime/state_snapshot.rs" => Some(include_str!("../../runtime/state_snapshot.rs")),
        "runtime/public_api_transcript.rs" => {
            Some(include_str!("../../runtime/public_api_transcript.rs"))
        }
        "runtime/public_api.rs" => Some(include_str!("../../runtime/public_api.rs")),
        "runtime/support/profile.rs" => Some(include_str!("../../runtime/support/profile.rs")),
        "runtime/public_api_naming.rs" => Some(include_str!("../../runtime/public_api_naming.rs")),
        "runtime/intent/preview.rs" => Some(include_str!("../../runtime/intent/preview.rs")),
        "runtime/intent/denial.rs" => Some(include_str!("../../runtime/intent/denial.rs")),
        "runtime/intent/branch.rs" => Some(include_str!("../../runtime/intent/branch.rs")),
        "runtime/support/authority_artifacts.rs" => {
            Some(include_str!("../../runtime/support/authority_artifacts.rs"))
        }
        "runtime/preview/evidence/closeout.rs" => {
            Some(include_str!("../../runtime/preview/evidence/closeout.rs"))
        }
        "runtime/preview/evidence/promotion.rs" => {
            Some(include_str!("../../runtime/preview/evidence/promotion.rs"))
        }
        "runtime/preview/evidence/execution.rs" => {
            Some(include_str!("../../runtime/preview/evidence/execution.rs"))
        }
        "runtime/preview/workflow_ops.rs" => Some(include_str!("../../runtime/preview/workflow_ops.rs")),
        "runtime/mutation/graph_composition/domain_invariant_denial.rs" => Some(include_str!(
            "../../runtime/mutation/graph_composition/domain_invariant_denial.rs"
        )),
        "runtime/mutation/graph_composition/hooks.rs" => Some(include_str!(
            "../../runtime/mutation/graph_composition/hooks.rs"
        )),
        "runtime/read_composition_hooks.rs" => {
            Some(include_str!("../../runtime/read_composition_hooks.rs"))
        }
        "runtime/surface/read_domain_invariant_denial.rs" => Some(include_str!(
            "../../runtime/surface/read_domain_invariant_denial.rs"
        )),
        _ => None,
    }
}

pub fn source_for_string_matching_path(path: &str) -> Option<&'static str> {
    match path {
        "runtime/tests/stop_class/consumer_support/routing.rs" => Some(include_str!(
            "../../runtime/tests/stop_class/consumer_support/routing.rs"
        )),
        _ => None,
    }
}

pub fn source_for_session_admission_path(path: &str) -> Option<&'static str> {
    match path {
        "runtime/runtime_sessions.rs" => Some(include_str!("../../runtime/runtime_sessions.rs")),
        "runtime/workspace.rs" => Some(include_str!("../../runtime/workspace.rs")),
        _ => None,
    }
}

pub fn format_digest_folklore_pattern_in(source: &str) -> Option<&'static str> {
    FORBIDDEN_DIGEST_FOLKLORE_PATTERNS
        .iter()
        .copied()
        .find(|pattern| source.contains(pattern))
}

pub fn scan_format_digest_residue_paths() -> Vec<&'static str> {
    let mut remaining = Vec::new();
    for &path in EXACT_ZERO_FORMAT_DIGEST_PATHS {
        let Some(source) = source_for_format_digest_path(path) else {
            remaining.push(path);
            continue;
        };
        if format_digest_folklore_pattern_in(source).is_some() {
            remaining.push(path);
        }
    }
    remaining
}

pub fn scan_string_matching_residue_paths() -> Vec<&'static str> {
    let mut remaining = Vec::new();
    for &path in EXACT_ZERO_STRING_MATCHING_PATHS {
        let Some(source) = source_for_string_matching_path(path) else {
            remaining.push(path);
            continue;
        };
        if source.contains("to_string().contains(")
            || source.contains("message.contains")
            || source.contains("error_message.contains")
        {
            remaining.push(path);
        }
    }
    remaining
}

pub fn scan_raw_session_admission_residue_paths() -> Vec<&'static str> {
    let mut remaining = Vec::new();
    for &path in EXACT_ZERO_RAW_SESSION_ADMISSION_PATHS {
        let Some(source) = source_for_session_admission_path(path) else {
            remaining.push(path);
            continue;
        };
        let normalized = normalize_source_text(source);
        if normalized.contains("label: impl Into<String>") {
            remaining.push(path);
            continue;
        }
        if !normalized.contains("label: ForgeQuerySessionLabel") {
            remaining.push(path);
        }
    }
    remaining
}

pub fn ordinary_session_entrypoint_audit_violations(
    runtime_sessions: &str,
    workspace: &str,
) -> Vec<String> {
    let runtime_sessions = normalize_source_text(runtime_sessions);
    let workspace = normalize_source_text(workspace);
    let mut violations = Vec::new();

    for required in REQUIRED_TYPED_SESSION_LABEL_SIGNATURES {
        if !runtime_sessions.contains(required) && !workspace.contains(required) {
            violations.push(format!("missing typed entrypoint signature: {required}"));
        }
    }
    for forbidden in FORBIDDEN_RAW_SESSION_LABEL_SIGNATURES {
        if runtime_sessions.contains(forbidden) || workspace.contains(forbidden) {
            violations.push(format!("raw-string entrypoint survived: {forbidden}"));
        }
    }
    violations
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inventory_lists_are_non_empty_and_unique() {
        assert!(!EVIDENCE_IDENTITY_COVERED_SURFACES.is_empty());
        assert!(!EXACT_ZERO_FORMAT_DIGEST_PATHS.is_empty());
        assert_eq!(
            EXACT_ZERO_FORMAT_DIGEST_PATHS.len(),
            EXACT_ZERO_FORMAT_DIGEST_PATHS
                .iter()
                .collect::<std::collections::BTreeSet<_>>()
                .len()
        );
    }

    #[test]
    fn every_format_digest_path_has_embedded_source() {
        for path in EXACT_ZERO_FORMAT_DIGEST_PATHS {
            assert!(
                source_for_format_digest_path(path).is_some(),
                "missing embedded source for {path}"
            );
        }
    }
}
