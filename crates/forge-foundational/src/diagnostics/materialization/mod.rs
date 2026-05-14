mod planning;
mod surfaces;
mod vocabulary;

pub use planning::{
    plan_diagnostic_explanation_bundle, plan_diagnostic_support_report,
    FoundationalDiagnosticExplanationInput, FoundationalDiagnosticMaterializationPlan,
    FoundationalDiagnosticSupportInput,
};
pub use surfaces::{
    materialize_diagnostic_explanation_bundle, materialize_diagnostic_support_report,
    FoundationalDiagnosticExplanationBundle, FoundationalDiagnosticSupportReport,
};
pub use vocabulary::{
    FoundationalDiagnosticAssemblyDebt, FoundationalDiagnosticAssemblyDebtClass,
    FoundationalDiagnosticCounterSnapshot, FoundationalDiagnosticGapClass,
    FoundationalDiagnosticGapClosurePosture, FoundationalDiagnosticGapTarget,
    FoundationalDiagnosticMaterializationDenial, FoundationalDiagnosticNamedGap,
    FoundationalDiagnosticPartiality, FoundationalDiagnosticSupportClaimStrength,
    FoundationalDiagnosticSurfaceAvailability,
};
