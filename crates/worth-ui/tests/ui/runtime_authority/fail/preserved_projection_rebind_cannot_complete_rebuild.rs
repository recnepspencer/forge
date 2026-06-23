use worth_ui::facade::{
    WorthUiAdmittedProjectionPlan, WorthUiHeaderMenuPlan, WorthUiPreservedProjectionRebindPlan,
};

fn main() {}

fn cannot_rebuild_from_preserve_only_plan(
    preserved: WorthUiPreservedProjectionRebindPlan<WorthUiHeaderMenuPlan>,
    rebound: WorthUiAdmittedProjectionPlan<WorthUiHeaderMenuPlan>,
) {
    let _ = preserved.complete_rebuild(rebound);
}
