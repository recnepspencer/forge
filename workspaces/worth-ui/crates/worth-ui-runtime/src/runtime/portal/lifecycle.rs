#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiPortalLifecyclePosture {
    Open,
    Visible,
    Closing,
    Closed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiPortalDismissalCause {
    Escape,
    OutsidePress,
    AcceptedSelection,
    ExplicitOwnerRequest,
    AnchorLoss,
    ParentClosed,
    #[cfg(test)]
    OwnerLoss,
    ApplicationShutdown,
    #[cfg(test)]
    WindowFocusPolicy,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiPortalInputShielding {
    ContentBounds,
    ModalSurface,
}
