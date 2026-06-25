use std::collections::BTreeMap;

use super::digest::digest_parts;
use super::receipt::{
    WorthUiAccessibilityAssociationReceipt, WorthUiAccessibilityNodeParticipationReceipt,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiAccessibilityRelationshipReceipt {
    kind: super::receipt::WorthUiAccessibilityAssociationKind,
    source_node_id: String,
    target_node_id: String,
    source_role: String,
    target_role: String,
    source_resolved_text: Option<String>,
    association_digest: u64,
    receipt_digest: u64,
}

pub(super) fn accessibility_relationships(
    associations: &[WorthUiAccessibilityAssociationReceipt],
    accessibility_nodes: &[WorthUiAccessibilityNodeParticipationReceipt],
) -> Vec<WorthUiAccessibilityRelationshipReceipt> {
    let nodes_by_id = accessibility_nodes
        .iter()
        .map(|node| (node.node_id().to_owned(), node))
        .collect::<BTreeMap<_, _>>();
    associations
        .iter()
        .filter_map(|association| {
            let source = nodes_by_id.get(association.source_node_id())?;
            let target = nodes_by_id.get(association.target_node_id())?;
            Some(WorthUiAccessibilityRelationshipReceipt::new(
                association,
                source,
                target,
            ))
        })
        .collect()
}

impl WorthUiAccessibilityRelationshipReceipt {
    fn new(
        association: &WorthUiAccessibilityAssociationReceipt,
        source: &WorthUiAccessibilityNodeParticipationReceipt,
        target: &WorthUiAccessibilityNodeParticipationReceipt,
    ) -> Self {
        let source_resolved_text = source.name().map(str::to_owned);
        let receipt_digest = digest_parts([
            "accessibility_relationship",
            association.kind().token(),
            association.source_node_id(),
            association.target_node_id(),
            source.role(),
            target.role(),
            source_resolved_text.as_deref().unwrap_or_default(),
            association.association_digest().to_string().as_str(),
        ]);
        Self {
            kind: association.kind(),
            source_node_id: association.source_node_id().to_owned(),
            target_node_id: association.target_node_id().to_owned(),
            source_role: source.role().to_owned(),
            target_role: target.role().to_owned(),
            source_resolved_text,
            association_digest: association.association_digest(),
            receipt_digest,
        }
    }

    pub fn kind(&self) -> super::receipt::WorthUiAccessibilityAssociationKind {
        self.kind
    }

    pub fn source_node_id(&self) -> &str {
        &self.source_node_id
    }

    pub fn target_node_id(&self) -> &str {
        &self.target_node_id
    }

    pub fn source_role(&self) -> &str {
        &self.source_role
    }

    pub fn target_role(&self) -> &str {
        &self.target_role
    }

    pub fn source_resolved_text(&self) -> Option<&str> {
        self.source_resolved_text.as_deref()
    }

    pub fn association_digest(&self) -> u64 {
        self.association_digest
    }

    pub fn receipt_digest(&self) -> u64 {
        self.receipt_digest
    }
}
