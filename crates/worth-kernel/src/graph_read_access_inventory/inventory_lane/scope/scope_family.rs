#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthGraphReadAccessScopeFamily {
    TopologyReadLedger,
    TopologyRuntimeReadExecution,
    KernelWorkloadComposition,
    KernelBindingNeighborhood,
    SpatialEvidenceLookup,
    PlanarBooleanContinuation,
    DeletedGraphReadSource,
    CertificationBoundary,
    NonGraphReadBoundary,
}
