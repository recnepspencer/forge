use std::collections::BTreeMap;

use super::receipt::{
    WorthUiAccessibilityAssociationKind, WorthUiAccessibilityAssociationReceipt,
    WorthUiAccessibilityNodeParticipationReceipt, WorthUiAccessibilityParticipationPosture,
    WorthUiFocusNodeParticipationReceipt, WorthUiFocusParticipationPosture,
};
use super::traversal::WorthUiCompositionParticipationTraversalReceipt;
use crate::runtime::{
    WorthUiCompositionParticipation, WorthUiEffectiveViewportParticipationReceipt,
    WorthUiEffectiveViewportParticipationRow, WorthUiMountedNodeReceipt,
    WorthUiPrimitiveContentRole,
};

pub(super) fn accessibility_nodes(
    traversal: &WorthUiCompositionParticipationTraversalReceipt,
    associations: &[WorthUiAccessibilityAssociationReceipt],
    effective_viewport: Option<&WorthUiEffectiveViewportParticipationReceipt>,
) -> Vec<WorthUiAccessibilityNodeParticipationReceipt> {
    let content_names = content_names_by_node_id(traversal);
    traversal
        .rows()
        .iter()
        .map(|row| {
            let child = row.mounted_child();
            accessibility_node_for_child(
                child.node_id(),
                child.composition_node().participation(),
                child.mounted_node(),
                effective_viewport.and_then(|receipt| receipt.row_for_node(child.node_id())),
                associations,
                &content_names,
            )
        })
        .collect()
}

pub(super) fn focus_nodes(
    traversal: &WorthUiCompositionParticipationTraversalReceipt,
    effective_viewport: Option<&WorthUiEffectiveViewportParticipationReceipt>,
) -> Vec<WorthUiFocusNodeParticipationReceipt> {
    traversal
        .rows()
        .iter()
        .map(|row| {
            let child = row.mounted_child();
            WorthUiFocusNodeParticipationReceipt::new(
                child.node_id(),
                row.parent_id(),
                row.graph_order(),
                focus_posture_for_node(
                    child.composition_node().participation(),
                    child.mounted_node(),
                    effective_viewport.and_then(|receipt| receipt.row_for_node(child.node_id())),
                ),
            )
        })
        .collect()
}

fn accessibility_node_for_child(
    node_id: &str,
    composition_participation: WorthUiCompositionParticipation,
    mounted_node: &WorthUiMountedNodeReceipt,
    effective_viewport: Option<&WorthUiEffectiveViewportParticipationRow>,
    associations: &[WorthUiAccessibilityAssociationReceipt],
    content_names: &BTreeMap<String, String>,
) -> WorthUiAccessibilityNodeParticipationReceipt {
    let role = role_for_node(mounted_node);
    let posture =
        accessibility_posture_for_node(composition_participation, mounted_node, effective_viewport);
    let name = name_for_node(node_id, mounted_node, associations, content_names);
    let description_node_ids = associations
        .iter()
        .filter(|association| {
            association.target_node_id() == node_id
                && association.kind() == WorthUiAccessibilityAssociationKind::Description
        })
        .map(|association| association.source_node_id().to_owned())
        .collect();
    let error_node_ids = associations
        .iter()
        .filter(|association| {
            association.target_node_id() == node_id
                && association.kind() == WorthUiAccessibilityAssociationKind::Error
        })
        .map(|association| association.source_node_id().to_owned())
        .collect();
    WorthUiAccessibilityNodeParticipationReceipt::new(
        node_id,
        role,
        name,
        description_node_ids,
        error_node_ids,
        posture,
    )
}

fn name_for_node(
    node_id: &str,
    mounted_node: &WorthUiMountedNodeReceipt,
    associations: &[WorthUiAccessibilityAssociationReceipt],
    content_names: &BTreeMap<String, String>,
) -> Option<String> {
    associations
        .iter()
        .find(|association| {
            association.target_node_id() == node_id
                && association.kind() == WorthUiAccessibilityAssociationKind::Label
        })
        .and_then(|association| content_names.get(association.source_node_id()).cloned())
        .or_else(|| match mounted_node {
            WorthUiMountedNodeReceipt::Content(content) => content
                .content()
                .accessibility_name()
                .map(str::to_owned)
                .or_else(|| Some(content.content().text().to_owned())),
            WorthUiMountedNodeReceipt::Control(control) => Some(control.label().to_owned()),
            WorthUiMountedNodeReceipt::Interaction(interaction) => {
                Some(interaction.interaction().label().to_owned())
            }
            _ => None,
        })
}

