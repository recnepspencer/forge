#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthGraphReadAccessDeclarationCloseoutErrorKind {
    MissingAdmissionPostureProof,
    SeedClaimedExecutionAuthority,
    SeedClaimedAccessPlanConsumption,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessDeclarationCloseoutError {
    kind: WorthGraphReadAccessDeclarationCloseoutErrorKind,
}

impl WorthGraphReadAccessDeclarationCloseoutError {
    pub const fn new(kind: WorthGraphReadAccessDeclarationCloseoutErrorKind) -> Self {
        Self { kind }
    }

    pub const fn kind(&self) -> WorthGraphReadAccessDeclarationCloseoutErrorKind {
        self.kind
    }
}
