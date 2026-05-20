use forge_query::facade::ForgeQueryWorkspace;

use crate::construction::certification::{
    prepare_primitive_construction_move_witness_resolution_report_with_catalog,
    prepare_primitive_construction_points_toward_witness_resolution_report_with_catalog,
    prepare_primitive_construction_reorient_witness_resolution_report_with_catalog,
    prepare_primitive_construction_rotate_witness_resolution_report_with_catalog,
    PrimitiveConstructionMotionWitnessResolutionFailureKind,
    PrimitiveConstructionMotionWitnessResolutionKind,
    PrimitiveConstructionMotionWitnessResolutionReport,
    PrimitiveConstructionMotionWitnessResolutionStatus,
    PrimitiveConstructionRequestedMotionWitness, PrimitiveConstructionResolvedMotionWitness,
};
use crate::construction::digest::digest_owned_parts;
use crate::construction::runtime_basis::{
    prepare_primitive_construction_branch_preview_runtime_report,
    PrimitiveConstructionBranchPreviewRuntimeReport, PrimitiveConstructionRuntimeBasisError,
};
use crate::construction::{PrimitiveConstructionFamily, PrimitiveConstructionIntent};
use crate::spatial_intent::{
    MoveSpatialIntent, PointsTowardSpatialIntent, PrimitiveConstructionSpatialIntentError,
    ReorientSpatialIntent, RotateSpatialIntent,
};
use worth_spatial::facade::{
    SpatialPlacementConstraintError, SpatialPlacementMotionError, SpatialWitnessCatalog,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveConstructionMotionRuntimeSurfaceStatus {
    Available,
    MotionRejected,
    PlacementLoweringBlocked(SpatialPlacementMotionError),
    ConstraintLoweringBlocked(SpatialPlacementConstraintError),
}

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveConstructionMotionBranchPreviewRuntimeReport {
    kind: PrimitiveConstructionMotionWitnessResolutionKind,
    family: PrimitiveConstructionFamily,
    anchor: worth_spatial::facade::SpatialAnchorRef,
    requested_witness: PrimitiveConstructionRequestedMotionWitness,
    motion_status: PrimitiveConstructionMotionWitnessResolutionStatus,
    resolved_witness: Option<PrimitiveConstructionResolvedMotionWitness>,
    resolution_class: Option<worth_spatial::facade::SpatialWitnessResolutionClass>,
    failure_kind: Option<PrimitiveConstructionMotionWitnessResolutionFailureKind>,
    runtime_surface_status: PrimitiveConstructionMotionRuntimeSurfaceStatus,
    runtime_report: Option<PrimitiveConstructionBranchPreviewRuntimeReport>,
    report_digest: String,
}

impl PrimitiveConstructionMotionBranchPreviewRuntimeReport {
    fn new(
        motion_report: PrimitiveConstructionMotionWitnessResolutionReport,
        runtime_surface_status: PrimitiveConstructionMotionRuntimeSurfaceStatus,
        runtime_report: Option<PrimitiveConstructionBranchPreviewRuntimeReport>,
    ) -> Self {
        let report_digest = digest_owned_parts(&[
            format!("{:?}", motion_report.kind()),
            motion_report.subject_family().as_str().to_string(),
            format!("{:?}", motion_report.anchor()),
            format!("{:?}", motion_report.requested_witness()),
            format!("{:?}", motion_report.status()),
            format!("{:?}", motion_report.resolved_witness()),
            format!("{:?}", motion_report.resolution_class()),
            format!("{:?}", motion_report.failure_kind()),
            format!("{runtime_surface_status:?}"),
            runtime_report
                .as_ref()
                .map(|report| report.report_digest().to_string())
                .unwrap_or_default(),
        ]);
        Self {
            kind: motion_report.kind(),
            family: motion_report.subject_family(),
            anchor: motion_report.anchor().clone(),
            requested_witness: motion_report.requested_witness().clone(),
            motion_status: motion_report.status(),
            resolved_witness: motion_report.resolved_witness(),
            resolution_class: motion_report.resolution_class(),
            failure_kind: motion_report.failure_kind(),
            runtime_surface_status,
            runtime_report,
            report_digest,
        }
    }

    pub fn kind(&self) -> PrimitiveConstructionMotionWitnessResolutionKind {
        self.kind
    }
    pub fn family(&self) -> PrimitiveConstructionFamily {
        self.family
    }
    pub fn anchor(&self) -> &worth_spatial::facade::SpatialAnchorRef {
        &self.anchor
    }
    pub fn requested_witness(&self) -> &PrimitiveConstructionRequestedMotionWitness {
        &self.requested_witness
    }
    pub fn motion_status(&self) -> PrimitiveConstructionMotionWitnessResolutionStatus {
        self.motion_status
    }
    pub fn resolved_witness(&self) -> Option<PrimitiveConstructionResolvedMotionWitness> {
        self.resolved_witness
    }
    pub fn resolution_class(&self) -> Option<worth_spatial::facade::SpatialWitnessResolutionClass> {
        self.resolution_class
    }
    pub fn failure_kind(&self) -> Option<PrimitiveConstructionMotionWitnessResolutionFailureKind> {
        self.failure_kind
    }
    pub fn runtime_surface_status(&self) -> PrimitiveConstructionMotionRuntimeSurfaceStatus {
        self.runtime_surface_status
    }
    pub fn runtime_report(&self) -> Option<&PrimitiveConstructionBranchPreviewRuntimeReport> {
        self.runtime_report.as_ref()
    }
    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

pub fn prepare_primitive_construction_move_branch_preview_runtime_report(
    workspace: &mut ForgeQueryWorkspace,
    intent: MoveSpatialIntent<PrimitiveConstructionIntent>,
) -> Result<
    PrimitiveConstructionMotionBranchPreviewRuntimeReport,
    PrimitiveConstructionRuntimeBasisError,
> {
    prepare_primitive_construction_move_branch_preview_runtime_report_with_catalog(
        workspace,
        intent,
        &worth_spatial::facade::EmptySpatialWitnessCatalog,
    )
}

pub fn prepare_primitive_construction_move_branch_preview_runtime_report_with_catalog(
    workspace: &mut ForgeQueryWorkspace,
    intent: MoveSpatialIntent<PrimitiveConstructionIntent>,
    catalog: &impl SpatialWitnessCatalog,
) -> Result<
    PrimitiveConstructionMotionBranchPreviewRuntimeReport,
    PrimitiveConstructionRuntimeBasisError,
> {
    let motion_report = prepare_primitive_construction_move_witness_resolution_report_with_catalog(
        intent.clone(),
        catalog,
    );
    prepare_branch_report(workspace, motion_report, || {
        intent.finish_with_catalog(catalog)
    })
}

pub fn prepare_primitive_construction_reorient_branch_preview_runtime_report(
    workspace: &mut ForgeQueryWorkspace,
    intent: ReorientSpatialIntent<PrimitiveConstructionIntent>,
) -> Result<
    PrimitiveConstructionMotionBranchPreviewRuntimeReport,
    PrimitiveConstructionRuntimeBasisError,
> {
    prepare_primitive_construction_reorient_branch_preview_runtime_report_with_catalog(
        workspace,
        intent,
        &worth_spatial::facade::EmptySpatialWitnessCatalog,
    )
}

pub fn prepare_primitive_construction_reorient_branch_preview_runtime_report_with_catalog(
    workspace: &mut ForgeQueryWorkspace,
    intent: ReorientSpatialIntent<PrimitiveConstructionIntent>,
    catalog: &impl SpatialWitnessCatalog,
) -> Result<
    PrimitiveConstructionMotionBranchPreviewRuntimeReport,
    PrimitiveConstructionRuntimeBasisError,
> {
    let motion_report =
        prepare_primitive_construction_reorient_witness_resolution_report_with_catalog(
            intent.clone(),
            catalog,
        );
    prepare_branch_report(workspace, motion_report, || {
        intent.finish_with_catalog(catalog)
    })
}

pub fn prepare_primitive_construction_rotate_branch_preview_runtime_report(
    workspace: &mut ForgeQueryWorkspace,
    intent: RotateSpatialIntent<PrimitiveConstructionIntent>,
) -> Result<
    PrimitiveConstructionMotionBranchPreviewRuntimeReport,
    PrimitiveConstructionRuntimeBasisError,
> {
    prepare_primitive_construction_rotate_branch_preview_runtime_report_with_catalog(
        workspace,
        intent,
        &worth_spatial::facade::EmptySpatialWitnessCatalog,
    )
}

pub fn prepare_primitive_construction_rotate_branch_preview_runtime_report_with_catalog(
    workspace: &mut ForgeQueryWorkspace,
    intent: RotateSpatialIntent<PrimitiveConstructionIntent>,
    catalog: &impl SpatialWitnessCatalog,
) -> Result<
    PrimitiveConstructionMotionBranchPreviewRuntimeReport,
    PrimitiveConstructionRuntimeBasisError,
> {
    let motion_report =
        prepare_primitive_construction_rotate_witness_resolution_report_with_catalog(
            intent.clone(),
            catalog,
        );
    prepare_branch_report(workspace, motion_report, || {
        intent.finish_with_catalog(catalog)
    })
}

pub fn prepare_primitive_construction_points_toward_branch_preview_runtime_report(
    workspace: &mut ForgeQueryWorkspace,
    intent: PointsTowardSpatialIntent<PrimitiveConstructionIntent>,
) -> Result<
    PrimitiveConstructionMotionBranchPreviewRuntimeReport,
    PrimitiveConstructionRuntimeBasisError,
> {
    prepare_primitive_construction_points_toward_branch_preview_runtime_report_with_catalog(
        workspace,
        intent,
        &worth_spatial::facade::EmptySpatialWitnessCatalog,
    )
}

pub fn prepare_primitive_construction_points_toward_branch_preview_runtime_report_with_catalog(
    workspace: &mut ForgeQueryWorkspace,
    intent: PointsTowardSpatialIntent<PrimitiveConstructionIntent>,
    catalog: &impl SpatialWitnessCatalog,
) -> Result<
    PrimitiveConstructionMotionBranchPreviewRuntimeReport,
    PrimitiveConstructionRuntimeBasisError,
> {
    let motion_report =
        prepare_primitive_construction_points_toward_witness_resolution_report_with_catalog(
            intent.clone(),
            catalog,
        );
    prepare_branch_report(workspace, motion_report, || {
        intent.finish_with_catalog(catalog)
    })
}

fn prepare_branch_report(
    workspace: &mut ForgeQueryWorkspace,
    motion_report: PrimitiveConstructionMotionWitnessResolutionReport,
    finish: impl FnOnce()
        -> Result<PrimitiveConstructionIntent, PrimitiveConstructionSpatialIntentError>,
) -> Result<
    PrimitiveConstructionMotionBranchPreviewRuntimeReport,
    PrimitiveConstructionRuntimeBasisError,
> {
    let runtime_surface_status = match motion_report.status() {
        PrimitiveConstructionMotionWitnessResolutionStatus::Rejected => {
            PrimitiveConstructionMotionRuntimeSurfaceStatus::MotionRejected
        }
        PrimitiveConstructionMotionWitnessResolutionStatus::Admitted => match finish() {
            Ok(intent) => {
                let runtime_report = prepare_primitive_construction_branch_preview_runtime_report(
                    workspace, intent,
                )?;
                return Ok(PrimitiveConstructionMotionBranchPreviewRuntimeReport::new(
                    motion_report,
                    PrimitiveConstructionMotionRuntimeSurfaceStatus::Available,
                    Some(runtime_report),
                ));
            }
            Err(PrimitiveConstructionSpatialIntentError::PlacementLowering(error)) => {
                PrimitiveConstructionMotionRuntimeSurfaceStatus::PlacementLoweringBlocked(error)
            }
            Err(PrimitiveConstructionSpatialIntentError::ConstraintLowering(error)) => {
                PrimitiveConstructionMotionRuntimeSurfaceStatus::ConstraintLoweringBlocked(error)
            }
            Err(PrimitiveConstructionSpatialIntentError::MotionAdmission(error)) => {
                unreachable!("unexpected motion admission failure after admitted report: {error}")
            }
            Err(PrimitiveConstructionSpatialIntentError::ConstraintAdmission(error)) => {
                unreachable!(
                    "unexpected constraint admission failure after admitted report: {error}"
                )
            }
        },
    };
    Ok(PrimitiveConstructionMotionBranchPreviewRuntimeReport::new(
        motion_report,
        runtime_surface_status,
        None,
    ))
}

#[cfg(test)]
#[path = "branch_runtime_tests.rs"]
mod tests;
