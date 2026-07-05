#[path = "trybuild_support.rs"]
mod trybuild_support;
// Preferred public architecture lives on named facade submodules.
// Root-level `worth_ui::facade::*` coverage remains here only as compatibility residue.

#[test]
fn named_app_surface_empty_app_compiles() {
    facade_compile_pass("tests/ui/facade/named_surface_pass/empty_app_uses_named_app_surface.rs");
}

#[test]
fn named_inspection_surface_compiles() {
    facade_compile_pass(
        "tests/ui/facade/named_surface_pass/inspection_query_uses_named_inspection_surface.rs",
    );
}

#[test]
fn compat_root_empty_app_compiles() {
    facade_compile_pass("tests/ui/facade/compat_pass/empty_app_uses_only_facade.rs");
}

#[test]
fn compat_root_identity_ids_compile() {
    facade_compile_pass("tests/ui/facade/compat_pass/identity_ids_use_only_facade.rs");
}

#[test]
fn compat_root_command_registration_compiles() {
    facade_compile_pass("tests/ui/facade/compat_pass/command_registration_uses_only_facade.rs");
}

#[test]
fn compat_root_component_registration_compiles() {
    facade_compile_pass("tests/ui/facade/compat_pass/component_registration_uses_only_facade.rs");
}

#[test]
fn compat_root_surface_registration_compiles() {
    facade_compile_pass("tests/ui/facade/compat_pass/surface_registration_uses_only_facade.rs");
}

#[test]
fn internal_registry_module_import_fails() {
    facade_compile_fail(
        "tests/ui/facade/topology/internal_modules/internal_registry_module_import_fails.rs",
    );
}

#[test]
fn internal_identity_module_import_fails() {
    facade_compile_fail(
        "tests/ui/facade/topology/internal_modules/internal_identity_module_import_fails.rs",
    );
}

#[test]
fn facade_reexport_does_not_expose_internal_topology() {
    facade_compile_fail(
        "tests/ui/facade/topology/facade_reexports/facade_reexport_does_not_expose_internal_topology.rs",
    );
}

#[test]
fn crate_root_does_not_offer_alternate_construction_path() {
    facade_compile_fail(
        "tests/ui/facade/construction/root_exports/crate_root_does_not_offer_alternate_construction_path.rs",
    );
}

#[test]
fn crate_root_does_not_export_facade_types() {
    facade_compile_fail(
        "tests/ui/facade/construction/root_exports/crate_root_does_not_export_facade_types.rs",
    );
}

#[test]
fn crate_root_does_not_export_identity_types() {
    facade_compile_fail(
        "tests/ui/facade/construction/root_exports/crate_root_does_not_export_identity_types.rs",
    );
}

#[test]
fn direct_builder_construction_fails() {
    facade_compile_fail(
        "tests/ui/facade/construction/sealed_lifecycle/direct_builder_construction_fails.rs",
    );
}

#[test]
fn direct_entry_construction_fails() {
    facade_compile_fail(
        "tests/ui/facade/construction/sealed_lifecycle/direct_entry_construction_fails.rs",
    );
}

#[test]
fn direct_app_construction_fails() {
    facade_compile_fail(
        "tests/ui/facade/construction/sealed_lifecycle/direct_app_construction_fails.rs",
    );
}

#[test]
fn direct_builder_new_constructor_fails() {
    facade_compile_fail(
        "tests/ui/facade/construction/associated_constructors/direct_builder_new_constructor_fails.rs",
    );
}

#[test]
fn direct_app_snapshot_constructor_fails() {
    facade_compile_fail(
        "tests/ui/facade/construction/associated_constructors/direct_app_snapshot_constructor_fails.rs",
    );
}

#[test]
fn internal_builder_module_import_fails() {
    facade_compile_fail(
        "tests/ui/facade/topology/internal_modules/internal_builder_module_import_fails.rs",
    );
}

#[test]
fn register_after_snapshot_freeze_fails() {
    facade_compile_fail("tests/ui/facade/lifecycle/freeze/register_after_snapshot_freeze_fails.rs");
}

#[test]
fn snapshot_fields_not_publicly_constructible() {
    facade_compile_fail(
        "tests/ui/facade/lifecycle/snapshot_authority/snapshot_types/snapshot_fields_not_publicly_constructible.rs",
    );
}

#[test]
fn registered_capability_set_not_publicly_mutable() {
    facade_compile_fail(
        "tests/ui/facade/lifecycle/snapshot_authority/registered_set/registered_capability_set_not_publicly_mutable.rs",
    );
}

#[test]
fn registered_capability_set_not_publicly_constructible() {
    facade_compile_fail(
        "tests/ui/facade/lifecycle/snapshot_authority/registered_set/registered_capability_set_not_publicly_constructible.rs",
    );
}

#[test]
fn snapshot_digest_not_publicly_constructible() {
    facade_compile_fail(
        "tests/ui/facade/lifecycle/snapshot_authority/snapshot_types/snapshot_digest_not_publicly_constructible.rs",
    );
}

