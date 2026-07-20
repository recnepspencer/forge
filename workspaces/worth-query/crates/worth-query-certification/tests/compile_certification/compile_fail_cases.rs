use trybuild::TestCases;

pub(crate) fn register(cases: &TestCases) {
    for fixture in [
        "tests/ui/declarative_read/declaration_requires_context_before_run.rs",
        "tests/ui/declarative_read/declaration_is_move_only.rs",
        "tests/ui/declarative_workflow/preview_context_cannot_authorize_writeback.rs",
        "tests/ui/installed_domain/boundaries/installation_receipt_constructor_is_private.rs",
        "tests/ui/managed_live/handle_cannot_be_forged.rs",
        "tests/ui/projection_consumption/construction/consumed_projection_authority_constructor_private.rs",
        "tests/ui/query_identity_authority/external_compose_forbidden.rs",
        "tests/ui/query_identity_authority/wrong_kind_cannot_satisfy_query_family.rs",
        "tests/ui/prohibition_registry/deep_phase_module_import_forbidden.rs",
        "tests/ui/prohibition_registry/workspace_direct_write_forbidden.rs",
    ] {
        cases.compile_fail(fixture);
    }
}
