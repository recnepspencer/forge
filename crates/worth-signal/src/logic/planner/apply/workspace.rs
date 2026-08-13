use crate::data::comparator::VersionComparatorPolicy;
use crate::data::graph::PreparedParallelApplyCommitPacket;
use crate::data::handle::NodeId;
use crate::data::node::NodeState;
use crate::data::proof::ClassifiedSnapshotBatchCommit;
#[cfg(feature = "parallel")]
use crate::data::proof::SingleConsumer;
use crate::data::trace::RuntimeArtifactFinalizeImage;
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
    task_index: usize,
    node: NodeId,
    identity: StageSemanticIdentity,
    before_state: NodeState,
    before_artifact_state: Option<RuntimeArtifactFinalizeImage>,
    dependency_updates: u32,
    recomputed: bool,
    partition_aware: bool,
    rewiring: Option<RewiringSummary>,
    comparator_policy: VersionComparatorPolicy,
    prepared: PreparedEvaluation,
    dependency_inputs: EffectDependencyInputs,
}

#[cfg_attr(not(feature = "parallel"), allow(dead_code))]
#[derive(Debug)]
pub(crate) struct ConcurrentApplyGroupInput {
    group_index: usize,
    worker_inputs: Vec<ConcurrentWorkerInput>,
}

#[cfg_attr(not(feature = "parallel"), allow(dead_code))]
#[derive(Debug)]
pub(crate) struct GroupLocalTaskCommit {
    task_index: usize,
    node: NodeId,
    identity: StageSemanticIdentity,
    before_state: NodeState,
    before_artifact_state: Option<RuntimeArtifactFinalizeImage>,
    dependency_updates: u32,
    recomputed: bool,
    partition_aware: bool,
    rewiring: Option<RewiringSummary>,
    commit_packet: PreparedParallelApplyCommitPacket,
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
    group_index: usize,
    task_count: usize,
    task_commits: Vec<GroupLocalTaskCommit>,
}

#[cfg(feature = "parallel")]
impl GroupLocalApplyPacket {
    pub(in crate::logic::planner) fn new(
        group_index: usize,
        task_commits: Vec<GroupLocalTaskCommit>,
    ) -> Self {
        let task_count = task_commits.len();
        Self {
            group_index,
            task_count,
            task_commits,
        }
    }

    pub(in crate::logic::planner) fn packet_breadth(&self) -> usize {
        self.task_count
    }

    pub(in crate::logic::planner) fn group_index(&self) -> usize {
        self.group_index
    }

    pub(in crate::logic::planner) fn into_task_commits(self) -> Vec<GroupLocalTaskCommit> {
        self.task_commits
    }
}

/// Stage-lifetime workspace for lowered apply, snapshot deferral, and semantic finalize.
#[derive(Debug)]
pub(in crate::logic::planner) struct StageScratch {
    finalize_work: StageFinalizeWork,
    pending_snapshots: ClassifiedSnapshotBatchCommit,
}

#[derive(Debug)]
pub(in crate::logic::planner) enum StageFinalizeWork {
    Serial(AppliedSerialStageBatch),
    #[cfg(feature = "parallel")]
    Parallel(SingleConsumer<StageSemanticBatch>),
}

#[cfg(feature = "parallel")]
impl ConcurrentWorkerInput {
    pub(in crate::logic::planner) fn new(
        task_index: usize,
        node: NodeId,
        identity: StageSemanticIdentity,
        before_state: NodeState,
        before_artifact_state: Option<RuntimeArtifactFinalizeImage>,
        dependency_updates: u32,
        recomputed: bool,
        partition_aware: bool,
        rewiring: Option<RewiringSummary>,
        comparator_policy: VersionComparatorPolicy,
        prepared: PreparedEvaluation,
        dependency_inputs: EffectDependencyInputs,
    ) -> Self {
        Self {
            task_index,
            node,
            identity,
            before_state,
            before_artifact_state,
            dependency_updates,
            recomputed,
            partition_aware,
            rewiring,
            comparator_policy,
            prepared,
            dependency_inputs,
        }
    }

    pub(in crate::logic::planner) fn into_parts(
        self,
    ) -> (
        usize,
        NodeId,
        StageSemanticIdentity,
        NodeState,
        Option<RuntimeArtifactFinalizeImage>,
        u32,
        bool,
        bool,
        Option<RewiringSummary>,
        VersionComparatorPolicy,
        PreparedEvaluation,
        EffectDependencyInputs,
    ) {
        (
            self.task_index,
            self.node,
            self.identity,
            self.before_state,
            self.before_artifact_state,
            self.dependency_updates,
            self.recomputed,
            self.partition_aware,
            self.rewiring,
            self.comparator_policy,
            self.prepared,
            self.dependency_inputs,
        )
    }
}

#[cfg(feature = "parallel")]
impl ConcurrentApplyGroupInput {
    pub(in crate::logic::planner) fn new(
        group_index: usize,
        worker_inputs: Vec<ConcurrentWorkerInput>,
    ) -> Self {
        Self {
            group_index,
            worker_inputs,
        }
    }

    pub(in crate::logic::planner) fn into_parts(self) -> (usize, Vec<ConcurrentWorkerInput>) {
        (self.group_index, self.worker_inputs)
    }
}

#[cfg(feature = "parallel")]
impl GroupLocalTaskCommit {
    pub(in crate::logic::planner) fn task_index(&self) -> usize {
        self.task_index
    }

    pub(in crate::logic::planner) fn new(
        task_index: usize,
        node: NodeId,
        identity: StageSemanticIdentity,
        before_state: NodeState,
        before_artifact_state: Option<RuntimeArtifactFinalizeImage>,
        dependency_updates: u32,
        recomputed: bool,
        partition_aware: bool,
        rewiring: Option<RewiringSummary>,
        commit_packet: PreparedParallelApplyCommitPacket,
    ) -> Self {
        Self {
            task_index,
            node,
            identity,
            before_state,
            before_artifact_state,
            dependency_updates,
            recomputed,
            partition_aware,
            rewiring,
            commit_packet,
        }
    }

    pub(in crate::logic::planner) fn into_parts(
        self,
    ) -> (
        usize,
        NodeId,
        StageSemanticIdentity,
        NodeState,
        Option<RuntimeArtifactFinalizeImage>,
        u32,
        bool,
        bool,
        Option<RewiringSummary>,
        PreparedParallelApplyCommitPacket,
    ) {
        (
            self.task_index,
            self.node,
            self.identity,
            self.before_state,
            self.before_artifact_state,
            self.dependency_updates,
            self.recomputed,
            self.partition_aware,
            self.rewiring,
            self.commit_packet,
        )
    }
}

impl StageScratch {
    pub(in crate::logic::planner) fn new(
        finalize_work: StageFinalizeWork,
        pending_snapshots: ClassifiedSnapshotBatchCommit,
    ) -> Self {
        Self {
            finalize_work,
            pending_snapshots,
        }
    }

    pub(in crate::logic::planner) fn into_parts(
        self,
    ) -> (StageFinalizeWork, ClassifiedSnapshotBatchCommit) {
        (self.finalize_work, self.pending_snapshots)
    }
}
