use crate::data::aspect::{Aspect, AspectMask, MAX_ASPECTS};
use crate::data::comparator::ComparatorPolicyResolver;
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;

use super::{InstalledSignalConditionalContract, SignalConditionalDecisionCounters};

#[derive(Clone)]
pub(super) struct SignalConditionalDependencyVersion {
    pub(super) node: NodeId,
    pub(super) aspect: Aspect,
    pub(super) scope: Option<crate::data::output::PartitionSubscription>,
    pub(super) version: u64,
}

pub(super) fn observed_dependency_versions(
    graph: &SignalGraph,
    contract: &InstalledSignalConditionalContract,
) -> Result<Vec<SignalConditionalDependencyVersion>, SignalError> {
    let mut coordinates = dependency_aspects(contract.dependency_aspects())
        .into_iter()
        .map(|aspect| (contract.node(), aspect, None))
        .collect::<Vec<_>>();
    coordinates.extend(
        graph
            .get_dep_snapshot(contract.node())?
            .entries()
            .iter()
            .map(|entry| (entry.source, entry.aspect, entry.scope.clone())),
    );
    coordinates.sort_by(|left, right| {
        (left.0.index(), left.0.generation(), left.1.index(), &left.2).cmp(&(
            right.0.index(),
            right.0.generation(),
            right.1.index(),
            &right.2,
        ))
    });
    coordinates.dedup();
    coordinates
        .into_iter()
        .map(|(node, aspect, scope)| {
            Ok(SignalConditionalDependencyVersion {
                node,
                aspect,
                version: graph.node_version_for_scope(node, aspect, scope.as_ref())?,
                scope,
            })
        })
        .collect()
}

pub(super) fn dependency_change_is_meaningful(
    graph: &SignalGraph,
    contract: &InstalledSignalConditionalContract,
    resolver: &mut impl ComparatorPolicyResolver,
    counters: &mut SignalConditionalDecisionCounters,
) -> Result<bool, SignalError> {
    if external_dependency_change_is_meaningful(graph, contract, resolver, counters)? {
        return Ok(true);
    }
    let snapshot = graph.get_dep_snapshot(contract.node())?;
    if snapshot.entries().is_empty() {
        return Ok(contract.dependency_aspects().is_empty());
    }
    for entry in snapshot.entries() {
        counters.dependency_version_checks += 1;
        let current =
            graph.node_version_for_scope(entry.source, entry.aspect, entry.scope.as_ref())?;
        counters.comparator_checks += 1;
        if contract.dependency_comparator().has_meaningful_change(
            entry.aspect,
            entry.cached_version,
            current,
            resolver,
        )? {
            return Ok(true);
        }
    }
    Ok(false)
}

fn external_dependency_change_is_meaningful(
    graph: &SignalGraph,
    contract: &InstalledSignalConditionalContract,
    resolver: &mut impl ComparatorPolicyResolver,
    counters: &mut SignalConditionalDecisionCounters,
) -> Result<bool, SignalError> {
    let aspects = dependency_aspects(contract.dependency_aspects());
    if aspects.is_empty() {
        return Ok(false);
    }
    let Some(cached) = graph.conditional_dependency_versions.get(&contract.node()) else {
        counters.dependency_version_checks += aspects.len();
        return Ok(true);
    };
    for (index, aspect) in aspects.into_iter().enumerate() {
        counters.dependency_version_checks += 1;
        let current = graph.node_version_for_scope(contract.node(), aspect, None)?;
        counters.comparator_checks += 1;
        if contract.dependency_comparator().has_meaningful_change(
            aspect,
            cached[index],
            current,
            resolver,
        )? {
            return Ok(true);
        }
    }
    Ok(false)
}

pub(super) fn record_dependency_versions(
    graph: &mut SignalGraph,
    contract: &InstalledSignalConditionalContract,
) -> Result<(), SignalError> {
    let versions = dependency_aspects(contract.dependency_aspects())
        .into_iter()
        .map(|aspect| graph.node_version_for_scope(contract.node(), aspect, None))
        .collect::<Result<Vec<_>, _>>()?;
    graph
        .conditional_dependency_versions
        .insert(contract.node(), versions);
    Ok(())
}

fn dependency_aspects(mask: AspectMask) -> Vec<Aspect> {
    (0..MAX_ASPECTS)
        .filter_map(|index| Aspect::try_new(index as u8))
        .filter(|aspect| mask.intersects((*aspect).into()))
        .collect()
}
