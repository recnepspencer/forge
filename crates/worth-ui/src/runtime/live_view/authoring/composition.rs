use crate::runtime::{
    AuthoredPrimitiveContentProp, WorthUiAccessibilityAssociationKind,
    WorthUiAdmittedCompositionGraphReceipt, WorthUiCompositionChildSizing,
    WorthUiCompositionContextDefinition, WorthUiCompositionGraphAdmissionDenial,
    WorthUiCompositionGraphDefinition, WorthUiCompositionNodeDefinition,
    WorthUiCompositionNodeKind, WorthUiCompositionParticipation, WorthUiCompositionPolicyKind,
    WorthUiCompositionRootDefinition, WorthUiCompositionRootKind, WorthUiPrimitiveSourceSpan,
};

mod identity;
mod source_admission;

use identity::composition_node_identity;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiAuthoredCompositionDeclaration {
    composition_id: String,
    root: WorthUiAuthoredCompositionRootDeclaration,
    nodes: Vec<WorthUiAuthoredCompositionNodeDeclaration>,
    edges: Vec<WorthUiAuthoredCompositionEdgeDeclaration>,
    policies: Vec<WorthUiAuthoredCompositionPolicyDeclaration>,
    contexts: Vec<WorthUiCompositionContextDefinition>,
    contents: Vec<WorthUiAuthoredCompositionContentDeclaration>,
    accessibility_associations: Vec<WorthUiAuthoredCompositionAccessibilityAssociationDeclaration>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiAuthoredCompositionRootDeclaration {
    kind: WorthUiCompositionRootKind,
    authority_identity: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiAuthoredCompositionNodeDeclaration {
    node_id: String,
    kind: WorthUiCompositionNodeKind,
    authority_identity: String,
    participation: WorthUiCompositionParticipation,
    source_span: Option<WorthUiPrimitiveSourceSpan>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiAuthoredCompositionEdgeDeclaration {
    parent_id: Option<String>,
    child_id: String,
    order: u32,
    sizing: WorthUiCompositionChildSizing,
    source_span: Option<WorthUiPrimitiveSourceSpan>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiAuthoredCompositionPolicyDeclaration {
    node_id: String,
    kind: WorthUiCompositionPolicyKind,
    policy_identity: String,
    source_span: Option<WorthUiPrimitiveSourceSpan>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiAuthoredCompositionContentDeclaration {
    node_id: String,
    authority_identity: String,
    props: Vec<AuthoredPrimitiveContentProp>,
    source_span: Option<WorthUiPrimitiveSourceSpan>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiAuthoredCompositionAccessibilityAssociationDeclaration {
    kind: WorthUiAccessibilityAssociationKind,
    source_identity: String,
    target_identity: String,
    source_span: Option<WorthUiPrimitiveSourceSpan>,
}

impl WorthUiAuthoredCompositionDeclaration {
    pub(super) fn new(
        composition_id: impl Into<String>,
        root: WorthUiAuthoredCompositionRootDeclaration,
    ) -> Self {
        Self {
            composition_id: composition_id.into(),
            root,
            nodes: Vec::new(),
            edges: Vec::new(),
            policies: Vec::new(),
            contexts: Vec::new(),
            contents: Vec::new(),
            accessibility_associations: Vec::new(),
        }
    }

    pub(super) fn push_node(&mut self, node: WorthUiAuthoredCompositionNodeDeclaration) {
        self.nodes.push(node);
    }

    pub(super) fn push_edge(&mut self, edge: WorthUiAuthoredCompositionEdgeDeclaration) {
        self.edges.push(edge);
    }

    pub(super) fn push_policy(&mut self, policy: WorthUiAuthoredCompositionPolicyDeclaration) {
        self.policies.push(policy);
    }

    pub(super) fn push_context(&mut self, context: WorthUiCompositionContextDefinition) {
        self.contexts.push(context);
    }

    pub(super) fn push_content(&mut self, content: WorthUiAuthoredCompositionContentDeclaration) {
        self.contents.push(content);
    }

    pub(super) fn push_accessibility_association(
        &mut self,
        association: WorthUiAuthoredCompositionAccessibilityAssociationDeclaration,
    ) {
        self.accessibility_associations.push(association);
    }

    pub fn composition_id(&self) -> &str {
        &self.composition_id
    }

    pub fn root(&self) -> &WorthUiAuthoredCompositionRootDeclaration {
        &self.root
    }

    pub fn nodes(&self) -> &[WorthUiAuthoredCompositionNodeDeclaration] {
        &self.nodes
    }

    pub fn edges(&self) -> &[WorthUiAuthoredCompositionEdgeDeclaration] {
        &self.edges
    }

    pub fn policies(&self) -> &[WorthUiAuthoredCompositionPolicyDeclaration] {
        &self.policies
    }

    pub fn contexts(&self) -> &[WorthUiCompositionContextDefinition] {
        &self.contexts
    }

    pub fn contents(&self) -> &[WorthUiAuthoredCompositionContentDeclaration] {
        &self.contents
    }

    pub fn accessibility_associations(
        &self,
    ) -> &[WorthUiAuthoredCompositionAccessibilityAssociationDeclaration] {
        &self.accessibility_associations
    }

    pub(crate) fn admit(
        &self,
    ) -> Result<WorthUiAdmittedCompositionGraphReceipt, Vec<WorthUiCompositionGraphAdmissionDenial>>
    {
        self.to_graph_definition().admit()
    }

    pub(crate) fn to_graph_definition(&self) -> WorthUiCompositionGraphDefinition {
        let root = WorthUiCompositionRootDefinition::new(
            self.root.kind,
            self.root.authority_identity.clone(),
        );
        let mut graph = WorthUiCompositionGraphDefinition::for_root(root);
        for node in &self.nodes {
            graph = graph.with_node(
                WorthUiCompositionNodeDefinition::new(
                    node.kind,
                    node.node_id.clone(),
                    node.authority_identity.clone(),
                )
                .with_participation(node.participation),
            );
        }
        for edge in &self.edges {
            graph = match &edge.parent_id {
                Some(parent_id) => graph.with_parent_at_with_sizing(
                    parent_id,
                    &edge.child_id,
                    edge.order,
                    edge.sizing,
                ),
                None => {
                    graph.with_root_child_at_with_sizing(&edge.child_id, edge.order, edge.sizing)
                }
            };
        }
        for policy in &self.policies {
            graph = graph.with_policy_attachment(
                &policy.node_id,
                policy.kind,
                policy.policy_identity.clone(),
            );
        }
        for context in &self.contexts {
            graph = graph.with_context(context.clone());
        }
        graph
    }
}

impl WorthUiAuthoredCompositionRootDeclaration {
    pub(super) fn surface(authority_identity: impl Into<String>) -> Self {
        Self::new(
            WorthUiCompositionRootKind::Surface,
            authority_identity.into(),
        )
    }

    pub(super) fn new(
        kind: WorthUiCompositionRootKind,
        authority_identity: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            authority_identity: authority_identity.into(),
        }
    }

    pub fn kind(&self) -> WorthUiCompositionRootKind {
        self.kind
    }

    pub fn authority_identity(&self) -> &str {
        &self.authority_identity
    }
}

impl WorthUiAuthoredCompositionNodeDeclaration {
    pub(super) fn new(kind: WorthUiCompositionNodeKind, id: impl Into<String>) -> Self {
        let id = id.into();
        let node_id = composition_node_identity(kind, &id);
        Self {
            node_id,
            kind,
            authority_identity: id,
            participation: WorthUiCompositionParticipation::Present,
            source_span: None,
        }
    }

    pub(super) fn spanned(
        kind: WorthUiCompositionNodeKind,
        id: impl Into<String>,
        source_span: WorthUiPrimitiveSourceSpan,
    ) -> Self {
        let mut node = Self::new(kind, id);
        node.source_span = Some(source_span);
        node
    }

    pub fn node_id(&self) -> &str {
        &self.node_id
    }

    pub fn kind(&self) -> WorthUiCompositionNodeKind {
        self.kind
    }

    pub fn source_span(&self) -> Option<WorthUiPrimitiveSourceSpan> {
        self.source_span
    }
}

impl WorthUiAuthoredCompositionEdgeDeclaration {
    pub(super) fn root_child(
        child_id: impl Into<String>,
        order: u32,
        sizing: WorthUiCompositionChildSizing,
    ) -> Self {
        Self {
            parent_id: None,
            child_id: child_id.into(),
            order,
            sizing,
            source_span: None,
        }
    }

    pub(super) fn child(
        parent_id: impl Into<String>,
        child_id: impl Into<String>,
        order: u32,
        sizing: WorthUiCompositionChildSizing,
    ) -> Self {
        Self {
            parent_id: Some(parent_id.into()),
            child_id: child_id.into(),
            order,
            sizing,
            source_span: None,
        }
    }

    pub(super) fn root_child_spanned(
        child_id: impl Into<String>,
        order: u32,
        sizing: WorthUiCompositionChildSizing,
        source_span: WorthUiPrimitiveSourceSpan,
    ) -> Self {
        let mut edge = Self::root_child(child_id, order, sizing);
        edge.source_span = Some(source_span);
        edge
    }

    pub(super) fn child_spanned(
        parent_id: impl Into<String>,
        child_id: impl Into<String>,
        order: u32,
        sizing: WorthUiCompositionChildSizing,
        source_span: WorthUiPrimitiveSourceSpan,
    ) -> Self {
        let mut edge = Self::child(parent_id, child_id, order, sizing);
        edge.source_span = Some(source_span);
        edge
    }

    pub fn sizing(&self) -> WorthUiCompositionChildSizing {
        self.sizing
    }

    pub fn parent_id(&self) -> Option<&str> {
        self.parent_id.as_deref()
    }

    pub fn child_id(&self) -> &str {
        &self.child_id
    }

    pub fn source_span(&self) -> Option<WorthUiPrimitiveSourceSpan> {
        self.source_span
    }
}

impl WorthUiAuthoredCompositionPolicyDeclaration {
    pub(super) fn new(
        node_id: impl Into<String>,
        kind: WorthUiCompositionPolicyKind,
        policy_identity: impl Into<String>,
    ) -> Self {
        Self {
            node_id: node_id.into(),
            kind,
            policy_identity: policy_identity.into(),
            source_span: None,
        }
    }

    pub(super) fn spanned(
        node_id: impl Into<String>,
        kind: WorthUiCompositionPolicyKind,
        policy_identity: impl Into<String>,
        source_span: WorthUiPrimitiveSourceSpan,
    ) -> Self {
        let mut policy = Self::new(node_id, kind, policy_identity);
        policy.source_span = Some(source_span);
        policy
    }

    pub fn source_span(&self) -> Option<WorthUiPrimitiveSourceSpan> {
        self.source_span
    }

    pub fn node_id(&self) -> &str {
        &self.node_id
    }
}

impl WorthUiAuthoredCompositionContentDeclaration {
    pub(super) fn new(
        authority_identity: impl Into<String>,
        props: Vec<AuthoredPrimitiveContentProp>,
        source_span: WorthUiPrimitiveSourceSpan,
    ) -> Self {
        let authority_identity = authority_identity.into();
        let node_id =
            composition_node_identity(WorthUiCompositionNodeKind::Content, &authority_identity);
        Self {
            node_id,
            authority_identity,
            props,
            source_span: Some(source_span),
        }
    }

    pub fn node_id(&self) -> &str {
        &self.node_id
    }

    pub fn authority_identity(&self) -> &str {
        &self.authority_identity
    }

    pub(crate) fn props(&self) -> &[AuthoredPrimitiveContentProp] {
        &self.props
    }

    pub fn source_span(&self) -> Option<WorthUiPrimitiveSourceSpan> {
        self.source_span
    }
}

impl WorthUiAuthoredCompositionAccessibilityAssociationDeclaration {
    pub(super) fn spanned(
        kind: WorthUiAccessibilityAssociationKind,
        source_identity: impl Into<String>,
        target_identity: impl Into<String>,
        source_span: WorthUiPrimitiveSourceSpan,
    ) -> Self {
        Self {
            kind,
            source_identity: source_identity.into(),
            target_identity: target_identity.into(),
            source_span: Some(source_span),
        }
    }

    pub fn kind(&self) -> WorthUiAccessibilityAssociationKind {
        self.kind
    }

    pub fn source_identity(&self) -> &str {
        &self.source_identity
    }

    pub fn target_identity(&self) -> &str {
        &self.target_identity
    }

    pub fn source_span(&self) -> Option<WorthUiPrimitiveSourceSpan> {
        self.source_span
    }
}
