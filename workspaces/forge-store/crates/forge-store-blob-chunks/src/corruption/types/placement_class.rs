#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobCorruptionPlacementClass {
    LocalPhysical,
    ExternalCold,
    ImportStaging,
    CapsuleMaterialization,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobCorruptionReferenceSharingScope {
    SingleReference,
    SharedReferenceEdges,
}