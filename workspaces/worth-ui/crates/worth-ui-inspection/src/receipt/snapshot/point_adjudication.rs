use super::{
    UiVisualHitTestOutcome, UiVisualInspectionCostReceipt, UiVisualQueryBudget,
    UiVisualVisibleOutcome,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiVisualPointAdjudication {
    visible: UiVisualVisibleOutcome,
    hit_test: UiVisualHitTestOutcome,
    budget: UiVisualQueryBudget,
    cost: UiVisualInspectionCostReceipt,
}

impl UiVisualPointAdjudication {
    #[doc(hidden)]
    pub const fn from_runtime_projection(
        visible: UiVisualVisibleOutcome,
        hit_test: UiVisualHitTestOutcome,
        budget: UiVisualQueryBudget,
        cost: UiVisualInspectionCostReceipt,
    ) -> Self {
        Self {
            visible,
            hit_test,
            budget,
            cost,
        }
    }

    pub const fn visible(&self) -> &UiVisualVisibleOutcome {
        &self.visible
    }

    pub const fn hit_test(&self) -> &UiVisualHitTestOutcome {
        &self.hit_test
    }

    pub const fn budget(&self) -> UiVisualQueryBudget {
        self.budget
    }

    pub const fn cost(&self) -> UiVisualInspectionCostReceipt {
        self.cost
    }
}
