use crate::data::comparator::VersionComparatorPolicy;
use crate::data::graph::SuppressionFreeApplyCommitPacket;
use crate::data::handle::NodeId;
use crate::data::node::NodeState;
use crate::data::proof::SingleConsumer;
use crate::data::trace::RuntimeArtifactState;
use crate::logic::evaluation::EffectDependencyInputs;
use crate::logic::evaluation::PendingDependencySnapshot;
use crate::logic::explain::RewiringSummary;
use crate::logic::prepared::PreparedEvaluation;
use crate::logic::planner::semantic::{StageSemanticBatch, StageSemanticIdentity};
use crate::logic::planner::{ExecutionRecordId, ReductionOrderingContract, ReductionWorkClass};

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

#[derive(Debug)]
pub(crate) struct GroupLocalApplyPacket {
    pub(in crate::logic::planner) group_index: usize,
    pub(in crate::logic::planner) task_count: usize,
    pub(in crate::logic::planner) task_commits: Vec<GroupLocalTaskCommit>,
    pub(in crate::logic::planner) semantic_batch: StageSemanticBatch,
    pub(in crate::logic::planner) pending_snapshots: Vec<PendingDependencySnapshot>,
}

impl GroupLocalApplyPacket {
    pub(in crate::logic::planner) fn packet_breadth(&self) -> usize {
        self.task_count
    }

    pub(in crate::logic::planner) fn publication_breadth(&self) -> usize {
        self.task_commits.len() + self.semantic_batch.segment_count() + self.pending_snapshots.len()
    }
}

/// Stage-lifetime workspace for lowered apply, snapshot deferral, and semantic finalize.
#[derive(Debug)]
pub(crate) struct StageScratch {
    pub(in crate::logic::planner) semantic_batch: SingleConsumer<StageSemanticBatch>,
    pub(in crate::logic::planner) pending_snapshots: Vec<PendingDependencySnapshot>,
}

pub(in crate::logic::planner) fn reduce_group_local_apply_packets(
    mut packets: Vec<GroupLocalApplyPacket>,
    ordering_contract: ReductionOrderingContract,
    allowed_work: ReductionWorkClass,
) -> StageScratch {
    debug_assert!(
        matches!(allowed_work, ReductionWorkClass::DeterministicPublicationOnly),
        "group-local apply reduction may only perform deterministic publication"
    );
    match ordering_contract {
        ReductionOrderingContract::StageTaskIndexOrder => {
            packets.sort_by_key(|packet| packet.group_index);
        }
    }

    let mut semantic_batch = StageSemanticBatch::default();
    let mut pending_snapshots = Vec::new();
    for packet in packets {
        semantic_batch.extend(packet.semantic_batch);
        pending_snapshots.extend(packet.pending_snapshots);
    }

    StageScratch {
        semantic_batch: SingleConsumer::new(semantic_batch),
        pending_snapshots,
    }
}
