use crate::projection_consumption::certification::{
    compile_fail_boundary_bundle_digest, golden_transcript_bundle_digest,
    projection_consumption_compile_fail_proofs, projection_consumption_golden_transcripts,
};
use crate::projection_consumption::{
    certify_projection_consumption_closeout_core, ProjectionConsumptionCertificationLane,
};

#[test]
fn closeout_bundle_emits_full_phase_six_surface() {
    let bundle = certify_projection_consumption_closeout_core();
    for lane in [
        ProjectionConsumptionCertificationLane::SupportMatrixSurface,
        ProjectionConsumptionCertificationLane::PublicBoundarySurface,
        ProjectionConsumptionCertificationLane::ProofShapeSurface,
        ProjectionConsumptionCertificationLane::ForbiddenFallbackSurface,
        ProjectionConsumptionCertificationLane::DxTranscriptSurface,
        ProjectionConsumptionCertificationLane::CompileFailBoundary,
        ProjectionConsumptionCertificationLane::OracleSurface,
        ProjectionConsumptionCertificationLane::SeededReplaySurface,
        ProjectionConsumptionCertificationLane::DownstreamAuthoritySurface,
    ] {
        assert!(bundle.rows().iter().any(|row| row.lane() == lane));
    }
    assert_eq!(bundle.authority_reopen_count(), 0);
    assert!(bundle.fact_extraction_width() > 0);
    assert!(!bundle.certification_bundle_digest().is_empty());
}

#[test]
fn closeout_bundle_emits_all_spec_required_outputs() {
    let bundle = certify_projection_consumption_closeout_core();
    for output in [
        "query_digest",
        "result_shape_digest",
        "authorized_projection_digest",
        "materialization_basis_digest",
        "projection_consumption_declaration_digest",
        "projection_consumption_eligibility_digest",
        "materialized_projection_contract_digest",
        "consumed_projection_fact_set_digest",
        "projection_consumption_receipt_digest",
        "projection_consumption_envelope_digest",
        "projection_source_digest",
        "projection_source_receipt_digest",
        "projection_fact_family_inventory_digest",
        "projection_support_matrix_digest",
        "projection_public_surface_digest",
        "projection_target_dx_digest",
        "projection_golden_transcript_digest",
        "projection_proof_shape_digest",
        "projection_forbidden_fallback_digest",
        "projection_forbidden_fallback_total_occurrences",
        "projection_phase_progression_digest",
        "projection_transition_rules_digest",
        "projection_oracle_digest",
        "projection_oracle_manifest_digest",
        "projection_support_traceability_digest",
        "seeded_sequence_digest",
        "seed_replay_digest",
        "seed_generator_class_digest",
        "compile_fail_boundary_digest",
        "negative_dx_boundary_digest",
        "failure_digest",
        "counter_snapshot",
        "authority_reopen_count",
        "fact_extraction_width",
        "projection_declaration_slope_digest",
        "projection_eligibility_slope_digest",
        "projection_contract_binding_slope_digest",
        "projection_fact_extraction_slope_digest",
        "projection_receipt_materialization_slope_digest",
        "projection_envelope_materialization_slope_digest",
        "projection_support_lookup_slope_digest",
    ] {
        assert!(
            bundle.output_digest(output).is_some(),
            "missing output {output}"
        );
    }
}

#[test]
fn closeout_bundle_still_binds_real_transcript_and_compile_fail_catalogs() {
    let bundle = certify_projection_consumption_closeout_core();
    let golden_digest = golden_transcript_bundle_digest();
    let compile_fail_digest = compile_fail_boundary_bundle_digest();
    assert_eq!(projection_consumption_golden_transcripts().len(), 5);
    assert_eq!(projection_consumption_compile_fail_proofs().len(), 17);
    assert_eq!(
        bundle.output_digest("projection_golden_transcript_digest"),
        Some(golden_digest.as_str())
    );
    assert_eq!(
        bundle.output_digest("compile_fail_boundary_digest"),
        Some(compile_fail_digest.as_str())
    );
}
