#[test]
fn facade_theme_token_registration_compiles() {
    facade_compile_pass("tests/ui/facade/pass/theme_token_registration_uses_only_facade.rs");
}

#[test]
fn theme_token_descriptor_fields_not_publicly_mintable() {
    facade_compile_fail(
        "tests/ui/facade/theme_token/construction/theme_token_descriptor_fields_not_publicly_mintable.rs",
    );
}

#[test]
fn raw_color_cannot_replace_theme_token_dependency() {
    facade_compile_fail(
        "tests/ui/facade/theme_token/construction/raw_color_cannot_replace_theme_token_dependency.rs",
    );
}

#[test]
fn theme_token_registry_internal_module_not_public() {
    facade_compile_fail(
        "tests/ui/facade/theme_token/facade/theme_token_registry_internal_module_not_public.rs",
    );
}

#[test]
fn theme_token_registry_type_not_publicly_importable() {
    facade_compile_fail(
        "tests/ui/facade/theme_token/facade/theme_token_registry_type_not_publicly_importable.rs",
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
