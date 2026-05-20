use forge_query::facade::ForgeQueryWorkspace;

use crate::construction::motion_branch_runtime::{
    prepare_primitive_construction_move_branch_preview_runtime_report_with_catalog,
    prepare_primitive_construction_points_toward_branch_preview_runtime_report_with_catalog,
    prepare_primitive_construction_reorient_branch_preview_runtime_report_with_catalog,
    prepare_primitive_construction_rotate_branch_preview_runtime_report_with_catalog,
    PrimitiveConstructionMotionBranchPreviewRuntimeReport,
};
use crate::construction::motion_replay::{
    prepare_primitive_construction_move_replay_parity_report_with_catalog,
    prepare_primitive_construction_points_toward_replay_parity_report_with_catalog,
    prepare_primitive_construction_reorient_replay_parity_report_with_catalog,
    prepare_primitive_construction_rotate_replay_parity_report_with_catalog,
    PrimitiveConstructionMotionReplayParityReport,
};
use crate::construction::query::{
    prepare_primitive_construction_query_motion_inspection_parity_report,
    prepare_primitive_construction_query_motion_projection_consumption_receipt_report,
    PrimitiveConstructionQueryMotionWitnessParityError,
    PrimitiveConstructionQueryMotionWitnessParityReport,
};
use crate::construction::runtime_basis::PrimitiveConstructionRuntimeBasisError;
use crate::construction::PrimitiveConstructionIntent;
use crate::spatial_intent::{
    MoveSpatialIntent, PointsTowardSpatialIntent, ReorientSpatialIntent, RotateSpatialIntent,
};
use worth_spatial::facade::SpatialWitnessCatalog;

use super::{
    prepare_primitive_construction_move_witness_resolution_report_with_catalog,
    prepare_primitive_construction_points_toward_witness_resolution_report_with_catalog,
    prepare_primitive_construction_reorient_witness_resolution_report_with_catalog,
    prepare_primitive_construction_rotate_witness_resolution_report_with_catalog,
    PrimitiveConstructionMotionWitnessResolutionReport,
};

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveConstructionMotionReportBundle {
    witness_report: PrimitiveConstructionMotionWitnessResolutionReport,
    replay_parity_report: PrimitiveConstructionMotionReplayParityReport,
    query_inspection_parity_report: PrimitiveConstructionQueryMotionWitnessParityReport,
    query_projection_receipt_report: PrimitiveConstructionQueryMotionWitnessParityReport,
    branch_preview_runtime_report: PrimitiveConstructionMotionBranchPreviewRuntimeReport,
    bundle_verified: bool,
}

impl PrimitiveConstructionMotionReportBundle {
    fn new(
        witness_report: PrimitiveConstructionMotionWitnessResolutionReport,
        replay_parity_report: PrimitiveConstructionMotionReplayParityReport,
        query_inspection_parity_report: PrimitiveConstructionQueryMotionWitnessParityReport,
        query_projection_receipt_report: PrimitiveConstructionQueryMotionWitnessParityReport,
        branch_preview_runtime_report: PrimitiveConstructionMotionBranchPreviewRuntimeReport,
    ) -> Self {
        let branch_matches = witness_report.kind() == branch_preview_runtime_report.kind()
            && witness_report.subject_family() == branch_preview_runtime_report.family()
            && witness_report.anchor() == branch_preview_runtime_report.anchor()
            && witness_report.requested_witness()
                == branch_preview_runtime_report.requested_witness()
            && witness_report.status() == branch_preview_runtime_report.motion_status()
            && witness_report.resolved_witness()
                == branch_preview_runtime_report.resolved_witness()
            && witness_report.resolution_class()
                == branch_preview_runtime_report.resolution_class()
            && witness_report.failure_kind() == branch_preview_runtime_report.failure_kind();
        let bundle_verified = replay_parity_report.parity_verified()
            && query_inspection_parity_report.parity_verified()
            && query_projection_receipt_report.parity_verified()
            && branch_matches;
        Self {
            witness_report,
            replay_parity_report,
            query_inspection_parity_report,
            query_projection_receipt_report,
            branch_preview_runtime_report,
            bundle_verified,
        }
    }

