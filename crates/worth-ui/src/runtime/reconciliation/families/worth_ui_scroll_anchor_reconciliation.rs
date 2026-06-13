use crate::runtime::WorthUiNodeLifecycleTransition;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiScrollAnchorReconciliation {
    requires_stable_anchor_identity: bool,
    rejects_offset_only_carry: bool,
}

impl WorthUiScrollAnchorReconciliation {
    pub fn stable_anchor_identity() -> Self {
        Self {
            requires_stable_anchor_identity: true,
            rejects_offset_only_carry: true,
        }
    }

    pub fn requires_stable_anchor_identity(&self) -> bool {
        self.requires_stable_anchor_identity
    }

    pub fn rejects_offset_only_carry(&self) -> bool {
        self.rejects_offset_only_carry
    }

    pub(crate) fn allows_carry_for_transition(transition: WorthUiNodeLifecycleTransition) -> bool {
        matches!(
            transition,
            WorthUiNodeLifecycleTransition::Preserve | WorthUiNodeLifecycleTransition::Move
        )
    }
}
