use crate::identity::hash_parts;

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
    origin_identity: String,
    digest: String,
}

impl ForgeQueryEffectWriteAdjacentTrigger {
    pub fn ordinary(effect_name: &str) -> Self {
        Self::new(
            ForgeQueryEffectWriteAdjacentTriggerClass::OrdinaryEffect,
            format!("effect:{effect_name}:ordinary"),
        )
    }

    pub fn new(
        class: ForgeQueryEffectWriteAdjacentTriggerClass,
        origin_identity: impl Into<String>,
    ) -> Self {
        let origin_identity = origin_identity.into();
        let digest = hash_parts(&[
            "forge_query_effect_write_adjacent_trigger_v1".to_string(),
            format!("class:{}", class.as_str()),
            format!("origin:{origin_identity}"),
        ]);
        Self {
            class,
            origin_identity,
            digest,
        }
    }

    pub fn class(&self) -> ForgeQueryEffectWriteAdjacentTriggerClass {
        self.class
    }

    pub fn origin_identity(&self) -> &str {
        &self.origin_identity
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }
}
