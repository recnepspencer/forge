use crate::data::aspect::Aspect;
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::data::output::PartitionSubscription;
use crate::logic::explain::{RewiringDependency, RewiringSummary};
use crate::logic::prepared::PreparedDependencyCapture;

pub(crate) fn rewiring_summary_from_capture(
    graph: &SignalGraph,
    node: NodeId,
    capture: &PreparedDependencyCapture,
) -> Result<Option<RewiringSummary>, SignalError> {
    let current_dependencies = graph
        .dependencies_of(node)?
        .iter()
        .map(|dependency| {
            (
                dependency.source(),
                dependency.aspect(),
                dependency.scope_ref().cloned(),
            )
        })
        .collect::<Vec<_>>();
    let next_dependencies = capture
        .as_slice()
        .iter()
        .map(|dependency| {
            (
                dependency.source,
                dependency.aspect,
                dependency.scope.clone(),
            )
        })
        .collect::<Vec<_>>();
    Ok(rewiring_summary_from_sets(
        &current_dependencies,
        &next_dependencies,
    ))
}

fn rewiring_summary_from_sets(
    current_dependencies: &[(NodeId, Aspect, Option<PartitionSubscription>)],
    next_dependencies: &[(NodeId, Aspect, Option<PartitionSubscription>)],
) -> Option<RewiringSummary> {
    let mut added = next_dependencies
        .iter()
        .filter(|candidate| !current_dependencies.contains(candidate))
        .map(|(source, aspect, subscription)| RewiringDependency {
            source: *source,
            aspect: *aspect,
            subscription: subscription.clone(),
        })
        .collect::<Vec<_>>();
    let mut removed = current_dependencies
        .iter()
        .filter(|candidate| !next_dependencies.contains(candidate))
        .map(|(source, aspect, subscription)| RewiringDependency {
            source: *source,
            aspect: *aspect,
            subscription: subscription.clone(),
        })
        .collect::<Vec<_>>();

    if added.is_empty() && removed.is_empty() {
        None
    } else {
        added.sort_by_key(rewiring_dependency_key);
        removed.sort_by_key(rewiring_dependency_key);
        Some(RewiringSummary { added, removed })
    }
}

fn rewiring_dependency_key(dependency: &RewiringDependency) -> (u32, u32, usize, String, u8) {
    let scope = dependency.subscription.as_ref().map(|subscription| {
        (
            subscription.detail.clone().unwrap_or_default(),
            subscription.match_mode as u8,
        )
    });
    (
        dependency.source.index(),
        dependency.source.generation(),
        dependency.aspect.index(),
        scope
            .as_ref()
            .map(|(detail, _)| detail.clone())
            .unwrap_or_default(),
        scope.as_ref().map(|(_, mode)| *mode).unwrap_or_default(),
    )
}
