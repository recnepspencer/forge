fn facade_visibility_pass(path: &str) {
    trybuild::TestCases::new().pass(path);
}

fn facade_visibility_fail(path: &str) {
    trybuild::TestCases::new().compile_fail(path);
}

#[test]
fn facade_only_app_remains_ergonomic() {
    facade_visibility_pass("tests/ui/facade_visibility/pass/facade_only_app_remains_ergonomic.rs");
}

#[test]
fn internal_registry_constructor_not_publicly_accessible() {
    facade_visibility_fail(
        "tests/ui/facade_visibility/internal_registry_constructor/registry_constructors_not_public.rs",
    );
}

#[test]
fn mutable_registry_storage_not_publicly_accessible() {
    facade_visibility_fail(
        "tests/ui/facade_visibility/mutable_registry_storage/builder_registry_storage_not_public.rs",
    );
}

#[test]
fn registration_candidate_storage_not_publicly_accessible() {
    facade_visibility_fail(
        "tests/ui/facade_visibility/mutable_registry_storage/builder_registration_candidate_storage_not_public.rs",
    );
}

#[test]
fn snapshot_index_storage_not_publicly_accessible() {
    facade_visibility_fail(
        "tests/ui/facade_visibility/mutable_registry_storage/snapshot_index_storage_not_public.rs",
    );
}

#[test]
fn snapshot_builder_not_publicly_importable() {
    facade_visibility_fail(
        "tests/ui/facade_visibility/snapshot_authority/snapshot_builder_not_public.rs",
    );
}

#[test]
fn snapshot_freeze_input_not_publicly_importable() {
    facade_visibility_fail(
        "tests/ui/facade_visibility/snapshot_authority/snapshot_freeze_input_not_public.rs",
    );
}

#[test]
fn snapshot_index_parts_not_publicly_importable() {
    facade_visibility_fail(
        "tests/ui/facade_visibility/snapshot_authority/snapshot_index_parts_not_public.rs",
    );
}

#[test]
fn snapshot_internal_fields_not_publicly_constructible() {
    facade_visibility_fail(
        "tests/ui/facade_visibility/snapshot_authority/snapshot_fields_not_publicly_constructible.rs",
    );
}

#[test]
fn validated_descriptor_fields_not_publicly_mintable() {
    facade_visibility_fail(
        "tests/ui/facade_visibility/descriptor_authority/representative_descriptor_fields_not_publicly_mintable.rs",
    );
}

#[test]
fn facade_does_not_reexport_registry_types() {
    facade_visibility_fail(
        "tests/ui/facade_visibility/topology/facade_does_not_reexport_registry_types.rs",
    );
}

#[test]
fn crate_root_does_not_bypass_facade() {
    facade_visibility_fail(
        "tests/ui/facade_visibility/topology/crate_root_does_not_bypass_facade.rs",
    );
}

#[test]
fn crate_root_does_not_export_snapshot_authority() {
    facade_visibility_fail(
        "tests/ui/facade_visibility/topology/crate_root_does_not_export_snapshot_authority.rs",
    );
}
