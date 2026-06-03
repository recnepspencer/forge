use crate::construction::certification::motion::dx_surface_report::{
    build_motion_dx_surface_row_from_policy_row, PrimitiveConstructionMotionDxSurfaceRow,
};
use crate::construction::certification::motion::policy_report::{
    build_motion_resolution_policy_row_from_inputs,
    PrimitiveConstructionMotionResolutionPolicyCase,
    PrimitiveConstructionMotionResolutionPolicyRow,
};
use crate::construction::certification::motion::representative_inputs::prepare_motion_representative_inputs;
use crate::construction::certification::motion::witness_report::PrimitiveConstructionMotionWitnessResolutionReport;
use crate::construction::digest::{digest_owned_parts_with_scope, ConstructionDigestScope};
use crate::construction::query::{
    PrimitiveConstructionQueryMotionWitnessParityError,
    PrimitiveConstructionQueryMotionWitnessParityReport,
};
use crate::construction::{
    PrimitiveConstructionMotionBranchPreviewRuntimeReport,
    PrimitiveConstructionMotionReplayParityReport, PrimitiveConstructionRuntimeBasisError,
};
use forge_query::facade::ForgeQueryWorkspace;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct PrimitiveConstructionMotionRepresentativeInputs {
    pub(crate) witness_report: PrimitiveConstructionMotionWitnessResolutionReport,
    pub(crate) replay_report: PrimitiveConstructionMotionReplayParityReport,
    pub(crate) inspection_report: PrimitiveConstructionQueryMotionWitnessParityReport,
    pub(crate) projection_report: PrimitiveConstructionQueryMotionWitnessParityReport,
    pub(crate) branch_runtime_report: PrimitiveConstructionMotionBranchPreviewRuntimeReport,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveConstructionMotionRepresentativeEvidence {
    case: PrimitiveConstructionMotionResolutionPolicyCase,
    policy_row: PrimitiveConstructionMotionResolutionPolicyRow,
    dx_row: PrimitiveConstructionMotionDxSurfaceRow,
    witness_report: PrimitiveConstructionMotionWitnessResolutionReport,
    replay_report: PrimitiveConstructionMotionReplayParityReport,
    inspection_report: PrimitiveConstructionQueryMotionWitnessParityReport,
    projection_report: PrimitiveConstructionQueryMotionWitnessParityReport,
    branch_runtime_report: PrimitiveConstructionMotionBranchPreviewRuntimeReport,
    parity_verified: bool,
    report_digest: String,
}

impl PrimitiveConstructionMotionRepresentativeEvidence {
    fn new(
        case: PrimitiveConstructionMotionResolutionPolicyCase,
        inputs: PrimitiveConstructionMotionRepresentativeInputs,
    ) -> Self {
        let policy_row = build_motion_resolution_policy_row_from_inputs(case, &inputs);
        let dx_row = build_motion_dx_surface_row_from_policy_row(&policy_row);
        let witness = &inputs.witness_report;
        let inspection = &inputs.inspection_report;
        let projection = &inputs.projection_report;
        let branch = &inputs.branch_runtime_report;
        let parity_verified = inputs.replay_report.parity_verified()
            && inspection.parity_verified()
            && projection.parity_verified()
            && witness.kind() == policy_row.kind()
            && witness.subject_family() == policy_row.subject_family()
            && witness.anchor() == policy_row.anchor()
            && witness.requested_witness() == policy_row.requested_witness()
            && witness.status() == policy_row.status()
            && witness.resolution_class() == policy_row.resolution_class()
            && witness.failure_kind() == policy_row.failure_kind()
            && witness.kind() == inspection.kind()
            && witness.subject_family() == inspection.subject_family()
            && witness.anchor() == inspection.anchor()
            && witness.requested_witness() == inspection.requested_witness()
            && witness.status() == inspection.status()
            && witness.resolved_witness() == inspection.resolved_witness()
            && witness.resolution_class() == inspection.resolution_class()
            && witness.failure_kind() == inspection.failure_kind()
            && inspection.kind() == projection.kind()
            && inspection.subject_family() == projection.subject_family()
            && inspection.anchor() == projection.anchor()
            && inspection.requested_witness() == projection.requested_witness()
            && inspection.status() == projection.status()
            && inspection.resolved_witness() == projection.resolved_witness()
            && inspection.resolution_class() == projection.resolution_class()
            && inspection.failure_kind() == projection.failure_kind()
            && witness.kind() == branch.kind()
            && witness.subject_family() == branch.family()
            && witness.anchor() == branch.anchor()
            && witness.requested_witness() == branch.requested_witness()
            && witness.status() == branch.motion_status()
            && witness.resolved_witness() == branch.resolved_witness()
            && witness.resolution_class() == branch.resolution_class()
            && witness.failure_kind() == branch.failure_kind()
            && policy_row.runtime_surface_status() == branch.runtime_surface_status()
            && dx_row.status() == policy_row.status()
            && dx_row.resolution_class() == policy_row.resolution_class()
            && dx_row.runtime_surface_status() == policy_row.runtime_surface_status();
        let report_digest = digest_owned_parts_with_scope(
            ConstructionDigestScope::ArtifactIdentity,
            &[
                format!("{case:?}"),
                policy_row.row_digest().to_string(),
                dx_row.row_digest().to_string(),
                inputs.witness_report.report_digest().to_string(),
                inputs.replay_report.report_digest().to_string(),
                inputs.inspection_report.report_digest().to_string(),
                inputs.projection_report.report_digest().to_string(),
                inputs.branch_runtime_report.report_digest().to_string(),
                parity_verified.to_string(),
            ],
        );
        Self {
            case,
            policy_row,
            dx_row,
            witness_report: inputs.witness_report,
            replay_report: inputs.replay_report,
            inspection_report: inputs.inspection_report,
            projection_report: inputs.projection_report,
            branch_runtime_report: inputs.branch_runtime_report,
            parity_verified,
            report_digest,
        }
    }

    #[cfg(test)]
    pub(crate) fn policy_row(&self) -> &PrimitiveConstructionMotionResolutionPolicyRow {
        &self.policy_row
    }

    pub fn dx_row(&self) -> &PrimitiveConstructionMotionDxSurfaceRow {
        &self.dx_row
    }

    #[cfg(test)]
    pub(crate) fn parity_verified(&self) -> bool {
        self.parity_verified
    }

    #[cfg(test)]
    pub(crate) fn report_digest(&self) -> &str {
        &self.report_digest
    }

    #[cfg(test)]
    pub(crate) fn replay_report(&self) -> &PrimitiveConstructionMotionReplayParityReport {
        &self.replay_report
    }

    #[cfg(test)]
    pub(crate) fn branch_runtime_report(
        &self,
    ) -> &PrimitiveConstructionMotionBranchPreviewRuntimeReport {
        &self.branch_runtime_report
    }
}

pub fn prepare_primitive_construction_motion_representative_evidence(
    workspace: &mut ForgeQueryWorkspace,
    case: PrimitiveConstructionMotionResolutionPolicyCase,
) -> Result<
    PrimitiveConstructionMotionRepresentativeEvidence,
    PrimitiveConstructionMotionRepresentativeEvidenceError,
> {
    prepare_motion_representative_inputs(workspace, case)
        .map(|inputs| PrimitiveConstructionMotionRepresentativeEvidence::new(case, inputs))
}

#[derive(Debug)]
pub enum PrimitiveConstructionMotionRepresentativeEvidenceError {
    Inspection(PrimitiveConstructionQueryMotionWitnessParityError),
    Projection(PrimitiveConstructionQueryMotionWitnessParityError),
    Runtime(PrimitiveConstructionRuntimeBasisError),
}

impl std::fmt::Display for PrimitiveConstructionMotionRepresentativeEvidenceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Inspection(error) => write!(f, "{error}"),
            Self::Projection(error) => write!(f, "{error}"),
            Self::Runtime(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for PrimitiveConstructionMotionRepresentativeEvidenceError {}
