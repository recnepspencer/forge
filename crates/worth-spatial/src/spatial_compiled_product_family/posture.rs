#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpatialSourceAuthorityDigestBasisPosture {
    EvidenceLookupLedgerBasisWithStageReceiptCoordinate,
    RetainedCancellationChainAuthorityDigest,
    RetainedPlanarHistoricalInspectionDigest,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpatialLocalityFootprintBasisPosture {
    GroupedBatchFootprintDigest,
    ProjectionConsumptionDigest,
    SpatialTouchDigest,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpatialPriorProofRolePosture {
    NotRequired,
    RetainedCancellationCheckpointHistoryBasis,
    SelectedPlanTopologyAndQuerySupportBasis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpatialEvidenceSupportRolePosture {
    QueryAndTopologySupportEvidence,
    RetainedCancellationProjectionEvidence,
    RetainedReplayProjectionEvidence,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpatialEquivalencePolicyPosture {
    EvidenceLookupIndexSemanticParity,
    RetainedCancellationSemanticParity,
    RetainedReplaySemanticParity,
}
