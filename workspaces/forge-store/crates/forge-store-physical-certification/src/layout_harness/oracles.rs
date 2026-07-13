#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LayoutOracleLane {
    DeclarationInventoryOracle,
    AccessShapeDenialOracle,
    BroadScanRejectionOracle,
    ExactCounterOracle,
    RebuildParityOracle,
    MigrationRollbackOracle,
    ReadmissionBoundaryOracle,
    MultiArtifactIntegrationOracle,
}
