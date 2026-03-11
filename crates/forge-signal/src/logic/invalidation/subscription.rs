use crate::data::aspect::{Aspect, AspectMask};
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::data::output::{
    scopes_overlap, InternedPartitionSubscription, PartitionMatchMode, PartitionSubscription,
};

#[derive(Debug, Clone, Copy)]
pub(super) enum SubscriptionDirtyMatch {
    WholeAspect,
    WholePartition,
    PartitionAndDetail,
    Unmatched,
}

pub(super) fn subscribes_to_aspect(
    graph: &mut SignalGraph,
    downstream: NodeId,
    source: NodeId,
    changed_aspect: Aspect,
    changed_scopes: &[PartitionSubscription],
    changed_scope_ids: &[InternedPartitionSubscription],
) -> Result<SubscriptionDirtyMatch, SignalError> {
    let changed_mask = AspectMask::from_aspect(changed_aspect);
    let (partition_checks, outcome) = {
        let dependencies = graph.runtime_dependencies_of(downstream)?;
        let mut outcome = SubscriptionDirtyMatch::Unmatched;
        let mut partition_checks = 0_u64;
        let source_key = (source.index(), source.generation());
        let start = dependencies
            .partition_point(|dep| (dep.source().index(), dep.source().generation()) < source_key);
        let end = dependencies
            .partition_point(|dep| (dep.source().index(), dep.source().generation()) <= source_key);

        for dep in &dependencies[start..end] {
            if !dep.aspect_mask().intersects(changed_mask) {
                continue;
            }
            let Some(scope) = dep.scope_ref() else {
                outcome = SubscriptionDirtyMatch::WholeAspect;
                break;
            };
            partition_checks += 1;
            if let Some(interned_scope) = dep.interned_scope() {
                for changed_scope_id in changed_scope_ids {
                    if scopes_overlap(&interned_scope, changed_scope_id) {
                        outcome = match scope.match_mode {
                            PartitionMatchMode::WholePartition => {
                                SubscriptionDirtyMatch::WholePartition
                            }
                            PartitionMatchMode::PartitionAndDetail => {
                                SubscriptionDirtyMatch::PartitionAndDetail
                            }
                        };
                        break;
                    }
                }
            } else {
                for changed_scope in changed_scopes {
                    if scopes_overlap(scope, changed_scope) {
                        outcome = match scope.match_mode {
                            PartitionMatchMode::WholePartition => {
                                SubscriptionDirtyMatch::WholePartition
                            }
                            PartitionMatchMode::PartitionAndDetail => {
                                SubscriptionDirtyMatch::PartitionAndDetail
                            }
                        };
                        break;
                    }
                }
            }
            if !matches!(outcome, SubscriptionDirtyMatch::Unmatched) {
                break;
            }
        }
        (partition_checks, outcome)
    };

    graph.telemetry_mut().partition_scoped_invalidation_checks += partition_checks;
    Ok(outcome)
}
