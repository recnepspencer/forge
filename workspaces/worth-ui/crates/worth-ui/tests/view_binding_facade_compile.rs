#[path = "trybuild_support.rs"]
mod trybuild_support;
#[test]
fn installed_query_view_registration_compiles() {
    facade_compile_pass("tests/ui/facade/query_binding/pass/installed_view_registration.rs");
}

#[test]
fn query_view_definition_fields_not_publicly_mintable() {
    facade_compile_fail(
        "tests/ui/facade/query_binding/construction/query_view_definition_fields_not_publicly_mintable.rs",
    );
}

#[test]
fn installed_query_view_fields_not_publicly_mintable() {
    facade_compile_fail(
        "tests/ui/facade/query_binding/construction/installed_query_view_fields_not_publicly_mintable.rs",
    );
}

#[test]
fn query_registration_requires_an_installed_view() {
    facade_compile_fail(
        "tests/ui/facade/query_binding/construction/query_registration_requires_installed_view.rs",
    );
}

#[test]
fn direct_view_binding_registration_is_not_public() {
    facade_compile_fail(
        "tests/ui/facade/query_binding/construction/direct_view_binding_registration_is_not_public.rs",
    );
}

#[test]
fn detached_view_binding_descriptor_constructor_is_not_public() {
    facade_compile_fail(
        "tests/ui/facade/query_binding/construction/detached_view_binding_descriptor_constructor_is_not_public.rs",
    );
}

#[test]
fn view_binding_registry_internal_module_not_public() {
    facade_compile_fail(
        "tests/ui/facade/view_binding/facade/view_binding_registry_internal_module_not_public.rs",
    );
}

#[test]
fn view_binding_registry_type_not_publicly_importable() {
    facade_compile_fail(
        "tests/ui/facade/view_binding/facade/view_binding_registry_type_not_publicly_importable.rs",
    );
}

fn facade_compile_pass(fixture_path: &str) {
    let tests = trybuild_support::new_test_cases();
    tests.pass(fixture_path);
}

fn facade_compile_fail(fixture_path: &str) {
    let tests = trybuild_support::new_test_cases();
    tests.compile_fail(fixture_path);
}
