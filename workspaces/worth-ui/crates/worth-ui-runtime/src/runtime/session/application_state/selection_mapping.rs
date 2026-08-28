use super::WorthUiApplicationSessionState;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiDeclaredSelectionMappingDenial {
    TargetUnavailable,
    GraphNodeChanged,
    SelectionInputUnavailable,
    SelectionInputChanged,
    SelectionKeyMappingUnavailable,
    Selection(crate::runtime::selection::UiSelectionRequestDenial),
}

impl WorthUiApplicationSessionState {
    /// Resolves the private Phase-5 single-item declaration shape. Phase 6 may
    /// add richer public collection mappings without changing Selection owner
    /// semantics or admitting Query identity as a key.
    pub(crate) fn declared_selection_for_intent_target(
        &self,
        handoff: &crate::runtime::intent_execution::UiIntentConsequenceHandoff,
        mounted: &crate::mounting::WorthUiMountedSessionState,
        selection: &crate::runtime::selection::UiSelectionRuntimeState,
    ) -> Result<
        Option<crate::runtime::selection::UiDeclaredSelectionBinding>,
        UiDeclaredSelectionMappingDenial,
    > {
        let target = handoff.target();
        let basis = mounted
            .current_mounted_identity_basis(target.mounted_instance())
            .ok_or(UiDeclaredSelectionMappingDenial::TargetUnavailable)?;
        if basis.graph_node_identity() != handoff.graph_node() {
            return Err(UiDeclaredSelectionMappingDenial::GraphNodeChanged);
        }
        if handoff.interaction_family()
            != crate::capability::UiSemanticInteractionFamily::SelectionCommit
        {
            return Ok(None);
        }
        let option = handoff
            .selection_option()
            .ok_or(UiDeclaredSelectionMappingDenial::SelectionInputUnavailable)?;
        let current = mounted
            .current_projection_input(option.owner_revision().slot())
            .ok_or(UiDeclaredSelectionMappingDenial::SelectionInputUnavailable)?;
        let worth_ui_query_binding::UiProjectionInputFactReference::Collection(collection) =
            current
        else {
            return Err(UiDeclaredSelectionMappingDenial::SelectionInputUnavailable);
        };
        if collection.revision() != option.owner_revision()
            || collection.posture() != worth_ui_query_binding::UiProjectionInputPosture::Current
        {
            return Err(UiDeclaredSelectionMappingDenial::SelectionInputChanged);
        }
        let selected_value = option
            .application_item_key()
            .ok_or(UiDeclaredSelectionMappingDenial::SelectionKeyMappingUnavailable)?;
        let family = crate::runtime::UiApplicationItemKeyFamily::from_projection_input(
            collection.revision().slot(),
        );
        let key = crate::runtime::selection::UiSelectionStableKey::new(
            crate::runtime::UiApplicationItemKey::from_projection_mapping(family, selected_value),
        );
        let owner = crate::runtime::selection::UiSelectionOwnerIdentity::new(
            basis.semantic_surface_identity(),
            basis.graph_node_identity(),
            family,
        );
        let incarnation = crate::runtime::selection::UiSelectionOwnerIncarnation::new(
            basis.mount_incarnation().diagnostic_value(),
        )
        .ok_or(UiDeclaredSelectionMappingDenial::SelectionInputUnavailable)?;
        let action =
            crate::runtime::session::service_proposal::UiDeclaredFocusSelectionAction::new(
                target.mounted_instance(),
                owner,
                incarnation,
                crate::runtime::selection::UiSelectionRequest::SelectSingle(key),
                crate::runtime::session::service_proposal::UiSelectionInvocationCause::DeclaredIntentActivation,
            );
        let catalog_revision = collection.revision().observation_order();
        if selection.catalog_is_current(owner, incarnation, catalog_revision) {
            return Ok(Some(
                crate::runtime::selection::UiDeclaredSelectionBinding::current(action),
            ));
        }
        let catalog = collection
            .current_application_item_keys()
            .ok_or(UiDeclaredSelectionMappingDenial::SelectionInputUnavailable)?
            .iter()
            .copied()
            .map(|value| {
                crate::runtime::selection::UiSelectionStableKey::new(
                    crate::runtime::UiApplicationItemKey::from_projection_mapping(family, value),
                )
            })
            .collect();
        let catalog_posture = match collection.completeness() {
            Some(worth_ui_query_binding::UiCollectionCompleteness::Complete) => {
                crate::runtime::selection::UiSelectionCatalogPosture::Complete
            }
            Some(worth_ui_query_binding::UiCollectionCompleteness::Partial) => {
                crate::runtime::selection::UiSelectionCatalogPosture::Partial
            }
            None => return Err(UiDeclaredSelectionMappingDenial::SelectionInputUnavailable),
        };
        let registration = crate::runtime::selection::UiSelectionRegistration::new(
            owner,
            incarnation,
            crate::runtime::selection::UiSelectionPolicy::Single,
            catalog,
            catalog_posture,
        )
        .map_err(UiDeclaredSelectionMappingDenial::Selection)?
        .with_catalog_revision(catalog_revision);
        Ok(Some(
            crate::runtime::selection::UiDeclaredSelectionBinding::new(action, registration),
        ))
    }
}
