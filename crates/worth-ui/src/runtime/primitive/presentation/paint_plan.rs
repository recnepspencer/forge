use crate::runtime::{
    WorthUiAppearanceStatePosture, WorthUiPrimitiveDrawPlan, WorthUiPrimitiveProofReceipt,
    WorthUiResolvedAppearanceStateReceipt,
};

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiPrimitiveObservedPostureReceipt {
    posture: WorthUiAppearanceStatePosture,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiPrimitivePaintPlan {
    draw_plan: WorthUiPrimitiveDrawPlan,
    active_appearance: WorthUiResolvedAppearanceStateReceipt,
    observed_posture: WorthUiPrimitiveObservedPostureReceipt,
}

impl WorthUiPrimitiveObservedPostureReceipt {
    pub fn from_renderer_observation(
        hovered: bool,
        pressed: bool,
        focused: bool,
        disabled: bool,
        selected: bool,
    ) -> Self {
        let (hovered, pressed, focused) = if disabled {
            (false, false, false)
        } else {
            (hovered, pressed, focused)
        };
        Self {
            posture: WorthUiAppearanceStatePosture::observed(
                hovered, pressed, focused, disabled, selected,
            ),
        }
    }

    pub fn rest() -> Self {
        Self {
            posture: WorthUiAppearanceStatePosture::rest(),
        }
    }

    pub fn posture(&self) -> WorthUiAppearanceStatePosture {
        self.posture
    }
}

impl WorthUiPrimitivePaintPlan {
    pub(crate) fn from_receipt(
        receipt: WorthUiPrimitiveProofReceipt,
        available_width: f32,
        available_height: f32,
        observed_posture: WorthUiPrimitiveObservedPostureReceipt,
    ) -> Self {
        let active_appearance = receipt
            .appearance_state()
            .resolve_active(observed_posture.posture());
        let draw_plan =
            WorthUiPrimitiveDrawPlan::from_receipt(receipt, available_width, available_height);
        Self {
            draw_plan,
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

    pub fn observed_posture(&self) -> &WorthUiPrimitiveObservedPostureReceipt {
        &self.observed_posture
    }
}
