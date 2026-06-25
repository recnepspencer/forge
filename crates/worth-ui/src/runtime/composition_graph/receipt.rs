use super::digest::digest_parts;
use super::{
    WorthUiCompositionChildSizing, WorthUiCompositionNodeDefinition, WorthUiCompositionNodeId,
    WorthUiCompositionNodeKind, WorthUiCompositionParentRef, WorthUiCompositionParticipation,
    WorthUiCompositionRootDefinition, WorthUiCompositionRootId, WorthUiCompositionRootKind,
};
use crate::runtime::{WorthUiQueryGraphExecutionReceipt, WorthUiRuntimeFactId};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiAdmittedCompositionGraphReceipt {
    root: WorthUiCompositionRootReceipt,
    nodes: Vec<WorthUiCompositionNodeReceipt>,
    edges: Vec<WorthUiCompositionEdgeReceipt>,
    policy_attachments: Vec<WorthUiCompositionPolicyAttachmentReceipt>,
    consumed_facts: Vec<WorthUiRuntimeFactId>,
    query_graph_execution: WorthUiQueryGraphExecutionReceipt,
    counters: WorthUiCompositionGraphCounters,
    receipt_digest: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiCompositionRootReceipt {
    root_id: WorthUiCompositionRootId,
    kind: WorthUiCompositionRootKind,
    authority_identity: String,
    fact_id: WorthUiRuntimeFactId,
    receipt_digest: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiCompositionNodeReceipt {
    node_id: WorthUiCompositionNodeId,
    kind: WorthUiCompositionNodeKind,
    authority_identity: String,
    participation: WorthUiCompositionParticipation,
    fact_id: WorthUiRuntimeFactId,
    receipt_digest: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiCompositionEdgeReceipt {
    parent: WorthUiCompositionParentRef,
    child: WorthUiCompositionNodeId,
    order: u32,
    sizing: WorthUiCompositionChildSizing,
    fact_id: WorthUiRuntimeFactId,
    receipt_digest: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiCompositionPolicyAttachmentReceipt {
    node_id: WorthUiCompositionNodeId,
    policy_kind: super::WorthUiCompositionPolicyKind,
    policy_identity: String,
    fact_id: WorthUiRuntimeFactId,
    receipt_digest: u64,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiCompositionGraphCounters {
    node_count: usize,
    edge_count: usize,
    policy_attachment_count: usize,
    selected_graph_obligation_count: usize,
    source_reparse_count: usize,
    renderer_parse_count: usize,
}

impl WorthUiAdmittedCompositionGraphReceipt {
    pub(crate) fn new(
        root: WorthUiCompositionRootReceipt,
        nodes: Vec<WorthUiCompositionNodeReceipt>,
        edges: Vec<WorthUiCompositionEdgeReceipt>,
        policy_attachments: Vec<WorthUiCompositionPolicyAttachmentReceipt>,
        query_graph_execution: WorthUiQueryGraphExecutionReceipt,
    ) -> Self {
        let mut consumed_facts = vec![root.fact_id().clone()];
        consumed_facts.extend(nodes.iter().map(|node| node.fact_id().clone()));
        consumed_facts.extend(edges.iter().map(|edge| edge.fact_id().clone()));
        consumed_facts.extend(
            policy_attachments
                .iter()
                .map(|attachment| attachment.fact_id().clone()),
        );
        consumed_facts.sort();
        consumed_facts.dedup();
        let counters = WorthUiCompositionGraphCounters {
            node_count: nodes.len(),
            edge_count: edges.len(),
            policy_attachment_count: policy_attachments.len(),
            selected_graph_obligation_count: query_graph_execution.selected_obligation_count(),
            source_reparse_count: 0,
            renderer_parse_count: 0,
        };
        let receipt_digest = digest_parts(
            std::iter::once(root.receipt_digest().to_string())
                .chain(nodes.iter().map(|node| node.receipt_digest().to_string()))
                .chain(edges.iter().map(|edge| edge.receipt_digest().to_string()))
                .chain(
                    policy_attachments
                        .iter()
                        .map(|attachment| attachment.receipt_digest().to_string()),
                )
                .chain(std::iter::once(
                    query_graph_execution.execution_digest().to_string(),
                )),
        );
        Self {
            root,
            nodes,
            edges,
            policy_attachments,
            consumed_facts,
            query_graph_execution,
            counters,
            receipt_digest,
        }
    }

    pub fn root(&self) -> &WorthUiCompositionRootReceipt {
        &self.root
    }

    pub fn nodes(&self) -> &[WorthUiCompositionNodeReceipt] {
        &self.nodes
    }

    pub fn edges(&self) -> &[WorthUiCompositionEdgeReceipt] {
        &self.edges
    }

    pub fn policy_attachments(&self) -> &[WorthUiCompositionPolicyAttachmentReceipt] {
        &self.policy_attachments
    }

    pub fn consumed_facts(&self) -> &[WorthUiRuntimeFactId] {
        &self.consumed_facts
    }

    pub fn query_graph_execution(&self) -> &WorthUiQueryGraphExecutionReceipt {
        &self.query_graph_execution
    }

    pub fn counters(&self) -> WorthUiCompositionGraphCounters {
        self.counters
    }

    pub fn receipt_digest(&self) -> u64 {
        self.receipt_digest
    }
}

impl WorthUiCompositionRootReceipt {
    pub(crate) fn from_definition(root: &WorthUiCompositionRootDefinition) -> Self {
        let fact_id = WorthUiRuntimeFactId::composition_root(root.root_id().as_str());
        let receipt_digest = digest_parts([
            "composition_root",
            root.root_id().as_str(),
            root.kind().token(),
            root.authority_identity(),
        ]);
        Self {
            root_id: root.root_id().clone(),
            kind: root.kind(),
            authority_identity: root.authority_identity().to_owned(),
            fact_id,
            receipt_digest,
        }
    }

    pub fn root_id(&self) -> &WorthUiCompositionRootId {
        &self.root_id
    }

    pub fn kind(&self) -> WorthUiCompositionRootKind {
        self.kind
    }

    pub fn authority_identity(&self) -> &str {
        &self.authority_identity
    }

    pub fn fact_id(&self) -> &WorthUiRuntimeFactId {
        &self.fact_id
    }

    pub fn receipt_digest(&self) -> u64 {
        self.receipt_digest
    }
}

impl WorthUiCompositionNodeReceipt {
    pub(crate) fn from_definition(node: &WorthUiCompositionNodeDefinition) -> Self {
        let fact_id = WorthUiRuntimeFactId::composition_node(node.node_id().as_str());
        let receipt_digest = digest_parts([
            "composition_node",
            node.node_id().as_str(),
            node.kind().token(),
            node.authority_identity(),
            node.participation().token(),
        ]);
        Self {
            node_id: node.node_id().clone(),
            kind: node.kind(),
            authority_identity: node.authority_identity().to_owned(),
            participation: node.participation(),
            fact_id,
            receipt_digest,
        }
    }

    pub fn node_id(&self) -> &WorthUiCompositionNodeId {
        &self.node_id
    }

    pub fn kind(&self) -> WorthUiCompositionNodeKind {
        self.kind
    }

    pub fn authority_identity(&self) -> &str {
        &self.authority_identity
    }

    pub fn participation(&self) -> WorthUiCompositionParticipation {
        self.participation
    }

    pub fn fact_id(&self) -> &WorthUiRuntimeFactId {
        &self.fact_id
    }

    pub fn receipt_digest(&self) -> u64 {
        self.receipt_digest
    }
}

impl WorthUiCompositionEdgeReceipt {
    pub(crate) fn new(
        parent: WorthUiCompositionParentRef,
        child: WorthUiCompositionNodeId,
        order: u32,
        sizing: WorthUiCompositionChildSizing,
    ) -> Self {
        let sizing_token = sizing.token();
        let identity = format!(
            "{}:{}->{}:{}:{}",
            parent.kind_token(),
            parent.identity(),
            order,
            sizing_token,
            child.as_str()
        );
        let fact_id = WorthUiRuntimeFactId::composition_edge(identity.clone());
        let receipt_digest = digest_parts(["composition_edge", identity.as_str()]);
        Self {
            parent,
            child,
            order,
            sizing,
            fact_id,
            receipt_digest,
        }
    }

    pub fn parent(&self) -> &WorthUiCompositionParentRef {
        &self.parent
    }

    pub fn child(&self) -> &WorthUiCompositionNodeId {
        &self.child
    }

    pub fn order(&self) -> u32 {
        self.order
    }

    pub fn sizing(&self) -> WorthUiCompositionChildSizing {
        self.sizing
    }

    pub fn fact_id(&self) -> &WorthUiRuntimeFactId {
        &self.fact_id
    }

    pub fn receipt_digest(&self) -> u64 {
        self.receipt_digest
    }
}

impl WorthUiCompositionPolicyAttachmentReceipt {
    pub(crate) fn new(
        node_id: WorthUiCompositionNodeId,
        policy_kind: super::WorthUiCompositionPolicyKind,
        policy_identity: impl Into<String>,
    ) -> Self {
        let policy_identity = policy_identity.into();
        let fact_identity = format!(
            "{}:{}:{}",
            node_id.as_str(),
            policy_kind.token(),
            policy_identity
        );
        let fact_id = WorthUiRuntimeFactId::composition_policy(fact_identity.clone());
        let receipt_digest = digest_parts(["composition_policy", fact_identity.as_str()]);
        Self {
            node_id,
            policy_kind,
            policy_identity,
            fact_id,
            receipt_digest,
        }
    }

    pub fn node_id(&self) -> &WorthUiCompositionNodeId {
        &self.node_id
    }

    pub fn policy_kind(&self) -> super::WorthUiCompositionPolicyKind {
        self.policy_kind
    }

    pub fn policy_identity(&self) -> &str {
        &self.policy_identity
    }

    pub fn fact_id(&self) -> &WorthUiRuntimeFactId {
        &self.fact_id
    }

    pub fn receipt_digest(&self) -> u64 {
        self.receipt_digest
    }
}

impl WorthUiCompositionGraphCounters {
    pub fn node_count(self) -> usize {
        self.node_count
    }

    pub fn edge_count(self) -> usize {
        self.edge_count
    }

    pub fn policy_attachment_count(self) -> usize {
        self.policy_attachment_count
    }

    pub fn selected_graph_obligation_count(self) -> usize {
        self.selected_graph_obligation_count
    }

    pub fn source_reparse_count(self) -> usize {
        self.source_reparse_count
    }

    pub fn renderer_parse_count(self) -> usize {
        self.renderer_parse_count
    }
}
