use crate::runtime::WorthUiNodeLifecycleTransition;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiFocusChainReconciliation {
    carries_only_durable_node_identity: bool,
}

impl WorthUiFocusChainReconciliation {
    pub fn preserve_by_durable_identity() -> Self {
        Self {
            carries_only_durable_node_identity: true,
        }
    }

    pub fn carries_only_durable_node_identity(&self) -> bool {
        self.carries_only_durable_node_identity
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
