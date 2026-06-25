use crate::graph_read_access_inventory::inventory_lane::WorthGraphReadAccessScopeFamily;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum WorthGraphReadTouchedAuthoritySourceFamily {
    TopologyClosure,
    SpatialContinuation,
}

impl WorthGraphReadTouchedAuthoritySourceFamily {
    pub(crate) const fn from_scope_family(
        scope_family: WorthGraphReadAccessScopeFamily,
    ) -> Option<Self> {
        match scope_family {
            WorthGraphReadAccessScopeFamily::TopologyReadLedger
            | WorthGraphReadAccessScopeFamily::TopologyRuntimeReadExecution
            | WorthGraphReadAccessScopeFamily::KernelWorkloadComposition
            | WorthGraphReadAccessScopeFamily::KernelBindingNeighborhood => {
                Some(Self::TopologyClosure)
            }
            WorthGraphReadAccessScopeFamily::PlanarBooleanContinuation
            | WorthGraphReadAccessScopeFamily::SpatialEvidenceLookup => {
                Some(Self::SpatialContinuation)
            }
            WorthGraphReadAccessScopeFamily::DeletedGraphReadSource
            | WorthGraphReadAccessScopeFamily::CertificationBoundary
            | WorthGraphReadAccessScopeFamily::NonGraphReadBoundary => None,
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TopologyClosure => "topology_closure",
            Self::SpatialContinuation => "spatial_continuation",
        }
    }
}
