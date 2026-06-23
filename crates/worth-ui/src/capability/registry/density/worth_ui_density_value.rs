use crate::capability::{WorthUiLengthValue, WorthUiPaddingValue, WorthUiSpacingValue};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiDensityValue {
    Padding(WorthUiPaddingValue),
    Spacing(WorthUiSpacingValue),
    HitTargetMinimum(WorthUiLengthValue),
    Posture(WorthUiDensityPostureValue),
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum WorthUiDensityPostureValue {
    Compact,
    Comfortable,
    Dense,
}

impl WorthUiDensityValue {
    pub fn padding(value: WorthUiPaddingValue) -> Self {
        Self::Padding(value)
    }

    pub fn spacing(value: WorthUiSpacingValue) -> Self {
        Self::Spacing(value)
    }

    pub fn hit_target_minimum(value: WorthUiLengthValue) -> Self {
        Self::HitTargetMinimum(value)
    }

    pub fn posture(value: WorthUiDensityPostureValue) -> Self {
        Self::Posture(value)
    }

    pub(crate) fn digest_basis(&self) -> String {
        match self {
            Self::Padding(value) => value.digest_basis(),
            Self::Spacing(value) => value.digest_basis(),
            Self::HitTargetMinimum(value) => format!("hit_target:{}", value.digest_basis()),
            Self::Posture(value) => format!("posture:{}", value.digest_basis()),
        }
    }
}

impl WorthUiDensityPostureValue {
    pub fn compact() -> Self {
        Self::Compact
    }

    pub fn comfortable() -> Self {
        Self::Comfortable
    }

    pub fn dense() -> Self {
        Self::Dense
    }

    pub(crate) fn digest_basis(&self) -> &'static str {
        match self {
            Self::Compact => "compact",
            Self::Comfortable => "comfortable",
            Self::Dense => "dense",
        }
    }
}
