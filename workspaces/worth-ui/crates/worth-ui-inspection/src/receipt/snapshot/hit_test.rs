use super::{UiVisualIdentityTrace, UiVisualQueryBudget};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiVisualHitTestTarget {
    total_order: u32,
    identity_trace: UiVisualIdentityTrace,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UiVisualHitTestOutcome {
    Target(UiVisualHitTestTarget),
    None,
    Incomplete(UiVisualQueryBudget),
    Unsupported,
}

impl UiVisualHitTestTarget {
    #[doc(hidden)]
    pub const fn from_runtime_projection(
        total_order: u32,
        identity_trace: UiVisualIdentityTrace,
    ) -> Self {
        Self {
            total_order,
            identity_trace,
        }
    }

    pub const fn total_order(&self) -> u32 {
        self.total_order
    }

    pub const fn identity_trace(&self) -> &UiVisualIdentityTrace {
        &self.identity_trace
    }
}
