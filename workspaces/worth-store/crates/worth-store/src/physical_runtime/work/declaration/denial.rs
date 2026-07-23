#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalWorkDeclarationDenial {
    EmptyScope,
    BatchRequiresMultipleMembers,
    DuplicateScopeMember,
    OverlappingScopeMembers,
    ScopeCapacityExceeded,
    ReadOnlyContractMismatch,
    EffectfulContractMismatch,
}
