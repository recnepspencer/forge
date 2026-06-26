#[test]
fn facade_view_binding_registration_compiles() {
    facade_compile_pass("tests/ui/facade/pass/view_binding_registration_uses_only_facade.rs");
}

#[test]
fn view_binding_descriptor_fields_not_publicly_mintable() {
    facade_compile_fail(
        "tests/ui/facade/view_binding/construction/view_binding_descriptor_fields_not_publicly_mintable.rs",
    );
}

#[test]
fn local_pseudo_query_binding_cannot_replace_query_reference() {
    facade_compile_fail(
        "tests/ui/facade/view_binding/construction/local_pseudo_query_binding_cannot_replace_query_reference.rs",
    );
}

#[test]
fn admitted_query_view_binding_witness_is_not_publicly_mintable() {
    facade_compile_fail(
        "tests/ui/facade/view_binding/construction/admitted_query_view_binding_witness_is_not_publicly_mintable.rs",
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
    let tests = trybuild::TestCases::new();
    tests.pass(fixture_path);
}

fn facade_compile_fail(fixture_path: &str) {
    let tests = trybuild::TestCases::new();
    tests.compile_fail(fixture_path);
}
