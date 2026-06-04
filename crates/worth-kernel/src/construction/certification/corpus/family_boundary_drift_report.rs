use forge_query::facade::ForgeQueryWorkspace;

use super::{
    prepare_primitive_construction_family_boundary_report,
    PrimitiveConstructionFamilyBoundaryReport, PrimitiveConstructionFamilyBoundaryReportError,
};

pub type PrimitiveConstructionFamilyBoundaryDriftReport = PrimitiveConstructionFamilyBoundaryReport;

pub fn prepare_primitive_construction_family_boundary_drift_report(
    workspace: &mut ForgeQueryWorkspace,
) -> Result<
    PrimitiveConstructionFamilyBoundaryDriftReport,
    PrimitiveConstructionFamilyBoundaryReportError,
> {
    prepare_primitive_construction_family_boundary_report(workspace)
}
