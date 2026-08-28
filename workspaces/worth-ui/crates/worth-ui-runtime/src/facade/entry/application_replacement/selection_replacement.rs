use super::WorthUiActiveApplicationSession;

pub(super) struct UiPreparedSelectionReplacement(
    crate::runtime::selection::UiSelectionRuntimeState,
);

impl UiPreparedSelectionReplacement {
    pub(super) fn into_state(self) -> crate::runtime::selection::UiSelectionRuntimeState {
        self.0
    }
}

impl WorthUiActiveApplicationSession {
    pub(super) fn prepare_selection_replacement(
        &self,
        successor: &crate::mounting::UiMountedGraphReplacementSuccessor,
        publication_is_current: bool,
    ) -> UiPreparedSelectionReplacement {
        let mut selection = self.selection.clone();
        let predecessor = self.mounted.view();
        let successor_view = successor.identity_view();
        for prior in predecessor.mounted_instances() {
            let retained_exactly = successor_view
                .mounted_instances()
                .iter()
                .any(|next| next.identity() == prior.identity() && next.basis() == prior.basis());
            if retained_exactly {
                continue;
            }
            selection.retire_mounted_owner(
                prior.basis().semantic_surface_identity(),
                prior.graph_node_identity(),
                crate::runtime::selection::UiSelectionOwnerIncarnation::from_mount_incarnation(
                    prior.mount_incarnation(),
                ),
            );
        }
        if publication_is_current {
            reconcile_successor_catalogs(&mut selection, &self.mounted);
        } else {
            selection.suspend_projection_catalogs();
        }
        UiPreparedSelectionReplacement(selection)
    }
}

fn reconcile_successor_catalogs(
    selection: &mut crate::runtime::selection::UiSelectionRuntimeState,
    mounted: &crate::mounting::WorthUiMountedSessionState,
) {
    for family in selection.projection_families().iter().copied() {
        let Some(slot) = family.projection_input_slot() else {
            continue;
        };
        let Some(worth_ui_query_binding::UiProjectionInputFactReference::Collection(collection)) =
            mounted.current_projection_input(slot)
        else {
            selection.retire_family(family);
            continue;
        };
        if collection.posture() != worth_ui_query_binding::UiProjectionInputPosture::Current {
            selection.retire_family(family);
            continue;
        }
        let revision = collection.revision().observation_order();
        if !selection.family_requires_catalog_reconciliation(family, revision) {
            continue;
        }
        let (Some(keys), Some(completeness)) = (
            collection.current_application_item_keys(),
            collection.completeness(),
        ) else {
            selection.retire_family(family);
            continue;
        };
        let posture = match completeness {
            worth_ui_query_binding::UiCollectionCompleteness::Complete => {
                crate::runtime::selection::UiSelectionCatalogPosture::Complete
            }
            worth_ui_query_binding::UiCollectionCompleteness::Partial => {
                crate::runtime::selection::UiSelectionCatalogPosture::Partial
            }
        };
        selection
            .reconcile_projection_catalog(family, revision, &keys, posture)
            .expect("prepared replacement retains a valid Selection catalog");
    }
}
