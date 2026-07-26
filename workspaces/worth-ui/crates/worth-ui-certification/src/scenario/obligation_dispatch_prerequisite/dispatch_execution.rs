//! Obligation selection, lowering, and verdict execution for one target.

use worth_ui::facade::admission::{UiAdmissionTarget, WorthUiAdmissionExt};
use worth_ui::facade::app::WorthUiApp;
use worth_ui::facade::graph::UiGraphTouchDescriptor;
use worth_ui_runtime::facade::obligations::{
    UiObligationDispatchPlan, UiObligationVerdict, UiSelectedObligationSet,
};

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
