mod bundle;
mod report;
mod suite;

pub use bundle::{
    prepare_primitive_construction_continuity_bundle_from_hostility_suite,
    prepare_primitive_construction_continuity_report_bundle,
    PrimitiveConstructionContinuityReportBundle, PrimitiveConstructionContinuityReportBundleError,
};
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
