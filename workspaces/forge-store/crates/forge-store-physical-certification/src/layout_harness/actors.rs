#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum S8LayoutActorLane {
    DeclarationCatalog,
    ForegroundAccess,
    Recovery,
    Migration,
    Maintenance,
    OfflineVerifier,
}
