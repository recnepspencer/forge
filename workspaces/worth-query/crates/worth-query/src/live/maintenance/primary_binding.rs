use crate::basis_lifecycle::BasisOperationLane;
use crate::domain_installation::{
    WorthQueryLiveBoundDomainProjection, WorthQueryOperationAuthorityBasis,
    WorthQuerySettledDomainProjection,
};

#[path = "primary_binding/dependency_index.rs"]
mod dependency_index;

/// Query-owned admission binding between one current live projection and one
/// exact primary application runtime.
///
/// The primary installation proves only runtime provenance. Query still
/// validates operation meaning, installed dependency membership, locality,
/// consumer authority, and publication currentness for every delivery.
pub struct WorthQueryPrimaryRuntimeInvalidationBinding {
    primary:
        worth_query_execution::facade::primary_graph::WorthQueryGranularInvalidationInstallation,
    query: WorthQueryOperationAuthorityBasis,
    consumer_dependencies: Vec<worth_runtime_bridge::facade::BridgeSemanticDependencyCandidate>,
    dependency_index: dependency_index::WorthQueryBoundPrimaryDependencyIndex,
    dependency_inventory_complete: bool,
}

pub fn bind_primary_runtime_granular_invalidations<
    D: 'static,
    O: 'static,
    F: 'static,
    L: BasisOperationLane,
>(
    live: &WorthQueryLiveBoundDomainProjection<D, O, F, L>,
    primary: worth_query_execution::facade::primary_graph::WorthQueryGranularInvalidationInstallation,
) -> WorthQueryPrimaryRuntimeInvalidationBinding {
    let query = live
        .snapshot()
        .semantic_aspect_dependency_closure()
        .affinity
        .clone();
    let consumer_dependencies = installed_consumer_dependencies(live.snapshot());
    let dependency_inventory_complete =
        dependency_inventory_complete(live.snapshot(), &consumer_dependencies);
    let dependency_index =
        dependency_index::WorthQueryBoundPrimaryDependencyIndex::build(&consumer_dependencies);
    WorthQueryPrimaryRuntimeInvalidationBinding {
        primary,
        query,
        consumer_dependencies,
        dependency_index,
        dependency_inventory_complete,
    }
}

pub fn bind_shared_primary_runtime_granular_invalidations<
    D: 'static,
    O: 'static,
    F: 'static,
    L: BasisOperationLane,
>(
    lease: &crate::domain_installation::WorthQuerySharedLiveProjectionLease<D, O, F, L>,
    primary: worth_query_execution::facade::primary_graph::WorthQueryGranularInvalidationInstallation,
) -> WorthQueryPrimaryRuntimeInvalidationBinding {
    let query = lease
        .snapshot()
        .semantic_aspect_dependency_closure()
        .affinity
        .clone();
    let consumer_dependencies = installed_consumer_dependencies(lease.snapshot());
    let dependency_inventory_complete =
        dependency_inventory_complete(lease.snapshot(), &consumer_dependencies);
    let dependency_index =
        dependency_index::WorthQueryBoundPrimaryDependencyIndex::build(&consumer_dependencies);
    WorthQueryPrimaryRuntimeInvalidationBinding {
        primary,
        query,
        consumer_dependencies,
        dependency_index,
        dependency_inventory_complete,
    }
}

impl WorthQueryPrimaryRuntimeInvalidationBinding {
    pub(crate) fn consumer_dependencies_for<'a>(
        &'a self,
        delivered: &worth_runtime_bridge::facade::BridgeSemanticDependencyCandidate,
        changes: &[worth_runtime_bridge::facade::BridgeDeliveredCorrespondenceChange],
    ) -> (
        Vec<&'a worth_runtime_bridge::facade::BridgeSemanticDependencyCandidate>,
        usize,
    ) {
        let (indices, probes) = self.dependency_index.lookup(delivered, changes);
        (
            indices
                .into_iter()
                .map(|index| &self.consumer_dependencies[index])
                .collect(),
            probes,
        )
    }

    pub(crate) fn readmits_workspace(
        &self,
        workspace: &crate::runtime::WorthQueryWorkspace,
    ) -> bool {
        workspace.is_attached_to_primary_runtime(&self.primary)
    }

    pub(crate) fn readmits<D, O, F, L: BasisOperationLane>(
        &self,
        current: &WorthQuerySettledDomainProjection<D, O, F, L>,
        batch: &worth_query_execution::facade::primary_graph::WorthQueryGranularInvalidationDeliveryBatch,
    ) -> bool {
        self.query == current.semantic_aspect_dependency_closure().affinity
            && self.dependency_inventory_complete
            && self.primary.admits_batch(batch)
    }
}

fn dependency_inventory_complete<D, O, F, L: BasisOperationLane>(
    current: &WorthQuerySettledDomainProjection<D, O, F, L>,
    installed: &[worth_runtime_bridge::facade::BridgeSemanticDependencyCandidate],
) -> bool {
    let manifest = current
        .semantic_aspect_dependency_closure()
        .invalidation_manifest();
    let mut identities = std::collections::BTreeSet::new();
    installed.len() == manifest.conditional_truth_count()
        && installed.iter().all(|dependency| {
            let location =
                crate::domain_installation::query_location_from_bridge_candidate(dependency);
            identities.insert((location.clone(), dependency.dependency_ordinal()))
                && manifest.admits_bridge_dependency(&location, dependency)
        })
}

fn installed_consumer_dependencies<D, O, F, L: BasisOperationLane>(
    current: &WorthQuerySettledDomainProjection<D, O, F, L>,
) -> Vec<worth_runtime_bridge::facade::BridgeSemanticDependencyCandidate> {
    current
        .conditional_provenance()
        .iter()
        .flat_map(|provenance| provenance._lowering.semantic_dependencies().cloned())
        .collect()
}
