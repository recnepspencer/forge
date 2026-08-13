use crate::data::handle::NodeId;
use crate::data::output::{scopes_overlap, PartitionSubscription};
use crate::data::proof::invalidation::binding::{
    DependencyRevision, OutputCommitOrdinal, ResolvedDependencyCause,
};
use crate::data::proof::invalidation::output_commit::ProducedAspectChange;
use crate::data::proof::PartitionScopeSet;

#[derive(Clone, Copy)]
pub(crate) struct CauseAdmissionContext {
    pub(crate) graph_instance: u64,
    pub(crate) consumer: NodeId,
    pub(crate) revision: DependencyRevision,
    pub(crate) producer: NodeId,
    pub(crate) output_commit_ordinal: OutputCommitOrdinal,
}

pub(crate) fn changed_scopes_for_edge(
    change: &ProducedAspectChange,
    edge_scope: Option<&PartitionSubscription>,
) -> Option<PartitionScopeSet> {
    let Some(edge_scope) = edge_scope else {
        return Some(PartitionScopeSet::default());
    };
    if change.changed_scopes.is_empty() {
        return Some(PartitionScopeSet::new([edge_scope.clone()]));
    }
    change
        .changed_scopes
        .iter()
        .any(|changed| scopes_overlap(changed, edge_scope))
        .then(|| PartitionScopeSet::new([edge_scope.clone()]))
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn reconcile_edge_cause(
    causes: &mut Vec<ResolvedDependencyCause>,
    context: CauseAdmissionContext,
    aspect: crate::data::aspect::Aspect,
    edge_scope: Option<PartitionSubscription>,
    cached_version: u64,
    committed_version: u64,
    changed_scopes: PartitionScopeSet,
    meaningful: bool,
) {
    let existing = causes.iter().position(|cause| {
        cause.key.producer == context.producer
            && cause.key.aspect == aspect
            && cause.key.edge_scope == edge_scope
    });
    let prior_scopes = existing.map(|index| causes[index].changed_scopes.clone());
    if let Some(index) = existing {
        causes.remove(index);
    }
    let Some(reconciled_scopes) =
        reconcile_changed_scopes(prior_scopes.as_ref(), &changed_scopes, meaningful)
    else {
        return;
    };
    causes.push(ResolvedDependencyCause::new(
        context.graph_instance,
        context.consumer,
        context.revision,
        context.producer,
        aspect,
        edge_scope,
        cached_version,
        context.output_commit_ordinal,
        committed_version,
        reconciled_scopes,
    ));
}

fn reconcile_changed_scopes(
    prior: Option<&PartitionScopeSet>,
    touched: &PartitionScopeSet,
    meaningful: bool,
) -> Option<PartitionScopeSet> {
    if touched.is_empty() {
        return meaningful.then(PartitionScopeSet::default);
    }
    if prior.is_some_and(PartitionScopeSet::is_empty) {
        return Some(PartitionScopeSet::default());
    }
    let retained = prior
        .into_iter()
        .flat_map(PartitionScopeSet::iter)
        .filter(|scope| {
            !touched
                .iter()
                .any(|changed| scopes_overlap(*scope, changed))
        })
        .cloned();
    let scopes = retained
        .chain(
            meaningful
                .then_some(touched)
                .into_iter()
                .flat_map(PartitionScopeSet::iter)
                .cloned(),
        )
        .collect::<Vec<_>>();
    (!scopes.is_empty()).then(|| PartitionScopeSet::new(scopes))
}
