#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthGraphReadAccessScopeKind {
    SelectedObligation,
    TouchedAuthorityDigest,
    TouchDescriptorDigest,
    TopologyReadProof,
    SpatialContinuationProof,
    DeletedGraphReadSource,
    CertificationOnlyBoundary,
    OutOfScopeNonGraphRead,
}
