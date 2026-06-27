#[path = "trybuild_support.rs"]
mod trybuild_support;
#[test]
fn compat_root_plugin_slot_registration_compiles() {
    facade_compile_pass(
        "tests/ui/facade/compat_pass/plugin_slot_registration_uses_only_facade.rs",
    );
}

#[test]
fn plugin_slot_descriptor_fields_not_publicly_mintable() {
    facade_compile_fail(
        "tests/ui/facade/plugin_slot/construction/plugin_slot_descriptor_fields_not_publicly_mintable.rs",
    );
}

#[test]
fn plugin_slot_registry_internal_module_not_public() {
    facade_compile_fail(
        "tests/ui/facade/plugin_slot/facade/plugin_slot_registry_internal_module_not_public.rs",
    );
}

#[test]
fn plugin_slot_registry_type_not_publicly_importable() {
    facade_compile_fail(
        "tests/ui/facade/plugin_slot/facade/plugin_slot_registry_type_not_publicly_importable.rs",
    );
}

#[test]
fn plugin_global_mutation_hook_is_diagnostic_only() {
    facade_compile_fail(
        "tests/ui/facade/plugin_slot/construction/plugin_global_mutation_hook_is_diagnostic_only.rs",
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

