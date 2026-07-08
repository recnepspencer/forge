#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiConstraintCycleParticipationPosture {
    Acyclic,
    AdmittedFixedPoint,
}

impl UiConstraintCycleParticipationPosture {
    pub(crate) const fn rank(self) -> u8 {
        match self {
            Self::Acyclic => 0,
            Self::AdmittedFixedPoint => 1,
        }
    }
}
