use forge_query::facade::ForgeQueryWorkspace;

use crate::construction::certification::motion::bundle_verified::{
    verify_bundle, PrimitiveConstructionMotionBundleVerificationFailure,
    PrimitiveConstructionMotionReportBundle, PrimitiveConstructionVerifiedMotionReportBundle,
};
use crate::construction::certification::motion::witness_report::{
    prepare_primitive_construction_move_witness_resolution_report_with_catalog,
    prepare_primitive_construction_points_toward_witness_resolution_report_with_catalog,
    prepare_primitive_construction_reorient_witness_resolution_report_with_catalog,
    prepare_primitive_construction_rotate_witness_resolution_report_with_catalog,
    PrimitiveConstructionMotionWitnessResolutionReport,
};
use crate::construction::runtime_basis::PrimitiveConstructionRuntimeBasisError;
use crate::construction::{
    prepare_primitive_construction_move_branch_preview_runtime_report_with_catalog,
    prepare_primitive_construction_move_replay_parity_report_with_catalog,
    prepare_primitive_construction_points_toward_branch_preview_runtime_report_with_catalog,
    prepare_primitive_construction_points_toward_replay_parity_report_with_catalog,
    prepare_primitive_construction_query_motion_inspection_parity_report,
    prepare_primitive_construction_query_motion_projection_consumption_receipt_report,
    prepare_primitive_construction_reorient_branch_preview_runtime_report_with_catalog,
    prepare_primitive_construction_reorient_replay_parity_report_with_catalog,
    prepare_primitive_construction_rotate_branch_preview_runtime_report_with_catalog,
    prepare_primitive_construction_rotate_replay_parity_report_with_catalog,
    PrimitiveConstructionIntent, PrimitiveConstructionMotionBranchPreviewRuntimeReport,
    PrimitiveConstructionMotionReplayParityReport,
    PrimitiveConstructionQueryMotionWitnessParityError,
};
use crate::spatial_intent::{
    MoveSpatialIntent, PointsTowardSpatialIntent, ReorientSpatialIntent, RotateSpatialIntent,
};
use worth_spatial::facade::witness_catalog::SpatialWitnessCatalog;

#[derive(Debug)]
pub enum PrimitiveConstructionMotionReportBundleError {
    Query(PrimitiveConstructionQueryMotionWitnessParityError),
    Runtime(PrimitiveConstructionRuntimeBasisError),
    Verification(PrimitiveConstructionMotionBundleVerificationFailure),
}

impl std::fmt::Display for PrimitiveConstructionMotionReportBundleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Query(error) => write!(f, "{error}"),
            Self::Runtime(error) => write!(f, "{error}"),
            Self::Verification(failure) => write!(
                f,
                "motion report bundle failed coherence verification: {:?}",
                failure.mismatches()
            ),
        }
    }
}

impl std::error::Error for PrimitiveConstructionMotionReportBundleError {}

pub fn prepare_primitive_construction_move_motion_report_bundle(
    workspace: &mut ForgeQueryWorkspace,
    intent: MoveSpatialIntent<PrimitiveConstructionIntent>,
) -> Result<
    PrimitiveConstructionVerifiedMotionReportBundle,
    PrimitiveConstructionMotionReportBundleError,
> {
    prepare_primitive_construction_move_motion_report_bundle_with_catalog(
        workspace,
        intent,
        &worth_spatial::facade::witness_catalog::EmptySpatialWitnessCatalog,
    )
}

pub fn prepare_primitive_construction_move_motion_report_bundle_with_catalog(
    workspace: &mut ForgeQueryWorkspace,
    intent: MoveSpatialIntent<PrimitiveConstructionIntent>,
    catalog: &impl SpatialWitnessCatalog,
) -> Result<
    PrimitiveConstructionVerifiedMotionReportBundle,
    PrimitiveConstructionMotionReportBundleError,
> {
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
) -> Result<
    PrimitiveConstructionVerifiedMotionReportBundle,
    PrimitiveConstructionMotionReportBundleError,
> {
    prepare_primitive_construction_rotate_motion_report_bundle_with_catalog(
        workspace,
        intent,
        &worth_spatial::facade::witness_catalog::EmptySpatialWitnessCatalog,
    )
}

pub fn prepare_primitive_construction_rotate_motion_report_bundle_with_catalog(
    workspace: &mut ForgeQueryWorkspace,
    intent: RotateSpatialIntent<PrimitiveConstructionIntent>,
    catalog: &impl SpatialWitnessCatalog,
) -> Result<
    PrimitiveConstructionVerifiedMotionReportBundle,
    PrimitiveConstructionMotionReportBundleError,
> {
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
) -> Result<
    PrimitiveConstructionVerifiedMotionReportBundle,
    PrimitiveConstructionMotionReportBundleError,
> {
    prepare_primitive_construction_reorient_motion_report_bundle_with_catalog(
        workspace,
        intent,
        &worth_spatial::facade::witness_catalog::EmptySpatialWitnessCatalog,
    )
}

pub fn prepare_primitive_construction_reorient_motion_report_bundle_with_catalog(
    workspace: &mut ForgeQueryWorkspace,
    intent: ReorientSpatialIntent<PrimitiveConstructionIntent>,
    catalog: &impl SpatialWitnessCatalog,
) -> Result<
    PrimitiveConstructionVerifiedMotionReportBundle,
    PrimitiveConstructionMotionReportBundleError,
> {
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
) -> Result<
    PrimitiveConstructionVerifiedMotionReportBundle,
    PrimitiveConstructionMotionReportBundleError,
> {
    prepare_primitive_construction_points_toward_motion_report_bundle_with_catalog(
        workspace,
        intent,
        &worth_spatial::facade::witness_catalog::EmptySpatialWitnessCatalog,
    )
}

pub fn prepare_primitive_construction_points_toward_motion_report_bundle_with_catalog(
    workspace: &mut ForgeQueryWorkspace,
    intent: PointsTowardSpatialIntent<PrimitiveConstructionIntent>,
    catalog: &impl SpatialWitnessCatalog,
) -> Result<
    PrimitiveConstructionVerifiedMotionReportBundle,
    PrimitiveConstructionMotionReportBundleError,
> {
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
) -> Result<
    PrimitiveConstructionVerifiedMotionReportBundle,
    PrimitiveConstructionMotionReportBundleError,
> {
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
    verify_bundle(PrimitiveConstructionMotionReportBundle::new(
        witness_report,
        replay_parity_report,
        query_inspection_parity_report,
        query_projection_receipt_report,
        branch_preview_runtime_report,
    ))
    .map_err(PrimitiveConstructionMotionReportBundleError::Verification)
}
