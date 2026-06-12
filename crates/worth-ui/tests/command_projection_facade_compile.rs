#[test]
fn facade_command_projection_registration_compiles() {
    facade_compile_pass("tests/ui/facade/pass/command_projection_registration_uses_only_facade.rs");
}

#[test]
fn command_projection_descriptor_fields_not_publicly_mintable() {
    facade_compile_fail(
        "tests/ui/facade/command_projection/construction/command_projection_descriptor_fields_not_publicly_mintable.rs",
    );
}

#[test]
fn projection_command_meaning_methods_are_not_available() {
    facade_compile_fail(
        "tests/ui/facade/command_projection/construction/projection_command_meaning_methods_are_not_available.rs",
    );
}

#[test]
fn command_projection_registry_internal_module_not_public() {
    facade_compile_fail(
        "tests/ui/facade/command_projection/facade/command_projection_registry_internal_module_not_public.rs",
    );
}

#[test]
fn command_projection_registry_type_not_publicly_importable() {
    facade_compile_fail(
        "tests/ui/facade/command_projection/facade/command_projection_registry_type_not_publicly_importable.rs",
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