fn content_names_by_node_id(
    traversal: &WorthUiCompositionParticipationTraversalReceipt,
) -> BTreeMap<String, String> {
    traversal
        .rows()
        .iter()
        .filter_map(|row| match row.mounted_child().mounted_node() {
            WorthUiMountedNodeReceipt::Content(content) => Some((
                row.node_id().to_owned(),
                content
                    .content()
                    .accessibility_name()
                    .map(str::to_owned)
                    .unwrap_or_else(|| content.content().text().to_owned()),
            )),
            _ => None,
        })
        .collect()
}

fn role_for_node(mounted_node: &WorthUiMountedNodeReceipt) -> String {
    match mounted_node {
        WorthUiMountedNodeReceipt::Content(content) => {
            content_role_token(content.content().role()).to_owned()
        }
        WorthUiMountedNodeReceipt::Control(control) => control.kind().token().to_owned(),
        WorthUiMountedNodeReceipt::Interaction(_) => "interaction".to_owned(),
        WorthUiMountedNodeReceipt::FlowContainer(_) => "group".to_owned(),
        WorthUiMountedNodeReceipt::Surface(_) => "surface".to_owned(),
        _ => "presentation".to_owned(),
    }
}

fn content_role_token(role: WorthUiPrimitiveContentRole) -> &'static str {
    match role {
        WorthUiPrimitiveContentRole::Body => "body",
        WorthUiPrimitiveContentRole::Label => "label",
        WorthUiPrimitiveContentRole::HelperText => "helper",
        WorthUiPrimitiveContentRole::ErrorText => "error",
        WorthUiPrimitiveContentRole::PrefixAdornment => "prefix_adornment",
        WorthUiPrimitiveContentRole::SuffixAdornment => "suffix_adornment",
    }
}

fn accessibility_posture_for_node(
    composition_participation: WorthUiCompositionParticipation,
    mounted_node: &WorthUiMountedNodeReceipt,
    effective_viewport: Option<&WorthUiEffectiveViewportParticipationRow>,
) -> WorthUiAccessibilityParticipationPosture {
    match composition_participation {
        WorthUiCompositionParticipation::AbsentRetainsState => {
            return WorthUiAccessibilityParticipationPosture::Hidden;
        }
        WorthUiCompositionParticipation::Inert => {
            return WorthUiAccessibilityParticipationPosture::Inert;
        }
        WorthUiCompositionParticipation::Present => {}
    }
    if effective_viewport.is_some_and(|row| !row.accessibility_participates()) {
        return WorthUiAccessibilityParticipationPosture::Hidden;
    }
    match mounted_node {
        WorthUiMountedNodeReceipt::Control(control)
            if control
                .participation()
                .is_some_and(|p| !p.participates_in_accessibility()) =>
        {
            WorthUiAccessibilityParticipationPosture::Hidden
        }
        WorthUiMountedNodeReceipt::Interaction(interaction) if !interaction.is_enabled() => {
            WorthUiAccessibilityParticipationPosture::Disabled
        }
        _ => WorthUiAccessibilityParticipationPosture::Exposed,
    }
}

fn focus_posture_for_node(
    composition_participation: WorthUiCompositionParticipation,
    mounted_node: &WorthUiMountedNodeReceipt,
    effective_viewport: Option<&WorthUiEffectiveViewportParticipationRow>,
) -> WorthUiFocusParticipationPosture {
    match composition_participation {
        WorthUiCompositionParticipation::AbsentRetainsState => {
            return WorthUiFocusParticipationPosture::Hidden;
        }
        WorthUiCompositionParticipation::Inert => {
            return WorthUiFocusParticipationPosture::Inert;
        }
        WorthUiCompositionParticipation::Present => {}
    }
    if effective_viewport.is_some_and(|row| !row.focus_participates()) {
        return WorthUiFocusParticipationPosture::Hidden;
    }
    match mounted_node {
        WorthUiMountedNodeReceipt::Control(control)
            if control
                .participation()
                .is_some_and(|p| !p.participates_in_events()) =>
        {
            WorthUiFocusParticipationPosture::Hidden
        }
        WorthUiMountedNodeReceipt::Control(control) if !control.editability().is_editable() => {
            WorthUiFocusParticipationPosture::Disabled
        }
        WorthUiMountedNodeReceipt::Control(_) => WorthUiFocusParticipationPosture::Focusable,
        WorthUiMountedNodeReceipt::Interaction(interaction) if interaction.is_enabled() => {
            WorthUiFocusParticipationPosture::Focusable
        }
        WorthUiMountedNodeReceipt::Interaction(_) => WorthUiFocusParticipationPosture::Disabled,
        _ => WorthUiFocusParticipationPosture::NotFocusable,
    }
}
