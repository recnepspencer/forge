#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiInteractionOperabilityPosture {
    Eligible,
    Disabled,
    ReadinessDisabled,
    Readonly,
    Inert,
    Unsupported,
    Denied,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiInteractionOperabilityBasis {
    Enabled,
    PrimitiveDisabled,
    InteractionReadinessDisabled,
    UnsupportedCommandTarget,
    NonFocusableTarget,
    GestureMismatch,
    UnsupportedInteraction,
    GraphDenied,
}
