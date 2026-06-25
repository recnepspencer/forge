#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadDeclarationDeletionFirewallError {
    kind: WorthGraphReadDeclarationDeletionFirewallErrorKind,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthGraphReadDeclarationDeletionFirewallErrorKind {
    MissingAdmissionPostureRecord,
    MissingDeletionLedgerItem,
    DeletionResidueRequiresBlocker,
    CappedResidueCapExceeded,
    SourceFirewallViolation,
    SeedClaimedExecutionAuthority,
    SeedClaimedAccessPlanConsumption,
}

impl WorthGraphReadDeclarationDeletionFirewallError {
    pub const fn new(kind: WorthGraphReadDeclarationDeletionFirewallErrorKind) -> Self {
        Self { kind }
    }

    pub const fn kind(&self) -> WorthGraphReadDeclarationDeletionFirewallErrorKind {
        self.kind
    }
}
