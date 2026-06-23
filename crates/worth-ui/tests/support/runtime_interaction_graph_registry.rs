#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RuntimeInteractionGraphRow {
    pub surface_name: &'static str,
    pub interaction_entrypoint: &'static str,
    pub state_authority: &'static str,
    pub fact_family: &'static str,
    pub fact_constructor: &'static str,
    pub projection_dependency_site: &'static str,
    pub runtime_change_admission: &'static str,
    pub graph_rebind_entrypoint: &'static str,
    pub behavior_proof: &'static str,
}

pub const REQUIRED_INTERACTION_GRAPH_SURFACES: &[&str] = &["dropdown_selection"];

pub const RUNTIME_INTERACTION_GRAPH_ROWS: &[RuntimeInteractionGraphRow] =
    &[RuntimeInteractionGraphRow {
        surface_name: "dropdown_selection",
        interaction_entrypoint: "WorthUiRuntimeHost::select_dropdown_command",
        state_authority: "WorthUiActiveRuntimeState::record_dropdown_selection_state",
        fact_family: "WorthUiRuntimeFactFamily::DropdownSelectionState",
        fact_constructor: "WorthUiRuntimeFactId::dropdown_selection_state",
        projection_dependency_site: "WorthUiDropdownProjectionPlan::from_snapshot",
        runtime_change_admission: "WorthUiRuntimeHost::admit_dropdown_selection_runtime_change",
        graph_rebind_entrypoint: "WorthUiRuntimeHost::rebind_header_frame_after_runtime_change",
        behavior_proof:
            "runtime::projection_rebind::projection_rebind_tests::header_rebind_after_interaction_uses_changed_fact_intersection_without_surface_shortcuts",
    }];
