use crate::runtime::live_view::digest::digest_parts;
use crate::runtime::{
    WorthUiCompositionNodeKind, WorthUiGraphBackedLiveViewProjectionReceipt,
    WorthUiPrimitiveContentReceipt,
};

use super::{
    WorthUiMountedIconNodeReceipt, WorthUiMountedNodeReceipt, WorthUiMountedTextNodeReceipt,
};

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiMountedContentNodeReceipt {
    node_id: String,
    content: WorthUiPrimitiveContentReceipt,
    semantic_slice: &'static str,
    receipt_digest: u64,
}

pub(super) fn static_content_nodes_for_projection(
    projection: &WorthUiGraphBackedLiveViewProjectionReceipt,
) -> Vec<WorthUiMountedNodeReceipt> {
    projection
        .composition_graph()
        .nodes()
        .iter()
        .filter(|node| node.kind() == WorthUiCompositionNodeKind::Content)
        .filter_map(|node| {
            projection
                .content_receipt_for_subject(node.node_id().as_str())
                .map(|content| {
                    WorthUiMountedNodeReceipt::Content(WorthUiMountedContentNodeReceipt::new(
                        node.node_id().as_str(),
                        content.clone(),
                    ))
                })
        })
        .collect()
}

impl WorthUiMountedContentNodeReceipt {
    fn new(node_id: &str, content: WorthUiPrimitiveContentReceipt) -> Self {
        let receipt_digest = digest_parts([
            "mounted_content",
            node_id,
            content.receipt_digest().to_string().as_str(),
        ]);
        Self {
            node_id: node_id.to_owned(),
            content,
            semantic_slice: "PrimitiveContent",
            receipt_digest,
        }
    }

    pub fn node_id(&self) -> &str {
        &self.node_id
    }

    pub fn content(&self) -> &WorthUiPrimitiveContentReceipt {
        &self.content
    }

    pub fn semantic_slice(&self) -> &'static str {
        self.semantic_slice
    }

    pub fn receipt_digest(&self) -> u64 {
        self.receipt_digest
    }
}

impl WorthUiMountedTextNodeReceipt {
    pub fn text(&self) -> &str {
        &self.text
    }
}

impl WorthUiMountedIconNodeReceipt {
    pub fn icon_name(&self) -> &str {
        &self.icon_name
    }
}
