use forge_query::facade::ForgeQueryWorkspace;

use crate::construction::certification::motion::policy_report::PrimitiveConstructionMotionResolutionPolicyRow;
use crate::construction::certification::motion::representative_evidence::prepare_primitive_construction_motion_representative_evidence;
use crate::construction::certification::motion::representative_inputs::required_motion_representative_cases;
use crate::construction::digest::digest_owned_parts;

use super::PrimitiveConstructionMotionResolutionPolicyCase;
use crate::construction::{
    PrimitiveConstructionMotionRuntimeSurfaceStatus,
    PrimitiveConstructionMotionWitnessResolutionStatus,
};
use worth_spatial::facade::witness_resolution::SpatialWitnessResolutionClass;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveConstructionMotionDxSurface {
    CommonPath,
    AdvancedPath,
    UnsafeOrDegradedPath,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveConstructionMotionDxSurfaceRow {
    case: PrimitiveConstructionMotionResolutionPolicyCase,
    dx_surface: PrimitiveConstructionMotionDxSurface,
    status: PrimitiveConstructionMotionWitnessResolutionStatus,
    resolution_class: Option<SpatialWitnessResolutionClass>,
    runtime_surface_status: PrimitiveConstructionMotionRuntimeSurfaceStatus,
    row_digest: String,
}

impl PrimitiveConstructionMotionDxSurfaceRow {
    pub fn case(&self) -> PrimitiveConstructionMotionResolutionPolicyCase {
        self.case
    }

    pub fn dx_surface(&self) -> PrimitiveConstructionMotionDxSurface {
        self.dx_surface
    }

    pub fn status(&self) -> PrimitiveConstructionMotionWitnessResolutionStatus {
        self.status
    }

    pub fn resolution_class(&self) -> Option<SpatialWitnessResolutionClass> {
        self.resolution_class
    }

    pub fn runtime_surface_status(&self) -> PrimitiveConstructionMotionRuntimeSurfaceStatus {
        self.runtime_surface_status
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

pub(crate) fn build_motion_dx_surface_row_from_policy_row(
    row: &PrimitiveConstructionMotionResolutionPolicyRow,
) -> PrimitiveConstructionMotionDxSurfaceRow {
    let dx_surface = match (row.status(), row.resolution_class()) {
        (
            PrimitiveConstructionMotionWitnessResolutionStatus::Admitted,
            Some(SpatialWitnessResolutionClass::DirectWorld),
        ) => PrimitiveConstructionMotionDxSurface::CommonPath,
        (
            PrimitiveConstructionMotionWitnessResolutionStatus::Admitted,
            Some(
                SpatialWitnessResolutionClass::FrameDerived
                | SpatialWitnessResolutionClass::CarrierDerived,
            ),
        ) => PrimitiveConstructionMotionDxSurface::AdvancedPath,
        _ => PrimitiveConstructionMotionDxSurface::UnsafeOrDegradedPath,
    };
    let row_digest = digest_owned_parts(&[
        format!("{:?}", row.case()),
        format!("{dx_surface:?}"),
        format!("{:?}", row.status()),
        format!("{:?}", row.resolution_class()),
        format!("{:?}", row.runtime_surface_status()),
    ]);
    PrimitiveConstructionMotionDxSurfaceRow {
        case: row.case(),
        dx_surface,
        status: row.status(),
        resolution_class: row.resolution_class(),
        runtime_surface_status: row.runtime_surface_status(),
        row_digest,
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveConstructionMotionDxSurfaceReport {
    rows: Vec<PrimitiveConstructionMotionDxSurfaceRow>,
    report_digest: String,
}

impl PrimitiveConstructionMotionDxSurfaceReport {
    pub fn rows(&self) -> &[PrimitiveConstructionMotionDxSurfaceRow] {
        &self.rows
    }

    pub fn row(
        &self,
        case: PrimitiveConstructionMotionResolutionPolicyCase,
    ) -> Option<&PrimitiveConstructionMotionDxSurfaceRow> {
        self.rows.iter().find(|row| row.case() == case)
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

pub type PrimitiveConstructionMotionDxSurfaceReportError =
    crate::construction::certification::motion::representative_evidence::PrimitiveConstructionMotionRepresentativeEvidenceError;

pub fn prepare_primitive_construction_motion_dx_surface_report(
    workspace: &mut ForgeQueryWorkspace,
) -> Result<
    PrimitiveConstructionMotionDxSurfaceReport,
    PrimitiveConstructionMotionDxSurfaceReportError,
> {
    let rows = required_motion_representative_cases()
        .iter()
        .copied()
        .map(|case| {
            prepare_primitive_construction_motion_representative_evidence(workspace, case)
                .map(|evidence| evidence.dx_row().clone())
        })
        .collect::<Result<Vec<_>, _>>()?;
    let report_digest = digest_owned_parts(
        &rows
            .iter()
            .map(|row| row.row_digest.clone())
            .collect::<Vec<_>>(),
    );
    Ok(PrimitiveConstructionMotionDxSurfaceReport {
        rows,
        report_digest,
    })
}
