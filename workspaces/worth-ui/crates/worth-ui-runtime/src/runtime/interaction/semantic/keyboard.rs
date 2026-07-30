pub(crate) struct UiKeyboardSemanticInput {
    pub(crate) target: crate::runtime::interaction::UiPresentedInteractionTargetView,
    pub(crate) presentation: worth_ui_host_contract::UiHostObservationPresentationBasis,
    pub(crate) sequence: worth_ui_host_contract::UiHostObservationSequence,
    pub(crate) key: worth_ui_host_contract::UiHostKey,
    pub(crate) modifiers: worth_ui_host_contract::UiHostKeyboardModifiers,
}
