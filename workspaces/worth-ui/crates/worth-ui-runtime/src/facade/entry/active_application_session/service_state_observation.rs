#[cfg(any(test, feature = "certification-support"))]
impl super::WorthUiActiveApplicationSession {
    pub(crate) fn inspect_scroll_runtime_for_certification(
        &self,
    ) -> crate::certification_support::UiScrollRuntimeCertificationSnapshot {
        let Some(scroll) = self.scroll.as_ref() else {
            return crate::certification_support::UiScrollRuntimeCertificationSnapshot::uninstalled(
            );
        };
        let (
            owners,
            ownership_instances,
            revision,
            admitted,
            rejected,
            visited,
            changed,
            ownership_resolutions,
            ownership_graph_nodes_visited,
            ownership_plan_nodes_visited,
        ) = scroll.inspect_for_certification();
        let owner_geometry = scroll
            .inspect_owner_geometry_for_certification()
            .iter()
            .map(|(owner, offset, bounds)| {
                crate::certification_support::UiScrollOwnerGeometryCertificationRow::new(
                    *owner, *offset, *bounds,
                )
            })
            .collect::<Vec<_>>()
            .into_boxed_slice();
        let ownership_incarnations = scroll.inspect_ownership_incarnations_for_certification();
        let ownership_mounted_instances =
            scroll.inspect_ownership_mounted_instances_for_certification();
        crate::certification_support::UiScrollRuntimeCertificationSnapshot::new(
            owners,
            ownership_instances,
            revision,
            admitted,
            rejected,
            visited,
            changed,
            ownership_resolutions,
            ownership_graph_nodes_visited,
            ownership_plan_nodes_visited,
            owner_geometry,
            ownership_incarnations,
            ownership_mounted_instances,
        )
    }

    pub(crate) fn inspect_selection_runtime_for_certification(
        &self,
    ) -> crate::certification_support::UiSelectionRuntimeCertificationSnapshot {
        let Some(selection) = self.selection.as_ref() else {
            return crate::certification_support::UiSelectionRuntimeCertificationSnapshot::uninstalled();
        };
        let (
            owners,
            available_catalog_owners,
            selected_keys,
            revision,
            requests,
            keys_visited,
            catalog_keys_reconciled,
            application_item_keys,
        ) = selection.inspect_for_certification();
        crate::certification_support::UiSelectionRuntimeCertificationSnapshot::new(
            owners,
            available_catalog_owners,
            selected_keys,
            revision,
            requests,
            keys_visited,
            catalog_keys_reconciled,
            application_item_keys,
        )
    }
}
