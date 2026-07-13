use super::model::{
    WorthQueryDeclarativeCapabilityFamily as Family,
    WorthQueryDeclarativePhaseResponsibility as Phase, WorthQueryDeclarativeSurfaceClass as Class,
    WorthQueryDeclarativeSurfaceRow as Row,
};

const ORDINARY_READ_DECLARATION: &str = "src/ordinary/read/declaration.rs";
const ORDINARY_READ_EXECUTION: &str = "src/ordinary/read/execution.rs";
const WORKSPACE_QUERIES: &str = "src/runtime/workspace_queries.rs";
const DECLARATION_ORCHESTRATION: &str =
    "src/application/domain_handle/admitted_handle/declaration_entry/orchestration.rs";
const DECLARATION_PRODUCTS: &str =
    "src/application/domain_handle/admitted_handle/declaration_entry/products.rs";

pub fn worth_query_declarative_surface_rows() -> &'static [Row] {
    DECLARATIVE_SURFACE_ROWS
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
        ORDINARY_READ_EXECUTION,
        "run",
        Family::Read,
        Phase::Execute,
        Class::OrdinaryDeclaration,
        Class::OrdinaryDeclaration,
        "ordinary read consumer",
        "WorthQueryReadDeclaration::run",
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
