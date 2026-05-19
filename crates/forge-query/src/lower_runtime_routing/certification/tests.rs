use super::surface::{
    forge_query_lower_runtime_acceptance_suite, required_phase_six_concrete_seams,
    ForgeQueryLowerRuntimeAcceptanceLane,
};
use super::{
    certify_lower_runtime_performance_slopes, certify_lower_runtime_routing,
    forge_query_lower_runtime_compile_fail_boundary_digest,
    forge_query_lower_runtime_phase_progression_digest,
    forge_query_lower_runtime_proof_shape_audit, forge_query_lower_runtime_proof_shape_digest,
    ForgeQueryLowerRuntimeCertificationLane,
};
use crate::identity::hash_parts;
use crate::lower_runtime_routing::{
    certify_lower_runtime_non_bypass, forge_query_lower_runtime_closeout_registry,
    forge_query_lower_runtime_crossing_inventory, forge_query_lower_runtime_gap_registry,
    forge_query_lower_runtime_support_matrix,
};

const REQUIRED_OUTPUTS: &[&str] = &[
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
    "route_target_dx_digest",
    "route_golden_transcript_digest",
    "route_concrete_surface_digest",
    "route_synthetic_surface_digest",
    "route_proof_shape_digest",
    "route_phase_progression_digest",
    "route_parity_digest",
    "route_non_bypass_digest",
    "lower_runtime_gap_registry_digest",
    "compile_fail_boundary_digest",
    "failure_digest",
    "counter_snapshot",
    "crossing_inventory_width",
    "compatibility_debt_width",
    "route_plan_width",
    "boundary_evidence_width",
    "route_concrete_surface_width",
    "route_synthetic_surface_width",
    "capability_eligibility_slope_digest",
    "route_plan_assembly_slope_digest",
    "boundary_receipt_assembly_slope_digest",
    "boundary_envelope_assembly_slope_digest",
    "support_lookup_slope_digest",
    "debt_registry_lookup_slope_digest",
];

#[test]
fn certification_bundle_contains_phase_seven_lanes() {
    let bundle = certify_lower_runtime_routing();

    for lane in [
        ForgeQueryLowerRuntimeCertificationLane::CrossingsSurface,
        ForgeQueryLowerRuntimeCertificationLane::AcceptanceEvidence,
        ForgeQueryLowerRuntimeCertificationLane::RouteParity,
        ForgeQueryLowerRuntimeCertificationLane::FormerSpecialistSeamClosure,
        ForgeQueryLowerRuntimeCertificationLane::DeferredNeighborDenial,
        ForgeQueryLowerRuntimeCertificationLane::DownstreamBoundaryAudit,
        ForgeQueryLowerRuntimeCertificationLane::ProofShapeSurface,
        ForgeQueryLowerRuntimeCertificationLane::CompileFailBoundary,
        ForgeQueryLowerRuntimeCertificationLane::Performance,
    ] {
        assert!(bundle.rows().iter().any(|row| row.lane() == lane));
    }
    assert!(!bundle.certification_bundle_digest().is_empty());
}

#[test]
fn certification_bundle_emits_required_outputs() {
    let bundle = certify_lower_runtime_routing();
    let crossings = forge_query_lower_runtime_crossing_inventory();
    let support = forge_query_lower_runtime_support_matrix();
    let non_bypass = certify_lower_runtime_non_bypass().expect("non-bypass should pass");

    for output in REQUIRED_OUTPUTS {
        assert!(
            bundle.output_digest(output).is_some(),
            "missing required output {output}"
        );
    }
    assert_eq!(bundle.output_digests().len(), REQUIRED_OUTPUTS.len());
    assert_eq!(
        bundle.output_digest("crossing_inventory_digest"),
        Some(crossings.inventory_digest().as_str())
    );
    assert_eq!(
        bundle.output_digest("route_support_matrix_digest"),
        Some(support.matrix_digest().as_str())
    );
    assert_eq!(
        bundle.output_digest("route_non_bypass_digest"),
        Some(non_bypass.route_non_bypass_digest())
    );
    assert_eq!(
        bundle.output_digest("compile_fail_boundary_digest"),
        Some(forge_query_lower_runtime_compile_fail_boundary_digest().as_str())
    );
    assert_eq!(
        bundle.output_digest("route_proof_shape_digest"),
        Some(forge_query_lower_runtime_proof_shape_digest().as_str())
    );
    assert_eq!(
        bundle.output_digest("route_phase_progression_digest"),
        Some(forge_query_lower_runtime_phase_progression_digest().as_str())
    );
    assert_ne!(
        bundle.output_digest("query_digest"),
        bundle.output_digest("capability_request_digest"),
        "query identity must not collapse into request identity"
    );
    let concrete_width = bundle
        .output_digest("route_concrete_surface_width")
        .expect("concrete width output should exist")
        .parse::<usize>()
        .expect("concrete width should parse as usize");
    let synthetic_width = bundle
        .output_digest("route_synthetic_surface_width")
        .expect("synthetic width output should exist")
        .parse::<usize>()
        .expect("synthetic width should parse as usize");
    assert_eq!(
        concrete_width + synthetic_width,
        crossings.rows().len(),
        "concrete and synthetic surface widths should partition crossing inventory"
    );
    assert!(
        concrete_width >= required_phase_six_concrete_seams().len(),
        "phase six certification should expose every required concrete seam"
    );
    assert_ne!(
        bundle.output_digest("route_concrete_surface_digest"),
        bundle.output_digest("route_synthetic_surface_digest"),
        "concrete and synthetic surfaces must remain distinguishable in the bundle"
    );
}

