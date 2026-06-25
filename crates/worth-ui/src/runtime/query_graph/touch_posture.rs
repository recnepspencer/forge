#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiPrimitiveEventGraphDispatchPosture {
    NoHit,
    EnabledHit,
    DisabledHit,
    Bubbled,
    Captured,
    Denied,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiPrimitiveContentGraphPosture {
    Accepted,
    NativeVector,
    FallbackEligible,
    UnsupportedCapability,
    Denied,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiLiveViewStateBindingGraphPosture {
    Admitted,
    MissingTarget,
    IncompatibleState,
    ReadOnlyWrite,
    EffectIntentDenied,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiLiveViewControlProjectionGraphPosture {
    Admitted,
    UnsupportedKind,
    UnsupportedOptionSource,
    IncompatibleReplacement,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiLiveViewConditionalProjectionGraphPosture {
    Admitted,
    UnsupportedCondition,
    UnsupportedParticipation,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiLiveViewEffectIntentGraphPosture {
    Supported,
    Unsupported,
}

impl WorthUiLiveViewStateBindingGraphPosture {
    pub(crate) fn has_bound_target(self) -> bool {
        !matches!(self, Self::MissingTarget)
    }

    pub(crate) fn has_compatible_state(self) -> bool {
        !matches!(self, Self::IncompatibleState)
    }

    pub(crate) fn has_write_posture(self) -> bool {
        !matches!(self, Self::ReadOnlyWrite)
    }

    pub(crate) fn has_effect_intent(self) -> bool {
        !matches!(self, Self::EffectIntentDenied)
    }
}

impl WorthUiLiveViewControlProjectionGraphPosture {
    pub(crate) fn has_supported_kind(self) -> bool {
        !matches!(self, Self::UnsupportedKind)
    }

    pub(crate) fn has_supported_options(self) -> bool {
        !matches!(self, Self::UnsupportedOptionSource)
    }

    pub(crate) fn has_compatible_replacement(self) -> bool {
        !matches!(self, Self::IncompatibleReplacement)
    }
}

impl WorthUiLiveViewConditionalProjectionGraphPosture {
    pub(crate) fn has_supported_condition(self) -> bool {
        !matches!(self, Self::UnsupportedCondition)
    }

    pub(crate) fn has_supported_participation(self) -> bool {
        !matches!(self, Self::UnsupportedParticipation)
    }
}

impl WorthUiLiveViewEffectIntentGraphPosture {
    pub(crate) fn has_supported_effect_intent(self) -> bool {
        matches!(self, Self::Supported)
    }
}
