fn runtime_source_ingress_pass(path: &str) {
    trybuild::TestCases::new().pass(path);
}

fn runtime_source_ingress_fail(path: &str) {
    trybuild::TestCases::new().compile_fail(path);
}

#[test]
fn source_ingress_facade_types_are_importable() {
    runtime_source_ingress_pass(
        "tests/ui/runtime_source_ingress/pass/source_ingress_facade_types.rs",
    );
}

#[test]
fn ordering_receipt_fields_are_not_publicly_mintable() {
    runtime_source_ingress_fail(
        "tests/ui/runtime_source_ingress/fail/ordering_receipt_fields_not_public.rs",
    );
}

#[test]
fn raw_watcher_event_cannot_declare_dependency_impact() {
    runtime_source_ingress_fail(
        "tests/ui/runtime_source_ingress/fail/raw_watcher_event_cannot_declare_dependency_impact.rs",
    );
}
