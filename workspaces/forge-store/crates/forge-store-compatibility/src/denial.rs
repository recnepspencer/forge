#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArtifactCompatibilityDenial {
    EmptyCompatibilityWindow,
    VersionOutsideCompatibilityWindow,
    WriteVersionOutsideCompatibilityWindow,
}
