#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiReplacementCandidateDenial {
    MissingArtifactDigest,
    MissingDependencyMetadata,
    MissingLoweringBasis,
    DependencyMetadataArtifactDigestMismatch,
    StaleDependencyMetadata,
}
