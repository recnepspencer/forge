mod chosen_report;
mod dx_surface_report;
mod policy_report;
mod preserved_report;
#[cfg(test)]
mod representative_evidence;
#[cfg(test)]
mod suite;

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
#[cfg(test)]
pub(crate) use representative_evidence::required_arbitration_representative_cases;
#[cfg(test)]
pub use representative_evidence::{
    prepare_primitive_construction_intent_arbitration_representative_evidence,
    PrimitiveConstructionIntentArbitrationBundleCase,
    PrimitiveConstructionIntentArbitrationRepresentativeEvidence,
    PrimitiveConstructionIntentArbitrationRepresentativeEvidenceError,
};
#[cfg(test)]
mod preserved_report_tests;
#[cfg(test)]
mod representative_evidence_tests;
#[cfg(test)]
mod tests;
