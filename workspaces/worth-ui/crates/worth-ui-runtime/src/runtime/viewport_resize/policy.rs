#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiViewportReceiptCommitStrategy {
    CoalescedPerResolvedFrame,
    ThresholdPerResolvedFrame,
    ImmediatePerResolvedFrame,
    TerminalPerResolvedFrame,
}

impl UiViewportReceiptCommitStrategy {
    pub(crate) fn from_resolved_policy(
        policy: crate::runtime::UiResolvedAllocationStreamPolicy,
    ) -> Self {
        match policy.cadence() {
            crate::runtime::UiAllocationCadenceKind::CoalescedWindow => {
                Self::CoalescedPerResolvedFrame
            }
            crate::runtime::UiAllocationCadenceKind::Threshold => Self::ThresholdPerResolvedFrame,
            crate::runtime::UiAllocationCadenceKind::Immediate => Self::ImmediatePerResolvedFrame,
            crate::runtime::UiAllocationCadenceKind::Terminal => Self::TerminalPerResolvedFrame,
        }
    }
}
