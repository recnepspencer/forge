mod aspect_kinds;
mod aspect_records;
mod binding_stops;
mod closure_reports;
mod dependency_edges;
mod dependency_graph;
mod promotion_rules;
mod query_aspect_mapping;
mod recompute_policies;

pub use aspect_kinds::{
    HadwigerAspectAuthorityError, HadwigerAspectKind, HadwigerAspectPosture, HadwigerAspectScope,
};
pub use aspect_records::{
    AdvisoryAspectRecord, ColorabilityAspectRecord, GraphShapeAspectRecord, HadwigerAspectRecord,
    UnitDistanceAspectRecord,
};
pub use binding_stops::AspectClosureStop;
pub use closure_reports::{
    HadwigerConservativeInvalidationPosture, HadwigerDependencyClosureBlocker,
    HadwigerDependencyClosureReport,
};
pub use dependency_edges::{
    HadwigerAspectDependencyEdge, HadwigerAspectDependencyRole, HadwigerAspectInvalidationScope,
};
pub use dependency_graph::{AspectDependencyGraph, AspectDependencyGraphBuilder};
pub use promotion_rules::HadwigerPromotionRuleDescriptor;
pub use query_aspect_mapping::{
    query_aspect_contract_for_hadwiger_kind, query_aspect_coverage_for_hadwiger_posture,
    query_aspect_publication_for_hadwiger_kind,
};
pub use recompute_policies::HadwigerRecomputePolicy;
