use crate::runtime::WorthUiNodeLifecycleTransition;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiSplitterPositionReconciliation {
    reconciles_on_stable_identity: bool,
}

impl WorthUiSplitterPositionReconciliation {
    pub fn stable_identity() -> Self {
        Self {
            reconciles_on_stable_identity: true,
        }
    }

    pub fn reconciles_on_stable_identity(&self) -> bool {
        self.reconciles_on_stable_identity
    }

    pub(crate) fn allows_carry_for_transition(transition: WorthUiNodeLifecycleTransition) -> bool {
        matches!(
            transition,
            WorthUiNodeLifecycleTransition::Preserve
                | WorthUiNodeLifecycleTransition::Move
                | WorthUiNodeLifecycleTransition::Rebind
        )
    }
}
