mod bundle;
mod chosen_report;
mod dx_surface_report;
mod policy_report;
mod preserved_report;
mod suite;

pub use bundle::{
    prepare_primitive_construction_intent_arbitration_report_bundle,
    PrimitiveConstructionIntentArbitrationBundleCase,
    PrimitiveConstructionIntentArbitrationReportBundle,
    PrimitiveConstructionIntentArbitrationReportBundleError,
};
pub use chosen_report::{
    prepare_primitive_chosen_intent_resolution_report,
    PrimitiveConstructionChosenIntentResolutionAuthority,
    PrimitiveConstructionChosenIntentResolutionCase,
    PrimitiveConstructionChosenIntentResolutionReport,
    PrimitiveConstructionChosenIntentResolutionReportError,
    PrimitiveConstructionChosenIntentResolutionRow,
};
pub use dx_surface_report::{
    prepare_primitive_intent_conflict_dx_surface_report,
    PrimitiveConstructionIntentArbitrationDxSurface,
    PrimitiveConstructionIntentArbitrationDxSurfaceReport,
    PrimitiveConstructionIntentArbitrationDxSurfaceRow,
};
pub use policy_report::{
    prepare_primitive_intent_arbitration_policy_report,
    PrimitiveConstructionIntentArbitrationConflictClass,
    PrimitiveConstructionIntentArbitrationPolicyCase,
    PrimitiveConstructionIntentArbitrationPolicyReport,
    PrimitiveConstructionIntentArbitrationPolicyReportError,
    PrimitiveConstructionIntentArbitrationPolicyRow, PrimitiveConstructionObservedIntentRelation,
};
pub use preserved_report::{
    prepare_primitive_construction_preserved_intent_resolution_report,
    PrimitiveConstructionPreservedIntentResolutionCase,
    PrimitiveConstructionPreservedIntentResolutionReport,
    PrimitiveConstructionPreservedIntentResolutionReportError,
    PrimitiveConstructionPreservedIntentResolutionRow, PrimitiveConstructionPreservedIntentTruth,
};
pub use suite::{
    prepare_primitive_construction_intent_arbitration_hostility_suite_report,
    PrimitiveConstructionIntentArbitrationHostilitySuiteReport,
};

#[cfg(test)]
mod bundle_tests;
#[cfg(test)]
mod preserved_report_tests;
#[cfg(test)]
mod tests;
