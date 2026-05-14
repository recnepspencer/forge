mod declaration;
mod eligibility;
mod facts;
mod support;

pub use declaration::{
    declare_projection_consumption, ProjectionConsumptionBindingContext,
    ProjectionConsumptionDeclaration, ProjectionConsumptionDeclarationError,
    ProjectionConsumptionSource, ProjectionSourceFamily,
};
pub use eligibility::{
    evaluate_projection_consumption_eligibility, AdmittedProjectionConsumption,
    DeferredProjectionConsumption, DeferredProjectionConsumptionReason,
    DeniedProjectionConsumption, ProjectionConsumptionDenialReason,
    ProjectionConsumptionEligibility, ProjectionConsumptionEligibilityCounters,
    ProjectionConsumptionEligibilityTrace, ProjectionConsumptionWarningKind,
    ProjectionConsumptionWarnings, SourceMismatchedProjectionConsumption,
};
pub use facts::{ProjectMaterializedFacts, ProjectionFactKind, ProjectionFactRequest};
pub use support::{
    discover_projection_consumption_support, ProjectionConsumptionSupportPosture,
    ProjectionConsumptionSupportReport, ProjectionConsumptionSupportRow,
};

#[cfg(test)]
mod tests;
