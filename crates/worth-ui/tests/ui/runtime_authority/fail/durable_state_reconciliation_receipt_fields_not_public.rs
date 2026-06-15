use worth_ui::facade::{
    WorthUiDurableStateFamilyId, WorthUiDurableStateReconciliationOutcome,
    WorthUiDurableStateReconciliationReceipt,
};

fn main() {
    let _ = WorthUiDurableStateReconciliationReceipt {
        identity_basis: "component:dashboard".to_owned(),
        family_id: WorthUiDurableStateFamilyId::FocusChain,
        outcome: WorthUiDurableStateReconciliationOutcome::CarryForward,
        carry_forward: None,
        replacement: None,
    };
}
