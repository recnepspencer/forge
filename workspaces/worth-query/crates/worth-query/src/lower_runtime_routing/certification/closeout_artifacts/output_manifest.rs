pub const REQUIRED_CERTIFICATION_OUTPUT_MANIFEST: [&str; 34] = [
    "query_digest",
    "capability_request_digest",
    "capability_family_digest",
    "capability_eligibility_digest",
    "lower_runtime_route_plan_digest",
    "boundary_execution_receipt_digest",
    "lower_runtime_boundary_envelope_digest",
    "crossing_inventory_digest",
    "crossing_classification_digest",
    "compatibility_debt_registry_digest",
    "debt_exit_criteria_digest",
    "route_authority_digest",
    "route_evidence_digest",
    "route_cost_posture_digest",
    "route_failure_topology_digest",
    "route_support_matrix_digest",
    "route_public_surface_digest",
    "route_proof_shape_digest",
    "route_phase_progression_digest",
    "route_parity_digest",
    "route_non_bypass_digest",
    "lower_runtime_gap_registry_digest",
    "failure_digest",
    "counter_snapshot",
    "crossing_inventory_width",
    "compatibility_debt_width",
    "route_plan_width",
    "boundary_evidence_width",
    "capability_eligibility_slope_digest",
    "route_plan_assembly_slope_digest",
    "boundary_receipt_assembly_slope_digest",
    "boundary_envelope_assembly_slope_digest",
    "support_lookup_slope_digest",
    "debt_registry_lookup_slope_digest",
];

pub const CLOSEOUT_EXTENSION_OUTPUT_MANIFEST: [&str; 12] = [
    "route_boundary_reconciliation_digest",
    "route_concrete_surface_digest",
    "route_phase_artifact_manifest_digest",
    "route_synthetic_surface_digest",
    "route_synthetic_tail_policy_digest",
    "route_synthetic_tail_report_digest",
    "route_synthetic_tail_justification_digest",
    "route_concrete_surface_width",
    "route_synthetic_surface_width",
    "route_boundary_reconciliation_width",
    "route_synthetic_tail_width",
    "route_typestate_transition_digest",
];

pub const CERTIFICATION_OUTPUT_MANIFEST: [&str; 46] = [
    "query_digest",
    "capability_request_digest",
    "capability_family_digest",
    "capability_eligibility_digest",
    "lower_runtime_route_plan_digest",
    "boundary_execution_receipt_digest",
    "lower_runtime_boundary_envelope_digest",
    "crossing_inventory_digest",
    "crossing_classification_digest",
    "compatibility_debt_registry_digest",
    "debt_exit_criteria_digest",
    "route_authority_digest",
    "route_evidence_digest",
    "route_cost_posture_digest",
    "route_failure_topology_digest",
    "route_support_matrix_digest",
    "route_public_surface_digest",
    "route_boundary_reconciliation_digest",
    "route_concrete_surface_digest",
    "route_phase_artifact_manifest_digest",
    "route_synthetic_surface_digest",
    "route_synthetic_tail_policy_digest",
    "route_synthetic_tail_report_digest",
    "route_synthetic_tail_justification_digest",
    "route_proof_shape_digest",
    "route_phase_progression_digest",
    "route_parity_digest",
    "route_non_bypass_digest",
    "route_typestate_transition_digest",
    "lower_runtime_gap_registry_digest",
    "failure_digest",
    "counter_snapshot",
    "crossing_inventory_width",
    "compatibility_debt_width",
    "route_plan_width",
    "boundary_evidence_width",
    "route_concrete_surface_width",
    "route_synthetic_surface_width",
    "route_boundary_reconciliation_width",
    "route_synthetic_tail_width",
    "capability_eligibility_slope_digest",
    "route_plan_assembly_slope_digest",
    "boundary_receipt_assembly_slope_digest",
    "boundary_envelope_assembly_slope_digest",
    "support_lookup_slope_digest",
    "debt_registry_lookup_slope_digest",
];

pub fn worth_query_lower_runtime_required_certification_outputs() -> &'static [&'static str] {
    &REQUIRED_CERTIFICATION_OUTPUT_MANIFEST
}

pub fn worth_query_lower_runtime_closeout_extension_outputs() -> &'static [&'static str] {
    &CLOSEOUT_EXTENSION_OUTPUT_MANIFEST
}

pub fn worth_query_lower_runtime_certification_output_manifest() -> &'static [&'static str] {
    &CERTIFICATION_OUTPUT_MANIFEST
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn closeout_manifest_extends_required_outputs_without_duplicates() {
        assert_eq!(
            worth_query_lower_runtime_required_certification_outputs().len()
                + worth_query_lower_runtime_closeout_extension_outputs().len(),
            worth_query_lower_runtime_certification_output_manifest().len()
        );
    }
}
