use worth_ui::facade::admission::{
    UiAdmissionQueryBasis, UiAdmissionSelectionBudget, UiAdmissionTarget, UiAdmissionWorld,
};
use worth_ui::facade::app::WorthUiApp;
use worth_ui::facade::graph::UiGraphTouchDescriptor;
use worth_ui_host_contract::{WorthUiHostCapabilityReport, WorthUiHostContract};
use worth_ui_runtime::facade::obligations::{
    UiObligationDispatchPlan, UiObligationVerdict, UiSelectedObligationSet,
};

use super::query_support::query_prerequisites;

pub struct DispatchExecutionBundle {
    pub selected: UiSelectedObligationSet,
    pub dispatch: UiObligationDispatchPlan,
    pub verdicts: Box<[UiObligationVerdict]>,
}

pub fn execute_for_target(
    app: &WorthUiApp,
    touch: &UiGraphTouchDescriptor,
    target: UiAdmissionTarget,
) -> DispatchExecutionBundle {
    let selected = app.admission().select_obligations_for_target(touch, target);
    let dispatch = app.admission().lower_obligation_dispatch(&selected);
    let verdicts = dispatch.execute();

    DispatchExecutionBundle {
        selected,
        dispatch,
        verdicts,
    }
}

pub fn selection_target(touch: &UiGraphTouchDescriptor) -> UiAdmissionTarget {
    UiAdmissionTarget::graph_node(
        touch.target().graph_node_identity(),
        UiAdmissionWorld::from_graph_world_profile(touch.world().world_profile().clone()),
    )
}

pub fn wrong_query_basis_target(touch: &UiGraphTouchDescriptor) -> UiAdmissionTarget {
    selection_target(touch).with_query_prerequisites(query_prerequisites(
        touch,
        UiAdmissionQueryBasis::WrongWorldProjection,
    ))
}

pub fn graph_aligned_query_target(touch: &UiGraphTouchDescriptor) -> UiAdmissionTarget {
    selection_target(touch).with_query_prerequisites(query_prerequisites(
        touch,
        UiAdmissionQueryBasis::GraphAligned,
    ))
}

pub fn stale_query_basis_target(touch: &UiGraphTouchDescriptor) -> UiAdmissionTarget {
    selection_target(touch).with_query_prerequisites(query_prerequisites(
        touch,
        UiAdmissionQueryBasis::StaleReceipt,
    ))
}

pub fn ambiguous_query_basis_target(touch: &UiGraphTouchDescriptor) -> UiAdmissionTarget {
    selection_target(touch).with_query_prerequisites(query_prerequisites(
        touch,
        UiAdmissionQueryBasis::AmbiguousSources,
    ))
}

pub fn missing_host_capability_target(touch: &UiGraphTouchDescriptor) -> UiAdmissionTarget {
    selection_target(touch).with_host_capability_report(WorthUiHostCapabilityReport::from_contract(
        WorthUiHostContract::headless(),
    ))
}

pub fn ambiguous_host_capability_target(touch: &UiGraphTouchDescriptor) -> UiAdmissionTarget {
    selection_target(touch).with_host_capability_report(WorthUiHostCapabilityReport::from_contract(
        WorthUiHostContract::capability_probe_inconclusive(),
    ))
}

pub fn available_host_capability_target(touch: &UiGraphTouchDescriptor) -> UiAdmissionTarget {
    selection_target(touch).with_host_capability_report(WorthUiHostCapabilityReport::from_contract(
        WorthUiHostContract::egui(),
    ))
}

pub fn diagnostic_only_host_capability_target(touch: &UiGraphTouchDescriptor) -> UiAdmissionTarget {
    selection_target(touch).with_host_capability_report(WorthUiHostCapabilityReport::from_contract(
        WorthUiHostContract::diagnostics_only(),
    ))
}

pub fn budget_exceeded_target(touch: &UiGraphTouchDescriptor) -> UiAdmissionTarget {
    selection_target(touch)
        .with_selection_budget(UiAdmissionSelectionBudget::ordinary_lane_budget(0))
}
