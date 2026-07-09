use super::{
    certify_basis_lifecycle, certify_basis_lifecycle_performance_slopes,
    BasisLifecycleCertificationLane, BasisLifecycleCertificationOutputPosture,
};
use crate::basis_lifecycle::{
    basis_lifecycle_adapter_shape_contract_digest, basis_lifecycle_migration_audit_digest,
    basis_lifecycle_phase_artifact_manifest_digest, basis_lifecycle_phase_progression_digest,
    basis_lifecycle_proof_shape_audit_digest, basis_lifecycle_public_boundary_audit_digest,
    basis_lifecycle_reuse_matrix_digest, basis_lifecycle_signal_authority_digest,
    basis_lifecycle_typestate_transition_digest,
};
use crate::identity::hash_parts;

const REQUIRED_OUTPUTS: &[&str] = &[
    "query_digest",
    "raw_basis_intent_digest",
    "normalized_basis_intent_digest",
    "basis_family_digest",
    "basis_authority_digest",
    "basis_scope_digest",
    "basis_visibility_digest",
    "basis_lifecycle_digest",
    "basis_policy_digest",
    "basis_tenant_schema_digest",
    "basis_operation_lane_digest",
    "basis_eligibility_digest",
    "admitted_basis_capability_digest",
    "denied_basis_capability_digest",
    "scoped_basis_digest",
    "basis_use_receipt_digest",
    "basis_envelope_digest",
    "relational_basis_authority_digest",
    "bridge_basis_authority_digest",
    "signal_basis_authority_digest",
    "lower_runtime_basis_binding_digest",
    "basis_readmission_proof_digest",
    "basis_target_dx_digest",
    "basis_golden_transcript_digest",
    "lower_runtime_api_reuse_matrix_digest",
    "adapter_shape_contract_digest",
    "typestate_transition_digest",
    "lane_witness_digest",
    "phase_artifact_manifest_digest",
    "compatibility_debt_registry_digest",
    "basis_transition_digest",
    "basis_support_matrix_digest",
    "basis_future_neighbor_denial_digest",
    "basis_proof_shape_digest",
    "basis_phase_progression_digest",
    "failure_digest",
    "counter_snapshot",
    "basis_normalization_slope_digest",
    "basis_eligibility_slope_digest",
    "basis_lower_runtime_binding_slope_digest",
    "basis_scoped_use_slope_digest",
    "basis_receipt_slope_digest",
    "basis_envelope_materialization_slope_digest",
    "basis_support_lookup_slope_digest",
    "compile_fail_boundary_digest",
];

#[test]
fn certification_bundle_contains_required_representative_lanes() {
    let bundle = certify_basis_lifecycle();

    for lane in [
        BasisLifecycleCertificationLane::Admitted,
        BasisLifecycleCertificationLane::Advisory,
        BasisLifecycleCertificationLane::Denied,
        BasisLifecycleCertificationLane::LowerRuntimeMismatch,
        BasisLifecycleCertificationLane::FutureNeighborDenial,
        BasisLifecycleCertificationLane::Performance,
    ] {
        assert!(
            bundle.rows().iter().any(|row| row.lane() == lane),
            "missing certification lane {}",
            lane.as_str()
        );
    }
    assert_eq!(
        bundle
            .counters()
            .basis_certification_bundle_assembly_count(),
        1
    );
    assert_eq!(bundle.counters().basis_certification_row_count(), 6);
    assert!(!bundle.certification_bundle_digest().is_empty());
}