#[test]
fn snapshot_metrics_not_publicly_constructible() {
    facade_compile_fail(
        "tests/ui/facade/lifecycle/snapshot_authority/snapshot_types/snapshot_metrics_not_publicly_constructible.rs",
    );
}

#[test]
fn registered_capability_set_empty_constructor_fails() {
    facade_compile_fail(
        "tests/ui/facade/lifecycle/snapshot_constructors/registered_capability_set_empty_constructor_fails.rs",
    );
}

#[test]
fn capability_snapshot_constructor_fails() {
    facade_compile_fail(
        "tests/ui/facade/lifecycle/snapshot_constructors/capability_snapshot_constructor_fails.rs",
    );
}

#[test]
fn snapshot_digest_constructor_fails() {
    facade_compile_fail(
        "tests/ui/facade/lifecycle/snapshot_constructors/snapshot_digest_constructor_fails.rs",
    );
}

#[test]
fn snapshot_metrics_constructor_fails() {
    facade_compile_fail(
        "tests/ui/facade/lifecycle/snapshot_constructors/snapshot_metrics_constructor_fails.rs",
    );
}

#[test]
fn same_text_different_id_families_are_not_interchangeable() {
    facade_compile_fail(
        "tests/ui/facade/identity/family_interchange/same_text_different_id_families_are_not_interchangeable.rs",
    );
}

#[test]
fn raw_text_cannot_replace_validated_id() {
    facade_compile_fail(
        "tests/ui/facade/identity/construction/raw_text_cannot_replace_validated_id.rs",
    );
}

#[test]
fn validated_id_fields_not_publicly_mintable() {
    facade_compile_fail(
        "tests/ui/facade/identity/construction/validated_id_fields_not_publicly_mintable.rs",
    );
}

#[test]
fn admitted_posture_witness_is_not_publicly_mintable() {
    facade_compile_fail(
        "tests/ui/facade/support/construction/admitted_capability_not_publicly_mintable.rs",
    );
}

#[test]
fn classified_posture_cannot_replace_admitted_witness() {
    facade_compile_fail(
        "tests/ui/facade/support/construction/classified_posture_cannot_replace_admitted_witness.rs",
    );
}

#[test]
fn raw_text_cannot_define_support_posture() {
    facade_compile_fail(
        "tests/ui/facade/support/construction/raw_text_cannot_define_support_posture.rs",
    );
}

#[test]
fn external_support_id_impl_is_forbidden() {
    facade_compile_fail(
        "tests/ui/facade/support/construction/external_support_id_impl_is_forbidden.rs",
    );
}

#[test]
fn diagnostic_report_fields_not_publicly_mintable() {
    facade_compile_fail(
        "tests/ui/facade/diagnostics/construction/diagnostic_report_fields_not_publicly_mintable.rs",
    );
}

#[test]
fn raw_strings_cannot_replace_diagnostic_codes() {
    facade_compile_fail(
        "tests/ui/facade/diagnostics/construction/raw_strings_cannot_replace_diagnostic_codes.rs",
    );
}

#[test]
fn command_descriptor_fields_not_publicly_mintable() {
    facade_compile_fail(
        "tests/ui/facade/command/construction/command_descriptor_fields_not_publicly_mintable.rs",
    );
}

#[test]
fn command_readiness_cannot_be_flattened_to_bool() {
    facade_compile_fail(
        "tests/ui/facade/command/construction/command_readiness_cannot_be_flattened_to_bool.rs",
    );
}

#[test]
fn command_registry_internal_module_not_public() {
    facade_compile_fail(
        "tests/ui/facade/command/facade/command_registry_internal_module_not_public.rs",
    );
}

#[test]
fn command_registry_type_not_publicly_importable() {
    facade_compile_fail(
        "tests/ui/facade/command/facade/command_registry_type_not_publicly_importable.rs",
    );
}

#[test]
fn component_descriptor_fields_not_publicly_mintable() {
    facade_compile_fail(
        "tests/ui/facade/component/construction/component_descriptor_fields_not_publicly_mintable.rs",
    );
}

#[test]
fn component_registry_internal_module_not_public() {
    facade_compile_fail(
        "tests/ui/facade/component/facade/component_registry_internal_module_not_public.rs",
    );
}

#[test]
fn component_registry_type_not_publicly_importable() {
    facade_compile_fail(
        "tests/ui/facade/component/facade/component_registry_type_not_publicly_importable.rs",
    );
}

#[test]
fn surface_descriptor_fields_not_publicly_mintable() {
    facade_compile_fail(
        "tests/ui/facade/surface/construction/surface_descriptor_fields_not_publicly_mintable.rs",
    );
}

#[test]
fn surface_registry_internal_module_not_public() {
    facade_compile_fail(
        "tests/ui/facade/surface/facade/surface_registry_internal_module_not_public.rs",
    );
}

#[test]
fn surface_registry_type_not_publicly_importable() {
    facade_compile_fail(
        "tests/ui/facade/surface/facade/surface_registry_type_not_publicly_importable.rs",
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
