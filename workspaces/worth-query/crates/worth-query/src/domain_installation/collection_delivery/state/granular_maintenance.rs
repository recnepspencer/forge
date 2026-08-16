use std::collections::BTreeSet;

use crate::basis_lifecycle::BasisOperationLane;
use crate::domain_installation::{
    WorthQueryCollectionDeliveryDenialKind, WorthQueryImpactClass, WorthQueryNativeAccessKey,
    WorthQuerySettledDomainProjection,
};
use crate::memory_workspace::{WorthQueryEntity, WorthQueryEntityIdentity};

use super::WorthQueryCollectionConsumerWindow;

impl WorthQueryCollectionConsumerWindow {
    pub(crate) fn prepare_granular_maintenance<D, O, F, L: BasisOperationLane>(
        &self,
        current: &WorthQuerySettledDomainProjection<D, O, F, L>,
        fresh_rows: &[WorthQueryEntity],
        affected: &BTreeSet<WorthQueryEntityIdentity>,
        keys: &[WorthQueryNativeAccessKey],
        replacement_targets: &[super::super::index::WorthQueryCollectionMaintenanceTarget],
        impact: WorthQueryImpactClass,
    ) -> Result<
        (
            crate::domain_installation::WorthQueryPerformedCollectionStateMutation,
            crate::domain_installation::WorthQueryPendingCollectionStateMutation,
        ),
        WorthQueryCollectionDeliveryDenialKind,
    > {
        super::super::planning::prepare_granular(
            self,
            current,
            fresh_rows,
            affected,
            keys,
            replacement_targets,
            impact,
        )
    }

    pub(crate) fn keys_for_granular_change(
        &self,
        broad_change: bool,
        changes: &[super::super::WorthQueryCollectionChangedNativeTarget],
    ) -> Vec<WorthQueryNativeAccessKey> {
        self.index.keys_for_change(broad_change, changes)
    }

    pub(crate) fn replacement_targets_for_granular_change(
        &self,
        broad_change: bool,
        changes: &[super::super::WorthQueryCollectionChangedNativeTarget],
    ) -> Vec<super::super::index::WorthQueryCollectionMaintenanceTarget> {
        self.index
            .replacement_targets_for_change(broad_change, changes)
    }
}
