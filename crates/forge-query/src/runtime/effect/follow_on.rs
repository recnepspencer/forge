use crate::evidence_identity::{
    ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryEffectWriteAdjacentTriggerClass {
    OrdinaryEffect,
    TimeOnlyWake,
    AsyncCompletion,
    MixedCause,
    ReplayDrift,
    RemaskDrift,
    PreviewCrossedResidue,
    StaleCompletion,
}

impl ForgeQueryEffectWriteAdjacentTriggerClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::OrdinaryEffect => "ordinary-effect",
            Self::TimeOnlyWake => "time-only-wake",
            Self::AsyncCompletion => "async-completion",
            Self::MixedCause => "mixed-cause",
            Self::ReplayDrift => "replay-drift",
            Self::RemaskDrift => "remask-drift",
            Self::PreviewCrossedResidue => "preview-crossed-residue",
            Self::StaleCompletion => "stale-completion",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryEffectWriteAdjacentTrigger {
    class: ForgeQueryEffectWriteAdjacentTriggerClass,
    origin_identity: ForgeQueryEvidenceIdentity,
    identity: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryEffectWriteAdjacentTrigger {
    pub fn ordinary(effect_name: &str) -> Self {
        let origin_identity =
            ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::EffectIntentReceiptPhase)
                .field_shape(
                    ForgeQueryEvidenceTag::new("identity_family"),
                    "ordinary_effect_origin_v1",
                )
                .field_value(ForgeQueryEvidenceTag::new("effect"), effect_name)
                .seal();
        Self::new(
            ForgeQueryEffectWriteAdjacentTriggerClass::OrdinaryEffect,
            origin_identity,
        )
    }

    pub fn new(
        class: ForgeQueryEffectWriteAdjacentTriggerClass,
        origin_identity: ForgeQueryEvidenceIdentity,
    ) -> Self {
        let identity =
            ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::EffectIntentReceiptPhase)
                .field_shape(
                    ForgeQueryEvidenceTag::new("identity_family"),
                    "forge_query_effect_write_adjacent_trigger_v1",
                )
                .field_shape(ForgeQueryEvidenceTag::new("class"), class.as_str())
                .field_evidence_identity(ForgeQueryEvidenceTag::new("origin"), &origin_identity)
                .seal();
        Self {
            class,
            origin_identity,
            identity,
        }
    }

    pub fn class(&self) -> ForgeQueryEffectWriteAdjacentTriggerClass {
        self.class
    }

    pub fn origin_evidence_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.origin_identity
    }

    pub fn digest(&self) -> &str {
        self.identity.as_str()
    }

    pub fn identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.identity
    }
}
