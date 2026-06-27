use worth_ui::facade::WorthUiPlanSwapDenialReason;

fn main() {
    let _ = WorthUiPlanSwapDenialReason::InjectedFailureBeforeCommit;
    let _ = WorthUiPlanSwapDenialReason::InjectedFailureAfterArtifactMutation;
}
