use crate::identity::hash_parts;

use super::{
    certify_basis_lifecycle_performance_slopes, BasisLifecycleCertificationLane,
    BasisLifecycleCertificationOutputDigest, BasisLifecycleCertificationRow,
};
use crate::basis_lifecycle::{
    basis_lifecycle_adapter_shape_contract_digest, basis_lifecycle_dx_certification_digest,
    basis_lifecycle_migration_audit_digest, basis_lifecycle_reuse_matrix_digest,
    basis_lifecycle_signal_authority_digest, basis_lifecycle_support_matrix,
};

pub(super) fn certification_output_digests(
    rows: &[BasisLifecycleCertificationRow],
) -> Vec<BasisLifecycleCertificationOutputDigest> {
    let row_digest = |lane| row_digest_for(rows, lane);
    let support_matrix = basis_lifecycle_support_matrix();
    let slope_report = certify_basis_lifecycle_performance_slopes();
    vec![
        output("query_digest", all_rows_digest(rows)),
        output(
            "raw_basis_intent_digest",
            row_digest(BasisLifecycleCertificationLane::Admitted),
        ),
        output(
            "normalized_basis_intent_digest",
            row_digest(BasisLifecycleCertificationLane::Admitted),
        ),
        output("basis_family_digest", families_digest(rows)),
        output(
            "basis_authority_digest",
            row_digest(BasisLifecycleCertificationLane::Admitted),
        ),
        output(
            "basis_scope_digest",
            row_digest(BasisLifecycleCertificationLane::Admitted),
        ),
        output(
            "basis_visibility_digest",
            row_digest(BasisLifecycleCertificationLane::Denied),
        ),
        output(
            "basis_lifecycle_digest",
            row_digest(BasisLifecycleCertificationLane::FutureNeighborDenial),
        ),
        output(
            "basis_policy_digest",
            row_digest(BasisLifecycleCertificationLane::Denied),
        ),
        output(
            "basis_tenant_schema_digest",
            row_digest(BasisLifecycleCertificationLane::Denied),
        ),
        output("basis_operation_lane_digest", operation_lanes_digest(rows)),
        output(
            "basis_eligibility_digest",
            row_digest(BasisLifecycleCertificationLane::Admitted),
        ),
        output(
            "admitted_basis_capability_digest",
            row_digest(BasisLifecycleCertificationLane::Admitted),
        ),
        output(
            "denied_basis_capability_digest",
            row_digest(BasisLifecycleCertificationLane::Denied),
        ),
        output(
            "scoped_basis_digest",
            row_digest(BasisLifecycleCertificationLane::Admitted),
        ),
        output(
            "basis_use_receipt_digest",
            row_digest(BasisLifecycleCertificationLane::Admitted),
        ),
        output(
            "basis_envelope_digest",
            row_digest(BasisLifecycleCertificationLane::Admitted),
        ),
        output(
            "relational_basis_authority_digest",
            row_digest(BasisLifecycleCertificationLane::Denied),
        ),
        output(
            "bridge_basis_authority_digest",
            row_digest(BasisLifecycleCertificationLane::LowerRuntimeMismatch),
        ),
        output(
            "signal_basis_authority_digest",
            basis_lifecycle_signal_authority_digest(),
        ),
        output(
            "lower_runtime_basis_binding_digest",
            row_digest(BasisLifecycleCertificationLane::Admitted),
        ),
        output(
            "basis_readmission_proof_digest",
            row_digest(BasisLifecycleCertificationLane::LowerRuntimeMismatch),
        ),
        output(
            "basis_target_dx_digest",
            basis_lifecycle_dx_certification_digest(),
        ),
        output(
            "basis_golden_transcript_digest",
            basis_lifecycle_dx_certification_digest(),
        ),
        output(
            "lower_runtime_api_reuse_matrix_digest",
            basis_lifecycle_reuse_matrix_digest(),
        ),
        output(
            "adapter_shape_contract_digest",
            basis_lifecycle_adapter_shape_contract_digest(),
        ),
        output(
            "typestate_transition_digest",
            super::basis_lifecycle_typestate_transition_digest(),
        ),
        output("lane_witness_digest", operation_lanes_digest(rows)),
        output(
            "phase_artifact_manifest_digest",
            super::basis_lifecycle_phase_artifact_manifest_digest(),
        ),
        output(
            "compatibility_debt_registry_digest",
            basis_lifecycle_migration_audit_digest(),
        ),
        output(
            "basis_transition_digest",
            row_digest(BasisLifecycleCertificationLane::Admitted),
        ),
        output(
            "basis_support_matrix_digest",
            support_matrix.matrix_digest().to_string(),
        ),
        output(
            "basis_future_neighbor_denial_digest",
            row_digest(BasisLifecycleCertificationLane::FutureNeighborDenial),
        ),
        output(
            "basis_proof_shape_digest",
            super::basis_lifecycle_proof_shape_audit_digest(),
        ),
        output(
            "basis_phase_progression_digest",
            super::basis_lifecycle_phase_progression_digest(),
        ),
        output("failure_digest", failures_digest(rows)),
        output("counter_snapshot", counters_digest(rows)),
        output(
            "basis_normalization_slope_digest",
            slope_digest(&slope_report, "basis_normalization_slope_digest"),
        ),
        output(
            "basis_eligibility_slope_digest",
            slope_digest(&slope_report, "basis_eligibility_slope_digest"),
        ),
        output(
            "basis_lower_runtime_binding_slope_digest",
            slope_digest(&slope_report, "basis_lower_runtime_binding_slope_digest"),
        ),
        output(
            "basis_scoped_use_slope_digest",
            slope_digest(&slope_report, "basis_scoped_use_slope_digest"),
        ),
        output(
            "basis_receipt_slope_digest",
            slope_digest(&slope_report, "basis_receipt_slope_digest"),
        ),
        output(
            "basis_envelope_materialization_slope_digest",
            slope_digest(&slope_report, "basis_envelope_materialization_slope_digest"),
        ),
        output(
            "basis_support_lookup_slope_digest",
            slope_digest(&slope_report, "basis_support_lookup_slope_digest"),
        ),
        output(
            "compile_fail_boundary_digest",
            super::basis_lifecycle_public_boundary_audit_digest(),
        ),
    ]
}

