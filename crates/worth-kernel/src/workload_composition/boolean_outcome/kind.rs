#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanOutcomeKind {
    Admitted,
    Unsupported,
    Blocked,
    Denied,
    PolicyRequired,
    IntegrityMismatch,
    NoOptions,
}

impl PlanarBooleanOutcomeKind {
    pub fn is_admitted(self) -> bool {
        matches!(self, Self::Admitted)
    }
}
