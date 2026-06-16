use crate::projection_consumption::identity::compose_certification_bundle_digest;

use super::audits::{
    projection_consumption_family_inventory, projection_consumption_forbidden_fallback_audit,
    projection_consumption_proof_shape_audit, projection_consumption_public_boundary_audit,
    projection_consumption_support_matrix, ProjectionConsumptionFamilyInventory,
    ProjectionConsumptionForbiddenFallbackAudit, ProjectionConsumptionProofShapeAudit,
    ProjectionConsumptionPublicBoundaryAudit, ProjectionConsumptionSupportMatrix,
};
use super::bundle_outputs::assemble_closeout_bundle_outputs;
use super::fixtures::control_row_set_lifecycle;
use super::oracle::{projection_consumption_oracle_report, ProjectionConsumptionOracleReport};
use super::proof_artifacts::{
    compile_fail_boundary_bundle_digest, golden_transcript_bundle_digest,
};
use super::seeded::{
    projection_consumption_seeded_certification_report,
    ProjectionConsumptionSeededCertificationReport,
};
use super::slopes::{
    projection_consumption_slope_report, ProjectionConsumptionCertificationCounterSnapshot,
    ProjectionConsumptionSlopeReport,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProjectionConsumptionCertificationLane {
    SupportMatrixSurface,
    PublicBoundarySurface,
    ProofShapeSurface,
    ForbiddenFallbackSurface,
    DxTranscriptSurface,
    CompileFailBoundary,
    OracleSurface,
    SeededReplaySurface,
}

impl ProjectionConsumptionCertificationLane {
    pub(super) fn as_str(&self) -> &'static str {
        match self {
            Self::SupportMatrixSurface => "support_matrix_surface",
            Self::PublicBoundarySurface => "public_boundary_surface",
            Self::ProofShapeSurface => "proof_shape_surface",
            Self::ForbiddenFallbackSurface => "forbidden_fallback_surface",
            Self::DxTranscriptSurface => "dx_transcript_surface",
            Self::CompileFailBoundary => "compile_fail_boundary",
            Self::OracleSurface => "oracle_surface",
            Self::SeededReplaySurface => "seeded_replay_surface",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectionConsumptionCertificationRow {
    pub(super) lane: ProjectionConsumptionCertificationLane,
    pub(super) evidence_detail: String,
    pub(super) row_digest: String,
}

impl ProjectionConsumptionCertificationRow {
    pub fn lane(&self) -> ProjectionConsumptionCertificationLane {
        self.lane
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectionConsumptionCertificationBundle {
    family_inventory: ProjectionConsumptionFamilyInventory,
    support_matrix: ProjectionConsumptionSupportMatrix,
    public_boundary_audit: ProjectionConsumptionPublicBoundaryAudit,
    proof_shape_audit: ProjectionConsumptionProofShapeAudit,
    forbidden_fallback_audit: ProjectionConsumptionForbiddenFallbackAudit,
    oracle_report: ProjectionConsumptionOracleReport,
    seeded_report: ProjectionConsumptionSeededCertificationReport,
    slope_report: ProjectionConsumptionSlopeReport,
    rows: Vec<ProjectionConsumptionCertificationRow>,
    outputs: Vec<(&'static str, String)>,
    certification_bundle_digest: String,
}

impl ProjectionConsumptionCertificationBundle {
    pub fn family_inventory(&self) -> &ProjectionConsumptionFamilyInventory {
        &self.family_inventory
    }

    pub fn support_matrix(&self) -> &ProjectionConsumptionSupportMatrix {
        &self.support_matrix
    }

    pub fn public_boundary_audit(&self) -> &ProjectionConsumptionPublicBoundaryAudit {
        &self.public_boundary_audit
    }

    pub fn proof_shape_audit(&self) -> &ProjectionConsumptionProofShapeAudit {
        &self.proof_shape_audit
    }

    pub fn forbidden_fallback_audit(&self) -> &ProjectionConsumptionForbiddenFallbackAudit {
        &self.forbidden_fallback_audit
    }

    pub fn oracle_report(&self) -> &ProjectionConsumptionOracleReport {
        &self.oracle_report
    }

    pub fn seeded_report(&self) -> &ProjectionConsumptionSeededCertificationReport {
        &self.seeded_report
    }

    pub fn counter_snapshot(&self) -> &ProjectionConsumptionCertificationCounterSnapshot {
        self.slope_report.counter_snapshot()
    }

    pub fn rows(&self) -> &[ProjectionConsumptionCertificationRow] {
        &self.rows
    }

    pub fn output_digest(&self, key: &str) -> Option<&str> {
        self.outputs
            .iter()
            .find_map(|(name, value)| (*name == key).then_some(value.as_str()))
    }

    pub fn authority_reopen_count(&self) -> usize {
        self.counter_snapshot().authority_reopen_count()
    }

    pub fn fact_extraction_width(&self) -> usize {
        self.counter_snapshot().source_row_width_consumed()
    }

    pub fn certification_bundle_digest(&self) -> &str {
        &self.certification_bundle_digest
    }
}

pub fn certify_projection_consumption_closeout_core() -> ProjectionConsumptionCertificationBundle {
    let lifecycle = control_row_set_lifecycle(2);
    let family_inventory = projection_consumption_family_inventory();
    let support_matrix = projection_consumption_support_matrix();
    let public_boundary_audit = projection_consumption_public_boundary_audit();
    let proof_shape_audit = projection_consumption_proof_shape_audit();
    let forbidden_fallback_audit = projection_consumption_forbidden_fallback_audit();
    let oracle_report = projection_consumption_oracle_report();
    let seeded_report = projection_consumption_seeded_certification_report();
    let slope_report = projection_consumption_slope_report();
    let assembled = assemble_closeout_bundle_outputs(
        &lifecycle,
        &family_inventory,
        &support_matrix,
        &public_boundary_audit,
        &proof_shape_audit,
        &forbidden_fallback_audit,
        &oracle_report,
        &seeded_report,
        &slope_report,
        compile_fail_boundary_bundle_digest(),
        golden_transcript_bundle_digest(),
    );
    let certification_bundle_digest = compose_certification_bundle_digest(
        assembled.rows.iter().map(|row| row.row_digest()),
        assembled
            .outputs
            .iter()
            .map(|(name, value)| (*name, value.as_str())),
    );
    ProjectionConsumptionCertificationBundle {
        family_inventory,
        support_matrix,
        public_boundary_audit,
        proof_shape_audit,
        forbidden_fallback_audit,
        oracle_report,
        seeded_report,
        slope_report,
        rows: assembled.rows,
        outputs: assembled.outputs,
        certification_bundle_digest,
    }
}
