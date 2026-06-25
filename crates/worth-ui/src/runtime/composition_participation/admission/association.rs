use std::collections::BTreeMap;

use super::super::projection::WorthUiAccessibilityAssociationReceipt;
use super::denial::{
    WorthUiCompositionParticipationDenial, WorthUiCompositionParticipationDenialCode,
};
use crate::runtime::{
    WorthUiAdmittedCompositionGraphReceipt,
    WorthUiAuthoredCompositionAccessibilityAssociationDeclaration, WorthUiCompositionNodeKind,
    WorthUiCompositionParticipation, WorthUiMountedCompositionTreeReceipt,
};

pub(in crate::runtime::composition_participation) fn denials_for_graph(
    graph: &WorthUiAdmittedCompositionGraphReceipt,
    associations: &[WorthUiAuthoredCompositionAccessibilityAssociationDeclaration],
) -> Vec<WorthUiCompositionParticipationDenial> {
    let nodes_by_identity = graph_nodes_by_identity(graph);
    associations
        .iter()
        .filter_map(|association| admit_association(&nodes_by_identity, association).err())
        .collect()
}

pub(in crate::runtime::composition_participation) fn admitted_associations_for_tree(
    tree: &WorthUiMountedCompositionTreeReceipt,
    associations: &[WorthUiAuthoredCompositionAccessibilityAssociationDeclaration],
) -> Result<Vec<WorthUiAccessibilityAssociationReceipt>, Vec<WorthUiCompositionParticipationDenial>>
{
    let nodes_by_identity = nodes_by_identity(tree);
    let mut denials = Vec::new();
    let mut admitted_associations = Vec::new();
    for association in associations {
        match admit_association(&nodes_by_identity, association) {
            Ok(receipt) => admitted_associations.push(receipt),
            Err(denial) => denials.push(denial),
        }
    }
    if denials.is_empty() {
        Ok(admitted_associations)
    } else {
        Err(denials)
    }
}

fn admit_association(
    nodes_by_identity: &BTreeMap<String, NodeAssociationBasis>,
    association: &WorthUiAuthoredCompositionAccessibilityAssociationDeclaration,
) -> Result<WorthUiAccessibilityAssociationReceipt, WorthUiCompositionParticipationDenial> {
    let Some(source) = nodes_by_identity.get(association.source_identity()) else {
        return Err(denial(
            WorthUiCompositionParticipationDenialCode::MissingSourceNode,
            association,
        ));
    };
    let Some(target) = nodes_by_identity.get(association.target_identity()) else {
        return Err(denial(
            WorthUiCompositionParticipationDenialCode::MissingTargetNode,
            association,
        ));
    };
    if source.kind != WorthUiCompositionNodeKind::Content {
        return Err(denial(
            WorthUiCompositionParticipationDenialCode::InvalidSourceKind,
            association,
        ));
    }
    if !matches!(
        target.kind,
        WorthUiCompositionNodeKind::Control | WorthUiCompositionNodeKind::Interaction
    ) {
        return Err(denial(
            WorthUiCompositionParticipationDenialCode::InvalidTargetKind,
            association,
        ));
    }
    if !source.accessibility_exposed {
        return Err(denial(
            WorthUiCompositionParticipationDenialCode::SourceNotAccessible,
            association,
        ));
    }
    Ok(WorthUiAccessibilityAssociationReceipt::new(
        association.kind(),
        source.node_id.clone(),
        target.node_id.clone(),
    ))
}

fn nodes_by_identity(
    tree: &WorthUiMountedCompositionTreeReceipt,
) -> BTreeMap<String, NodeAssociationBasis> {
    tree.graph_access()
        .root_children()
        .iter()
        .chain(tree.graph_access().child_rows())
        .map(|row| {
            let node = row.node();
            (
                node.authority_identity().to_owned(),
                NodeAssociationBasis {
                    node_id: node.node_id().as_str().to_owned(),
                    kind: node.kind(),
                    accessibility_exposed: node.participation()
                        == WorthUiCompositionParticipation::Present,
                },
            )
        })
        .collect()
}

fn graph_nodes_by_identity(
    graph: &WorthUiAdmittedCompositionGraphReceipt,
) -> BTreeMap<String, NodeAssociationBasis> {
    graph
        .nodes()
        .iter()
        .map(|node| {
            (
                node.authority_identity().to_owned(),
                NodeAssociationBasis {
                    node_id: node.node_id().as_str().to_owned(),
                    kind: node.kind(),
                    accessibility_exposed: node.participation()
                        == WorthUiCompositionParticipation::Present,
                },
            )
        })
        .collect()
}

fn denial(
    code: WorthUiCompositionParticipationDenialCode,
    association: &WorthUiAuthoredCompositionAccessibilityAssociationDeclaration,
) -> WorthUiCompositionParticipationDenial {
    WorthUiCompositionParticipationDenial::new(
        code,
        association.kind(),
        association.source_identity(),
        association.target_identity(),
    )
}

struct NodeAssociationBasis {
    node_id: String,
    kind: WorthUiCompositionNodeKind,
    accessibility_exposed: bool,
}
