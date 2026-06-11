#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PlanarMotionStep {
    ExactTranslation { step_identity: String },
    ExactRotation { step_identity: String },
    Reorientation { posture: PlanarReorientation },
}

impl PlanarMotionStep {
    pub fn exact_translation(step_identity: impl Into<String>) -> Self {
        Self::ExactTranslation {
            step_identity: step_identity.into(),
        }
    }

    pub fn exact_rotation(step_identity: impl Into<String>) -> Self {
        Self::ExactRotation {
            step_identity: step_identity.into(),
        }
    }

    pub fn reorientation(posture: PlanarReorientation) -> Self {
        Self::Reorientation { posture }
    }

    pub(crate) fn kind(&self) -> &'static str {
        match self {
            Self::ExactTranslation { .. } => "exact-translation",
            Self::ExactRotation { .. } => "exact-rotation",
            Self::Reorientation { .. } => "reorientation",
        }
    }

    pub(crate) fn authority_value(&self) -> String {
        match self {
            Self::ExactTranslation { step_identity } | Self::ExactRotation { step_identity } => {
                step_identity.clone()
            }
            Self::Reorientation { posture } => posture.as_str().to_string(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarRotationPosture {
    None,
    ExactRotation,
    ExactCancellation,
}

impl PlanarRotationPosture {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::ExactRotation => "exact-rotation",
            Self::ExactCancellation => "exact-cancellation",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarReorientation {
    PreservesHandedness,
    ReversesHandedness,
}

impl PlanarReorientation {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PreservesHandedness => "preserves-handedness",
            Self::ReversesHandedness => "reverses-handedness",
        }
    }

    pub(crate) const fn invalidates_planar_basis(self) -> bool {
        matches!(self, Self::ReversesHandedness)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarMotionCancellation {
    None,
    ExactBasisReplay,
}

impl PlanarMotionCancellation {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::ExactBasisReplay => "exact-basis-replay",
        }
    }
}
