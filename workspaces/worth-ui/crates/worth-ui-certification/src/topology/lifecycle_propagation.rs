pub fn lifecycle_propagation_fixture_paths() -> &'static [&'static str] {
    &[
        "tests/ui/lifecycle/external_runtime_bootstrap_fields_are_private.rs",
        "tests/ui/lifecycle/external_runtime_bootstrap_constructor_is_private.rs",
        "tests/ui/lifecycle/external_runtime_facade_builder_and_bootstrap_entries_are_not_public.rs",
        "tests/ui/lifecycle/external_inspection_subsystem_bootstrap_is_private.rs",
    ]
}

pub fn expected_phase3_lifecycle_subsystems() -> &'static [&'static str] {
    &["dsl_package", "inspection", "query_binding", "host_contract"]
}
