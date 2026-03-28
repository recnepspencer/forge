pub(crate) mod artifacts;
pub(crate) mod reporting;
pub(crate) mod stage_recording;

use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
#[cfg(feature = "parallel")]
use crate::data::handle::NodeId;
#[cfg(feature = "parallel")]
use crate::data::node::NodeState;
#[cfg(feature = "parallel")]
use crate::data::output::MemoizedResultOrigin;
#[cfg(feature = "parallel")]
use crate::data::reuse::ReuseBasis;
#[cfg(feature = "parallel")]
use crate::data::trace::RuntimeArtifactFinalizeImage;
use crate::diagnostics::recorder::stamp_trace_summary_and_record_lineage_transition_from_image;
#[cfg(feature = "parallel")]
use crate::logic::evaluation::EvaluationVerdict;
#[cfg(feature = "parallel")]
use crate::logic::explain::RewiringSummary;

use super::apply::serial_batch::{FinalizedSerialStageBatch, ReadySerialFinalizeBatch};

use self::artifacts::record_semantic_artifacts;
use self::reporting::record_semantic_update;
use super::reporting::classify_task_execution_record;
#[cfg(feature = "parallel")]
use super::types::EligibleTask;
use super::types::{
    ExecutionRecordId, ExecutionReport, SemanticSegmentId, SemanticTaskRange, StageExecutionRecord,
};

#[derive(Debug, Clone, Copy)]
pub(in crate::logic::planner) struct StageSemanticIdentity {
    pub record_id: ExecutionRecordId,
    pub segment_id: SemanticSegmentId,
}

#[cfg(feature = "parallel")]
#[derive(Debug, Clone)]
pub(super) struct SemanticTaskUpdate {
    task_index: usize,
    node: NodeId,
    identity: StageSemanticIdentity,
    before_state: NodeState,
    before_artifact_state: Option<RuntimeArtifactFinalizeImage>,
    after_state: NodeState,
    dependency_updates: u32,
    recomputed: bool,
    partition_aware: bool,
    rewiring: Option<RewiringSummary>,
    verdict: EvaluationVerdict,
    memoized_origin: MemoizedResultOrigin,
    reuse_basis: ReuseBasis,
}

#[cfg(feature = "parallel")]
#[derive(Debug, Clone)]
pub(super) struct SemanticSegment {
    id: SemanticSegmentId,
    task_range: SemanticTaskRange,
    updates: Vec<SemanticTaskUpdate>,
}

#[cfg(feature = "parallel")]
#[derive(Debug, Clone, Default)]
pub(super) struct StageSemanticBatch {
    segments: Vec<SemanticSegment>,
}

#[cfg(feature = "parallel")]
impl StageSemanticBatch {
    #[cfg(feature = "parallel")]
    pub fn push_segment(&mut self, segment: SemanticSegment) {
        self.segments.push(segment);
    }

    #[cfg(feature = "parallel")]
    pub(in crate::logic::planner) fn segment_count(&self) -> usize {
        self.segments.len()
    }

    pub fn is_empty(&self) -> bool {
        self.segments.is_empty()
    }

    fn into_segments(self) -> Vec<SemanticSegment> {
        self.segments
    }
}

#[cfg(feature = "parallel")]
impl SemanticTaskUpdate {
    pub(super) fn new(
        task_index: usize,
        node: NodeId,
        identity: StageSemanticIdentity,
        before_state: NodeState,
        before_artifact_state: Option<RuntimeArtifactFinalizeImage>,
        after_state: NodeState,
        dependency_updates: u32,
        recomputed: bool,
        partition_aware: bool,
        rewiring: Option<RewiringSummary>,
        verdict: EvaluationVerdict,
        memoized_origin: MemoizedResultOrigin,
        reuse_basis: ReuseBasis,
    ) -> Self {
        Self {
            task_index,
            node,
            identity,
            before_state,
            before_artifact_state,
            after_state,
            dependency_updates,
            recomputed,
            partition_aware,
            rewiring,
            verdict,
            memoized_origin,
            reuse_basis,
        }
    }

