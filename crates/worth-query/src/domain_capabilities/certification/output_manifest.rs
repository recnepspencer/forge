pub const DOMAIN_CAPABILITY_CERTIFICATION_OUTPUT_MANIFEST: &[&str] = &[
    "query_digest",
    "intent_declaration_digest",
    "domain_capability_contribution_request_digest",
    "domain_capability_contribution_eligibility_digest",
    "admitted_domain_capability_contribution_digest",
    "canonical_runtime_materialization_digest",
    "admission_artifact_digest",
    "support_artifact_digest",
    "workflow_artifact_digest",
    "continuity_artifact_digest",
    "aftermath_artifact_digest",
    "explanation_artifact_digest",
    "capability_support_row_digest",
    "domain_invariant_denial_digest",
    "decision_trace_digest",
    "support_traceability_digest",
    "public_boundary_digest",
    "compile_fail_boundary_digest",
    "failure_digest",
    "counter_snapshot",
    "contribution_width",
    "trace_width",
    "category_width",
    "support_width",
    "contribution_materialization_slope_digest",
    "trace_materialization_slope_digest",
    "category_materialization_slope_digest",
    "support_materialization_slope_digest",
];

pub fn worth_query_domain_capability_certification_output_manifest() -> &'static [&'static str] {
    DOMAIN_CAPABILITY_CERTIFICATION_OUTPUT_MANIFEST
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn output_manifest_is_duplicate_free_and_matches_spec_shape() {
        let manifest = worth_query_domain_capability_certification_output_manifest();

        assert_eq!(manifest.len(), 28);
        assert_eq!(
            manifest.len(),
            manifest.iter().copied().collect::<BTreeSet<_>>().len()
        );
        assert_eq!(manifest[0], "query_digest");
        assert_eq!(manifest[17], "compile_fail_boundary_digest");
        assert_eq!(manifest[27], "support_materialization_slope_digest");
    }
}
