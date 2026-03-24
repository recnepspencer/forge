use crate::data::comparator::VersionComparatorPolicy;
use crate::data::graph::SuppressionFreeApplyCommitPacket;
use crate::data::handle::NodeId;
use crate::data::node::NodeState;
#[cfg(feature = "parallel")]
use crate::data::proof::SingleConsumer;
use crate::data::proof::SnapshotBatchCommit;
use crate::data::trace::RuntimeArtifactState;
use crate::logic::evaluation::EffectDependencyInputs;
use crate::logic::explain::RewiringSummary;
#[cfg(not(feature = "parallel"))]
use crate::logic::planner::semantic::StageSemanticIdentity;
#[cfg(feature = "parallel")]
use crate::logic::planner::semantic::{StageSemanticBatch, StageSemanticIdentity};
use crate::logic::planner::ExecutionRecordId;
use crate::logic::prepared::PreparedEvaluation;

use super::serial_batch::AppliedSerialStageBatch;

#[cfg_attr(not(feature = "parallel"), allow(dead_code))]
#[derive(Debug)]
pub(crate) struct ConcurrentWorkerInput {
    pub(in crate::logic::planner) task_index: usize,
    pub(in crate::logic::planner) node: NodeId,
    pub(in crate::logic::planner) identity: StageSemanticIdentity,
    pub(in crate::logic::planner) before_state: NodeState,
    pub(in crate::logic::planner) before_artifact_state: Option<RuntimeArtifactState>,
    pub(in crate::logic::planner) dependency_updates: u32,
    pub(in crate::logic::planner) recomputed: bool,
    pub(in crate::logic::planner) partition_aware: bool,
    pub(in crate::logic::planner) rewiring: Option<RewiringSummary>,
    pub(in crate::logic::planner) comparator_policy: VersionComparatorPolicy,
    pub(in crate::logic::planner) prepared: PreparedEvaluation,
    pub(in crate::logic::planner) dependency_inputs: EffectDependencyInputs,
}

#[cfg_attr(not(feature = "parallel"), allow(dead_code))]
#[derive(Debug)]
pub(crate) struct ConcurrentApplyGroupInput {
    pub(in crate::logic::planner) group_index: usize,
    pub(in crate::logic::planner) worker_inputs: Vec<ConcurrentWorkerInput>,
}

#[cfg_attr(not(feature = "parallel"), allow(dead_code))]
#[derive(Debug)]
pub(crate) struct GroupLocalTaskCommit {
    pub(in crate::logic::planner) task_index: usize,
    pub(in crate::logic::planner) node: NodeId,
    pub(in crate::logic::planner) identity: StageSemanticIdentity,
    pub(in crate::logic::planner) before_state: NodeState,
    pub(in crate::logic::planner) before_artifact_state: Option<RuntimeArtifactState>,
    pub(in crate::logic::planner) dependency_updates: u32,
    pub(in crate::logic::planner) recomputed: bool,
    pub(in crate::logic::planner) partition_aware: bool,
    pub(in crate::logic::planner) rewiring: Option<RewiringSummary>,
    pub(in crate::logic::planner) commit_packet: SuppressionFreeApplyCommitPacket,
}

#[cfg_attr(not(feature = "parallel"), allow(dead_code))]
#[derive(Debug)]
pub(crate) struct GroupedApplyFailure {
    pub(in crate::logic::planner) node: NodeId,
    pub(in crate::logic::planner) record_id: ExecutionRecordId,
    pub(in crate::logic::planner) error: crate::data::error::SignalError,
    pub(in crate::logic::planner) reuse_failure: Option<crate::data::reuse::ReuseBoundaryFailure>,
}

#[cfg_attr(not(feature = "parallel"), allow(dead_code))]
#[derive(Debug)]
pub(crate) struct GroupLocalApplyPacket {
    pub(in crate::logic::planner) group_index: usize,
    pub(in crate::logic::planner) task_count: usize,
    pub(in crate::logic::planner) task_commits: Vec<GroupLocalTaskCommit>,
}

#[cfg(feature = "parallel")]
impl GroupLocalApplyPacket {
    pub(in crate::logic::planner) fn packet_breadth(&self) -> usize {
        self.task_count
    }
}

/// Stage-lifetime workspace for lowered apply, snapshot deferral, and semantic finalize.
#[derive(Debug)]
pub(in crate::logic::planner) struct StageScratch {
    pub(in crate::logic::planner) finalize_work: StageFinalizeWork,
    pub(in crate::logic::planner) pending_snapshots: SnapshotBatchCommit,
}

#[derive(Debug)]
pub(in crate::logic::planner) enum StageFinalizeWork {
    Serial(AppliedSerialStageBatch),
    #[cfg(feature = "parallel")]
    Parallel(SingleConsumer<StageSemanticBatch>),
}
