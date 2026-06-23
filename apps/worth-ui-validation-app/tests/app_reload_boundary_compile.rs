const APP_RELOAD_FAIL_CASES: &[&str] = &[
    "tests/ui/reload_boundary/fail/raw_appearance_package_cannot_enter_validation_reload_input.rs",
    "tests/ui/reload_boundary/fail/appearance_value_map_cannot_enter_validation_reload_workbench.rs",
    "tests/ui/reload_boundary/fail/raw_density_package_cannot_enter_validation_reload_input.rs",
    "tests/ui/reload_boundary/fail/density_value_map_cannot_enter_validation_reload_workbench.rs",
    "tests/ui/reload_boundary/fail/raw_component_package_cannot_enter_validation_reload_workbench.rs",
    "tests/ui/reload_boundary/fail/local_dropdown_mode_state_cannot_enter_validation_command_projection_reload.rs",
];

#[test]
fn raw_reload_material_cannot_enter_validation_app_reload_lanes() {
    let tests = trybuild::TestCases::new();

    for path in APP_RELOAD_FAIL_CASES {
        tests.compile_fail(*path);
    }
}
