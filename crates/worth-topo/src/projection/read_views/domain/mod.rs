pub(crate) mod error;
pub(crate) mod request;
mod topology;
mod views;

#[allow(unused_imports)]
pub(crate) use crate::projection::diagnostic_surfaces::read_proof::{
    closeout, no_n_plus_one, parity, report,
};
pub use crate::projection::diagnostic_surfaces::read_proof::{
    TopologyDomainQueryAggregateReport, TopologyDomainQueryCloseoutReport,
    TopologyDomainQueryCloseoutRow, TopologyDomainQueryCloseoutStatus, TopologyDomainQueryDebtRow,
    TopologyDomainQueryExecutionAggregateRow, TopologyDomainQueryExecutionEngine,
    TopologyDomainQueryFallbackPosture, TopologyDomainQueryFamilyAggregateRow,
    TopologyDomainQueryParityAggregateReport, TopologyDomainQueryParityAggregateRow,
    TopologyDomainQueryParityKind, TopologyDomainQueryPhaseThreeBlocker,
    TopologyDomainQueryPhaseThreeBlockerRow, TopologyDomainQueryPhaseThreeBlockerStatus,
    TopologyDomainQueryProofReport, TopologyDomainQueryRequestFamily,
    TopologyDomainQueryRequestReport, TopologyNoNPlusOneContract, TopologyNoNPlusOneContractRow,
    TopologyNoNPlusOneContractStatus,
};
#[allow(unused_imports)]
pub(crate) use crate::projection::runtime_boundary::read_lowering::schema;
pub use crate::projection::runtime_boundary::read_lowering::{
    TopologyDomainQueryLoweringPosture, TopologyDomainQueryRelationshipProofPosture,
};
pub use error::{TopologyDomainQueryError, TopologyDomainQueryErrorKind};
pub use topology::TopologyDomainQuery;
#[allow(unused_imports)]
pub use views::{
    TopologyAdjacentHalfEdgeEvidence, TopologyHalfEdgeRadialNeighborhoodView,
    TopologyHalfEdgeSharedVertexNeighborhoodView, TopologyLocalRewireNeighborhoodView,
    TopologyLoopCycleView, TopologyLoopNeighborEvidence, TopologyRadialCandidateEvidence,
};
