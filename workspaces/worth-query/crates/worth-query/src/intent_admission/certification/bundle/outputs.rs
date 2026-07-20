use crate::identity::hash_parts;

use super::super::audits::{
    WorthQueryIntentAdmissionProofShapeAudit, WorthQueryIntentAdmissionPublicBoundaryAudit,
    WorthQueryIntentAdmissionTopologyAudit,
};
use super::super::oracles::WorthQueryIntentAdmissionOracleReport;
use super::super::reports::{
    WorthQueryIntentAdmissionLegacyParityReport,
    WorthQueryIntentAdmissionRepresentativeFamilyReport,
    WorthQueryIntentAdmissionRepresentativeOutputReport,
    WorthQueryIntentAdmissionSeededCertificationReport, WorthQueryIntentAdmissionSlopeReport,
    WorthQueryIntentAdmissionSupportTraceabilityReport,
};
use crate::intent_admission::{
    WorthQueryIntentAdmissionCoverageInventory, WorthQueryIntentAdmissionFamilyInventory,
    WorthQueryIntentAdmissionSupportMatrix,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryIntentAdmissionCertificationOutputSpec {
    name: &'static str,
    digest: String,
}

impl WorthQueryIntentAdmissionCertificationOutputSpec {
    pub(crate) fn new(name: &'static str, digest: String) -> Self {
        Self { name, digest }
    }

    pub(crate) fn name(&self) -> &'static str {
        self.name
    }

    pub(crate) fn digest(&self) -> &str {
        &self.digest
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn assemble_certification_outputs(
    family_inventory: &WorthQueryIntentAdmissionFamilyInventory,
    coverage_inventory: &WorthQueryIntentAdmissionCoverageInventory,
    support_matrix: &WorthQueryIntentAdmissionSupportMatrix,
    public_boundary_audit: &WorthQueryIntentAdmissionPublicBoundaryAudit,
    proof_shape_audit: &WorthQueryIntentAdmissionProofShapeAudit,
    topology_audit: &WorthQueryIntentAdmissionTopologyAudit,
    representative_output_report: &WorthQueryIntentAdmissionRepresentativeOutputReport,
    representative_family_report: &WorthQueryIntentAdmissionRepresentativeFamilyReport,
    oracle_report: &WorthQueryIntentAdmissionOracleReport,
    legacy_parity_report: &WorthQueryIntentAdmissionLegacyParityReport,
    support_traceability_report: &WorthQueryIntentAdmissionSupportTraceabilityReport,
    seeded_report: &WorthQueryIntentAdmissionSeededCertificationReport,
    slope_report: &WorthQueryIntentAdmissionSlopeReport,
) -> Vec<WorthQueryIntentAdmissionCertificationOutputSpec> {
    let representative_output_names = [
        "query_digest",
        "raw_intent_digest",
        "intent_eligibility_digest",
        "admission_decision_digest",
        "admitted_intent_plan_digest",
        "admitted_execution_handoff_digest",
        "advisory_decision_digest",
        "violation_decision_digest",
        "decision_trace_digest",
        "decision_trace_envelope_digest",
        "policy_decision_digest",
        "capability_decision_digest",
        "invariant_decision_digest",
        "basis_decision_digest",
        "projection_decision_digest",
        "routing_posture_digest",
        "execution_provenance_chain_digest",
        "failure_digest",
        "basis_observation_fixture_digest",
        "projection_consumption_fixture_digest",
    ];
    let mut outputs = vec![
        WorthQueryIntentAdmissionCertificationOutputSpec::new(
            "intent_family_digest",
            intent_family_digest(family_inventory),
        ),
        WorthQueryIntentAdmissionCertificationOutputSpec::new(
            "intent_family_inventory_digest",
            family_inventory_digest(family_inventory),
        ),
        WorthQueryIntentAdmissionCertificationOutputSpec::new(
            "covered_entrypoint_inventory_digest",
            coverage_inventory_digest(coverage_inventory),
        ),
        WorthQueryIntentAdmissionCertificationOutputSpec::new(
            "execution_seam_inventory_digest",
            execution_seam_inventory_digest(coverage_inventory),
        ),
        WorthQueryIntentAdmissionCertificationOutputSpec::new(
            "intent_support_matrix_digest",
            support_matrix_digest(support_matrix),
        ),
        WorthQueryIntentAdmissionCertificationOutputSpec::new(
            "intent_public_surface_digest",
            public_boundary_audit.public_surface_digest().to_string(),
        ),
        WorthQueryIntentAdmissionCertificationOutputSpec::new(
            "intent_target_dx_digest",
            public_boundary_audit.target_dx_digest().to_string(),
        ),
        WorthQueryIntentAdmissionCertificationOutputSpec::new(
            "decision_phase_progression_digest",
            proof_shape_audit
                .decision_phase_progression_digest()
                .to_string(),
        ),
        WorthQueryIntentAdmissionCertificationOutputSpec::new(
            "decision_proof_shape_digest",
            proof_shape_audit.decision_proof_shape_digest().to_string(),
        ),
        WorthQueryIntentAdmissionCertificationOutputSpec::new(
            "intent_topology_audit_digest",
            topology_audit.topology_digest().to_string(),
        ),
        WorthQueryIntentAdmissionCertificationOutputSpec::new(
            "representative_family_coverage_digest",
            representative_family_report
                .representative_family_coverage_digest()
                .to_string(),
        ),
    ];
    outputs.extend(
        representative_output_names
            .into_iter()
            .map(|name| {
                WorthQueryIntentAdmissionCertificationOutputSpec::new(
                    name,
                    representative_output_report
                        .digest_for(name)
                        .unwrap_or_else(|| panic!("missing representative digest {name}"))
                        .to_string(),
                )
            })
            .collect::<Vec<_>>(),
    );
    outputs.extend(vec![
        WorthQueryIntentAdmissionCertificationOutputSpec::new(
            "decision_oracle_digest",
            oracle_report.oracle_digest().to_string(),
        ),
        WorthQueryIntentAdmissionCertificationOutputSpec::new(
            "decision_support_traceability_digest",
            support_traceability_report
                .decision_support_traceability_digest()
                .to_string(),
        ),
        WorthQueryIntentAdmissionCertificationOutputSpec::new(
            "seeded_sequence_digest",
            seeded_report.seeded_sequence_digest().to_string(),
        ),
        WorthQueryIntentAdmissionCertificationOutputSpec::new(
            "seed_replay_digest",
            seeded_report.seed_replay_digest().to_string(),
        ),
        WorthQueryIntentAdmissionCertificationOutputSpec::new(
            "seed_generator_class_digest",
            seeded_report.seed_generator_class_digest().to_string(),
        ),
        WorthQueryIntentAdmissionCertificationOutputSpec::new(
            "legacy_delegation_parity_digest",
            legacy_parity_report
                .legacy_delegation_parity_digest()
                .to_string(),
        ),
        WorthQueryIntentAdmissionCertificationOutputSpec::new(
            "counter_snapshot",
            slope_report.counter_snapshot().digest().to_string(),
        ),
        WorthQueryIntentAdmissionCertificationOutputSpec::new(
            "intent_family_lookup_width",
            slope_report
                .counter_snapshot()
                .intent_family_lookup_width()
                .to_string(),
        ),
        WorthQueryIntentAdmissionCertificationOutputSpec::new(
            "covered_entrypoint_lookup_width",
            slope_report
                .counter_snapshot()
                .covered_entrypoint_lookup_width()
                .to_string(),
        ),
        WorthQueryIntentAdmissionCertificationOutputSpec::new(
            "decision_trace_width",
            slope_report
                .counter_snapshot()
                .decision_trace_width()
                .to_string(),
        ),
        WorthQueryIntentAdmissionCertificationOutputSpec::new(
            "execution_provenance_width",
            slope_report
                .counter_snapshot()
                .execution_provenance_width()
                .to_string(),
        ),
        WorthQueryIntentAdmissionCertificationOutputSpec::new(
            "admission_classification_slope_digest",
            slope_report
                .admission_classification_slope_digest()
                .to_string(),
        ),
        WorthQueryIntentAdmissionCertificationOutputSpec::new(
            "decision_trace_assembly_slope_digest",
            slope_report
                .decision_trace_assembly_slope_digest()
                .to_string(),
        ),
        WorthQueryIntentAdmissionCertificationOutputSpec::new(
            "decision_support_lookup_slope_digest",
            slope_report
                .decision_support_lookup_slope_digest()
                .to_string(),
        ),
        WorthQueryIntentAdmissionCertificationOutputSpec::new(
            "covered_entrypoint_inventory_slope_digest",
            slope_report
                .covered_entrypoint_inventory_slope_digest()
                .to_string(),
        ),
        WorthQueryIntentAdmissionCertificationOutputSpec::new(
            "execution_provenance_assembly_slope_digest",
            slope_report
                .execution_provenance_assembly_slope_digest()
                .to_string(),
        ),
        WorthQueryIntentAdmissionCertificationOutputSpec::new(
            "legacy_delegation_parity_slope_digest",
            slope_report
                .legacy_delegation_parity_slope_digest()
                .to_string(),
        ),
        WorthQueryIntentAdmissionCertificationOutputSpec::new(
            "decision_certification_coverage_slope_digest",
            slope_report
                .decision_certification_coverage_slope_digest()
                .to_string(),
        ),
    ]);
    outputs
}

pub(crate) fn certification_bundle_digest(
    outputs: &[WorthQueryIntentAdmissionCertificationOutputSpec],
) -> String {
    hash_parts(
        &outputs
            .iter()
            .map(|output| format!("{}:{}", output.name(), output.digest()))
            .collect::<Vec<_>>(),
    )
}

fn intent_family_digest(inventory: &WorthQueryIntentAdmissionFamilyInventory) -> String {
    hash_parts(
        &inventory
            .rows()
            .iter()
            .map(|row| row.family().as_str().to_string())
            .collect::<Vec<_>>(),
    )
}

fn family_inventory_digest(inventory: &WorthQueryIntentAdmissionFamilyInventory) -> String {
    hash_parts(
        &inventory
            .rows()
            .iter()
            .map(|row| {
                format!(
                    "{}:{}:{}:{}",
                    row.family().as_str(),
                    row.raw_authoring_constructor().label(),
                    row.common_path_front_door().label(),
                    row.advanced_path_front_door().label(),
                )
            })
            .collect::<Vec<_>>(),
    )
}

fn coverage_inventory_digest(inventory: &WorthQueryIntentAdmissionCoverageInventory) -> String {
    hash_parts(
        &inventory
            .rows()
            .iter()
            .map(|row| {
                format!(
                    "{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}",
                    row.family().as_str(),
                    row.entrypoint().as_str(),
                    row.execution_boundary().as_str(),
                    row.status().as_str(),
                    row.eligibility_authority().as_str(),
                    row.admitted_plan_kind().as_str(),
                    row.admitted_execution_handoff().label(),
                    row.advisory_decision_class().as_str(),
                    row.violation_decision_class().as_str(),
                    row.result_artifact().as_str(),
                    row.raw_authoring_constructor().label(),
                    row.common_path_front_door().label(),
                    row.advanced_path_front_door().label(),
                )
            })
            .collect::<Vec<_>>(),
    )
}

fn execution_seam_inventory_digest(
    inventory: &WorthQueryIntentAdmissionCoverageInventory,
) -> String {
    hash_parts(
        &inventory
            .rows()
            .iter()
            .map(|row| {
                format!(
                    "{}:{}",
                    row.entrypoint().as_str(),
                    row.execution_boundary().as_str(),
                )
            })
            .collect::<Vec<_>>(),
    )
}

fn support_matrix_digest(matrix: &WorthQueryIntentAdmissionSupportMatrix) -> String {
    hash_parts(
        &matrix
            .rows()
            .iter()
            .map(|row| {
                format!(
                    "{}:{}:{}:{}:{}",
                    row.family().as_str(),
                    row.entrypoint().as_str(),
                    row.posture().as_str(),
                    row.execution_boundary().as_str(),
                    row.detail().as_str(),
                )
            })
            .collect::<Vec<_>>(),
    )
}
