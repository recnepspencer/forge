use super::{
    admission::admit_composition_graph, WorthUiAdmittedCompositionGraphReceipt,
    WorthUiCompositionChildSizing, WorthUiCompositionContextDefinition,
    WorthUiCompositionGraphAdmissionDenial, WorthUiCompositionNodeId, WorthUiCompositionNodeKind,
    WorthUiCompositionParentRef, WorthUiCompositionParticipation, WorthUiCompositionPolicyKind,
    WorthUiCompositionRootId, WorthUiCompositionRootKind,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiCompositionRootDefinition {
    root_id: WorthUiCompositionRootId,
    kind: WorthUiCompositionRootKind,
    authority_identity: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiCompositionNodeDefinition {
    node_id: WorthUiCompositionNodeId,
    kind: WorthUiCompositionNodeKind,
    authority_identity: String,
    participation: WorthUiCompositionParticipation,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiCompositionGraphDefinition {
    root: WorthUiCompositionRootDefinition,
    nodes: Vec<WorthUiCompositionNodeDefinition>,
    edges: Vec<WorthUiCompositionEdgeDefinition>,
    policy_attachments: Vec<WorthUiCompositionPolicyAttachmentDefinition>,
    contexts: Vec<WorthUiCompositionContextDefinition>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiCompositionEdgeDefinition {
    pub(crate) parent: WorthUiCompositionParentRef,
    pub(crate) child: WorthUiCompositionNodeId,
    pub(crate) order: u32,
    pub(crate) sizing: WorthUiCompositionChildSizing,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiCompositionPolicyAttachmentDefinition {
    pub(crate) node_id: WorthUiCompositionNodeId,
    pub(crate) policy_kind: WorthUiCompositionPolicyKind,
    pub(crate) policy_identity: String,
}

impl WorthUiCompositionRootDefinition {
    pub fn surface(identity: impl Into<String>) -> Self {
        Self::new(WorthUiCompositionRootKind::Surface, identity)
    }

    pub fn new(kind: WorthUiCompositionRootKind, identity: impl Into<String>) -> Self {
        let authority_identity = identity.into();
        let root_id = WorthUiCompositionRootId::new(format!(
            "composition.root.{}.{}",
            kind.token(),
            authority_identity
        ))
        .expect("composition root definitions use non-empty authority identities");
        Self {
            root_id,
            kind,
            authority_identity,
        }
    }

    pub fn root_id(&self) -> &WorthUiCompositionRootId {
        &self.root_id
    }
}

impl WorthUiCompositionNodeDefinition {
    pub fn new(
        kind: WorthUiCompositionNodeKind,
        node_id: impl Into<String>,
        authority_identity: impl Into<String>,
    ) -> Self {
        Self {
            node_id: WorthUiCompositionNodeId::new(node_id)
                .expect("composition node definitions use non-empty ids"),
            kind,
            authority_identity: authority_identity.into(),
            participation: WorthUiCompositionParticipation::Present,
        }
    }

    pub fn container(node_id: impl Into<String>) -> Self {
        let node_id = node_id.into();
        Self::new(
            WorthUiCompositionNodeKind::Container,
            node_id.clone(),
            node_id,
        )
    }

    pub fn with_participation(mut self, participation: WorthUiCompositionParticipation) -> Self {
        self.participation = participation;
        self
    }

    pub fn node_id(&self) -> &WorthUiCompositionNodeId {
        &self.node_id
    }
}

impl WorthUiCompositionGraphDefinition {
    pub fn for_root(root: WorthUiCompositionRootDefinition) -> Self {
        Self {
            root,
            nodes: Vec::new(),
            edges: Vec::new(),
            policy_attachments: Vec::new(),
            contexts: Vec::new(),
        }
    }

    pub fn with_node(mut self, node: WorthUiCompositionNodeDefinition) -> Self {
        self.nodes.push(node);
        self
    }

    pub fn with_policy_attachment(
        mut self,
        node_id: impl AsRef<str>,
        policy_kind: WorthUiCompositionPolicyKind,
        policy_identity: impl Into<String>,
    ) -> Self {
        self.policy_attachments
            .push(WorthUiCompositionPolicyAttachmentDefinition {
                node_id: WorthUiCompositionNodeId::new(node_id.as_ref())
                    .expect("policy attachment node ids must not be empty"),
                policy_kind,
                policy_identity: policy_identity.into(),
            });
        self
    }

    pub fn with_context(mut self, context: WorthUiCompositionContextDefinition) -> Self {
        self.contexts.push(context);
        self
    }

    pub fn with_root_child(mut self, child_id: impl AsRef<str>) -> Self {
        let order = self.next_order_for_parent(self.root.root_id.as_str());
        self = self.with_root_child_at(child_id, order);
        self
    }

    pub fn with_root_child_at(mut self, child_id: impl AsRef<str>, order: u32) -> Self {
        self = self.with_root_child_at_with_sizing(
            child_id,
            order,
            WorthUiCompositionChildSizing::Auto,
        );
        self
    }

    pub fn with_root_child_at_with_sizing(
        mut self,
        child_id: impl AsRef<str>,
        order: u32,
        sizing: WorthUiCompositionChildSizing,
    ) -> Self {
        self.edges.push(WorthUiCompositionEdgeDefinition {
            parent: WorthUiCompositionParentRef::Root(self.root.root_id.clone()),
            child: WorthUiCompositionNodeId::new(child_id.as_ref())
                .expect("root child ids must not be empty"),
            order,
            sizing,
        });
        self
    }

    pub fn with_parent(mut self, parent_id: impl AsRef<str>, child_id: impl AsRef<str>) -> Self {
        let parent_id = WorthUiCompositionNodeId::new(parent_id.as_ref())
            .expect("parent node ids must not be empty");
        let order = self.next_order_for_parent(parent_id.as_str());
        self = self.with_parent_at(parent_id.as_str(), child_id, order);
        self
    }

    pub fn with_parent_at(
        mut self,
        parent_id: impl AsRef<str>,
        child_id: impl AsRef<str>,
        order: u32,
    ) -> Self {
        self = self.with_parent_at_with_sizing(
            parent_id,
            child_id,
            order,
            WorthUiCompositionChildSizing::Auto,
        );
        self
    }

    pub fn with_parent_at_with_sizing(
        mut self,
        parent_id: impl AsRef<str>,
        child_id: impl AsRef<str>,
        order: u32,
        sizing: WorthUiCompositionChildSizing,
    ) -> Self {
        let parent_id = WorthUiCompositionNodeId::new(parent_id.as_ref())
            .expect("parent node ids must not be empty");
        self.edges.push(WorthUiCompositionEdgeDefinition {
            parent: WorthUiCompositionParentRef::Node(parent_id),
            child: WorthUiCompositionNodeId::new(child_id.as_ref())
                .expect("child node ids must not be empty"),
            order,
            sizing,
        });
        self
    }

    pub fn admit(
        self,
    ) -> Result<WorthUiAdmittedCompositionGraphReceipt, Vec<WorthUiCompositionGraphAdmissionDenial>>
    {
        admit_composition_graph(self)
    }

    pub(crate) fn root(&self) -> &WorthUiCompositionRootDefinition {
        &self.root
    }

    pub(crate) fn nodes(&self) -> &[WorthUiCompositionNodeDefinition] {
        &self.nodes
    }

    pub(crate) fn edges(&self) -> &[WorthUiCompositionEdgeDefinition] {
        &self.edges
    }

    pub(crate) fn policy_attachments(&self) -> &[WorthUiCompositionPolicyAttachmentDefinition] {
        &self.policy_attachments
    }

    pub fn context_definitions(&self) -> &[WorthUiCompositionContextDefinition] {
        &self.contexts
    }

    fn next_order_for_parent(&self, parent_identity: &str) -> u32 {
        self.edges
            .iter()
            .filter(|edge| edge.parent.identity() == parent_identity)
            .count() as u32
    }
}

impl WorthUiCompositionRootDefinition {
    pub(crate) fn kind(&self) -> WorthUiCompositionRootKind {
        self.kind
    }

    pub(crate) fn authority_identity(&self) -> &str {
        &self.authority_identity
    }
}

impl WorthUiCompositionNodeDefinition {
    pub(crate) fn kind(&self) -> WorthUiCompositionNodeKind {
        self.kind
    }

    pub(crate) fn authority_identity(&self) -> &str {
        &self.authority_identity
    }

    pub(crate) fn participation(&self) -> WorthUiCompositionParticipation {
        self.participation
    }
}
