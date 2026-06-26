#[test]
fn facade_icon_registration_compiles() {
    facade_compile_pass("tests/ui/facade/pass/icon_registration_uses_only_facade.rs");
}

#[test]
fn icon_descriptor_fields_not_publicly_mintable() {
    facade_compile_fail(
        "tests/ui/facade/icon/construction/icon_descriptor_fields_not_publicly_mintable.rs",
    );
}

#[test]
fn raw_asset_path_cannot_replace_icon_dependency() {
    facade_compile_fail(
        "tests/ui/facade/icon/construction/raw_asset_path_cannot_replace_icon_dependency.rs",
    );
}

#[test]
fn icon_registry_internal_module_not_public() {
    facade_compile_fail("tests/ui/facade/icon/facade/icon_registry_internal_module_not_public.rs");
}

#[test]
fn icon_registry_type_not_publicly_importable() {
    facade_compile_fail(
        "tests/ui/facade/icon/facade/icon_registry_type_not_publicly_importable.rs",
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
