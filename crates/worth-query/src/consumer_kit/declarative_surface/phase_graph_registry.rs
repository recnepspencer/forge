use super::model::{
    WorthQueryDeclarativeCapabilityFamily as Family,
    WorthQueryDeclarativePhaseResponsibility as Phase, WorthQueryDeclarativeSurfaceClass as Class,
    WorthQueryDeclarativeSurfaceRow as Row,
};

const HISTORICAL_PLANNER: &str = "src/historical/planner.rs";
const CORRESPONDENCE_RESOLUTION: &str = "src/correspondence/resolution.rs";
const PREVIEW_SCOPED: &str = "src/preview/scoped.rs";
const WORKFLOW_FOUNDATION: &str = "src/workflow/foundation.rs";
const WORKFLOW_MERGE: &str = "src/workflow/lowering/merge.rs";
const WORKFLOW_MUTATION: &str = "src/workflow/lowering/mutation.rs";
const WORKFLOW_WRITEBACK: &str = "src/workflow/lowering/writeback.rs";
const WORKFLOW_INSPECTION: &str = "src/workflow/inspection/operations.rs";
const DECLARATION_CONTEXT_BINDING: &str =
    "src/application/domain_handle/admitted_handle/declaration_entry/binding/context.rs";
const DECLARATION_TARGET_BINDING: &str =
    "src/application/domain_handle/admitted_handle/declaration_entry/binding/target.rs";
const DECLARATION_PROGRESSION: &str =
    "src/application/domain_handle/admitted_handle/declaration_entry/progression.rs";
const DECLARATION_SEAM: &str =
    "src/application/domain_handle/admitted_handle/declaration_entry/seam.rs";

pub(super) fn phase_graph_surface_rows() -> &'static [Row] {
    PHASE_GRAPH_ROWS
}