#[test]
fn certification_bundle_emits_required_verification_outputs() {
    let bundle = certify_basis_lifecycle();

    for required in REQUIRED_OUTPUTS {
        let output = bundle
            .output_digests()
            .iter()
            .find(|output| output.name() == *required)
            .unwrap_or_else(|| panic!("missing required output {required}"));
        assert!(!output.digest().is_empty(), "empty digest for {required}");
    }
    assert_eq!(bundle.output_digests().len(), REQUIRED_OUTPUTS.len());
    assert_eq!(
        output_posture(&bundle, "signal_basis_authority_digest"),
        BasisLifecycleCertificationOutputPosture::Certified
    );
    assert_eq!(
        output_posture(&bundle, "basis_target_dx_digest"),
        BasisLifecycleCertificationOutputPosture::Certified
    );
    assert_eq!(
        output_posture(&bundle, "basis_golden_transcript_digest"),
        BasisLifecycleCertificationOutputPosture::Certified
    );
    assert_eq!(
        output_posture(&bundle, "compatibility_debt_registry_digest"),
        BasisLifecycleCertificationOutputPosture::Certified
    );
    assert_eq!(
        output_posture(&bundle, "basis_envelope_digest"),
        BasisLifecycleCertificationOutputPosture::Certified
    );
    assert_eq!(
        bundle.output_digest("compatibility_debt_registry_digest"),
        Some(basis_lifecycle_migration_audit_digest().as_str())
    );
    assert_eq!(
        bundle.output_digest("lower_runtime_api_reuse_matrix_digest"),
        Some(basis_lifecycle_reuse_matrix_digest().as_str())
    );
    assert_eq!(
        bundle.output_digest("signal_basis_authority_digest"),
        Some(basis_lifecycle_signal_authority_digest().as_str())
    );
    assert_eq!(
        bundle.output_digest("adapter_shape_contract_digest"),
        Some(basis_lifecycle_adapter_shape_contract_digest().as_str())
    );
    assert_eq!(
        bundle.output_digest("compile_fail_boundary_digest"),
        Some(basis_lifecycle_public_boundary_audit_digest().as_str())
    );
    assert_eq!(
        bundle.output_digest("basis_proof_shape_digest"),
        Some(basis_lifecycle_proof_shape_audit_digest().as_str())
    );
    assert_eq!(
        bundle.output_digest("basis_phase_progression_digest"),
        Some(basis_lifecycle_phase_progression_digest().as_str())
    );
    assert_eq!(
        bundle.output_digest("typestate_transition_digest"),
        Some(basis_lifecycle_typestate_transition_digest().as_str())
    );
    assert_eq!(
        bundle.output_digest("phase_artifact_manifest_digest"),
        Some(basis_lifecycle_phase_artifact_manifest_digest().as_str())
    );
    assert_ne!(
        bundle.output_digest("typestate_transition_digest"),
        bundle.output_digest("query_digest"),
        "typestate output must not be a generic row summary"
    );
    assert_ne!(
        bundle.output_digest("phase_artifact_manifest_digest"),
        bundle.output_digest("query_digest"),
        "phase manifest output must not be a generic row summary"
    );
}

#[test]
fn certification_rows_bind_failures_and_performance_counters() {
    let bundle = certify_basis_lifecycle();
    let slope_report = certify_basis_lifecycle_performance_slopes();
    let denied_rows = bundle
        .rows()
        .iter()
        .filter(|row| row.failure_digest().is_some())
        .count();
    let performance = bundle
        .rows()
        .iter()
        .find(|row| row.lane() == BasisLifecycleCertificationLane::Performance)
        .expect("performance lane must be present");

    assert!(denied_rows >= 3);
    assert_eq!(performance.operation_lane(), "observation");
    assert!(!performance.counter_snapshot_digest().is_empty());
    for row in slope_report.rows() {
        assert_eq!(
            bundle.output_digest(row.family().output_name()),
            Some(row.slope_digest())
        );
        assert_ne!(
            bundle.output_digest(row.family().output_name()),
            Some(performance.row_digest()),
            "{} must be a stage-specific slope digest",
            row.family().output_name()
        );
    }
}

#[test]
fn certification_rows_exercise_control_hostile_and_parity_lanes() {
    let bundle = certify_basis_lifecycle();
    let admitted = row_for(&bundle, BasisLifecycleCertificationLane::Admitted);
    let denied = row_for(&bundle, BasisLifecycleCertificationLane::Denied);
    let mismatch = row_for(
        &bundle,
        BasisLifecycleCertificationLane::LowerRuntimeMismatch,
    );
    let future = row_for(
        &bundle,
        BasisLifecycleCertificationLane::FutureNeighborDenial,
    );

    assert_eq!(admitted.basis_family().as_str(), "current_head");
    assert_eq!(admitted.operation_lane(), "observation");
    assert!(admitted.failure_digest().is_none());
    assert!(denied.failure_digest().is_some());
    assert!(mismatch.failure_digest().is_some());
    assert!(future.failure_digest().is_some());
    assert_ne!(admitted.artifact_digest(), denied.artifact_digest());
    assert_ne!(denied.failure_digest(), mismatch.failure_digest());
    assert_ne!(mismatch.failure_digest(), future.failure_digest());
}

#[test]
fn certification_failure_digest_is_derived_from_hostile_rows() {
    let bundle = certify_basis_lifecycle();
    let expected = hash_parts(
        &bundle
            .rows()
            .iter()
            .filter_map(|row| row.failure_digest().map(str::to_string))
            .collect::<Vec<_>>(),
    );

    assert_eq!(
        bundle.output_digest("failure_digest"),
        Some(expected.as_str())
    );
}

fn row_for(
    bundle: &super::BasisLifecycleCertificationBundle,
    lane: BasisLifecycleCertificationLane,
) -> &super::BasisLifecycleCertificationRow {
    bundle
        .rows()
        .iter()
        .find(|row| row.lane() == lane)
        .unwrap_or_else(|| panic!("missing lane {}", lane.as_str()))
}

fn output_posture(
    bundle: &super::BasisLifecycleCertificationBundle,
    name: &str,
) -> BasisLifecycleCertificationOutputPosture {
    bundle
        .output_digests()
        .iter()
        .find(|output| output.name() == name)
        .map(|output| output.posture())
        .unwrap_or_else(|| panic!("missing output {name}"))
}
