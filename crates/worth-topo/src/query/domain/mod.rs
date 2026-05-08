pub(crate) mod error;
mod execution;
pub(crate) mod lowering;
pub(crate) mod proof;
pub(crate) mod request;
mod topology;
mod views;

pub use error::{TopologyDomainQueryError, TopologyDomainQueryErrorKind};
#[allow(unused_imports)]
pub(crate) use lowering::schema;
pub use lowering::{
    TopologyDomainQueryLoweringPosture, TopologyDomainQueryRelationshipProofPosture,
};
#[allow(unused_imports)]
pub(crate) use proof::{closeout, no_n_plus_one, parity, report};
pub use proof::{
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
pub use topology::TopologyDomainQuery;
#[allow(unused_imports)]
pub use views::{
    TopologyAdjacentHalfEdgeEvidence, TopologyHalfEdgeRadialNeighborhoodView,
    TopologyHalfEdgeSharedVertexNeighborhoodView, TopologyLocalRewireNeighborhoodView,
    TopologyLoopCycleView, TopologyLoopNeighborEvidence, TopologyRadialCandidateEvidence,
};
