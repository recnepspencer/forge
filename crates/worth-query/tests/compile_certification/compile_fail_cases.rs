use trybuild::TestCases;

pub(crate) fn register(cases: &TestCases) {
    for pattern in [
        "tests/ui/*.rs",
        "tests/ui/aspect_native_query/*.rs",
        "tests/ui/basis_lifecycle/dx/*.rs",
        "tests/ui/declarative_history_comparison/*.rs",
        "tests/ui/declarative_read/*.rs",
        "tests/ui/declarative_workflow/*.rs",
        "tests/ui/domain_capabilities/dx_boundaries/*.rs",
        "tests/ui/graph_read_access/*.rs",
        "tests/ui/graph_read_access_admission/*.rs",
        "tests/ui/graph_read_access_cost/*.rs",
        "tests/ui/graph_read_access_persistent_requirement/*.rs",
        "tests/ui/installed_domain/boundaries/*.rs",
        "tests/ui/intent_admission/authoring/*.rs",
        "tests/ui/managed_live/*.rs",
        "tests/ui/prohibition_registry/*.rs",
        "tests/ui/projection_consumption/boundaries/*.rs",
        "tests/ui/projection_consumption/construction/*.rs",
        "tests/ui/public_authority_surface/*.rs",
        "tests/ui/public_bridge_reader_lane/*.rs",
        "tests/ui/query_identity_authority/*.rs",
        "tests/ui/subscription_phase_seven/*.rs",
    ] {
        cases.compile_fail(pattern);
    }
}
