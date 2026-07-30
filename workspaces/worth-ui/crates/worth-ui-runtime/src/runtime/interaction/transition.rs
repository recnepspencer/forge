#[derive(Debug)]
pub enum UiInteractionStop {
    PointerGesture(super::UiPointerGestureStop),
    LocalInput(super::UiLocalInputStop),
}

#[derive(Debug)]
pub enum UiInteractionTransition {
    PointerPressed(super::UiPointerGesturePressReceipt),
    DraftMutation(super::UiDraftMutationReceipt),
    Semantic(super::UiSemanticInteraction),
    Stopped(UiInteractionStop),
}

impl UiInteractionStop {
    pub const fn pointer_gesture(&self) -> Option<&super::UiPointerGestureStop> {
        match self {
            Self::PointerGesture(stop) => Some(stop),
            Self::LocalInput(_) => None,
        }
    }

    pub const fn local_input(&self) -> Option<&super::UiLocalInputStop> {
        match self {
            Self::PointerGesture(_) => None,
            Self::LocalInput(stop) => Some(stop),
        }
    }
}
