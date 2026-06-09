mod catalog;
mod denials;
mod graph_authoring;
mod graph_legality;
mod graph_obligations;
mod operations;
mod requests;
mod runtime_projection;
mod runtime_registration;
mod runtime_vocabulary;
mod violations;

pub use catalog::{
    HadwigerResearchInvariantCatalog, ResearchGraphInvariantCompatibilitySurface,
    ResearchGraphInvariantCompatibilitySurfaces, ResearchGraphInvariantCounters,
    ResearchGraphInvariantFamily, ResearchGraphInvariantRegistrationPlan,
    ResearchGraphInvariantRegistrationPosture, ResearchGraphInvariantRule,
    ResearchGraphInvariantScope,
};
pub use denials::ResearchGraphInvariantDenial;
pub use graph_legality::{
    ResearchGraphLegalityPosture, ResearchGraphLegalityReport, ResearchGraphLegalityViolation,
};
pub use graph_obligations::{
    ResearchGraphInvariantObligation, ResearchGraphInvariantObligationSet,
};
pub use operations::{
    certify_research_graph_invariant_violation, draft_research_graph_invariant_catalog,
    materialize_research_graph_invariant_denial, plan_research_graph_invariant_registration,
    project_research_graph_for_invariant_registration_checked,
    register_research_graph_invariants_checked, ResearchGraphInvariantError,
};
pub use requests::{ResearchGraphInvariantCheckRequest, ResearchGraphInvariantDenialRequest};
pub use runtime_projection::{
    ResearchGraphInvariantRuntimeProjection, ResearchGraphRuntimeEntityProjection,
    ResearchGraphRuntimeRelationProjection,
};
pub use runtime_registration::HadwigerResearchInvariantRegistrationChecked;
pub use violations::{ResearchGraphInvariantViolation, ResearchGraphInvariantViolationKind};

pub(crate) use graph_authoring::legality_for_experiment_batch;
