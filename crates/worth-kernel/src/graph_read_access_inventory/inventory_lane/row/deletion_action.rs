#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthGraphReadAccessDeletionAction {
    MigrateToQueryDeclaration,
    DeleteAfterConsumerCutover,
    CapUntilQueryCapabilityExists,
    KeepCertificationOnly,
    OutOfScopeNoGraphRead,
}
