use std::collections::{BTreeMap, BTreeSet};

mod conditional_delivery;
mod controlled_fault;
mod owner_delivery_queue;
mod target_index;

use super::{WorthQueryLiveArtifactTarget, WorthQueryRuntime};
use owner_delivery_queue::WorthQueryInstalledOwnerDeliveryQueue;

pub(crate) use conditional_delivery::{
    WorthQueryAdmittedStagedOwnerDelivery, WorthQueryClassifiedOwnerDeliveryEmissionError,
};
pub(crate) use target_index::{
    WorthQueryInstalledTargetSelection, WorthQueryInstalledTargetSelectionWork,
};

type InstalledOperationKey = (std::any::TypeId, std::any::TypeId, std::any::TypeId);

struct WorthQueryInstalledOwnerDeliveryRoute {
    operation: InstalledOperationKey,
    impact_classifier: crate::domain_installation::WorthQueryInstalledLiveImpactClassifier,
    owner_deliveries: WorthQueryInstalledOwnerDeliveryQueue,
}

pub(super) enum WorthQueryInstalledLiveMutationClassification {
    Ordinary,
    InstalledUnaffected,
    Affected(crate::domain_installation::WorthQueryPreclassifiedInstalledLiveImpact),
}

impl WorthQueryInstalledLiveMutationClassification {
    pub(super) const fn is_installed_but_unaffected(&self) -> bool {
        matches!(self, Self::InstalledUnaffected)
    }

    pub(super) fn into_impact(
        self,
    ) -> Option<crate::domain_installation::WorthQueryPreclassifiedInstalledLiveImpact> {
        match self {
            Self::Affected(impact) => Some(impact),
            Self::Ordinary | Self::InstalledUnaffected => None,
        }
    }
}

#[derive(Default)]
pub(super) struct WorthQueryInstalledLiveRoutes {
    routes: BTreeMap<WorthQueryLiveArtifactTarget, WorthQueryInstalledOwnerDeliveryRoute>,
    target_index: target_index::WorthQueryInstalledLiveTargetIndex,
    injected_classified_emission_failures: usize,
}

impl WorthQueryInstalledLiveRoutes {
    pub(super) fn contains_target(&self, target: &WorthQueryLiveArtifactTarget) -> bool {
        self.routes.contains_key(target)
    }

    pub(super) fn affected_targets(
        &self,
        mutation: &crate::memory_workspace::WorthQueryMutationDelta,
    ) -> WorthQueryInstalledTargetSelection {
        self.target_index.affected_targets(mutation)
    }

    pub(super) fn classify_live_mutation(
        &self,
        target: &WorthQueryLiveArtifactTarget,
        mutation: &crate::memory_workspace::WorthQueryMutationDelta,
        affected_installed_targets: &BTreeSet<WorthQueryLiveArtifactTarget>,
    ) -> WorthQueryInstalledLiveMutationClassification {
        let Some(route) = self.routes.get(target) else {
            return WorthQueryInstalledLiveMutationClassification::Ordinary;
        };
        if !affected_installed_targets.contains(target) {
            return WorthQueryInstalledLiveMutationClassification::InstalledUnaffected;
        }
        WorthQueryInstalledLiveMutationClassification::Affected(
            route.impact_classifier.classify(mutation),
        )
    }
}

impl WorthQueryRuntime {
    pub(crate) fn register_installed_live_route<D: 'static, O: 'static, F: 'static>(
        &mut self,
        target: WorthQueryLiveArtifactTarget,
        closure: &crate::domain_installation::WorthQueryCompiledSemanticAspectDependencyClosure,
    ) {
        let operation = operation_key::<D, O, F>();
        self.unregister_installed_live_route(&target);
        let impact_classifier =
            crate::domain_installation::WorthQueryInstalledLiveImpactClassifier::from_closure(
                closure,
            );
        let target_collection = self
            .live_subscriptions
            .get(&target)
            .expect("installed live route retains its live subscription")
            .request
            .target_collection_identity()
            .as_str()
            .to_owned();
        self.installed_live_routes.target_index.register(
            target.clone(),
            operation,
            target_collection,
            impact_classifier.routing_selector(),
        );
        self.installed_live_routes.routes.insert(
            target,
            WorthQueryInstalledOwnerDeliveryRoute {
                operation,
                impact_classifier,
                owner_deliveries: WorthQueryInstalledOwnerDeliveryQueue::default(),
            },
        );
    }

    pub(crate) fn unregister_installed_live_route(
        &mut self,
        target: &WorthQueryLiveArtifactTarget,
    ) {
        if self.installed_live_routes.routes.remove(target).is_none() {
            return;
        }
        self.installed_live_routes.target_index.unregister(target);
    }
}

fn operation_key<D: 'static, O: 'static, F: 'static>() -> InstalledOperationKey {
    (
        std::any::TypeId::of::<D>(),
        std::any::TypeId::of::<O>(),
        std::any::TypeId::of::<F>(),
    )
}
