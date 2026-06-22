#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanMicroIntervalPolicy {
    DenyBelowTolerance,
    AdmitExplicitCollapse,
    RequireExplicitDecision,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanMicroIntervalAction {
    Retain,
    AdmittedCollapse,
    PolicyRequired,
}
