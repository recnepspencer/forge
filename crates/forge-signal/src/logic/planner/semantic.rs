use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::data::node::NodeState;
use crate::data::trace::TraceSummary;

use super::reporting::{accumulate_report_counters, classify_task_record};
use super::types::{
    EvaluationTask, ExecutionRecordId, ExecutionReport, SemanticSegmentId, SemanticTaskRange,
    StageExecutionRecord, TaskExecutionRecord,
};

#[derive(Debug, Clone, Copy)]
pub(super) struct StageSemanticIdentity {
    pub record_id: ExecutionRecordId,
    pub segment_id: SemanticSegmentId,
}

#[derive(Debug, Clone)]
pub(super) struct SemanticTaskUpdate {
    pub task_index: usize,
    pub node: NodeId,
    pub identity: StageSemanticIdentity,
    pub before_state: NodeState,
    pub before_trace: Option<TraceSummary>,
    pub after_state: NodeState,
    pub dependency_updates: u32,
    pub recomputed: bool,
    pub partition_aware: bool,
}

#[derive(Debug, Clone)]
pub(super) struct SemanticSegment {
    pub id: SemanticSegmentId,
    pub task_range: SemanticTaskRange,
    pub updates: Vec<SemanticTaskUpdate>,
}

#[derive(Debug, Clone, Default)]
pub(super) struct StageSemanticBatch {
    pub segments: Vec<SemanticSegment>,
}

impl StageSemanticBatch {
    pub fn push_segment(&mut self, segment: SemanticSegment) {
        self.segments.push(segment);
    }

    pub fn is_empty(&self) -> bool {
        self.segments.is_empty()
    }
}

pub(super) fn reserve_stage_identities(
    next_record_id: &mut u64,
    next_segment_id: &mut u64,
    task_count: usize,
) -> Vec<StageSemanticIdentity> {
    let first_record = *next_record_id;
    let first_segment = *next_segment_id;
    *next_record_id += task_count as u64;
    *next_segment_id += task_count as u64;
    (0..task_count)
        .map(|offset| StageSemanticIdentity {
            record_id: ExecutionRecordId(first_record + offset as u64),
            segment_id: SemanticSegmentId(first_segment + offset as u64),
        })
        .collect()
}

pub(super) fn segment_for_single_update(update: SemanticTaskUpdate) -> SemanticSegment {
    SemanticSegment {
        id: update.identity.segment_id,
        task_range: SemanticTaskRange {
            start: update.identity.record_id,
            end: update.identity.record_id,
        },
        updates: vec![update],
    }
}

pub(super) fn segment_for_updates(mut updates: Vec<SemanticTaskUpdate>) -> SemanticSegment {
    updates.sort_by_key(|update| update.task_index);
    let first = updates
        .first()
        .expect("semantic segment requires at least one task update");
    let last = updates
        .last()
        .expect("semantic segment requires at least one task update");
    SemanticSegment {
        id: first.identity.segment_id,
        task_range: SemanticTaskRange {
            start: first.identity.record_id,
            end: last.identity.record_id,
        },
        updates,
    }
}

pub(super) fn finalize_stage_batch(
    graph: &mut SignalGraph,
    stage_tasks: &[EvaluationTask],
    batch: StageSemanticBatch,
    report: &mut ExecutionReport,
    stage_record: &mut StageExecutionRecord,
) -> Result<(), SignalError> {
    if batch.is_empty() {
        return Ok(());
    }

    let mut segments = batch.segments;
    segments.sort_by_key(|segment| (segment.task_range.start.0, segment.id.0));
    stage_record.semantic_segment_count = segments.len() as u32;
    report.semantic_segment_count += segments.len() as u32;
    stage_record.semantic_task_range = Some(SemanticTaskRange {
        start: segments
            .first()
            .expect("segments not empty")
            .task_range
            .start,
        end: segments.last().expect("segments not empty").task_range.end,
    });

    let mut task_records = Vec::new();
    for segment in segments {
        for update in segment.updates {
            stamp_trace_summary(
                graph,
                update.node,
                update.identity.record_id,
                update.identity.segment_id,
            )?;
            let task = &stage_tasks[update.task_index];
            let task_record = classify_task_record(
                update.identity.record_id,
                update.identity.segment_id,
                task,
                update.before_state,
                update.after_state,
                update.before_trace.as_ref(),
                graph.get_entry(update.node)?.get_trace_summary(),
            );
            accumulate_report_counters(report, &task_record);
            task_records.push(task_record);
            graph.telemetry_mut().prepared_evaluations_applied += 1;
            graph.telemetry_mut().dependency_capture_updates += update.dependency_updates as u64;
            if update.recomputed {
                graph.telemetry_mut().nodes_recomputed += 1;
            }
            if update.partition_aware {
                graph.telemetry_mut().partition_aware_recomputations += 1;
            }
            report.prepared_evaluations_applied += 1;
            report.dependency_capture_updates += update.dependency_updates;
        }
    }
    task_records.sort_by_key(|record| record.id.0);
    stage_record.task_records = task_records;
    Ok(())
}

fn stamp_trace_summary(
    graph: &mut SignalGraph,
    node: NodeId,
    record_id: ExecutionRecordId,
    segment_id: SemanticSegmentId,
) -> Result<(), SignalError> {
    let Some(mut summary) = graph.get_entry(node)?.get_trace_summary().cloned() else {
        return Ok(());
    };
    summary.execution_record_id = Some(record_id.0);
    summary.semantic_segment_id = Some(segment_id.0);
    graph.get_entry_mut(node)?.set_trace_summary(Some(summary));
    Ok(())
}

#[allow(dead_code)]
fn _assert_task_records_are_sorted(records: &[TaskExecutionRecord]) -> bool {
    records
        .windows(2)
        .all(|window| window[0].id.0 <= window[1].id.0)
}
