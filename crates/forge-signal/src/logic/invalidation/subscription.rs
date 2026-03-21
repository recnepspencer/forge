use crate::data::aspect::{Aspect, AspectMask};
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::data::output::{
    scopes_overlap, InternedPartitionSubscription, PartitionMatchMode, PartitionSubscription,
};
use crate::data::proof::{FrontierEntryClassification, FrontierInclusionBasis};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct SubscriptionInvalidationEvidence {
    pub classification: FrontierEntryClassification,
    pub inclusion_basis: FrontierInclusionBasis,
    pub partition_scoped_checks: u64,
}

pub(super) fn subscriber_invalidation_evidence(
    graph: &mut SignalGraph,
    downstream: NodeId,
    source: NodeId,
    changed_aspect: Aspect,
    changed_scopes: &[PartitionSubscription],
    changed_scope_ids: &[InternedPartitionSubscription],
) -> Result<Option<SubscriptionInvalidationEvidence>, SignalError> {
    let changed_mask = AspectMask::from_aspect(changed_aspect);
    let dependencies = graph.runtime_dependencies_of(downstream)?;
    let source_key = (source.index(), source.generation());
    let start =
        dependencies.partition_point(|dep| (dep.source().index(), dep.source().generation()) < source_key);
    let end =
        dependencies.partition_point(|dep| (dep.source().index(), dep.source().generation()) <= source_key);

    let mut partition_checks = 0_u64;
    let mut fallback_unmatched = false;
    for dep in &dependencies[start..end] {
        if !dep.aspect_mask().intersects(changed_mask) {
            continue;
        }
        let Some(scope) = dep.scope_ref() else {
            return Ok(Some(SubscriptionInvalidationEvidence {
                classification: FrontierEntryClassification::DirectDirty,
                inclusion_basis: FrontierInclusionBasis::DirectSubscriptionMatch,
                partition_scoped_checks: partition_checks,
            }));
        };

        partition_checks += 1;
        if let Some(interned_scope) = dep.interned_scope() {
            for changed_scope_id in changed_scope_ids {
                if scopes_overlap(&interned_scope, changed_scope_id) {
                    return Ok(Some(SubscriptionInvalidationEvidence {
                        classification: FrontierEntryClassification::DirectDirty,
                        inclusion_basis: match scope.match_mode {
                            PartitionMatchMode::WholePartition => {
                                FrontierInclusionBasis::PartitionScopeOverlap
                            }
                            PartitionMatchMode::PartitionAndDetail => {
                                FrontierInclusionBasis::DetailScopeOverlap
                            }
                        },
                        partition_scoped_checks: partition_checks,
                    }));
                }
            }
            fallback_unmatched = true;
            continue;
        }

        for changed_scope in changed_scopes {
            if scopes_overlap(scope, changed_scope) {
                return Ok(Some(SubscriptionInvalidationEvidence {
                    classification: FrontierEntryClassification::DirectDirty,
                    inclusion_basis: match scope.match_mode {
                        PartitionMatchMode::WholePartition => {
                            FrontierInclusionBasis::PartitionScopeOverlap
                        }
                        PartitionMatchMode::PartitionAndDetail => {
                            FrontierInclusionBasis::DetailScopeOverlap
                        }
                    },
                    partition_scoped_checks: partition_checks,
                }));
            }
        }
        fallback_unmatched = true;
    }

    if fallback_unmatched {
        return Ok(Some(SubscriptionInvalidationEvidence {
            classification: FrontierEntryClassification::MaybeStale,
            inclusion_basis: FrontierInclusionBasis::DirectSubscriptionMatch,
            partition_scoped_checks: partition_checks,
        }));
    }

    Ok(None)
}
