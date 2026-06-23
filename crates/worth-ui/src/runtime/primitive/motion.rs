use super::WorthUiPrimitiveResolvedMeasurement;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiPrimitiveMotionKind {
    None,
    Transition,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiPrimitiveMotionTarget {
    Background,
    Foreground,
    Radius,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiPrimitiveMotionEasing {
    Linear,
    Standard,
    EaseIn,
    EaseOut,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiPrimitiveMotionReceipt {
    kind: WorthUiPrimitiveMotionKind,
    target: WorthUiPrimitiveMotionTarget,
    duration: WorthUiPrimitiveResolvedMeasurement,
    easing: WorthUiPrimitiveMotionEasing,
}

impl WorthUiPrimitiveMotionReceipt {
    pub(crate) fn new(
        kind: WorthUiPrimitiveMotionKind,
        target: WorthUiPrimitiveMotionTarget,
        duration: WorthUiPrimitiveResolvedMeasurement,
        easing: WorthUiPrimitiveMotionEasing,
    ) -> Self {
        Self {
            kind,
            target,
            duration,
            easing,
        }
    }

    pub fn kind(&self) -> WorthUiPrimitiveMotionKind {
        self.kind
    }

    pub fn target(&self) -> WorthUiPrimitiveMotionTarget {
        self.target
    }

    pub fn duration(&self) -> &WorthUiPrimitiveResolvedMeasurement {
        &self.duration
    }

    pub fn easing(&self) -> WorthUiPrimitiveMotionEasing {
        self.easing
    }
}
