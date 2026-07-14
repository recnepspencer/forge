use std::collections::BTreeMap;
use std::sync::Arc;

use crate::merge::data::{StrategyConflictClass, StrategyConflictEvidence};

pub(super) fn strategy_conflict_evidence(
    runtime: &crate::logic::runtime::RelationalRuntime,
    source_delta: Option<&crate::merge::data::BranchTouchedRecordDelta>,
    target_delta: Option<&crate::merge::data::BranchTouchedRecordDelta>,
) -> Option<StrategyConflictEvidence> {
    let source_descriptors = strategy_descriptors_for_delta(runtime, source_delta);
    let target_descriptors = strategy_descriptors_for_delta(runtime, target_delta);
    if source_descriptors.is_empty() && target_descriptors.is_empty() {
        return None;
    }
    let class = match (source_descriptors.is_empty(), target_descriptors.is_empty()) {
        (false, false) => {
            let any_same_descriptor = source_descriptors.iter().any(|source_descriptor| {
                target_descriptors.iter().any(|target_descriptor| {
                    target_descriptor.descriptor_digest() == source_descriptor.descriptor_digest()
                })
            });
            let any_same_scope = source_descriptors.iter().any(|source_descriptor| {
                target_descriptors.iter().any(|target_descriptor| {
                    strategy_scope_overlaps(source_descriptor, target_descriptor)
                })
            });
            if any_same_descriptor && any_same_scope {
                StrategyConflictClass::SameStrategyDivergentOutput
            } else if any_same_scope {
                StrategyConflictClass::DifferentStrategyOverlappingIntent
            } else {
                return None;
            }
        }
        (false, true) => StrategyConflictClass::SourceStrategyOnly,
        (true, false) => StrategyConflictClass::TargetStrategyOnly,
        (true, true) => return None,
    };
    Some(StrategyConflictEvidence {
        class,
        source_commit_ids: Arc::from(
            source_delta
                .map(|delta| delta.commit_ids.iter().copied().collect::<Vec<_>>())
                .unwrap_or_default(),
        ),
        target_commit_ids: Arc::from(
            target_delta
                .map(|delta| delta.commit_ids.iter().copied().collect::<Vec<_>>())
                .unwrap_or_default(),
        ),
        source_descriptors: Arc::from(source_descriptors),
        target_descriptors: Arc::from(target_descriptors),
    })
}

fn strategy_descriptors_for_delta(
    runtime: &crate::logic::runtime::RelationalRuntime,
    delta: Option<&crate::merge::data::BranchTouchedRecordDelta>,
) -> Vec<crate::commit_strategies::data::StrategyMergeDescriptor> {
    let Some(delta) = delta else {
        return Vec::new();
    };
    let history = runtime.history();
    let mut dedup = BTreeMap::<
        (
            [u8; 32],
            [u8; 32],
            Vec<worth_foundational::facade::AspectFieldLocator>,
        ),
        crate::commit_strategies::data::StrategyMergeDescriptor,
    >::new();
    for commit_id in delta.commit_ids.iter().rev().copied() {
        let Some(envelope) = history.commit_envelope(commit_id) else {
            continue;
        };
        let Some(strategy_artifacts) = envelope.strategy_artifacts.as_ref() else {
            continue;
        };
        let descriptor = strategy_artifacts.merge_descriptor().clone();
        let key = (
            descriptor.descriptor_digest().0,
            *descriptor.intent_scope_digest().bytes(),
            descriptor
                .intent_scope_targets()
                .iter()
                .cloned()
                .collect::<Vec<_>>(),
        );
        dedup.entry(key).or_insert(descriptor);
    }
    dedup.into_values().collect()
}

fn strategy_scope_overlaps(
    source: &crate::commit_strategies::data::StrategyMergeDescriptor,
    target: &crate::commit_strategies::data::StrategyMergeDescriptor,
) -> bool {
    if source.intent_scope_digest() == target.intent_scope_digest() {
        return true;
    }
    source.intent_scope_targets().iter().any(|field| {
        target
            .intent_scope_targets()
            .iter()
            .any(|other| other == field)
    })
}
