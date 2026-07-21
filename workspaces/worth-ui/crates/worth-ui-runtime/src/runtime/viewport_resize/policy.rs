#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// Commit posture for the single receipt emitted by a resolved viewport frame.
pub enum UiViewportReceiptCommitStrategy {
    Coalesced,
    Threshold,
    Immediate,
    Terminal,
}

impl UiViewportReceiptCommitStrategy {
    pub(crate) fn from_resolved_policy(
        policy: crate::runtime::UiResolvedAllocationStreamPolicy,
    ) -> Self {
        match policy.cadence() {
            crate::runtime::UiAllocationCadenceKind::CoalescedWindow => Self::Coalesced,
            crate::runtime::UiAllocationCadenceKind::Threshold => Self::Threshold,
            crate::runtime::UiAllocationCadenceKind::Immediate => Self::Immediate,
            crate::runtime::UiAllocationCadenceKind::Terminal => Self::Terminal,
        }
    }
}
