#[test]
fn facade_runtime_outcome_projection_registration_compiles() {
    facade_compile_pass(
        "tests/ui/facade/pass/runtime_outcome_projection_registration_uses_only_facade.rs",
    );
}

#[test]
fn runtime_outcome_projection_descriptor_fields_not_publicly_mintable() {
    facade_compile_fail(
        "tests/ui/facade/runtime_outcome_projection/construction/runtime_outcome_projection_descriptor_fields_not_publicly_mintable.rs",
    );
}

#[test]
fn local_status_enum_cannot_replace_runtime_outcome_reference() {
    facade_compile_fail(
        "tests/ui/facade/runtime_outcome_projection/construction/local_status_enum_cannot_replace_runtime_outcome_reference.rs",
    );
}

#[test]
fn runtime_outcome_projection_registry_internal_module_not_public() {
    facade_compile_fail(
        "tests/ui/facade/runtime_outcome_projection/facade/runtime_outcome_projection_registry_internal_module_not_public.rs",
    );
}

#[test]
fn runtime_outcome_projection_registry_type_not_publicly_importable() {
    facade_compile_fail(
        "tests/ui/facade/runtime_outcome_projection/facade/runtime_outcome_projection_registry_type_not_publicly_importable.rs",
    );
}

fn facade_compile_pass(fixture_path: &str) {
    let tests = trybuild::TestCases::new();
    tests.pass(fixture_path);
}

fn facade_compile_fail(fixture_path: &str) {
    let tests = trybuild::TestCases::new();
    tests.compile_fail(fixture_path);
}