    fn into_parts(
        self,
    ) -> (
        usize,
        NodeId,
        StageSemanticIdentity,
        NodeState,
        Option<RuntimeArtifactFinalizeImage>,
        NodeState,
        u32,
        bool,
        bool,
        Option<RewiringSummary>,
        EvaluationVerdict,
        MemoizedResultOrigin,
        ReuseBasis,
    ) {
        (
            self.task_index,
            self.node,
            self.identity,
            self.before_state,
            self.before_artifact_state,
            self.after_state,
            self.dependency_updates,
            self.recomputed,
            self.partition_aware,
            self.rewiring,
            self.verdict,
            self.memoized_origin,
            self.reuse_basis,
        )
    }
}

#[cfg(feature = "parallel")]
impl SemanticSegment {
    fn single(update: SemanticTaskUpdate) -> Self {
        Self {
            id: update.identity.segment_id,
            task_range: SemanticTaskRange {
                start: update.identity.record_id,
                end: update.identity.record_id,
            },
            updates: vec![update],
        }
    }

    fn into_updates(self) -> Vec<SemanticTaskUpdate> {
        self.updates
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

#[cfg(feature = "parallel")]
pub(super) fn segment_for_single_update(update: SemanticTaskUpdate) -> SemanticSegment {
    SemanticSegment::single(update)
}

#[cfg(feature = "parallel")]
pub(super) fn finalize_stage_batch(
    graph: &mut SignalGraph,
    stage_tasks: &[EligibleTask],
    batch: StageSemanticBatch,
    report: &mut ExecutionReport,
    stage_record: &mut StageExecutionRecord,
) -> Result<(), SignalError> {
    if batch.is_empty() {
        return Ok(());
    }

    let mut segments = batch.into_segments();
    if !segments_are_sorted(segments.as_slice()) {
        segments.sort_by_key(|segment| (segment.task_range.start.0, segment.id.0));
    }
    let Some(first_segment) = segments.first() else {
        return Err(SignalError::internal(
            "semantic finalize expected at least one segment after non-empty batch validation",
        ));
    };
    let Some(last_segment) = segments.last() else {
        return Err(SignalError::internal(
            "semantic finalize expected a tail segment after non-empty batch validation",
        ));
    };
    stage_record.semantic_segment_count = segments.len() as u32;
    report.semantic_segment_count += segments.len() as u32;
    stage_record.semantic_task_range = Some(SemanticTaskRange {
        start: first_segment.task_range.start,
        end: last_segment.task_range.end,
    });

    let mut task_records = Vec::with_capacity(stage_tasks.len());
    for segment in segments {
        for update in segment.into_updates() {
            let (
                task_index,
                node,
                identity,
                before_state,
                before_artifact_state,
                after_state,
                dependency_updates,
                recomputed,
                partition_aware,
                rewiring,
                verdict,
                memoized_origin,
                reuse_basis,
            ) = update.into_parts();
            let after_finalize_image = graph.node_runtime_artifact_finalize_image(node)?;
            if let Some(after_finalize_image) = after_finalize_image.as_ref() {
                stamp_trace_summary_and_record_lineage_transition_from_image(
                    graph,
                    node,
                    before_artifact_state.as_ref(),
                    after_finalize_image,
                    identity.record_id,
                    identity.segment_id,
                )?;
            }
            let task = &stage_tasks[task_index];
            let task_record = classify_task_execution_record(
                identity.record_id,
                identity.segment_id,
                task,
                before_state,
                after_state,
                before_artifact_state.as_ref(),
                after_finalize_image.as_ref(),
                verdict,
                memoized_origin,
                reuse_basis,
            );
            record_semantic_update(
                graph,
                report,
                &task_record,
                dependency_updates,
                recomputed,
                partition_aware,
            );
            task_records.push(task_record);
            record_semantic_artifacts(graph, node, rewiring.as_ref())?;
        }
    }
    if !task_records_are_sorted(task_records.as_slice()) {
        task_records.sort_by_key(|record| record.id.0);
    }
    stage_record.task_records = task_records;
    Ok(())
}

pub(in crate::logic::planner) fn finalize_serial_stage_batch(
    graph: &mut SignalGraph,
    batch: ReadySerialFinalizeBatch,
    report: &mut ExecutionReport,
    stage_record: &mut StageExecutionRecord,
) -> Result<FinalizedSerialStageBatch, SignalError> {
    let (stage_tasks, finalize_seeds, applied_tasks, _stage_order) = batch.into_parts();

    if finalize_seeds.is_empty() {
        let empty_range = SemanticTaskRange {
            start: ExecutionRecordId(0),
            end: ExecutionRecordId(0),
        };
        return Ok(FinalizedSerialStageBatch::new(empty_range, Vec::new(), 0));
    }
    let Some(first_seed) = finalize_seeds.first() else {
        return Err(SignalError::internal(
            "serial finalize expected a first seed after non-empty seed validation",
        ));
    };
    let Some(last_seed) = finalize_seeds.last() else {
        return Err(SignalError::internal(
            "serial finalize expected a last seed after non-empty seed validation",
        ));
    };

    let semantic_task_range = SemanticTaskRange {
        start: first_seed.identity.record_id,
        end: last_seed.identity.record_id,
    };
    let mut task_records = Vec::with_capacity(applied_tasks.len());

    for (seed, applied) in finalize_seeds.into_iter().zip(applied_tasks.into_iter()) {
        if seed.node != applied.node {
            return Err(SignalError::internal(
                "serial finalize proof was violated: stage-ordered seed and applied node diverged",
            ));
        }
        let after_finalize_image = graph.node_runtime_artifact_finalize_image(applied.node)?;
        if let Some(after_finalize_image) = after_finalize_image.as_ref() {
            stamp_trace_summary_and_record_lineage_transition_from_image(
                graph,
                applied.node,
                seed.before_artifact_state.as_ref(),
                after_finalize_image,
                seed.identity.record_id,
                seed.identity.segment_id,
            )?;
        }
        let task = &stage_tasks[seed.task_index];
        let task_record = classify_task_execution_record(
            seed.identity.record_id,
            seed.identity.segment_id,
            task,
            seed.before_state,
            applied.after_state,
            seed.before_artifact_state.as_ref(),
            after_finalize_image.as_ref(),
            applied.verdict,
            applied.memoized_origin,
            applied.reuse_basis,
        );
        record_semantic_update(
            graph,
            report,
            &task_record,
            seed.dependency_updates,
            seed.recomputed,
            seed.partition_aware,
        );
        record_semantic_artifacts(graph, applied.node, seed.rewiring.as_ref())?;
        task_records.push(task_record);
    }

    let semantic_segment_count = task_records.len() as u32;
    stage_record.semantic_segment_count = semantic_segment_count;
    Ok(FinalizedSerialStageBatch::new(
        semantic_task_range,
        task_records,
        semantic_segment_count,
    ))
}

#[cfg(feature = "parallel")]
fn segments_are_sorted(segments: &[SemanticSegment]) -> bool {
    segments.windows(2).all(|pair| {
        pair[0].task_range.start.0 < pair[1].task_range.start.0
            || (pair[0].task_range.start.0 == pair[1].task_range.start.0
                && pair[0].id.0 <= pair[1].id.0)
    })
}

#[cfg(feature = "parallel")]
fn task_records_are_sorted(records: &[super::types::TaskExecutionRecord]) -> bool {
    records.windows(2).all(|pair| pair[0].id.0 <= pair[1].id.0)
}
