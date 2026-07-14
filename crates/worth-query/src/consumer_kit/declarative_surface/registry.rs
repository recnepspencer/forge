use super::core_phase_registry::core_phase_surface_rows;
use super::exposure_registry::public_phase_exposure_rows;
use super::model::{
    WorthQueryDeclarativeCapabilityFamily as Family,
    WorthQueryDeclarativePhaseResponsibility as Phase, WorthQueryDeclarativeSurfaceClass as Class,
    WorthQueryDeclarativeSurfaceRow as Row,
};
use super::phase_graph_registry::phase_graph_surface_rows;
use super::phase_seven_registry::phase_seven_surface_rows;
use super::policy_phase_registry::policy_phase_surface_rows;
use super::preview_phase_registry::preview_phase_surface_rows;

const ORDINARY_READ_DECLARATION: &str = "src/ordinary/read/declaration.rs";
const ORDINARY_READ_EXECUTION: &str = "src/ordinary/read/execution.rs";
const ORDINARY_READ_REQUEST: &str = "src/ordinary/read/request.rs";
const ORDINARY_READ_CONTEXT: &str = "src/ordinary/read/context/declaration.rs";
const ORDINARY_COUNT_EXECUTION: &str = "src/ordinary/count/execution.rs";
const ORDINARY_COUNT_DECLARATION: &str = "src/ordinary/count/declaration.rs";
const ORDINARY_COUNT_REQUEST: &str = "src/ordinary/count/request.rs";
const ORDINARY_LIVE_DECLARATION: &str = "src/ordinary/live/declaration.rs";
const ORDINARY_LIVE_DISPOSAL: &str = "src/ordinary/live/disposal.rs";
const ORDINARY_LIVE_EXECUTION: &str = "src/ordinary/live/execution.rs";
const ORDINARY_LIVE_REQUEST: &str = "src/ordinary/live/request.rs";
const ORDINARY_LIVE_CONTINUATION_OUTCOME: &str = "src/ordinary/live/continuation/outcome.rs";
const WORKSPACE_QUERIES: &str = "src/runtime/workspace_queries.rs";
const DECLARATION_ORCHESTRATION: &str =
    "src/application/domain_handle/admitted_handle/declaration_entry/orchestration.rs";
const DECLARATION_PRODUCTS: &str =
    "src/application/domain_handle/admitted_handle/declaration_entry/products.rs";

pub fn worth_query_declarative_surface_rows() -> &'static [Row] {
    static ROWS: std::sync::OnceLock<Vec<Row>> = std::sync::OnceLock::new();
    ROWS.get_or_init(|| {
        DECLARATIVE_SURFACE_ROWS
            .iter()
            .chain(core_phase_surface_rows())
            .chain(public_phase_exposure_rows())
            .chain(phase_graph_surface_rows())
            .chain(phase_seven_surface_rows())
            .chain(policy_phase_surface_rows())
            .chain(preview_phase_surface_rows())
            .copied()
            .collect()
    })
}

