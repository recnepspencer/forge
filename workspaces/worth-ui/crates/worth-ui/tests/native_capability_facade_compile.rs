#[test]
fn facade_native_capability_registration_compiles() {
    facade_compile_pass("tests/ui/facade/pass/native_capability_registration_uses_only_facade.rs");
}

#[test]
fn native_capability_descriptor_fields_not_publicly_mintable() {
    facade_compile_fail(
        "tests/ui/facade/native_capability/construction/native_capability_descriptor_fields_not_publicly_mintable.rs",
    );
}

#[test]
fn native_capability_registry_internal_module_not_public() {
    facade_compile_fail(
        "tests/ui/facade/native_capability/facade/native_capability_registry_internal_module_not_public.rs",
    );
}

#[test]
fn native_capability_registry_type_not_publicly_importable() {
    facade_compile_fail(
        "tests/ui/facade/native_capability/facade/native_capability_registry_type_not_publicly_importable.rs",
    );
}

#[test]
fn ambient_host_check_is_diagnostic_only() {
    facade_compile_fail(
        "tests/ui/facade/native_capability/construction/ambient_host_check_cannot_replace_native_capability_posture.rs",
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
