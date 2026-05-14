use crate::identity::hash_parts;

use super::super::receipt_transitions::ProjectionConsumptionTransitionRules;
use super::boundary::ProjectionConsumptionPublicBoundaryAudit;
use super::bundle::{
    ProjectionConsumptionCertificationLane, ProjectionConsumptionCertificationRow,
};
use super::fixtures::{
    denied_masked_field_failure_digest, source_digest, source_mismatch_failure_digest,
    source_receipt_digest, ProjectionConsumptionCertifiedLifecycle,
};
use super::oracles::ProjectionConsumptionOracleReport;
use super::proof_shape::ProjectionConsumptionProofShapeAudit;
use super::seeded::ProjectionConsumptionSeededCertificationReport;
use super::slopes::ProjectionConsumptionSlopeReport;
use super::support_matrix::{
    ProjectionConsumptionFamilyInventory, ProjectionConsumptionSupportMatrix,
};

pub struct ProjectionConsumptionBundleOutputs {
    pub rows: Vec<ProjectionConsumptionCertificationRow>,
    pub outputs: Vec<(&'static str, String)>,
}

pub fn assemble_closeout_bundle_outputs(
    lifecycle: &ProjectionConsumptionCertifiedLifecycle,
    family_inventory: &ProjectionConsumptionFamilyInventory,
    support_matrix: &ProjectionConsumptionSupportMatrix,
    public_boundary_audit: &ProjectionConsumptionPublicBoundaryAudit,
    proof_shape_audit: &ProjectionConsumptionProofShapeAudit,
    oracle_report: &ProjectionConsumptionOracleReport,
    seeded_report: &ProjectionConsumptionSeededCertificationReport,
    slope_report: &ProjectionConsumptionSlopeReport,
    compile_fail_boundary_digest: String,
    golden_transcript_digest: String,
) -> ProjectionConsumptionBundleOutputs {
    let transition_rules = ProjectionConsumptionTransitionRules::current_phase_five_surface();
    let target_dx_digest = target_dx_digest();
    let negative_dx_boundary_digest = hash_parts(&[
        "projection_consumption_negative_dx_boundary_v1".to_string(),
        public_boundary_audit.audit_digest().to_string(),
        compile_fail_boundary_digest.clone(),
    ]);
    let failure_digest = hash_parts(&[
        denied_masked_field_failure_digest(),
        source_mismatch_failure_digest(),
    ]);
    let rows = vec![
        certification_row(
            ProjectionConsumptionCertificationLane::SupportMatrixSurface,
            format!(
                "inventory:{}|matrix:{}|traceability:{}",
                family_inventory.inventory_digest(),
                support_matrix.matrix_digest(),
                support_matrix.support_traceability_digest()
            ),
        ),
        certification_row(
            ProjectionConsumptionCertificationLane::PublicBoundarySurface,
            format!(
                "public_surface:{}|negative_dx:{}",
                public_boundary_audit.audit_digest(),
                negative_dx_boundary_digest
            ),
        ),
        certification_row(
            ProjectionConsumptionCertificationLane::ProofShapeSurface,
            format!(
                "proof_shape:{}|phase_progression:{}",
                proof_shape_audit.proof_shape_digest(),
                proof_shape_audit.phase_progression_digest()
            ),
        ),
        certification_row(
            ProjectionConsumptionCertificationLane::DxTranscriptSurface,
            format!("target_dx:{target_dx_digest}|golden:{golden_transcript_digest}"),
        ),
        certification_row(
            ProjectionConsumptionCertificationLane::CompileFailBoundary,
            format!("compile_fail:{compile_fail_boundary_digest}"),
        ),
        certification_row(
            ProjectionConsumptionCertificationLane::OracleSurface,
            format!(
                "oracle:{}|manifest:{}",
                oracle_report.oracle_digest(),
                oracle_report.manifest_digest()
            ),
        ),
        certification_row(
            ProjectionConsumptionCertificationLane::SeededReplaySurface,
            format!(
                "seeded:{}|replay:{}|classes:{}",
                seeded_report.seeded_sequence_digest(),
                seeded_report.seed_replay_digest(),
                seeded_report.seed_generator_class_digest()
            ),
        ),
    ];
    let outputs = vec![
        (
            "query_digest",
            lifecycle
                .declaration()
                .binding()
                .authorized_projection_query_digest()
                .to_string(),
        ),
        (
            "result_shape_digest",
            lifecycle
                .declaration()
                .binding()
                .result_shape_digest()
                .to_string(),
        ),
        (
            "authorized_projection_digest",
            lifecycle
                .declaration()
                .binding()
                .authorized_projection_identity()
                .to_string(),
        ),
        (
            "materialization_basis_digest",
            lifecycle
                .contract()
                .basis_digest()
                .unwrap_or("none")
                .to_string(),
        ),
        (
            "projection_consumption_declaration_digest",
            lifecycle.declaration().declaration_digest().to_string(),
        ),
        (
            "projection_consumption_eligibility_digest",
            lifecycle.contract().eligibility_digest().to_string(),
        ),
        (
            "materialized_projection_contract_digest",
            lifecycle.contract().contract_digest().to_string(),
        ),
        (
            "consumed_projection_fact_set_digest",
            lifecycle.facts().fact_set_digest().to_string(),
        ),
        (
            "projection_consumption_receipt_digest",
            lifecycle.receipt().receipt_digest().to_string(),
        ),
        (
            "projection_consumption_envelope_digest",
            lifecycle.envelope().envelope_digest().to_string(),
        ),
        (
            "projection_source_digest",
            source_digest(lifecycle.contract()),
        ),
        (
            "projection_source_receipt_digest",
            source_receipt_digest(lifecycle.contract()),
        ),
        (
            "projection_fact_family_inventory_digest",
            family_inventory.inventory_digest().to_string(),
        ),
        (
            "projection_support_matrix_digest",
            support_matrix.matrix_digest().to_string(),
        ),
        (
            "projection_public_surface_digest",
            public_boundary_audit.audit_digest().to_string(),
        ),
        ("projection_target_dx_digest", target_dx_digest),
        (
            "projection_golden_transcript_digest",
            golden_transcript_digest,
        ),
        (
            "projection_proof_shape_digest",
            proof_shape_audit.proof_shape_digest().to_string(),
        ),
        (
            "projection_phase_progression_digest",
            proof_shape_audit.phase_progression_digest().to_string(),
        ),
        (
            "projection_transition_rules_digest",
            transition_rules.rules_digest().to_string(),
        ),
        (
            "projection_oracle_digest",
            oracle_report.oracle_digest().to_string(),
        ),
        (
            "projection_oracle_manifest_digest",
            oracle_report.manifest_digest().to_string(),
        ),
        (
            "projection_support_traceability_digest",
            support_matrix.support_traceability_digest().to_string(),
        ),
        (
            "seeded_sequence_digest",
            seeded_report.seeded_sequence_digest().to_string(),
        ),
        (
            "seed_replay_digest",
            seeded_report.seed_replay_digest().to_string(),
        ),
        (
            "seed_generator_class_digest",
            seeded_report.seed_generator_class_digest().to_string(),
        ),
        ("compile_fail_boundary_digest", compile_fail_boundary_digest),
        ("negative_dx_boundary_digest", negative_dx_boundary_digest),
        ("failure_digest", failure_digest),
        (
            "counter_snapshot",
            slope_report.counter_snapshot().digest().to_string(),
        ),
        (
            "authority_reopen_count",
            slope_report
                .counter_snapshot()
                .authority_reopen_count()
                .to_string(),
        ),
        (
            "fact_extraction_width",
            slope_report
                .counter_snapshot()
                .source_row_width_consumed()
                .to_string(),
        ),
        (
            "projection_declaration_slope_digest",
            slope_report.declaration_slope_digest().to_string(),
        ),
        (
            "projection_eligibility_slope_digest",
            slope_report.eligibility_slope_digest().to_string(),
        ),
        (
            "projection_contract_binding_slope_digest",
            slope_report.contract_binding_slope_digest().to_string(),
        ),
        (
            "projection_fact_extraction_slope_digest",
            slope_report.fact_extraction_slope_digest().to_string(),
        ),
        (
            "projection_receipt_materialization_slope_digest",
            slope_report
                .receipt_materialization_slope_digest()
                .to_string(),
        ),
        (
            "projection_envelope_materialization_slope_digest",
            slope_report
                .envelope_materialization_slope_digest()
                .to_string(),
        ),
        (
            "projection_support_lookup_slope_digest",
            slope_report.support_lookup_slope_digest().to_string(),
        ),
    ];
    ProjectionConsumptionBundleOutputs { rows, outputs }
}

fn certification_row(
    lane: ProjectionConsumptionCertificationLane,
    evidence_detail: String,
) -> ProjectionConsumptionCertificationRow {
    let row_digest = hash_parts(&[
        "projection_consumption_certification_row_v1".to_string(),
        format!("lane:{}", lane.as_str()),
        format!("detail:{evidence_detail}"),
    ]);
    ProjectionConsumptionCertificationRow {
        lane,
        evidence_detail,
        row_digest,
    }
}

fn target_dx_digest() -> String {
    hash_parts(&[
        "projection_consumption_target_dx_v1".to_string(),
        "common_path_read_backed_consumption".to_string(),
        "common_path_effect_backed_consumption".to_string(),
        "support_discovery_before_consumption".to_string(),
        "typed_denial_and_deferred_handling".to_string(),
        "receipt_first_inspection_and_envelope_derivation".to_string(),
    ])
}