const DECLARATIVE_SURFACE_ROWS: &[Row] = &[
    Row::new(
        ORDINARY_READ_DECLARATION,
        "declare",
        Family::Read,
        Phase::Declare,
        Class::OrdinaryDeclaration,
        Class::OrdinaryDeclaration,
        "ordinary read consumer",
        "facade::read::declare",
    ),
    Row::new(
        ORDINARY_READ_REQUEST,
        "using",
        Family::Read,
        Phase::Refine,
        Class::OrdinaryDeclaration,
        Class::OrdinaryDeclaration,
        "ordinary read consumer",
        "WorthQueryReadDeclaration::using",
    ),
    Row::new(
        ORDINARY_READ_CONTEXT,
        "current",
        Family::Read,
        Phase::Refine,
        Class::OrdinaryDeclaration,
        Class::OrdinaryDeclaration,
        "ordinary read consumer",
        "facade::read::current",
    ),
    Row::new(
        ORDINARY_READ_CONTEXT,
        "under_policy_tenant",
        Family::Read,
        Phase::Refine,
        Class::OrdinaryDeclaration,
        Class::OrdinaryDeclaration,
        "ordinary read consumer",
        "WorthQueryCurrentReadContext::under_policy_tenant",
    ),
    Row::new(
        ORDINARY_READ_CONTEXT,
        "with_relationship_proofs",
        Family::Read,
        Phase::Refine,
        Class::OrdinaryDeclaration,
        Class::OrdinaryDeclaration,
        "ordinary read consumer",
        "WorthQueryCurrentPolicyTenantReadContext::with_relationship_proofs",
    ),
    Row::new(
        ORDINARY_READ_EXECUTION,
        "run",
        Family::Read,
        Phase::Execute,
        Class::OrdinaryDeclaration,
        Class::OrdinaryDeclaration,
        "ordinary read consumer",
        "WorthQueryReadRequest::run",
    ),
    Row::new(
        ORDINARY_COUNT_REQUEST,
        "using",
        Family::Aggregate,
        Phase::Refine,
        Class::OrdinaryDeclaration,
        Class::OrdinaryDeclaration,
        "ordinary aggregate consumer",
        "WorthQueryCountDeclaration::using",
    ),
    Row::new(
        ORDINARY_COUNT_DECLARATION,
        "declare_count",
        Family::Aggregate,
        Phase::Declare,
        Class::OrdinaryDeclaration,
        Class::OrdinaryDeclaration,
        "ordinary aggregate consumer",
        "facade::read::declare_count",
    ),
    Row::new(
        ORDINARY_COUNT_EXECUTION,
        "run",
        Family::Aggregate,
        Phase::Execute,
        Class::OrdinaryDeclaration,
        Class::OrdinaryDeclaration,
        "ordinary aggregate consumer",
        "WorthQueryCountRequest::run",
    ),
    Row::new(
        ORDINARY_LIVE_REQUEST,
        "using",
        Family::Live,
        Phase::Refine,
        Class::OrdinaryDeclaration,
        Class::OrdinaryDeclaration,
        "ordinary live consumer",
        "WorthQueryLiveDeclaration::using",
    ),
    Row::new(
        ORDINARY_LIVE_DECLARATION,
        "declare_live",
        Family::Live,
        Phase::Declare,
        Class::OrdinaryDeclaration,
        Class::OrdinaryDeclaration,
        "ordinary live consumer",
        "facade::live::declare",
    ),
    Row::new(
        ORDINARY_LIVE_EXECUTION,
        "open",
        Family::Live,
        Phase::Execute,
        Class::OrdinaryDeclaration,
        Class::OrdinaryDeclaration,
        "ordinary live consumer",
        "WorthQueryLiveRequest::open",
    ),
    Row::new(
        ORDINARY_LIVE_DISPOSAL,
        "close",
        Family::Live,
        Phase::Dispose,
        Class::OrdinaryDeclaration,
        Class::OrdinaryDeclaration,
        "ordinary live consumer",
        "WorthQueryManagedLiveHandle::close",
    ),
    Row::new(
        ORDINARY_LIVE_CONTINUATION_OUTCOME,
        "close",
        Family::Live,
        Phase::Dispose,
        Class::OrdinaryOutcome,
        Class::OrdinaryOutcome,
        "ordinary live consumer recovering from a stopped resume",
        "WorthQueryManagedLiveResumeStop::close",
    ),
    read_mechanism("compose_read", Phase::Declare),
    read_mechanism("compose_read_with_invariant_pack", Phase::Declare),
    read_mechanism("define_read_family", Phase::Declare),
    read_mechanism("define_read_family_with_invariant_pack", Phase::Declare),
    read_mechanism("execute_read_family", Phase::Execute),
    read_mechanism("execute_read_family_with_access_plan", Phase::Execute),
    read_mechanism(
        "execute_read_family_in_basis_context_with_access_plan",
        Phase::Execute,
    ),
    read_mechanism("execute_read_family_in_basis_context", Phase::Execute),
    read_diagnostic("explain_graph_read_access_shape"),
    read_mechanism("admit_graph_read_access_authority", Phase::Admit),
    read_mechanism(
        "admit_graph_read_access_authority_from_policy_tenant_request",
        Phase::Admit,
    ),
    read_diagnostic("explain_graph_read_access_shape_in_authority"),
    read_mechanism("admit_graph_read_access_in_authority", Phase::Admit),
    read_mechanism("plan_graph_read_access_in_authority", Phase::Plan),
    read_diagnostic("explain_boolean_selectivity_shape"),
    Row::new(
        WORKSPACE_QUERIES,
        "plan_live_graph_read_access",
        Family::Live,
        Phase::Plan,
        Class::InternalMechanism,
        Class::InternalMechanism,
        "Query live capability implementation",
        "managed live declaration",
    ),
    inspection_mechanism("inspect_intent"),
    Row::new(
        WORKSPACE_QUERIES,
        "install_program",
        Family::GeneralDeclaration,
        Phase::Execute,
        Class::InternalMechanism,
        Class::InternalMechanism,
        "Query program implementation",
        "capability-specific declaration",
    ),
    inspection_mechanism("inspect"),
    inspection_mechanism("inspect_live"),
    inspection_mechanism("inspect_live_target"),
    declaration_mechanism("orchestrate_declaration_entry", Phase::Execute),
    declaration_mechanism(
        "orchestrate_declaration_entry_outcome",
        Phase::AssembleOutcome,
    ),
    declaration_mechanism("orchestrate_declaration_entry_checked", Phase::Execute),
    declaration_mechanism("orchestrate_declaration_entry_proof", Phase::Execute),
    product_mechanism("orchestrate_routes_from_progressed", Phase::Plan),
    product_mechanism(
        "orchestrate_routes_from_progressed_with_intent",
        Phase::Plan,
    ),
    product_mechanism("orchestrate_routes_from_progressed_checked", Phase::Plan),
    product_mechanism(
        "orchestrate_routes_from_progressed_checked_with_intent",
        Phase::Plan,
    ),
    product_mechanism("orchestrate_routes_from_progressed_proof", Phase::Plan),
    product_mechanism(
        "orchestrate_routes_from_progressed_proof_with_intent",
        Phase::Plan,
    ),
    product_mechanism(
        "orchestrate_receipt_from_progressed",
        Phase::AssembleOutcome,
    ),
    product_mechanism(
        "orchestrate_receipt_from_progressed_with_intent",
        Phase::AssembleOutcome,
    ),
    product_mechanism(
        "orchestrate_receipt_from_progressed_checked",
        Phase::AssembleOutcome,
    ),
    product_mechanism(
        "orchestrate_receipt_from_progressed_checked_with_intent",
        Phase::AssembleOutcome,
    ),
    product_mechanism(
        "orchestrate_receipt_from_progressed_proof",
        Phase::AssembleOutcome,
    ),
    product_mechanism(
        "orchestrate_receipt_from_progressed_proof_with_intent",
        Phase::AssembleOutcome,
    ),
    product_mechanism(
        "orchestrate_envelope_from_progressed",
        Phase::AssembleOutcome,
    ),
    product_mechanism(
        "orchestrate_envelope_from_progressed_with_intent",
        Phase::AssembleOutcome,
    ),
    product_mechanism(
        "orchestrate_envelope_from_progressed_checked",
        Phase::AssembleOutcome,
    ),
    product_mechanism(
        "orchestrate_envelope_from_progressed_checked_with_intent",
        Phase::AssembleOutcome,
    ),
    product_mechanism(
        "orchestrate_envelope_from_progressed_proof",
        Phase::AssembleOutcome,
    ),
    product_mechanism(
        "orchestrate_envelope_from_progressed_proof_with_intent",
        Phase::AssembleOutcome,
    ),
];

