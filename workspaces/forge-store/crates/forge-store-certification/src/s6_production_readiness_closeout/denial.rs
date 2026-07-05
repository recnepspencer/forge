#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum S6ProductionReadinessClosureDenial {
    Phase13EvidenceCannotSatisfyReadiness,
    ResidualDebtCannotBePlatformGrade,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum S6ProductionReadinessPosture {
    PlatformGrade,
    ResidualDebtPresent,
}
