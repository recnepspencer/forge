use super::PlanarMotionPostureCounters;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarMotionPostureDenialKind {
    MissingBooleanReadinessReceipt,
    MissingMotionStep,
    CoordinateOnlyMotionBasis,
    OrientationFlipInvalidatesPlanarBasis,
    ExactCancellationMissingRotation,
}

impl PlanarMotionPostureDenialKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MissingBooleanReadinessReceipt => "missing-boolean-readiness-receipt",
            Self::MissingMotionStep => "missing-motion-step",
            Self::CoordinateOnlyMotionBasis => "coordinate-only-motion-basis",
            Self::OrientationFlipInvalidatesPlanarBasis => {
                "orientation-flip-invalidates-planar-basis"
            }
            Self::ExactCancellationMissingRotation => "exact-cancellation-missing-rotation",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarMotionPostureDenial {
    kind: PlanarMotionPostureDenialKind,
    reason: String,
    counters: PlanarMotionPostureCounters,
}

impl PlanarMotionPostureDenial {
    pub(crate) fn new(kind: PlanarMotionPostureDenialKind, reason: impl Into<String>) -> Self {
        let counters = match kind {
            PlanarMotionPostureDenialKind::CoordinateOnlyMotionBasis => {
                PlanarMotionPostureCounters::rejected_coordinate_only()
            }
            PlanarMotionPostureDenialKind::OrientationFlipInvalidatesPlanarBasis => {
                PlanarMotionPostureCounters::rejected_orientation_flip()
            }
            _ => PlanarMotionPostureCounters::certified(0, 0, 0, 0),
        };
        Self {
            kind,
            reason: reason.into(),
            counters,
        }
    }

    pub fn kind(&self) -> PlanarMotionPostureDenialKind {
        self.kind
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }

    pub fn counters(&self) -> PlanarMotionPostureCounters {
        self.counters
    }
}
