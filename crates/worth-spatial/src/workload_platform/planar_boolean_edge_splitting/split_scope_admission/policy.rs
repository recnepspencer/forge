#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanEdgeSplitDegeneracyPolicy {
    FailClosed,
}

impl PlanarBooleanEdgeSplitDegeneracyPolicy {
    pub fn fail_closed() -> Self {
        Self::FailClosed
    }

    pub fn stable_name(self) -> &'static str {
        match self {
            Self::FailClosed => "fail-closed",
        }
    }
}

impl Default for PlanarBooleanEdgeSplitDegeneracyPolicy {
    fn default() -> Self {
        Self::FailClosed
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanEdgeSplitDeterminismPolicy {
    CanonicalReplay,
}

impl PlanarBooleanEdgeSplitDeterminismPolicy {
    pub fn canonical_replay() -> Self {
        Self::CanonicalReplay
    }

    pub fn stable_name(self) -> &'static str {
        match self {
            Self::CanonicalReplay => "canonical-replay",
        }
    }
}

impl Default for PlanarBooleanEdgeSplitDeterminismPolicy {
    fn default() -> Self {
        Self::CanonicalReplay
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanEdgeSplitOverlapPolicy {
    PreserveIntervalPosture,
}

impl PlanarBooleanEdgeSplitOverlapPolicy {
    pub fn preserve_interval_posture() -> Self {
        Self::PreserveIntervalPosture
    }

    pub fn stable_name(self) -> &'static str {
        match self {
            Self::PreserveIntervalPosture => "preserve-interval-posture",
        }
    }
}

impl Default for PlanarBooleanEdgeSplitOverlapPolicy {
    fn default() -> Self {
        Self::PreserveIntervalPosture
    }
}
