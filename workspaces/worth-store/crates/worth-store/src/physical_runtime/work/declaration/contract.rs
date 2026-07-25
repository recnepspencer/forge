use worth_store_physical_backend::ArtifactRangeWriteDurabilityRequirement;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalWorkOperationFamily {
    ArtifactMetadataRead,
    ArtifactRangeRead,
    ArtifactRangeWrite,
    ArtifactPublication,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalWorkEffectClass {
    ReadOnly,
    ReversibleBeforePublication,
    IdempotentExactWrite,
    PublicationBoundary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalWorkRecoveryDisposition {
    NoEffect,
    RetryExact,
    ContinueSettlement,
    InspectionRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalWorkDurabilityRequirement {
    ReadOnly,
    ArtifactRangeWrite(ArtifactRangeWriteDurabilityRequirement),
}
