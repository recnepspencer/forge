#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuarantineSealDenialKind {
    MissingExecutedPhysicalBasis,
    LaterLifecycleOwnerRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QuarantineSealDenial {
    kind: QuarantineSealDenialKind,
}

impl QuarantineSealDenial {
    pub(crate) const fn new(kind: QuarantineSealDenialKind) -> Self {
        Self { kind }
    }

    pub const fn kind(self) -> QuarantineSealDenialKind {
        self.kind
    }
}
