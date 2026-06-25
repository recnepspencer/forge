#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthGraphReadAccessExecutionReceiptAccountingErrorKind {
    SeedAlreadyClaimsValidatorSelection,
    EmptyReceiptAccountingInput,
    CallerOwnedGraphWorkDetected,
    BatchCounterReceiptAssociationLost,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessExecutionReceiptAccountingError {
    kind: WorthGraphReadAccessExecutionReceiptAccountingErrorKind,
}

impl WorthGraphReadAccessExecutionReceiptAccountingError {
    pub(crate) const fn new(kind: WorthGraphReadAccessExecutionReceiptAccountingErrorKind) -> Self {
        Self { kind }
    }

    pub const fn kind(&self) -> WorthGraphReadAccessExecutionReceiptAccountingErrorKind {
        self.kind
    }
}
