use super::{MutationOwnershipDenial, NamespaceConfinementDenial, OwnershipReleaseOutcome};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum MediaOwnerAdmissionEffectFate {
    DeniedBeforeEffect,
    EffectPossible,
}

impl MediaOwnerAdmissionEffectFate {
    pub(super) const fn combine(self, later: Self) -> Self {
        if matches!(self, Self::EffectPossible) || matches!(later, Self::EffectPossible) {
            Self::EffectPossible
        } else {
            Self::DeniedBeforeEffect
        }
    }

    pub(super) const fn effect_possible(self) -> bool {
        matches!(self, Self::EffectPossible)
    }
}

pub(super) struct NamespaceAdmissionFailure {
    denial: NamespaceConfinementDenial,
    effect_fate: MediaOwnerAdmissionEffectFate,
}

impl NamespaceAdmissionFailure {
    pub(super) const fn new(
        denial: NamespaceConfinementDenial,
        effect_fate: MediaOwnerAdmissionEffectFate,
    ) -> Self {
        Self {
            denial,
            effect_fate,
        }
    }

    pub(super) const fn denial(&self) -> NamespaceConfinementDenial {
        self.denial
    }

    pub(super) const fn effect_fate(&self) -> MediaOwnerAdmissionEffectFate {
        self.effect_fate
    }
}

pub(super) struct MutationOwnershipAcquisitionFailure {
    denial: MutationOwnershipDenial,
    effect_fate: MediaOwnerAdmissionEffectFate,
    release: Option<OwnershipReleaseOutcome>,
}

impl MutationOwnershipAcquisitionFailure {
    pub(super) const fn new(
        denial: MutationOwnershipDenial,
        effect_fate: MediaOwnerAdmissionEffectFate,
        release: Option<OwnershipReleaseOutcome>,
    ) -> Self {
        Self {
            denial,
            effect_fate,
            release,
        }
    }

    pub(super) const fn before_effect(denial: MutationOwnershipDenial) -> Self {
        Self::new(
            denial,
            MediaOwnerAdmissionEffectFate::DeniedBeforeEffect,
            None,
        )
    }

    pub(super) const fn denial(&self) -> MutationOwnershipDenial {
        self.denial
    }

    pub(super) const fn effect_fate(&self) -> MediaOwnerAdmissionEffectFate {
        self.effect_fate
    }

    pub(super) const fn release(&self) -> Option<OwnershipReleaseOutcome> {
        self.release
    }
}
