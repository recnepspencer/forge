pub(crate) mod error;
mod handle_reads;
pub(crate) mod request;
mod session_state;
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
pub use handle_reads::{
    TopologyConfiguredDomainReadSession, TopologyCurrentHeadReadHandleExt,
    TopologyCurrentHeadReadSession, TopologySnapshotReadOnlyReadHandleExt,
    TopologySnapshotReadOnlyReadSession,
};
pub(crate) use session_state::TopologyReadLedger;
#[allow(unused_imports)]
pub use views::{
    TopologyAdjacentHalfEdgeEvidence, TopologyHalfEdgeRadialNeighborhoodView,
    TopologyHalfEdgeSharedVertexNeighborhoodView, TopologyLocalRewireNeighborhoodView,
    TopologyLoopCycleView, TopologyLoopNeighborEvidence, TopologyRadialCandidateEvidence,
};
