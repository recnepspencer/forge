mod report;
mod suite;

pub(crate) use report::prepare_primitive_construction_continuity_row;
pub use report::{
    prepare_primitive_construction_continuity_surface_report, PrimitiveConstructionContinuityCase,
    PrimitiveConstructionContinuityResolutionSource, PrimitiveConstructionContinuityRow,
    PrimitiveConstructionContinuitySurfaceReport,
    PrimitiveConstructionContinuitySurfaceReportError,
};
pub use suite::{
    prepare_primitive_construction_continuity_hostility_suite_report,
    PrimitiveConstructionContinuityHostilitySuiteReport,
};
