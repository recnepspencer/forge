use crate::runtime::WorthUiTransientInteractionPolicy;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum WorthUiTransientInteractionState {
    Hover,
    Pressed,
    DragCapture,
    PointerCapture,
    AnimationTick,
    InFlightGesture,
}

impl WorthUiTransientInteractionState {
    pub fn all() -> &'static [Self] {
        &[
            Self::Hover,
            Self::Pressed,
            Self::DragCapture,
            Self::PointerCapture,
            Self::AnimationTick,
            Self::InFlightGesture,
        ]
    }

    pub fn default_policy(self) -> WorthUiTransientInteractionPolicy {
        WorthUiTransientInteractionPolicy::Drop
    }

    pub fn drops_by_default(self) -> bool {
        self.default_policy() == WorthUiTransientInteractionPolicy::Drop
    }
}
