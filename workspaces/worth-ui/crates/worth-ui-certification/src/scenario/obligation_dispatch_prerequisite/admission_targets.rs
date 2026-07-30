//! Admitted and deliberately denied obligation-selection targets.

use worth_ui::facade::admission::{
    UiAdmissionSelectionBudget, UiAdmissionTarget, UiAdmissionWorld,
};
use worth_ui::facade::graph::UiGraphTouchDescriptor;
use worth_ui_host_contract::{WorthUiHostCapability, WorthUiHostCapabilityReport};

pub fn selection_target(touch: &UiGraphTouchDescriptor) -> UiAdmissionTarget {
    UiAdmissionTarget::graph_node(
        touch.target().graph_node_identity(),
        UiAdmissionWorld::from_graph_world_profile(touch.world().world_profile().clone()),
    )
}

pub fn graph_aligned_query_target(touch: &UiGraphTouchDescriptor) -> UiAdmissionTarget {
    selection_target(touch)
}

pub fn missing_host_capability_target(touch: &UiGraphTouchDescriptor) -> UiAdmissionTarget {
    selection_target(touch)
        .with_host_capability_report(WorthUiHostCapabilityReport::missing(Vec::new()))
}

pub fn ambiguous_host_capability_target(touch: &UiGraphTouchDescriptor) -> UiAdmissionTarget {
    selection_target(touch).with_host_capability_report(WorthUiHostCapabilityReport::ambiguous(
        modeled_text_capabilities(),
    ))
}

pub fn available_host_capability_target(touch: &UiGraphTouchDescriptor) -> UiAdmissionTarget {
    selection_target(touch).with_host_capability_report(WorthUiHostCapabilityReport::available(
        modeled_text_capabilities(),
    ))
}

pub fn diagnostic_only_host_capability_target(touch: &UiGraphTouchDescriptor) -> UiAdmissionTarget {
    selection_target(touch).with_host_capability_report(
        WorthUiHostCapabilityReport::diagnostic_only(modeled_text_capabilities()),
    )
}

pub fn budget_exceeded_target(touch: &UiGraphTouchDescriptor) -> UiAdmissionTarget {
    selection_target(touch)
        .with_selection_budget(UiAdmissionSelectionBudget::ordinary_lane_budget(0))
}

fn modeled_text_capabilities() -> Vec<WorthUiHostCapability> {
    vec![
        WorthUiHostCapability::TextInput,
        WorthUiHostCapability::TextIntrinsicMeasurement,
    ]
}
