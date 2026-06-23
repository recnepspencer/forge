#[path = "support/trybuild_helpers.rs"]
mod trybuild_helpers;

#[test]
fn runtime_source_ingress_public_types_compile() {
    trybuild_helpers::run_pass_cases(&[
        "tests/ui/runtime_source_ingress/pass/source_ingress_facade_types.rs",
    ]);
}

#[test]
fn runtime_source_ingress_boundary_stays_sealed() {
    trybuild_helpers::run_compile_fail_cases(&[
        "tests/ui/runtime_source_ingress/fail/ordering_receipt_fields_not_public.rs",
        "tests/ui/runtime_source_ingress/fail/raw_watcher_event_cannot_declare_dependency_impact.rs",
    ]);
}
