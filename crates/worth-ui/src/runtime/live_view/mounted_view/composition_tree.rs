use std::collections::BTreeMap;

use crate::runtime::live_view::digest::digest_parts;
use crate::runtime::{
    admit_composition_graph_access, WorthUiCompositionGraphAccessReceipt,
    WorthUiCompositionGraphAccessRequest, WorthUiCompositionNodeReceipt,
    WorthUiCompositionPolicyAttachmentReceipt, WorthUiCompositionRootReceipt,
    WorthUiMountedNodeReceipt, WorthUiMountedViewReceipt,
};

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiMountedCompositionTreeReceipt {
    root: WorthUiCompositionRootReceipt,
    policy_attachments: Vec<WorthUiCompositionPolicyAttachmentReceipt>,
    children: Vec<WorthUiMountedCompositionChildReceipt>,
    children_by_node_id: BTreeMap<String, WorthUiMountedCompositionChildReceipt>,
    graph_access: WorthUiCompositionGraphAccessReceipt,
    counters: WorthUiMountedCompositionTraversalCounters,
    receipt_digest: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiMountedCompositionChildReceipt {
    parent_id: String,
    order: u32,
    composition_node: WorthUiCompositionNodeReceipt,
    mounted_node: WorthUiMountedNodeReceipt,
    receipt_digest: u64,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiMountedCompositionTraversalCounters {
    mounted_node_index_entry_count: usize,
    child_index_entry_count: usize,
    flat_node_scan_count: usize,
    source_reparse_count: usize,
    renderer_parse_count: usize,
}

impl WorthUiMountedCompositionTreeReceipt {
    pub(in crate::runtime::live_view) fn from_mounted_view(
        mounted_view: &WorthUiMountedViewReceipt,
    ) -> Self {
        let graph = mounted_view.composition_graph();
        let policy_attachments = graph.policy_attachments().to_vec();
        let graph_access = admit_composition_graph_access(
            graph,
            WorthUiCompositionGraphAccessRequest::mounted_product_tree(),
        )
        .expect("mounted product tree access must admit for an admitted composition graph");
        let children = mounted_children_from_access(mounted_view.nodes(), &graph_access);
        let children_by_node_id = children
            .iter()
            .cloned()
            .map(|child| (child.node_id().to_owned(), child))
            .collect();
        let counters = WorthUiMountedCompositionTraversalCounters {
            mounted_node_index_entry_count: mounted_view.nodes().len(),
            child_index_entry_count: graph_access.child_rows().len(),
            flat_node_scan_count: 0,
            source_reparse_count: 0,
            renderer_parse_count: 0,
        };
        let receipt_digest = digest_parts(
            [
                graph.root().receipt_digest().to_string(),
                graph_access.access_digest().to_string(),
            ]
            .into_iter()
            .chain(
                children
                    .iter()
                    .map(|child| child.receipt_digest().to_string()),
            ),
        );
        Self {
            root: graph.root().clone(),
            policy_attachments,
            children,
            children_by_node_id,
            graph_access,
            counters,
            receipt_digest,
        }
    }

    pub fn root(&self) -> &WorthUiCompositionRootReceipt {
        &self.root
    }

    pub fn policy_attachments(&self) -> &[WorthUiCompositionPolicyAttachmentReceipt] {
        &self.policy_attachments
    }

    pub fn ordered_children(&self, parent_id: &str) -> &[WorthUiMountedCompositionChildReceipt] {
        let start = self
            .children
            .iter()
            .position(|child| child.parent_id() == parent_id);
        let Some(start) = start else {
            return &[];
        };
        let len = self.children[start..]
            .iter()
            .take_while(|child| child.parent_id() == parent_id)
            .count();
        &self.children[start..start + len]
    }

    pub fn root_children(&self) -> &[WorthUiMountedCompositionChildReceipt] {
        self.ordered_children(self.root.root_id().as_str())
    }

    pub fn child_for_node_id(
        &self,
        node_id: &str,
    ) -> Option<&WorthUiMountedCompositionChildReceipt> {
        self.children_by_node_id.get(node_id)
    }

    pub fn counters(&self) -> WorthUiMountedCompositionTraversalCounters {
        self.counters
    }

    pub fn graph_access(&self) -> &WorthUiCompositionGraphAccessReceipt {
        &self.graph_access
    }

    pub fn receipt_digest(&self) -> u64 {
        self.receipt_digest
    }
}

impl WorthUiMountedCompositionChildReceipt {
    fn new(
        parent_id: String,
        order: u32,
        composition_node: WorthUiCompositionNodeReceipt,
        mounted_node: WorthUiMountedNodeReceipt,
    ) -> Self {
        let receipt_digest = digest_parts([
            parent_id.as_str(),
            order.to_string().as_str(),
            composition_node.receipt_digest().to_string().as_str(),
            mounted_node.receipt_digest().to_string().as_str(),
        ]);
        Self {
            parent_id,
            order,
            composition_node,
            mounted_node,
            receipt_digest,
        }
    }

    pub fn parent_id(&self) -> &str {
        &self.parent_id
    }

    pub fn order(&self) -> u32 {
        self.order
    }

    pub fn composition_node(&self) -> &WorthUiCompositionNodeReceipt {
        &self.composition_node
    }

    pub fn mounted_node(&self) -> &WorthUiMountedNodeReceipt {
        &self.mounted_node
    }

    pub fn node_id(&self) -> &str {
        self.composition_node.node_id().as_str()
    }

    pub fn receipt_digest(&self) -> u64 {
        self.receipt_digest
    }
}

impl WorthUiMountedCompositionTraversalCounters {
    pub fn mounted_node_index_entry_count(self) -> usize {
        self.mounted_node_index_entry_count
    }

    pub fn child_index_entry_count(self) -> usize {
        self.child_index_entry_count
    }

    pub fn flat_node_scan_count(self) -> usize {
        self.flat_node_scan_count
    }

    pub fn source_reparse_count(self) -> usize {
        self.source_reparse_count
    }

    pub fn renderer_parse_count(self) -> usize {
        self.renderer_parse_count
    }
}

fn mounted_children_from_access(
    mounted_nodes: &[WorthUiMountedNodeReceipt],
    graph_access: &WorthUiCompositionGraphAccessReceipt,
) -> Vec<WorthUiMountedCompositionChildReceipt> {
    let mounted_nodes_by_composition_identity =
        mounted_nodes_by_composition_identity(mounted_nodes);
    let mut children = graph_access
        .child_rows()
        .iter()
        .filter_map(|child| {
            mounted_nodes_by_composition_identity
                .get(child.node().node_id().as_str())
                .map(|node| {
                    WorthUiMountedCompositionChildReceipt::new(
                        child.parent_id().to_owned(),
                        child.order(),
                        child.node().clone(),
                        (*node).clone(),
                    )
                })
        })
        .collect::<Vec<_>>();
    children.sort_by(|left, right| {
        left.parent_id()
            .cmp(right.parent_id())
            .then_with(|| left.order().cmp(&right.order()))
    });
    children
}

fn mounted_nodes_by_composition_identity<'a>(
    mounted_nodes: &'a [WorthUiMountedNodeReceipt],
) -> BTreeMap<String, &'a WorthUiMountedNodeReceipt> {
    mounted_nodes
        .iter()
        .map(|node| (mounted_node_composition_id(node), node))
        .collect()
}

fn mounted_node_composition_id(node: &WorthUiMountedNodeReceipt) -> String {
    match node {
        WorthUiMountedNodeReceipt::Surface(node) => node.node_id().to_owned(),
        WorthUiMountedNodeReceipt::FlowContainer(node) => node.node_id().to_owned(),
        WorthUiMountedNodeReceipt::Content(node) => node.node_id().to_owned(),
        WorthUiMountedNodeReceipt::Control(node) => node
            .composition_child_binding()
            .composition_node_id()
            .to_owned(),
        WorthUiMountedNodeReceipt::Interaction(node) => node
            .composition_child_binding()
            .composition_node_id()
            .to_owned(),
        WorthUiMountedNodeReceipt::Evidence(node) => node.node_id().to_owned(),
        WorthUiMountedNodeReceipt::Text(node) => node.node_id().to_owned(),
        WorthUiMountedNodeReceipt::Icon(node) => node.node_id().to_owned(),
        WorthUiMountedNodeReceipt::DiagnosticPanel(node) => node.node_id().to_owned(),
        WorthUiMountedNodeReceipt::PortalHost(node) => node.node_id().to_owned(),
        WorthUiMountedNodeReceipt::MosaicRegion(node) => node.node_id().to_owned(),
    }
}
