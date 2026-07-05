#[path = "trybuild_support.rs"]
mod trybuild_support;
#[test]
fn compat_root_task_presentation_registration_compiles() {
    facade_compile_pass(
        "tests/ui/facade/compat_pass/task_presentation_registration_uses_only_facade.rs",
    );
}

#[test]
fn task_presentation_descriptor_fields_not_publicly_mintable() {
    facade_compile_fail(
        "tests/ui/facade/task_presentation/construction/task_presentation_descriptor_fields_not_publicly_mintable.rs",
    );
}

#[test]
fn task_runtime_handle_cannot_replace_task_presentation_descriptor() {
    facade_compile_fail(
        "tests/ui/facade/task_presentation/construction/task_runtime_handle_cannot_replace_task_presentation_descriptor.rs",
    );
}

#[test]
fn task_presentation_registry_internal_module_not_public() {
    facade_compile_fail(
        "tests/ui/facade/task_presentation/facade/task_presentation_registry_internal_module_not_public.rs",
    );
}

#[test]
fn task_presentation_registry_type_not_publicly_importable() {
    facade_compile_fail(
        "tests/ui/facade/task_presentation/facade/task_presentation_registry_type_not_publicly_importable.rs",
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