#[test]
fn certification_bundle_acceptance_lane_matches_named_suite() {
    let bundle = certify_lower_runtime_routing();
    let suite = forge_query_lower_runtime_acceptance_suite();
    let row = bundle
        .rows()
        .iter()
        .find(|row| row.lane() == ForgeQueryLowerRuntimeCertificationLane::AcceptanceEvidence)
        .expect("acceptance evidence lane should exist");

    assert_eq!(row.artifact_digest(), suite.suite_digest());
    assert_eq!(
        row.failure_digest(),
        Some(
            suite
                .lane(ForgeQueryLowerRuntimeAcceptanceLane::Hostile)
                .digest()
        )
    );
}

#[test]
fn certification_bundle_failure_digest_is_hostile_row_aggregate() {
    let bundle = certify_lower_runtime_routing();
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

#[test]
fn parity_digest_changes_when_intentionally_different_route_families_are_compared() {
    let bundle = certify_lower_runtime_routing();
    let route_parity = bundle
        .output_digest("route_parity_digest")
        .expect("route parity output should exist");
    let route_failure = bundle
        .rows()
        .iter()
        .find(|row| row.lane() == ForgeQueryLowerRuntimeCertificationLane::DownstreamBoundaryAudit)
        .and_then(|row| row.failure_digest())
        .expect("downstream boundary hostile digest should exist");

    assert_ne!(
        route_parity, route_failure,
        "parity control digest must not collapse into hostile divergence evidence"
    );
}

#[test]
fn proof_shape_and_slope_surfaces_stay_exported() {
    let proof = forge_query_lower_runtime_proof_shape_audit();
    let support = forge_query_lower_runtime_support_matrix();
    let closeout = forge_query_lower_runtime_closeout_registry();
    let crossings = forge_query_lower_runtime_crossing_inventory();
    let slopes = certify_lower_runtime_performance_slopes(
        crossings.rows().len(),
        18,
        crossings.rows().len(),
        support.rows().len(),
        closeout.rows().len(),
    );

    assert_eq!(proof.rows().len(), 5);
    assert_eq!(slopes.rows().len(), 6);
    assert!(slopes
        .digest_for_output("debt_registry_lookup_slope_digest")
        .is_some());
}

#[test]
fn compatibility_debt_registry_digest_tracks_closed_gap_registry() {
    let bundle = certify_lower_runtime_routing();
    let gaps = forge_query_lower_runtime_gap_registry();

    assert_eq!(gaps.rows().len(), 0);
    assert_eq!(
        bundle.output_digest("compatibility_debt_registry_digest"),
        Some(gaps.registry_digest().as_str())
    );
}

#[test]
fn certification_bundle_phase_six_required_seams_are_concrete() {
    let surface = super::surface::forge_query_lower_runtime_representative_surface();

    for seam_key in required_phase_six_concrete_seams() {
        assert_eq!(
            surface.evidence_source_for(*seam_key),
            Some(
                super::surface::ForgeQueryLowerRuntimeRepresentativeEvidenceSource::RuntimeBackedFixture
            ),
            "required phase six seam {} must remain runtime-backed",
            seam_key.as_str()
        );
    }
}
