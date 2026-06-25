use super::super::admission::{
    admitted_associations_for_tree, denials_for_graph, WorthUiCompositionParticipationDenial,
    WorthUiCompositionParticipationDenialReport,
};
use super::super::host_inspection::WorthUiAccessibilityHostInspectionReceipt;
use super::accessibility_relationships::accessibility_relationships;
use super::focus_scopes::focus_scopes;
use super::node_participation::{accessibility_nodes, focus_nodes};
use super::WorthUiCompositionParticipationTraversalReceipt;
use super::{WorthUiAccessibilityAssociationReceipt, WorthUiCompositionParticipationReceipt};
use crate::runtime::{
    WorthUiAdmittedCompositionGraphReceipt,
    WorthUiAuthoredCompositionAccessibilityAssociationDeclaration,
    WorthUiEffectiveViewportParticipationReceipt, WorthUiMountedCompositionTreeReceipt,
    WorthUiMountedProductViewReceipt, WorthUiRuntimeFactId, WorthUiRuntimeGraphAuthority,
    WorthUiRuntimeHost,
};

pub(crate) fn composition_participation_denial_report_for_graph(
    graph: &WorthUiAdmittedCompositionGraphReceipt,
    associations: &[WorthUiAuthoredCompositionAccessibilityAssociationDeclaration],
) -> Option<WorthUiCompositionParticipationDenialReport> {
    let denials = denials_for_graph(graph, associations);
    (!denials.is_empty())
        .then(|| WorthUiCompositionParticipationDenialReport::denied(denials, associations.len()))
}

pub fn resolve_composition_participation(
    graph_authority: &WorthUiRuntimeGraphAuthority,
    tree: &WorthUiMountedCompositionTreeReceipt,
    associations: &[WorthUiAuthoredCompositionAccessibilityAssociationDeclaration],
) -> Result<WorthUiCompositionParticipationReceipt, Vec<WorthUiCompositionParticipationDenial>> {
    let admitted_associations = admitted_associations_for_tree(tree, associations)?;
    Ok(resolve_admitted_composition_participation(
        graph_authority,
        tree,
        admitted_associations,
        None,
    ))
}

impl WorthUiRuntimeHost {
    pub fn resolve_composition_participation_with_effective_viewport(
        &self,
        mounted: &WorthUiMountedProductViewReceipt,
        effective_viewport: &WorthUiEffectiveViewportParticipationReceipt,
    ) -> WorthUiCompositionParticipationReceipt {
        resolve_admitted_composition_participation(
            self.graph_authority(),
            mounted.composition_tree(),
            mounted.composition_participation().associations().to_vec(),
            Some(effective_viewport),
        )
    }

    pub fn inspect_composition_accessibility_host(
        &self,
        participation: &WorthUiCompositionParticipationReceipt,
    ) -> WorthUiAccessibilityHostInspectionReceipt {
        WorthUiAccessibilityHostInspectionReceipt::from_participation(participation)
    }
}

fn resolve_admitted_composition_participation(
    graph_authority: &WorthUiRuntimeGraphAuthority,
    tree: &WorthUiMountedCompositionTreeReceipt,
    admitted_associations: Vec<WorthUiAccessibilityAssociationReceipt>,
    effective_viewport: Option<&WorthUiEffectiveViewportParticipationReceipt>,
) -> WorthUiCompositionParticipationReceipt {
    let traversal = WorthUiCompositionParticipationTraversalReceipt::from_tree(tree);
    let accessibility_nodes =
        accessibility_nodes(&traversal, &admitted_associations, effective_viewport);
    let focus_nodes = focus_nodes(&traversal, effective_viewport);
    let focus_scopes = focus_scopes(&focus_nodes);
    let relationships = accessibility_relationships(&admitted_associations, &accessibility_nodes);
    let consumed_facts = consumed_composition_participation_facts(tree, effective_viewport);
    let query_graph_execution = graph_authority
        .plan_composition_participation_graph_operation(
            tree.root().root_id().as_str(),
            consumed_facts.clone(),
        )
        .into_execution_receipt();
    WorthUiCompositionParticipationReceipt::new(
        tree.root().root_id().as_str(),
        accessibility_nodes,
        focus_nodes,
        focus_scopes,
        admitted_associations,
        relationships,
        traversal,
        consumed_facts,
        query_graph_execution,
    )
}

fn consumed_composition_participation_facts(
    tree: &WorthUiMountedCompositionTreeReceipt,
    effective_viewport: Option<&WorthUiEffectiveViewportParticipationReceipt>,
) -> Vec<WorthUiRuntimeFactId> {
    let mut consumed_facts = tree
        .graph_access()
        .plan()
        .consumed_facts()
        .iter()
        .cloned()
        .collect::<Vec<_>>();
    consumed_facts.push(WorthUiRuntimeFactId::composition_participation(
        tree.root().root_id().as_str(),
    ));
    if let Some(effective_viewport) = effective_viewport {
        consumed_facts.extend(effective_viewport.consumed_facts().iter().cloned());
    }
    consumed_facts
}
