#[path = "support/trybuild_helpers.rs"]
mod trybuild_helpers;

const FACADE_PASS_CASES: &[&str] = &[
    "tests/ui/facade/pass/empty_app_uses_only_facade.rs",
    "tests/ui/facade/pass/identity_ids_use_only_facade.rs",
    "tests/ui/facade/pass/component_registration_uses_only_facade.rs",
];

const FACADE_FAIL_CASES: &[&str] = &[
    "tests/ui/facade/topology/internal_modules/internal_registry_module_import_fails.rs",
    "tests/ui/facade/topology/internal_modules/internal_identity_module_import_fails.rs",
    "tests/ui/facade/topology/facade_reexports/facade_reexport_does_not_expose_internal_topology.rs",
    "tests/ui/facade_visibility/topology/crate_root_does_not_bypass_facade.rs",
    "tests/ui/facade/construction/sealed_lifecycle/direct_builder_construction_fails.rs",
    "tests/ui/facade/construction/associated_constructors/direct_app_snapshot_constructor_fails.rs",
    "tests/ui/facade/lifecycle/freeze/register_after_snapshot_freeze_fails.rs",
    "tests/ui/facade/lifecycle/snapshot_authority/snapshot_types/snapshot_fields_not_publicly_constructible.rs",
    "tests/ui/facade/lifecycle/snapshot_authority/registered_set/registered_capability_set_not_publicly_mutable.rs",
    "tests/ui/facade/snapshot/snapshot_internal_indexes_not_publicly_mutable.rs",
    "tests/ui/facade/identity/family_interchange/same_text_different_id_families_are_not_interchangeable.rs",
    "tests/ui/facade/identity/construction/raw_text_cannot_replace_validated_id.rs",
    "tests/ui/facade/support/construction/external_support_id_impl_is_forbidden.rs",
    "tests/ui/facade/diagnostics/construction/raw_strings_cannot_replace_diagnostic_codes.rs",
    "tests/ui/facade/command/construction/command_descriptor_fields_not_publicly_mintable.rs",
    "tests/ui/facade/command_projection/construction/projection_command_meaning_methods_are_not_available.rs",
    "tests/ui/facade/component/facade/component_registry_type_not_publicly_importable.rs",
    "tests/ui/facade/icon/construction/raw_asset_path_cannot_replace_icon_dependency.rs",
    "tests/ui/facade/mosaic_sizing/construction/raw_number_cannot_replace_named_mosaic_sizing_measurement.rs",
    "tests/ui/facade/mosaic_state/construction/raw_text_cannot_replace_mosaic_state_owner_scope_id.rs",
    "tests/ui/facade/native_capability/construction/ambient_host_check_cannot_replace_native_capability_posture.rs",
    "tests/ui/facade/plugin_slot/construction/plugin_global_mutation_hook_is_diagnostic_only.rs",
    "tests/ui/facade/runtime_outcome_projection/construction/local_status_enum_cannot_replace_runtime_outcome_reference.rs",
    "tests/ui/facade/settings/construction/raw_map_cannot_replace_setting_descriptor.rs",
    "tests/ui/facade/task_presentation/construction/task_runtime_handle_cannot_replace_task_presentation_descriptor.rs",
    "tests/ui/facade/theme_token/construction/raw_color_cannot_replace_theme_token_dependency.rs",
    "tests/ui/facade/view_binding/construction/local_pseudo_query_binding_cannot_replace_query_reference.rs",
];

#[test]
fn facade_examples_compile_through_public_surface() {
    trybuild_helpers::run_pass_cases(FACADE_PASS_CASES);
}

#[test]
fn facade_boundaries_stay_sealed() {
    trybuild_helpers::run_compile_fail_cases(FACADE_FAIL_CASES);
}
