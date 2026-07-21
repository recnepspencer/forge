use crate::runtime::WorthUiNodeLifecycleTransition;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiScrollAnchorReconciliation;

impl WorthUiScrollAnchorReconciliation {
    pub(crate) fn allows_carry_for_transition(transition: WorthUiNodeLifecycleTransition) -> bool {
        matches!(
            transition,
            WorthUiNodeLifecycleTransition::Preserve | WorthUiNodeLifecycleTransition::Move
        )
    }
}
