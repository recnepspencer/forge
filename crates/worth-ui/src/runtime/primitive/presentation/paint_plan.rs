use crate::runtime::{
    WorthUiPrimitiveActiveAppearancePlan, WorthUiPrimitiveDrawPlan,
    WorthUiPrimitiveObservedPostureReceipt, WorthUiPrimitiveProofReceipt,
    WorthUiResolvedAppearanceStateReceipt,
};

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiPrimitivePaintPlan {
    draw_plan: WorthUiPrimitiveDrawPlan,
    active_appearance_plan: WorthUiPrimitiveActiveAppearancePlan,
    active_appearance: WorthUiResolvedAppearanceStateReceipt,
    observed_posture: WorthUiPrimitiveObservedPostureReceipt,
}

impl WorthUiPrimitivePaintPlan {
    pub(crate) fn from_receipt(
        receipt: WorthUiPrimitiveProofReceipt,
        available_width: f32,
        available_height: f32,
        observed_posture: WorthUiPrimitiveObservedPostureReceipt,
    ) -> Self {
        let active_appearance_plan =
            WorthUiPrimitiveActiveAppearancePlan::from_receipt(&receipt, observed_posture.clone());
        let active_appearance = active_appearance_plan.active_appearance().clone();
        let draw_plan =
            WorthUiPrimitiveDrawPlan::from_receipt(receipt, available_width, available_height);
        Self {
            draw_plan,
            active_appearance_plan,
            active_appearance,
            observed_posture,
        }
    }

    pub fn draw_plan(&self) -> &WorthUiPrimitiveDrawPlan {
        &self.draw_plan
    }

    pub fn active_appearance(&self) -> &WorthUiResolvedAppearanceStateReceipt {
        &self.active_appearance
    }

    pub fn active_appearance_plan(&self) -> &WorthUiPrimitiveActiveAppearancePlan {
        &self.active_appearance_plan
    }

    pub fn observed_posture(&self) -> &WorthUiPrimitiveObservedPostureReceipt {
        &self.observed_posture
    }
}
