pub(crate) mod error;
mod handle_reads;
pub(crate) mod read_proof;
pub(crate) mod request;
mod session_state;
mod views;

#[allow(unused_imports)]
pub(crate) use self::read_proof::{closeout, no_n_plus_one, parity, report};
pub use self::read_proof::{
    TopologyNoNPlusOneContract, TopologyNoNPlusOneContractRow, TopologyNoNPlusOneContractStatus,
    TopologyReadAggregateReport, TopologyReadCloseoutReport, TopologyReadCloseoutRow,
    TopologyReadCloseoutStatus, TopologyReadDebtRow, TopologyReadExecutionAggregateRow,
    TopologyReadExecutionEngine, TopologyReadFallbackPosture, TopologyReadFamilyAggregateRow,
    TopologyReadParityAggregateReport, TopologyReadParityAggregateRow, TopologyReadParityKind,
    TopologyReadPhaseThreeBlocker, TopologyReadPhaseThreeBlockerRow,
    TopologyReadPhaseThreeBlockerStatus, TopologyReadProofReport, TopologyReadRequestFamily,
    TopologyReadRequestReport,
};
#[allow(unused_imports)]
pub(crate) use crate::projection::runtime_boundary::read_lowering::schema;
pub use crate::projection::runtime_boundary::read_lowering::{
    TopologyReadLoweringPosture, TopologyReadRelationshipProofPosture,
};
pub use error::{TopologyReadError, TopologyReadErrorKind};
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
