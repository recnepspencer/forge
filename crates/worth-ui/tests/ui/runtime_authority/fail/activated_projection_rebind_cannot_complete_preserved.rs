use worth_ui::facade::{WorthUiActivatedProjectionRebindPlan, WorthUiHeaderMenuPlan};

fn main() {}

fn cannot_preserve_from_activated_plan(
    activated: WorthUiActivatedProjectionRebindPlan<WorthUiHeaderMenuPlan>,
) {
    let _ = activated.complete_preserved();
}
