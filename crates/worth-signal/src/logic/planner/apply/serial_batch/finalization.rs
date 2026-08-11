use crate::data::error::SignalError;
use crate::logic::planner::types::{StageExecutionRecord, TaskExecutionRecord};

use super::application::{AppliedSerialStageBatch, AppliedSerialTask, StageOrderedAppliedTasks};
use super::preparation::SerialFinalizeSeed;
use super::witness::StageTaskOrderProof;

impl AppliedSerialStageBatch {
    pub(in crate::logic::planner) fn split_pending_snapshots(
        self,
    ) -> (Self, crate::data::proof::ClassifiedSnapshotBatchCommit) {
        let AppliedSerialStageBatch {
            stage_index,
            exact_width,
            stage_tasks,
            finalize_seeds,
            applied_tasks,
            pending_snapshots,
            stage_order,
        } = self;
        (
            Self {
                stage_index,
                exact_width,
                stage_tasks,
                finalize_seeds,
                applied_tasks,
                pending_snapshots: super::preparation::DeferredSnapshotBatch::default(),
                stage_order,
            },
            pending_snapshots.classify(),
        )
    }

    pub(in crate::logic::planner) fn into_ready_for_finalize(
        self,
    ) -> Result<ReadySerialFinalizeBatch, SignalError> {
        let AppliedSerialStageBatch {
            stage_index: _stage_index,
            exact_width,
            stage_tasks,
            finalize_seeds,
            applied_tasks,
            pending_snapshots,
            stage_order,
        } = self;
        if !pending_snapshots.pending_snapshots.is_empty() {
            return Err(SignalError::internal(
                "serial finalize input must not retain uncommitted stage-owned snapshots",
            ));
        }
        if finalize_seeds.len() != exact_width.get() {
            return Err(SignalError::internal(
                "serial finalize seeds must match the prepared stage width",
            ));
        }
        if stage_tasks.len() != exact_width.get() {
            return Err(SignalError::internal(
                "serial stage task witness must match the prepared stage width",
            ));
        }
        if applied_tasks.exact_width().get() != exact_width.get() {
            return Err(SignalError::internal(
                "serial applied task batch width must remain aligned with the prepared stage width",
            ));
        }

        Ok(ReadySerialFinalizeBatch::new(
            stage_tasks,
            finalize_seeds,
            applied_tasks,
            stage_order,
        ))
    }
}

#[derive(Debug, Clone)]
pub(in crate::logic::planner) struct ReadySerialFinalizeBatch {
    stage_tasks: Vec<crate::logic::planner::types::EligibleTask>,
    finalize_seeds: Vec<SerialFinalizeSeed>,
    applied_tasks: StageOrderedAppliedTasks,
    stage_order: StageTaskOrderProof,
}

impl ReadySerialFinalizeBatch {
    pub(super) fn new(
        stage_tasks: Vec<crate::logic::planner::types::EligibleTask>,
        finalize_seeds: Vec<SerialFinalizeSeed>,
        applied_tasks: StageOrderedAppliedTasks,
        stage_order: StageTaskOrderProof,
    ) -> Self {
        Self {
            stage_tasks,
            finalize_seeds,
            applied_tasks,
            stage_order,
        }
    }

    pub(in crate::logic::planner) fn into_parts(
        self,
    ) -> (
        Vec<crate::logic::planner::types::EligibleTask>,
        Vec<SerialFinalizeSeed>,
        Vec<AppliedSerialTask>,
        StageTaskOrderProof,
    ) {
        (
            self.stage_tasks,
            self.finalize_seeds,
            self.applied_tasks.tasks,
            self.stage_order,
        )
    }
}

#[derive(Debug, Clone)]
pub(in crate::logic::planner) struct FinalizedSerialStageBatch {
    semantic_task_range: crate::logic::planner::types::SemanticTaskRange,
    task_records: Vec<TaskExecutionRecord>,
    semantic_segment_count: u32,
}

impl FinalizedSerialStageBatch {
    pub(in crate::logic::planner) fn new(
        semantic_task_range: crate::logic::planner::types::SemanticTaskRange,
        task_records: Vec<TaskExecutionRecord>,
        semantic_segment_count: u32,
    ) -> Self {
        Self {
            semantic_task_range,
            task_records,
            semantic_segment_count,
        }
    }

    pub(in crate::logic::planner) fn record_into(
        self,
        report: &mut crate::logic::planner::types::ExecutionReport,
        stage_record: &mut StageExecutionRecord,
    ) {
        stage_record.semantic_task_range = Some(self.semantic_task_range);
        stage_record.semantic_segment_count = self.semantic_segment_count;
        report.semantic_segment_count += self.semantic_segment_count;
        stage_record.task_records = self.task_records;
    }
}
