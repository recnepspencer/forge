#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalDamageCause {
    WrongMagic,
    FamilyMismatch,
    FramingLengthMismatch,
    ChecksumMismatch,
    StoreIdentityMismatch,
    ArtifactIdentityMismatch,
    PhysicalGenerationMismatch,
    SelectorRoleMismatch,
    ChildReferenceMismatch,
    Truncated,
    MissingArtifact,
    DuplicateArtifact,
}
