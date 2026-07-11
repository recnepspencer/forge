#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum S8LayoutOracleLane {
    DeclarationInventoryOracle,
    AccessShapeDenialOracle,
    BroadScanRejectionOracle,
    ExactCounterOracle,
    RebuildParityOracle,
    MigrationRollbackOracle,
    ReadmissionBoundaryOracle,
    MultiArtifactIntegrationOracle,
}
