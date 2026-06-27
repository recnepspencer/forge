#[path = "trybuild_support.rs"]
mod trybuild_support;
#[test]
fn compat_root_mosaic_region_registration_compiles() {
    facade_compile_pass(
        "tests/ui/facade/compat_pass/mosaic_region_registration_uses_only_facade.rs",
    );
}

#[test]
fn compat_root_mosaic_placement_registration_compiles() {
    facade_compile_pass(
        "tests/ui/facade/compat_pass/mosaic_placement_registration_uses_only_facade.rs",
    );
}

#[test]
fn compat_root_mosaic_sizing_registration_compiles() {
    facade_compile_pass(
        "tests/ui/facade/compat_pass/mosaic_sizing_registration_uses_only_facade.rs",
    );
}

#[test]
fn compat_root_mosaic_state_registration_compiles() {
    facade_compile_pass(
        "tests/ui/facade/compat_pass/mosaic_state_registration_uses_only_facade.rs",
    );
}

#[test]
fn mosaic_region_descriptor_fields_not_publicly_mintable() {
    facade_compile_fail(
        "tests/ui/facade/mosaic_region/construction/mosaic_region_descriptor_fields_not_publicly_mintable.rs",
    );
}

#[test]
fn mosaic_region_registry_internal_module_not_public() {
    facade_compile_fail(
        "tests/ui/facade/mosaic_region/facade/mosaic_region_registry_internal_module_not_public.rs",
    );
}

#[test]
fn mosaic_region_registry_type_not_publicly_importable() {
    facade_compile_fail(
        "tests/ui/facade/mosaic_region/facade/mosaic_region_registry_type_not_publicly_importable.rs",
    );
}

#[test]
fn mosaic_placement_descriptor_fields_not_publicly_mintable() {
    facade_compile_fail(
        "tests/ui/facade/mosaic_placement/construction/mosaic_placement_descriptor_fields_not_publicly_mintable.rs",
    );
}

#[test]
fn mosaic_placement_registry_internal_module_not_public() {
    facade_compile_fail(
        "tests/ui/facade/mosaic_placement/facade/mosaic_placement_registry_internal_module_not_public.rs",
    );
}

#[test]
fn mosaic_placement_registry_type_not_publicly_importable() {
    facade_compile_fail(
        "tests/ui/facade/mosaic_placement/facade/mosaic_placement_registry_type_not_publicly_importable.rs",
    );
}

#[test]
fn mosaic_sizing_descriptor_fields_not_publicly_mintable() {
    facade_compile_fail(
        "tests/ui/facade/mosaic_sizing/construction/mosaic_sizing_descriptor_fields_not_publicly_mintable.rs",
    );
}

#[test]
fn raw_number_cannot_replace_named_mosaic_sizing_measurement() {
    facade_compile_fail(
        "tests/ui/facade/mosaic_sizing/construction/raw_number_cannot_replace_named_mosaic_sizing_measurement.rs",
    );
}

#[test]
fn mosaic_sizing_registry_internal_module_not_public() {
    facade_compile_fail(
        "tests/ui/facade/mosaic_sizing/facade/mosaic_sizing_registry_internal_module_not_public.rs",
    );
}

#[test]
fn mosaic_sizing_registry_type_not_publicly_importable() {
    facade_compile_fail(
        "tests/ui/facade/mosaic_sizing/facade/mosaic_sizing_registry_type_not_publicly_importable.rs",
    );
}

#[test]
fn mosaic_state_descriptor_fields_not_publicly_mintable() {
    facade_compile_fail(
        "tests/ui/facade/mosaic_state/construction/mosaic_state_descriptor_fields_not_publicly_mintable.rs",
    );
}

#[test]
fn raw_text_cannot_replace_mosaic_state_owner_scope_id() {
    facade_compile_fail(
        "tests/ui/facade/mosaic_state/construction/raw_text_cannot_replace_mosaic_state_owner_scope_id.rs",
    );
}

#[test]
fn mosaic_state_registry_internal_module_not_public() {
    facade_compile_fail(
        "tests/ui/facade/mosaic_state/facade/mosaic_state_registry_internal_module_not_public.rs",
    );
}

#[test]
fn mosaic_state_registry_type_not_publicly_importable() {
    facade_compile_fail(
        "tests/ui/facade/mosaic_state/facade/mosaic_state_registry_type_not_publicly_importable.rs",
    );
}

fn facade_compile_pass(fixture_path: &str) {
    trybuild_support::new_test_cases().pass(fixture_path);
}

fn facade_compile_fail(fixture_path: &str) {
    trybuild_support::new_test_cases().compile_fail(fixture_path);
}

