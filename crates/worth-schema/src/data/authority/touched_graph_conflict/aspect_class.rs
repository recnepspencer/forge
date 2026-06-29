use serde::Serialize;

use crate::data::authority::touched_graph_basis::WorthTopologyTouchedAspect;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum ConflictAspectClass {
    WorthTopologyTouched(WorthTopologyTouchedAspect),
    QueryDeclarationEnvelope,
    QueryLowerRuntimeBoundaryEnvelope,
    QueryProjectionConsumption,
    SpatialEvidence,
    ValidatorInvariant,
}

impl ConflictAspectClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorthTopologyTouched(aspect) => aspect.as_str(),
            Self::QueryDeclarationEnvelope => "query.declaration-envelope",
            Self::QueryLowerRuntimeBoundaryEnvelope => "query.lower-runtime-boundary-envelope",
            Self::QueryProjectionConsumption => "query.projection-consumption",
            Self::SpatialEvidence => "spatial.evidence",
            Self::ValidatorInvariant => "topology.validator-invariant",
        }
    }
}
