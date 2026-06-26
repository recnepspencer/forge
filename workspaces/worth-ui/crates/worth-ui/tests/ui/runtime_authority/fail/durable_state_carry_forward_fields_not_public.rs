use worth_ui::facade::{
    WorthUiDurableStateCarryForward, WorthUiDurableStateFamilyId,
    WorthUiNodeLifecycleTransition,
};

fn main() {
    let _ = WorthUiDurableStateCarryForward {
        identity_basis: "component:dashboard".to_owned(),
        family_id: WorthUiDurableStateFamilyId::FocusChain,
        transition: WorthUiNodeLifecycleTransition::Preserve,
    };
}