    pub fn witness_report(&self) -> &PrimitiveConstructionMotionWitnessResolutionReport {
        &self.witness_report
    }

    pub fn replay_parity_report(&self) -> &PrimitiveConstructionMotionReplayParityReport {
        &self.replay_parity_report
    }

    pub fn query_inspection_parity_report(
        &self,
    ) -> &PrimitiveConstructionQueryMotionWitnessParityReport {
        &self.query_inspection_parity_report
    }

    pub fn query_projection_receipt_report(
        &self,
    ) -> &PrimitiveConstructionQueryMotionWitnessParityReport {
        &self.query_projection_receipt_report
    }

    pub fn branch_preview_runtime_report(
        &self,
    ) -> &PrimitiveConstructionMotionBranchPreviewRuntimeReport {
        &self.branch_preview_runtime_report
    }

    pub fn bundle_verified(&self) -> bool {
        self.bundle_verified
    }
}

#[derive(Debug)]
pub enum PrimitiveConstructionMotionReportBundleError {
    Query(PrimitiveConstructionQueryMotionWitnessParityError),
    Runtime(PrimitiveConstructionRuntimeBasisError),
}

impl std::fmt::Display for PrimitiveConstructionMotionReportBundleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Query(error) => write!(f, "{error}"),
            Self::Runtime(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for PrimitiveConstructionMotionReportBundleError {}

pub fn prepare_primitive_construction_move_motion_report_bundle(
    workspace: &mut ForgeQueryWorkspace,
    intent: MoveSpatialIntent<PrimitiveConstructionIntent>,
) -> Result<PrimitiveConstructionMotionReportBundle, PrimitiveConstructionMotionReportBundleError> {
    prepare_primitive_construction_move_motion_report_bundle_with_catalog(
        workspace,
        intent,
        &worth_spatial::facade::EmptySpatialWitnessCatalog,
    )
}

pub fn prepare_primitive_construction_move_motion_report_bundle_with_catalog<
    C: SpatialWitnessCatalog,
>(
    workspace: &mut ForgeQueryWorkspace,
    intent: MoveSpatialIntent<PrimitiveConstructionIntent>,
    catalog: &C,
) -> Result<PrimitiveConstructionMotionReportBundle, PrimitiveConstructionMotionReportBundleError> {
    prepare_bundle(
        workspace,
        intent,
        catalog,
        prepare_primitive_construction_move_witness_resolution_report_with_catalog,
        prepare_primitive_construction_move_replay_parity_report_with_catalog,
        prepare_primitive_construction_move_branch_preview_runtime_report_with_catalog,
    )
}

pub fn prepare_primitive_construction_rotate_motion_report_bundle(
    workspace: &mut ForgeQueryWorkspace,
    intent: RotateSpatialIntent<PrimitiveConstructionIntent>,
) -> Result<PrimitiveConstructionMotionReportBundle, PrimitiveConstructionMotionReportBundleError> {
    prepare_primitive_construction_rotate_motion_report_bundle_with_catalog(
        workspace,
        intent,
        &worth_spatial::facade::EmptySpatialWitnessCatalog,
    )
}

pub fn prepare_primitive_construction_rotate_motion_report_bundle_with_catalog<
    C: SpatialWitnessCatalog,
>(
    workspace: &mut ForgeQueryWorkspace,
    intent: RotateSpatialIntent<PrimitiveConstructionIntent>,
    catalog: &C,
) -> Result<PrimitiveConstructionMotionReportBundle, PrimitiveConstructionMotionReportBundleError> {
    prepare_bundle(
        workspace,
        intent,
        catalog,
        prepare_primitive_construction_rotate_witness_resolution_report_with_catalog,
        prepare_primitive_construction_rotate_replay_parity_report_with_catalog,
        prepare_primitive_construction_rotate_branch_preview_runtime_report_with_catalog,
    )
}

pub fn prepare_primitive_construction_reorient_motion_report_bundle(
    workspace: &mut ForgeQueryWorkspace,
    intent: ReorientSpatialIntent<PrimitiveConstructionIntent>,
) -> Result<PrimitiveConstructionMotionReportBundle, PrimitiveConstructionMotionReportBundleError> {
    prepare_primitive_construction_reorient_motion_report_bundle_with_catalog(
        workspace,
        intent,
        &worth_spatial::facade::EmptySpatialWitnessCatalog,
    )
}

pub fn prepare_primitive_construction_reorient_motion_report_bundle_with_catalog<
    C: SpatialWitnessCatalog,
>(
    workspace: &mut ForgeQueryWorkspace,
    intent: ReorientSpatialIntent<PrimitiveConstructionIntent>,
    catalog: &C,
) -> Result<PrimitiveConstructionMotionReportBundle, PrimitiveConstructionMotionReportBundleError> {
    prepare_bundle(
        workspace,
        intent,
        catalog,
        prepare_primitive_construction_reorient_witness_resolution_report_with_catalog,
        prepare_primitive_construction_reorient_replay_parity_report_with_catalog,
        prepare_primitive_construction_reorient_branch_preview_runtime_report_with_catalog,
    )
}

pub fn prepare_primitive_construction_points_toward_motion_report_bundle(
    workspace: &mut ForgeQueryWorkspace,
    intent: PointsTowardSpatialIntent<PrimitiveConstructionIntent>,
) -> Result<PrimitiveConstructionMotionReportBundle, PrimitiveConstructionMotionReportBundleError> {
    prepare_primitive_construction_points_toward_motion_report_bundle_with_catalog(
        workspace,
        intent,
        &worth_spatial::facade::EmptySpatialWitnessCatalog,
    )
}

pub fn prepare_primitive_construction_points_toward_motion_report_bundle_with_catalog<
    C: SpatialWitnessCatalog,
>(
    workspace: &mut ForgeQueryWorkspace,
    intent: PointsTowardSpatialIntent<PrimitiveConstructionIntent>,
    catalog: &C,
) -> Result<PrimitiveConstructionMotionReportBundle, PrimitiveConstructionMotionReportBundleError> {
    prepare_bundle(
        workspace,
        intent,
        catalog,
        prepare_primitive_construction_points_toward_witness_resolution_report_with_catalog,
        prepare_primitive_construction_points_toward_replay_parity_report_with_catalog,
        prepare_primitive_construction_points_toward_branch_preview_runtime_report_with_catalog,
    )
}

fn prepare_bundle<I: Clone, C: SpatialWitnessCatalog>(
    workspace: &mut ForgeQueryWorkspace,
    intent: I,
    catalog: &C,
    prepare_witness: impl Fn(I, &C) -> PrimitiveConstructionMotionWitnessResolutionReport,
    prepare_replay: impl Fn(I, &C) -> PrimitiveConstructionMotionReplayParityReport,
    prepare_branch: impl Fn(
        &mut ForgeQueryWorkspace,
        I,
        &C,
    ) -> Result<
        PrimitiveConstructionMotionBranchPreviewRuntimeReport,
        PrimitiveConstructionRuntimeBasisError,
    >,
) -> Result<PrimitiveConstructionMotionReportBundle, PrimitiveConstructionMotionReportBundleError> {
    let witness_report = prepare_witness(intent.clone(), catalog);
    let replay_parity_report = prepare_replay(intent.clone(), catalog);
    let query_inspection_parity_report =
        prepare_primitive_construction_query_motion_inspection_parity_report(
            workspace,
            witness_report.clone(),
        )
        .map_err(PrimitiveConstructionMotionReportBundleError::Query)?;
    let query_projection_receipt_report =
        prepare_primitive_construction_query_motion_projection_consumption_receipt_report(
            workspace,
            witness_report.clone(),
        )
        .map_err(PrimitiveConstructionMotionReportBundleError::Query)?;
    let branch_preview_runtime_report = prepare_branch(workspace, intent, catalog)
        .map_err(PrimitiveConstructionMotionReportBundleError::Runtime)?;
    Ok(PrimitiveConstructionMotionReportBundle::new(
        witness_report,
        replay_parity_report,
        query_inspection_parity_report,
        query_projection_receipt_report,
        branch_preview_runtime_report,
    ))
}
