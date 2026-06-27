use worth_ui::facade::{
    WorthUiDurableStateFamilyId, WorthUiDurableStateReconciliationOutcome,
    WorthUiDurableStateReplacement, WorthUiNodeLifecycleTransition,
};

fn main() {
    let _ = WorthUiDurableStateReplacement {
        identity_basis: "component:dashboard".to_owned(),
        family_id: WorthUiDurableStateFamilyId::FocusChain,
        transition: WorthUiNodeLifecycleTransition::Replace,
        outcome: WorthUiDurableStateReconciliationOutcome::Drop,
        reason: "forged",
    };
}
