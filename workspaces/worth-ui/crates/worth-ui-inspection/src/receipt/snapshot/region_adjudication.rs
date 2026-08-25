use super::{
    UiClientPhysicalRect, UiVisualIdentityTrace, UiVisualInspectionCostReceipt, UiVisualQueryBudget,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiVisualRegionIntersection {
    region: UiClientPhysicalRect,
    identity_trace: UiVisualIdentityTrace,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiVisualRegionCompleteness {
    Complete,
    EmptyAndComplete,
    Truncated,
    Unsupported,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiVisualRegionAdjudication {
    intersections: Box<[UiVisualRegionIntersection]>,
    completeness: UiVisualRegionCompleteness,
    budget: UiVisualQueryBudget,
    cost: UiVisualInspectionCostReceipt,
}

impl UiVisualRegionAdjudication {
    #[doc(hidden)]
    pub fn from_runtime_projection(
        intersections: Vec<UiVisualRegionIntersection>,
        completeness: UiVisualRegionCompleteness,
        budget: UiVisualQueryBudget,
        cost: UiVisualInspectionCostReceipt,
    ) -> Self {
        Self {
            intersections: intersections.into_boxed_slice(),
            completeness,
            budget,
            cost,
        }
    }

    pub fn intersections(&self) -> &[UiVisualRegionIntersection] {
        &self.intersections
    }

    pub const fn completeness(&self) -> UiVisualRegionCompleteness {
        self.completeness
    }

    pub const fn budget(&self) -> UiVisualQueryBudget {
        self.budget
    }

    pub const fn cost(&self) -> UiVisualInspectionCostReceipt {
        self.cost
    }
}

impl UiVisualRegionIntersection {
    #[doc(hidden)]
    pub const fn from_runtime_projection(
        region: UiClientPhysicalRect,
        identity_trace: UiVisualIdentityTrace,
    ) -> Self {
        Self {
            region,
            identity_trace,
        }
    }

    pub const fn region(&self) -> UiClientPhysicalRect {
        self.region
    }

    pub const fn identity_trace(&self) -> &UiVisualIdentityTrace {
        &self.identity_trace
    }
}
