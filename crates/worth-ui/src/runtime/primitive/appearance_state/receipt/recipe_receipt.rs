use super::{
    WorthUiAppearanceStateFieldSet, WorthUiAppearanceStateName, WorthUiAppearanceStatePosture,
    WorthUiResolvedAppearanceStateReceipt,
};

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiStatefulAppearanceRecipeReceipt {
    rest: WorthUiAppearanceStateFieldSet,
    hover: WorthUiAppearanceStateFieldSet,
    pressed: WorthUiAppearanceStateFieldSet,
    focus: WorthUiAppearanceStateFieldSet,
    disabled: WorthUiAppearanceStateFieldSet,
    selected: WorthUiAppearanceStateFieldSet,
    receipt_digest: u64,
}

impl WorthUiStatefulAppearanceRecipeReceipt {
    pub(crate) fn new(
        rest: WorthUiAppearanceStateFieldSet,
        hover: WorthUiAppearanceStateFieldSet,
        pressed: WorthUiAppearanceStateFieldSet,
        focus: WorthUiAppearanceStateFieldSet,
        disabled: WorthUiAppearanceStateFieldSet,
        selected: WorthUiAppearanceStateFieldSet,
        receipt_digest: u64,
    ) -> Self {
        Self {
            rest,
            hover,
            pressed,
            focus,
            disabled,
            selected,
            receipt_digest,
        }
    }

    pub fn resolve_active(
        &self,
        posture: WorthUiAppearanceStatePosture,
    ) -> WorthUiResolvedAppearanceStateReceipt {
        let mut active = self.rest.clone();
        let active_states = posture.active_states();
        for state in active_states.iter().skip(1) {
            match state {
                WorthUiAppearanceStateName::Rest => {}
                WorthUiAppearanceStateName::Selected => active.overlay(&self.selected),
                WorthUiAppearanceStateName::Hover => active.overlay(&self.hover),
                WorthUiAppearanceStateName::Pressed => active.overlay(&self.pressed),
                WorthUiAppearanceStateName::Focus => active.overlay(&self.focus),
                WorthUiAppearanceStateName::Disabled => {
                    active.overlay(&self.disabled);
                    break;
                }
            }
        }
        WorthUiResolvedAppearanceStateReceipt::from_fields(active_states, active)
    }

    pub fn resolve_rest(&self) -> WorthUiResolvedAppearanceStateReceipt {
        WorthUiResolvedAppearanceStateReceipt::from_fields(
            vec![WorthUiAppearanceStateName::Rest],
            self.rest.clone(),
        )
    }

    pub fn rest(&self) -> &WorthUiAppearanceStateFieldSet {
        &self.rest
    }

    pub fn receipt_digest(&self) -> u64 {
        self.receipt_digest
    }
}
