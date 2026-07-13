#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LayoutCoverageRowKind {
    DeclarationInventory,
    AccessShapeDenial,
    BroadScanRejection,
    ExactCounter,
    RebuildParity,
    MigrationRollback,
    ReadmissionBoundary,
    MultiArtifactIntegration,
}
