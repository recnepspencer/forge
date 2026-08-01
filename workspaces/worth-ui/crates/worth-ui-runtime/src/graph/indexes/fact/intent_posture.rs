use std::collections::BTreeMap;

use crate::declaration::{UiAspectName, UiAspectSemanticSlice};
use crate::fact_contract::UiConsumedFactContract;
use crate::graph::{
    UiGraphFactConsumerIdentity, UiGraphFactConsumerKey, UiGraphFactConsumerKind,
    UiGraphFactIndexEntry, UiGraphSnapshot,
};

pub(super) fn intent_posture_consumers(
    snapshot: &UiGraphSnapshot,
) -> BTreeMap<crate::graph::UiGraphNodeIdentity, Box<[UiGraphFactIndexEntry]>> {
    let affected_aspect = UiAspectName::from_semantic_slice(UiAspectSemanticSlice::ContentText);
    snapshot
        .nodes()
        .iter()
        .map(|node| {
            let graph_node = node.graph_node_identity();
            let authored_identity: Box<str> =
                node.declaration_identity().authored_semantic_name().into();
            let entry = UiGraphFactIndexEntry::new(
                UiGraphFactConsumerKey::new(
                    UiGraphFactConsumerKind::GraphNode,
                    authored_identity,
                    node.repeated_instance_basis().identity_digest(),
                ),
                UiGraphFactConsumerIdentity::GraphNode(graph_node),
                Some(affected_aspect.clone()),
                UiConsumedFactContract::intent_posture(graph_node),
            );
            (
                graph_node,
                Box::new([entry]) as Box<[UiGraphFactIndexEntry]>,
            )
        })
        .collect()
}
