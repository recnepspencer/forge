use std::collections::BTreeSet;

use crate::data::aspect::AspectVersion;
use crate::data::core_profile::StableHashValue;
use crate::data::dependency::DependencySnapshot;
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::data::output::{MemoizedResultOrigin, NodeEvaluationResult, OutputChange};
use crate::data::trace::TraceSummary;
use crate::logic::evaluation::EvaluationExecutionMetadata;
use crate::logic::prepared::PreparedEvaluation;

pub(super) fn build_trace_summary(
    graph: &SignalGraph,
    node: NodeId,
    result: &NodeEvaluationResult,
    snapshot: &DependencySnapshot,
    output_identity_unchanged: bool,
    execution_metadata: Option<EvaluationExecutionMetadata>,
    recomputed: bool,
) -> Result<TraceSummary, SignalError> {
    let mut changed_regions = result.changed_regions.clone();
    changed_regions.sort_by(|left, right| {
        (
            left.partition.0.as_str(),
            left.detail.as_deref().unwrap_or_default(),
        )
            .cmp(&(
                right.partition.0.as_str(),
                right.detail.as_deref().unwrap_or_default(),
            ))
    });
    changed_regions.dedup();
    let mut labels = result.labels.clone();
    labels.sort();
    labels.dedup();
    Ok(TraceSummary {
        output_hash: result
            .output_identity
            .as_ref()
            .map(trace_identity_hash)
            .unwrap_or_else(|| trace_output_hash(result.aspect_version)),
        output_identity: result.output_identity.clone(),
        output_change: normalize_output_change(
            result.output_change,
            output_identity_unchanged,
            result.output_identity.is_some(),
        ),
        recomputed,
        dependency_count: snapshot.entries().len() as u32,
        meaningful_input_changes: count_meaningful_input_changes(graph, node, snapshot)?,
        changed_partition_count: count_changed_partitions(&changed_regions),
        changed_regions,
        propagation_suppressed: false,
        keyed_family: execution_metadata.as_ref().and_then(|metadata| {
            metadata
                .keyed
                .as_ref()
                .map(|keyed| keyed.family.as_str().to_owned())
        }),
        keyed_key: execution_metadata.as_ref().and_then(|metadata| {
            metadata
                .keyed
                .as_ref()
                .map(|keyed| keyed.key.as_str().to_owned())
        }),
        memoized_origin: execution_metadata
            .map(|metadata| metadata.memoized_origin)
            .unwrap_or(MemoizedResultOrigin::DirectCompute),
        labels,
        execution_record_id: None,
        semantic_segment_id: None,
    })
}

fn count_meaningful_input_changes(
    graph: &SignalGraph,
    node: NodeId,
    snapshot: &DependencySnapshot,
) -> Result<u32, SignalError> {
    let mut changes = 0_u32;
    for (source, aspect, _cached_version, scope) in snapshot.entries() {
        let cached = graph
            .get_dep_snapshot(node)?
            .entries()
            .iter()
            .find(|(candidate_source, candidate_aspect, _, candidate_scope)| {
                candidate_source == source && candidate_aspect == aspect && candidate_scope == scope
            })
            .map(|(_, _, version, _)| *version);
        let Some(cached) = cached else {
            continue;
        };
        if !graph.is_alive(*source)
            || graph.get_entry(*source)?.get_aspect_version().get(*aspect) != cached
        {
            changes += 1;
        }
    }
    Ok(changes)
}

fn normalize_output_change(
    declared: OutputChange,
    output_identity_unchanged: bool,
    has_output_identity: bool,
) -> OutputChange {
    if has_output_identity && output_identity_unchanged {
        OutputChange::Unchanged
    } else {
        declared
    }
}

fn count_changed_partitions(changed_regions: &[crate::data::output::ChangedRegion]) -> u32 {
    changed_regions
        .iter()
        .map(|region| region.partition.clone())
        .collect::<BTreeSet<_>>()
        .len() as u32
}

fn trace_identity_hash(identity: &crate::data::output::OutputIdentity) -> StableHashValue {
    identity.stable_hash()
}

fn trace_output_hash(version: AspectVersion) -> StableHashValue {
    version
        .slots()
        .iter()
        .fold(0xcbf29ce484222325_u128, |hash, slot| {
            hash.wrapping_mul(0x100000001b3_u128) ^ (*slot as u128)
        }) as StableHashValue
}

pub(super) fn execution_metadata_for(
    prepared: &PreparedEvaluation,
) -> Option<EvaluationExecutionMetadata> {
    let keyed = prepared.keyed.as_ref()?;
    let (family, key) = keyed.family.clone().zip(keyed.key.clone())?;
    Some(EvaluationExecutionMetadata {
        keyed: Some(crate::data::output::KeyedComputation {
            family,
            key,
            memo_key: keyed.memo_key.clone(),
        }),
        memoized_origin: keyed.memoized_origin,
    })
}
