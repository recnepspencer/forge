#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum UiMotionPropertyChannel {
    Opacity,
    TranslationX,
    TranslationY,
    Geometry,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct UiMotionPropertyChannels(u8);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiMotionEasing {
    Linear,
    EaseOutCubic,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiMotionFillPolicy {
    FinalState,
    ExitRetention,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiMotionInterruptionPolicy {
    RetargetFromCurrentSample,
    RestartFromSemanticPredecessor,
    FinishThenApply,
    SnapToTarget,
    CancelDrop,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiMotionReducedMotionPolicy {
    SystemRespecting,
    PreserveNecessary,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct UiMotionDeclaration {
    channels: UiMotionPropertyChannels,
    easing: UiMotionEasing,
    duration_ticks: u32,
    delay_ticks: u32,
    fill: UiMotionFillPolicy,
    interruption: UiMotionInterruptionPolicy,
    reduced_motion: UiMotionReducedMotionPolicy,
    decorative: bool,
}

impl UiMotionPropertyChannels {
    pub(in crate::runtime) const fn one(channel: UiMotionPropertyChannel) -> Self {
        Self(bit(channel))
    }

    pub(in crate::runtime) const fn with(mut self, channel: UiMotionPropertyChannel) -> Self {
        self.0 |= bit(channel);
        self
    }

    pub(crate) const fn contains(self, channel: UiMotionPropertyChannel) -> bool {
        self.0 & bit(channel) != 0
    }

    pub(in crate::runtime) const fn bits(self) -> u8 {
        self.0
    }
}

impl UiMotionDeclaration {
    pub(crate) const fn portal_entrance() -> Self {
        Self {
            channels: UiMotionPropertyChannels::one(UiMotionPropertyChannel::Opacity)
                .with(UiMotionPropertyChannel::TranslationY),
            easing: UiMotionEasing::EaseOutCubic,
            duration_ticks: 140,
            delay_ticks: 0,
            fill: UiMotionFillPolicy::FinalState,
            interruption: UiMotionInterruptionPolicy::RetargetFromCurrentSample,
            reduced_motion: UiMotionReducedMotionPolicy::SystemRespecting,
            decorative: true,
        }
    }

    pub(crate) const fn portal_exit() -> Self {
        Self {
            channels: UiMotionPropertyChannels::one(UiMotionPropertyChannel::Opacity),
            easing: UiMotionEasing::EaseOutCubic,
            duration_ticks: 110,
            delay_ticks: 0,
            fill: UiMotionFillPolicy::ExitRetention,
            interruption: UiMotionInterruptionPolicy::RetargetFromCurrentSample,
            reduced_motion: UiMotionReducedMotionPolicy::SystemRespecting,
            decorative: true,
        }
    }

    pub(crate) const fn rebind_geometry() -> Self {
        Self {
            channels: UiMotionPropertyChannels::one(UiMotionPropertyChannel::Geometry),
            easing: UiMotionEasing::EaseOutCubic,
            duration_ticks: 160,
            delay_ticks: 0,
            fill: UiMotionFillPolicy::FinalState,
            interruption: UiMotionInterruptionPolicy::RetargetFromCurrentSample,
            reduced_motion: UiMotionReducedMotionPolicy::SystemRespecting,
            decorative: false,
        }
    }

    pub(crate) const fn channels(self) -> UiMotionPropertyChannels {
        self.channels
    }

    pub(crate) const fn easing(self) -> UiMotionEasing {
        self.easing
    }

    pub(crate) const fn duration_ticks(self) -> u32 {
        self.duration_ticks
    }

    pub(crate) const fn delay_ticks(self) -> u32 {
        self.delay_ticks
    }

    pub(in crate::runtime) const fn fill(self) -> UiMotionFillPolicy {
        self.fill
    }

    pub(in crate::runtime) const fn interruption(self) -> UiMotionInterruptionPolicy {
        self.interruption
    }

    pub(crate) const fn reduced_motion(self) -> UiMotionReducedMotionPolicy {
        self.reduced_motion
    }

    pub(crate) const fn decorative(self) -> bool {
        self.decorative
    }

    #[cfg(test)]
    pub(crate) const fn with_interruption(
        mut self,
        interruption: UiMotionInterruptionPolicy,
    ) -> Self {
        self.interruption = interruption;
        self
    }
}

const fn bit(channel: UiMotionPropertyChannel) -> u8 {
    1 << channel as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn portal_defaults_are_system_respecting_and_exit_retention_is_explicit() {
        let entrance = UiMotionDeclaration::portal_entrance();
        assert!(entrance
            .channels()
            .contains(UiMotionPropertyChannel::Opacity));
        assert!(entrance
            .channels()
            .contains(UiMotionPropertyChannel::TranslationY));
        assert_eq!(
            entrance.reduced_motion(),
            UiMotionReducedMotionPolicy::SystemRespecting
        );
        assert_eq!(
            UiMotionDeclaration::portal_exit().fill(),
            UiMotionFillPolicy::ExitRetention
        );
    }
}
