use crate::data::comparator::{ComparatorPolicyResolver, VersionComparatorPolicy};
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::logic::evaluation::{AppliedEffectReport, EvaluationEffect, PendingDependencySnapshot};

pub(super) fn apply_evaluation_effect(
    graph: &mut SignalGraph,
    effect: EvaluationEffect,
    comparator: VersionComparatorPolicy,
    comparator_resolver: &mut impl ComparatorPolicyResolver,
    defer_snapshot_commit: bool,
) -> Result<(AppliedEffectReport, Option<PendingDependencySnapshot>), SignalError> {
    graph.apply_effect(
        effect,
        comparator,
        comparator_resolver,
        defer_snapshot_commit,
    )
}
