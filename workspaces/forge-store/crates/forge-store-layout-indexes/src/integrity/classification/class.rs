#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutCorruptionClass {
    DerivedProjectionCorruption,
    AuthoritativeArtifactCorruption,
    ReadmissionRequired,
}