fn output(name: &'static str, digest: String) -> BasisLifecycleCertificationOutputDigest {
    BasisLifecycleCertificationOutputDigest::certified(name, digest)
}

fn slope_digest(report: &super::BasisLifecyclePerformanceSlopeReport, output_name: &str) -> String {
    report
        .digest_for_output(output_name)
        .unwrap_or_else(|| panic!("missing slope digest {output_name}"))
        .to_string()
}

fn row_digest_for(
    rows: &[BasisLifecycleCertificationRow],
    lane: BasisLifecycleCertificationLane,
) -> String {
    rows.iter()
        .find(|row| row.lane() == lane)
        .map(|row| row.row_digest().to_string())
        .unwrap_or_else(|| digest_label("missing_certification_row"))
}

fn all_rows_digest(rows: &[BasisLifecycleCertificationRow]) -> String {
    hash_parts(
        &rows
            .iter()
            .map(|row| row.row_digest().to_string())
            .collect::<Vec<_>>(),
    )
}

fn counters_digest(rows: &[BasisLifecycleCertificationRow]) -> String {
    hash_parts(
        &rows
            .iter()
            .map(|row| row.counter_snapshot_digest().to_string())
            .collect::<Vec<_>>(),
    )
}

fn failures_digest(rows: &[BasisLifecycleCertificationRow]) -> String {
    hash_parts(
        &rows
            .iter()
            .filter_map(|row| row.failure_digest().map(str::to_string))
            .collect::<Vec<_>>(),
    )
}

fn families_digest(rows: &[BasisLifecycleCertificationRow]) -> String {
    hash_parts(
        &rows
            .iter()
            .map(|row| row.basis_family().as_str().to_string())
            .collect::<Vec<_>>(),
    )
}

fn operation_lanes_digest(rows: &[BasisLifecycleCertificationRow]) -> String {
    hash_parts(
        &rows
            .iter()
            .map(|row| row.operation_lane().to_string())
            .collect::<Vec<_>>(),
    )
}

fn digest_label(label: &str) -> String {
    hash_parts(&[label.to_string()])
}