#[rustfmt::skip]
const PHASE_GRAPH_ROWS: &[Row] = &[
    mechanism(HISTORICAL_PLANNER, "admit_historical_evaluation_path", Family::Historical, Phase::Admit, "ordinary historical declaration"),
    mechanism(HISTORICAL_PLANNER, "resolve_historical_materialization_path", Family::Historical, Phase::Plan, "ordinary historical declaration"),
    mechanism(HISTORICAL_PLANNER, "materialization_metadata_from_resolved", Family::Historical, Phase::AssembleOutcome, "historical outcome evidence"),
    mechanism(CORRESPONDENCE_RESOLUTION, "resolve_correspondence_evidence", Family::Comparison, Phase::Execute, "ordinary correspondence declaration"),
    mechanism(PREVIEW_SCOPED, "admit_scoped_preview_session_plan_binding", Family::Preview, Phase::Bind, "ordinary preview declaration"),
    mechanism(PREVIEW_SCOPED, "admit_scoped_preview_session_plan_binding_from_preview_binding", Family::Preview, Phase::Bind, "ordinary preview declaration"),
    mechanism(PREVIEW_SCOPED, "admit_scoped_preview_live_session_plan", Family::Preview, Phase::Admit, "ordinary preview declaration"),
    mechanism(PREVIEW_SCOPED, "execute_scoped_preview_live_session_plan", Family::Preview, Phase::Execute, "ordinary preview declaration"),
    mechanism(PREVIEW_SCOPED, "scoped_observation_basis_for_preview_binding", Family::Preview, Phase::Bind, "ordinary preview declaration"),
    workflow(WORKFLOW_FOUNDATION, "bind_workflow_context", Phase::Bind),
    workflow(WORKFLOW_FOUNDATION, "admit_query_workflow_declaration", Phase::Admit),
    workflow(WORKFLOW_MERGE, "lower_merge_workflow_declaration", Phase::Lower),
    workflow(WORKFLOW_MUTATION, "lower_mutation_intent_declaration", Phase::Lower),
    workflow(WORKFLOW_WRITEBACK, "lower_query_writeback_declaration", Phase::Lower),
    workflow(WORKFLOW_INSPECTION, "inspect_merge_conflicts", Phase::Inspect),
    workflow(WORKFLOW_INSPECTION, "inspect_post_merge_outcome", Phase::Inspect),
    declaration_binding(DECLARATION_CONTEXT_BINDING, "bind_declaration_from_context"),
    declaration_binding(DECLARATION_CONTEXT_BINDING, "bind_declaration_from_context_outcome"),
    declaration_binding(DECLARATION_CONTEXT_BINDING, "bind_declaration_from_context_checked"),
    declaration_binding(DECLARATION_CONTEXT_BINDING, "bind_declaration_from_context_proof"),
    declaration_binding(DECLARATION_CONTEXT_BINDING, "bind_route_request_from_context"),
    declaration_binding(DECLARATION_CONTEXT_BINDING, "bind_route_request_from_context_outcome"),
    declaration_binding(DECLARATION_CONTEXT_BINDING, "bind_route_request_from_context_checked"),
    declaration_binding(DECLARATION_CONTEXT_BINDING, "bind_route_request_from_context_proof"),
    declaration_binding(DECLARATION_CONTEXT_BINDING, "bind_receipt_request_from_context"),
    declaration_binding(DECLARATION_CONTEXT_BINDING, "bind_receipt_request_from_context_outcome"),
    declaration_binding(DECLARATION_CONTEXT_BINDING, "bind_receipt_request_from_context_checked"),
    declaration_binding(DECLARATION_CONTEXT_BINDING, "bind_receipt_request_from_context_proof"),
    declaration_binding(DECLARATION_CONTEXT_BINDING, "bind_envelope_request_from_context"),
    declaration_binding(DECLARATION_CONTEXT_BINDING, "bind_envelope_request_from_context_outcome"),
    declaration_binding(DECLARATION_CONTEXT_BINDING, "bind_envelope_request_from_context_checked"),
    declaration_binding(DECLARATION_CONTEXT_BINDING, "bind_envelope_request_from_context_proof"),
    declaration_binding(DECLARATION_CONTEXT_BINDING, "bind_continuation_request_from_context"),
    declaration_binding(DECLARATION_CONTEXT_BINDING, "bind_continuation_request_from_context_outcome"),
    declaration_binding(DECLARATION_CONTEXT_BINDING, "bind_continuation_request_from_context_checked"),
    declaration_binding(DECLARATION_CONTEXT_BINDING, "bind_continuation_request_from_context_proof"),
    declaration_binding(DECLARATION_TARGET_BINDING, "bind_route_from_target"),
    declaration_binding(DECLARATION_TARGET_BINDING, "bind_route_from_target_outcome"),
    declaration_binding(DECLARATION_TARGET_BINDING, "bind_route_from_target_checked"),
    declaration_binding(DECLARATION_TARGET_BINDING, "bind_route_from_target_proof"),
    declaration_binding(DECLARATION_TARGET_BINDING, "bind_receipt_from_target"),
    declaration_binding(DECLARATION_TARGET_BINDING, "bind_receipt_from_target_outcome"),
    declaration_binding(DECLARATION_TARGET_BINDING, "bind_receipt_from_target_checked"),
    declaration_binding(DECLARATION_TARGET_BINDING, "bind_receipt_from_target_proof"),
    declaration_binding(DECLARATION_TARGET_BINDING, "bind_envelope_from_target"),
    declaration_binding(DECLARATION_TARGET_BINDING, "bind_envelope_from_target_outcome"),
    declaration_binding(DECLARATION_TARGET_BINDING, "bind_envelope_from_target_checked"),
    declaration_binding(DECLARATION_TARGET_BINDING, "bind_envelope_from_target_proof"),
    declaration_binding(DECLARATION_TARGET_BINDING, "bind_continuation_from_target"),
    declaration_binding(DECLARATION_TARGET_BINDING, "bind_continuation_from_target_outcome"),
    declaration_binding(DECLARATION_TARGET_BINDING, "bind_continuation_from_target_checked"),
    declaration_binding(DECLARATION_TARGET_BINDING, "bind_continuation_from_target_proof"),
    mechanism(DECLARATION_PROGRESSION, "declare_review_and_progress", Family::DomainExtension, Phase::Declare, "one Query-owned domain declaration capability"),
    mechanism(DECLARATION_SEAM, "inspect_declaration_entry", Family::DomainExtension, Phase::Inspect, "declaration outcome inspection"),
];

const fn workflow(source: &'static str, function: &'static str, phase: Phase) -> Row {
    mechanism(
        source,
        function,
        Family::Workflow,
        phase,
        "ordinary workflow declaration",
    )
}

const fn declaration_binding(source: &'static str, function: &'static str) -> Row {
    mechanism(
        source,
        function,
        Family::DomainExtension,
        Phase::Bind,
        "one Query-owned domain declaration capability",
    )
}

const fn mechanism(
    source: &'static str,
    function: &'static str,
    family: Family,
    phase: Phase,
    replacement: &'static str,
) -> Row {
    Row::new(
        source,
        function,
        family,
        phase,
        Class::Compatibility,
        Class::InternalMechanism,
        "advanced integration or Query implementation",
        replacement,
    )
}
