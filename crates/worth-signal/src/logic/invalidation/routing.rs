mod application;
mod counters;
mod evidence;
mod planning;
mod seeds;

use std::ops::DerefMut;

use crate::data::aspect::Aspect;
use crate::data::error::SignalError;
use crate::data::graph::{ScratchLeaseKind, SignalGraph, TraversalScratch};
#[cfg(any(test, doctest))]
use crate::data::handle::NodeId;
use crate::data::node::NodeState;
#[cfg(any(test, doctest))]
use crate::data::output::ChangedRegion;
use crate::data::proof::{
    DirtyBatch, FrontierEntryClassification, FrontierPlan, FrontierWaveEntryPlan, InvalidationSeed,
    InvalidationSeedBatch, InvalidationTraceRecord, SemanticBatchCommit,
};
use crate::diagnostics::failure::{ExecutionFailureContext, ExecutionFailurePhase};
use crate::diagnostics::lineage::InvalidationCause;
use crate::diagnostics::recorder::{record_invalidation_lineage, DiagnosticsRecorder};

#[cfg(any(test, doctest))]
pub fn mark_dirty(
    mut graph: impl DerefMut<Target = SignalGraph>,
    source: NodeId,
    changed_aspect: Aspect,
) -> Result<(), SignalError> {
    let _ = mark_dirty_batch(
        graph.deref_mut(),
        &DirtyBatch::singleton(source, changed_aspect, Vec::<ChangedRegion>::new()),
    )?;
    Ok(())
}

#[cfg(any(test, doctest))]
pub fn mark_dirty_with_regions(
    mut graph: impl DerefMut<Target = SignalGraph>,
    source: NodeId,
    changed_aspect: Aspect,
    changed_regions: &[ChangedRegion],
) -> Result<(), SignalError> {
    let _ = mark_dirty_batch(
        graph.deref_mut(),
        &DirtyBatch::singleton(source, changed_aspect, changed_regions.to_vec()),
    )?;
    Ok(())
}

pub fn mark_dirty_batch(
    mut graph: impl DerefMut<Target = SignalGraph>,
    dirty: &DirtyBatch,
) -> Result<SemanticBatchCommit, SignalError> {
    let graph = graph.deref_mut();
    graph.telemetry_mut().invalidation.batch_width += dirty.as_slice().len() as u64;
    for entry in dirty.as_slice() {
        graph.note_change_input(
            entry.source,
            entry.changed_aspect,
            entry.changed_regions.as_slice(),
        );
    }

    let result = graph.with_scratch(ScratchLeaseKind::Invalidation, |graph, scratch| {
        let scratch = scratch.traversal_mut();
        let plan = plan_invalidation_frontier(graph, dirty)?;
        let mut summary = execute_invalidation_frontier(graph, scratch, &plan)?;
        let trace_records = retained_trace_records(graph, &plan)?;
        summary.counters.frontier_trace_retained_count = trace_records.len() as u64;
        graph
            .telemetry_mut()
            .invalidation
            .partition_scoped_invalidation_checks += plan.predicted.partition_scoped_checks;
        graph
            .telemetry_mut()
            .invalidation
            .frontier_trace_retained_count += summary.counters.frontier_trace_retained_count;
        graph.record_frontier_execution_diagnostics(summary, trace_records);
        Ok(())
    });

    if let Err(err) = result {
        graph.clear_pending_diagnostics_input();
        DiagnosticsRecorder::new(graph).record_failure(ExecutionFailureContext::from_error(
            ExecutionFailurePhase::Invalidation,
            &err,
            None,
        ));
        return Err(err);
    }
    Ok(SemanticBatchCommit::new(dirty.clone()))
}

fn plan_invalidation_frontier(
    graph: &mut SignalGraph,
    dirty: &DirtyBatch,
) -> Result<FrontierPlan, SignalError> {
    planning::plan_invalidation_frontier(graph, dirty)
}

fn execute_invalidation_frontier(
    graph: &mut SignalGraph,
    scratch: &mut TraversalScratch,
    plan: &FrontierPlan,
) -> Result<crate::data::proof::FrontierExecutionSummary, SignalError> {
    application::execute_invalidation_frontier(graph, scratch, plan)
}

fn retained_trace_records(
    graph: &SignalGraph,
    plan: &FrontierPlan,
) -> Result<Vec<InvalidationTraceRecord>, SignalError> {
    evidence::retained_trace_records(graph, plan)
}

fn apply_direct_entry(
    graph: &mut SignalGraph,
    entry: &FrontierWaveEntryPlan,
    aspect: Aspect,
    seed_batch: &InvalidationSeedBatch,
) -> Result<(), SignalError> {
    let previous_state = graph.get_state(entry.node)?;
    match entry.classification {
        FrontierEntryClassification::DirectDirty => {
            graph.transition_node_dirty(entry.node, aspect, entry.narrowed_scopes.as_slice())?
        }
        FrontierEntryClassification::MaybeStale => {
            graph.transition_node_maybe_stale(entry.node, aspect)?
        }
    }
    if matches!(previous_state, NodeState::Clean) {
        record_invalidation_lineage(
            graph,
            entry.node,
            InvalidationCause::DirectDependencyChanged {
                dependency: entry
                    .source_seed_refs
                    .first()
                    .copied()
                    .and_then(|idx| seed_batch.as_slice().get(idx as usize))
                    .map(|seed| seed.source_node)
                    .unwrap_or(entry.node),
                aspect_index: aspect.index(),
            },
        );
    }
    Ok(())
}

fn mark_source_seed(graph: &mut SignalGraph, seed: &InvalidationSeed) -> Result<(), SignalError> {
    let source_was_clean = matches!(graph.get_state(seed.source_node)?, NodeState::Clean);
    graph.transition_node_dirty(
        seed.source_node,
        seed.aspect,
        seed.changed_scopes.as_slice(),
    )?;
    if source_was_clean {
        record_invalidation_lineage(
            graph,
            seed.source_node,
            InvalidationCause::SourceAspectChanged {
                aspect_index: seed.aspect.index(),
            },
        );
    }
    Ok(())
}
