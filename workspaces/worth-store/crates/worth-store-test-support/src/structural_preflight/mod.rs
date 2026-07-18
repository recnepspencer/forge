mod evidence;
mod integrity;
mod plan;
mod predicate;

pub use evidence::{
    PreflightEvidenceFreshness, PreflightEvidenceIdentity, StructuralPredicateEvidence,
    StructuralPredicateFailure, StructuralPredicateVerdict, StructuralPreflightEvidence,
    StructuralToolExecutionEvidence,
};
pub use integrity::StructuralPreflightIntegrityDenial;
pub use plan::{
    PreflightInputScope, StructuralPredicatePlan, StructuralPreflightEvaluatorIdentity,
    StructuralPreflightPlan, StructuralPreflightProfile, StructuralPreflightRequest,
    StructuralSupportingToolIdentity, StructuralToolDeclaration, StructuralToolEnvironmentBinding,
};
pub use predicate::{DependencyBoundaryPredicate, StructuralPredicate};

pub const STRUCTURAL_PREFLIGHT_BUNDLE_ENV: &str = "WORTH_STORE_STRUCTURAL_PREFLIGHT_BUNDLE";
