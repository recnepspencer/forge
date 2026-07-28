use super::{UiClientPhysicalRect, UiVisualIdentityTrace, UiVisualQueryBudget};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiVisualVisibleContributor {
    region: UiClientPhysicalRect,
    layer_order: u32,
    paint_order: u32,
    identity_trace: UiVisualIdentityTrace,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiVisualContributorStack {
    contributors: Box<[UiVisualVisibleContributor]>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UiVisualVisibleOutcome {
    Contributors(UiVisualContributorStack),
    None,
    Incomplete(UiVisualQueryBudget),
    Unsupported,
}

impl UiVisualVisibleContributor {
    #[doc(hidden)]
    pub const fn from_runtime_projection(
        region: UiClientPhysicalRect,
        layer_order: u32,
        paint_order: u32,
        identity_trace: UiVisualIdentityTrace,
    ) -> Self {
        Self {
            region,
            layer_order,
            paint_order,
            identity_trace,
        }
    }

    pub const fn region(&self) -> UiClientPhysicalRect {
        self.region
    }

    pub const fn layer_order(&self) -> u32 {
        self.layer_order
    }

    pub const fn paint_order(&self) -> u32 {
        self.paint_order
    }

    pub const fn identity_trace(&self) -> &UiVisualIdentityTrace {
        &self.identity_trace
    }
}

impl UiVisualContributorStack {
    #[doc(hidden)]
    pub fn from_runtime_projection(contributors: Vec<UiVisualVisibleContributor>) -> Self {
        Self {
            contributors: contributors.into_boxed_slice(),
        }
    }

    pub fn contributors(&self) -> &[UiVisualVisibleContributor] {
        &self.contributors
    }

    pub fn frontmost(&self) -> Option<&UiVisualVisibleContributor> {
        self.contributors.first()
    }
}