const fn read_mechanism(function_name: &'static str, phase: Phase) -> Row {
    Row::new(
        WORKSPACE_QUERIES,
        function_name,
        Family::Read,
        phase,
        Class::Compatibility,
        Class::InternalMechanism,
        "legacy read consumer",
        "facade::read declaration journey",
    )
}

const fn read_diagnostic(function_name: &'static str) -> Row {
    Row::new(
        WORKSPACE_QUERIES,
        function_name,
        Family::Read,
        Phase::Inspect,
        Class::Diagnostics,
        Class::Diagnostics,
        "advanced read observer",
        "read outcome inspection",
    )
}

const fn inspection_mechanism(function_name: &'static str) -> Row {
    Row::new(
        WORKSPACE_QUERIES,
        function_name,
        Family::Inspection,
        Phase::Inspect,
        Class::Compatibility,
        Class::InternalMechanism,
        "legacy inspection consumer",
        "declarative inspection journey",
    )
}

const fn declaration_mechanism(function_name: &'static str, phase: Phase) -> Row {
    Row::new(
        DECLARATION_ORCHESTRATION,
        function_name,
        Family::GeneralDeclaration,
        phase,
        Class::Compatibility,
        Class::InternalMechanism,
        "legacy declaration coordinator",
        "capability-specific ordinary declaration",
    )
}

const fn product_mechanism(function_name: &'static str, phase: Phase) -> Row {
    Row::new(
        DECLARATION_PRODUCTS,
        function_name,
        Family::GeneralDeclaration,
        phase,
        Class::Compatibility,
        Class::InternalMechanism,
        "legacy declaration coordinator",
        "Query-owned outcome assembly",
    )
}
