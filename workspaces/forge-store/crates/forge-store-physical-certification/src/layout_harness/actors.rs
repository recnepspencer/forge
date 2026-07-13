#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LayoutActorLane {
    DeclarationCatalog,
    ForegroundAccess,
    Recovery,
    Migration,
    Maintenance,
    OfflineVerifier,
}
