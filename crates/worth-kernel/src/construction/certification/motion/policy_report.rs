use forge_query::facade::ForgeQueryWorkspace;

use crate::construction::certification::motion::representative_evidence::PrimitiveConstructionMotionRepresentativeEvidenceError;
use crate::construction::certification::motion::representative_inputs::{
    prepare_motion_representative_inputs, required_motion_representative_cases,
};
use crate::construction::certification::motion::{
    PrimitiveConstructionMotionWitnessResolutionFailureKind,
    PrimitiveConstructionMotionWitnessResolutionKind,
    PrimitiveConstructionMotionWitnessResolutionReport,
    PrimitiveConstructionMotionWitnessResolutionStatus,
    PrimitiveConstructionRequestedMotionWitness,
};
use crate::construction::digest::{digest_owned_parts_with_scope, ConstructionDigestScope};
use crate::construction::motion_branch_runtime::{
    PrimitiveConstructionMotionBranchPreviewRuntimeReport,
    PrimitiveConstructionMotionRuntimeSurfaceStatus,
};
use crate::construction::motion_replay::PrimitiveConstructionMotionReplayParityReport;
use crate::construction::query::motion_parity::PrimitiveConstructionQueryMotionWitnessParityReport;
use crate::construction::request::PrimitiveConstructionFamily;
use worth_spatial::facade::refs::SpatialAnchorRef;
use worth_spatial::facade::witness_resolution::SpatialWitnessResolutionClass;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveConstructionMotionResolutionPolicyCase {
    DirectMove,
    FrameReorient,
    CarrierReorient,
    FallbackPointsToward,
    AmbiguousMove,
    UndefinedReorient,
    UnsupportedReorient,
    ExhaustedRotate,
    CoincidentPointsToward,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveConstructionMotionResolutionPolicyRow {
    case: PrimitiveConstructionMotionResolutionPolicyCase,
    subject_family: PrimitiveConstructionFamily,
    anchor: SpatialAnchorRef,
    kind: PrimitiveConstructionMotionWitnessResolutionKind,
    requested_witness: PrimitiveConstructionRequestedMotionWitness,
    status: PrimitiveConstructionMotionWitnessResolutionStatus,
    resolution_class: Option<SpatialWitnessResolutionClass>,
    failure_kind: Option<PrimitiveConstructionMotionWitnessResolutionFailureKind>,
    runtime_surface_status: PrimitiveConstructionMotionRuntimeSurfaceStatus,
    row_digest: String,
}

impl PrimitiveConstructionMotionResolutionPolicyRow {
    pub(crate) fn new(
        case: PrimitiveConstructionMotionResolutionPolicyCase,
        witness_report: &PrimitiveConstructionMotionWitnessResolutionReport,
        replay_report: &PrimitiveConstructionMotionReplayParityReport,
        inspection_report: &PrimitiveConstructionQueryMotionWitnessParityReport,
        projection_report: &PrimitiveConstructionQueryMotionWitnessParityReport,
        branch_report: &PrimitiveConstructionMotionBranchPreviewRuntimeReport,
    ) -> Self {
        let runtime_surface_status = branch_report.runtime_surface_status();
        let row_digest = digest_owned_parts_with_scope(
            ConstructionDigestScope::ParityIdentity,
            &[
                format!("{case:?}"),
                witness_report.subject_family().as_str().to_string(),
                format!("{:?}", witness_report.anchor()),
                format!("{:?}", witness_report.kind()),
                format!("{:?}", witness_report.requested_witness()),
                format!("{:?}", witness_report.status()),
                format!("{:?}", witness_report.resolution_class()),
                format!("{:?}", witness_report.failure_kind()),
                format!("{runtime_surface_status:?}"),
                witness_report.report_digest().to_string(),
                replay_report.report_digest().to_string(),
                inspection_report.report_digest().to_string(),
                projection_report.report_digest().to_string(),
                branch_report.report_digest().to_string(),
            ],
        );
        Self {
            case,
            subject_family: witness_report.subject_family(),
            anchor: witness_report.anchor().clone(),
            kind: witness_report.kind(),
            requested_witness: witness_report.requested_witness().clone(),
            status: witness_report.status(),
            resolution_class: witness_report.resolution_class(),
            failure_kind: witness_report.failure_kind(),
            runtime_surface_status,
            row_digest,
        }
    }

    pub fn case(&self) -> PrimitiveConstructionMotionResolutionPolicyCase {
        self.case
    }

    pub fn subject_family(&self) -> PrimitiveConstructionFamily {
        self.subject_family
    }

    pub fn anchor(&self) -> &SpatialAnchorRef {
        &self.anchor
    }

    pub fn kind(&self) -> PrimitiveConstructionMotionWitnessResolutionKind {
        self.kind
    }

    pub fn requested_witness(&self) -> &PrimitiveConstructionRequestedMotionWitness {
        &self.requested_witness
    }

    pub fn status(&self) -> PrimitiveConstructionMotionWitnessResolutionStatus {
        self.status
    }

    pub fn resolution_class(&self) -> Option<SpatialWitnessResolutionClass> {
        self.resolution_class
    }

    pub fn failure_kind(&self) -> Option<PrimitiveConstructionMotionWitnessResolutionFailureKind> {
        self.failure_kind
    }

    pub fn runtime_surface_status(&self) -> PrimitiveConstructionMotionRuntimeSurfaceStatus {
        self.runtime_surface_status
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

pub(crate) fn build_motion_resolution_policy_row_from_inputs(
    case: PrimitiveConstructionMotionResolutionPolicyCase,
    inputs: &crate::construction::certification::motion::representative_evidence::PrimitiveConstructionMotionRepresentativeInputs,
) -> PrimitiveConstructionMotionResolutionPolicyRow {
    PrimitiveConstructionMotionResolutionPolicyRow::new(
        case,
        &inputs.witness_report,
        &inputs.replay_report,
        &inputs.inspection_report,
        &inputs.projection_report,
        &inputs.branch_runtime_report,
    )
}

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveConstructionMotionResolutionPolicyReport {
    rows: Vec<PrimitiveConstructionMotionResolutionPolicyRow>,
    report_digest: String,
}

impl PrimitiveConstructionMotionResolutionPolicyReport {
    fn new(rows: Vec<PrimitiveConstructionMotionResolutionPolicyRow>) -> Self {
        let report_digest = digest_owned_parts_with_scope(
            ConstructionDigestScope::ParityIdentity,
            &rows
                .iter()
                .map(|row| row.row_digest().to_string())
                .collect::<Vec<_>>(),
        );
        Self {
            rows,
            report_digest,
        }
    }

    pub fn rows(&self) -> &[PrimitiveConstructionMotionResolutionPolicyRow] {
        &self.rows
    }

    pub fn row(
        &self,
        case: PrimitiveConstructionMotionResolutionPolicyCase,
    ) -> Option<&PrimitiveConstructionMotionResolutionPolicyRow> {
        self.rows.iter().find(|row| row.case() == case)
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

#[derive(Debug)]
pub enum PrimitiveConstructionMotionResolutionPolicyReportError {
    Representative(PrimitiveConstructionMotionRepresentativeEvidenceError),
}

impl std::fmt::Display for PrimitiveConstructionMotionResolutionPolicyReportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Representative(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for PrimitiveConstructionMotionResolutionPolicyReportError {}

pub fn prepare_primitive_construction_motion_resolution_policy_report(
    workspace: &mut ForgeQueryWorkspace,
) -> Result<
    PrimitiveConstructionMotionResolutionPolicyReport,
    PrimitiveConstructionMotionResolutionPolicyReportError,
> {
    let rows = required_motion_representative_cases()
        .iter()
        .copied()
        .map(|case| {
            prepare_motion_representative_inputs(workspace, case)
                .map(|inputs| build_motion_resolution_policy_row_from_inputs(case, &inputs))
                .map_err(PrimitiveConstructionMotionResolutionPolicyReportError::Representative)
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(PrimitiveConstructionMotionResolutionPolicyReport::new(rows))
}
