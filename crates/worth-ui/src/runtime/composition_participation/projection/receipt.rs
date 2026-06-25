use super::accessibility_relationships::WorthUiAccessibilityRelationshipReceipt;
use super::digest::digest_parts;
use super::focus_scopes::WorthUiFocusScopeParticipationReceipt;
use super::traversal::WorthUiCompositionParticipationTraversalReceipt;
use crate::runtime::{WorthUiQueryGraphExecutionReceipt, WorthUiRuntimeFactId};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiAccessibilityAssociationKind {
    Label,
    Description,
    Error,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiAccessibilityParticipationPosture {
    Exposed,
    Hidden,
    Inert,
    Disabled,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiFocusParticipationPosture {
    Focusable,
    NotFocusable,
    Disabled,
    Inert,
    Hidden,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiAccessibilityAssociationReceipt {
    kind: WorthUiAccessibilityAssociationKind,
    source_node_id: String,
    target_node_id: String,
    association_digest: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiAccessibilityNodeParticipationReceipt {
    node_id: String,
    role: String,
    name: Option<String>,
    description_node_ids: Vec<String>,
    error_node_ids: Vec<String>,
    posture: WorthUiAccessibilityParticipationPosture,
    receipt_digest: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiFocusNodeParticipationReceipt {
    node_id: String,
    focus_scope_id: String,
    graph_order: u32,
    posture: WorthUiFocusParticipationPosture,
    receipt_digest: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiCompositionParticipationReceipt {
    root_id: String,
    accessibility_nodes: Vec<WorthUiAccessibilityNodeParticipationReceipt>,
    focus_nodes: Vec<WorthUiFocusNodeParticipationReceipt>,
    focus_scopes: Vec<WorthUiFocusScopeParticipationReceipt>,
    associations: Vec<WorthUiAccessibilityAssociationReceipt>,
    relationships: Vec<WorthUiAccessibilityRelationshipReceipt>,
    traversal: WorthUiCompositionParticipationTraversalReceipt,
    consumed_facts: Vec<WorthUiRuntimeFactId>,
    query_graph_execution: WorthUiQueryGraphExecutionReceipt,
    counters: WorthUiCompositionParticipationCounters,
    receipt_digest: u64,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiCompositionParticipationCounters {
    pub(super) accessibility_node_count: usize,
    pub(super) focus_node_count: usize,
    pub(super) focus_scope_count: usize,
    pub(super) association_count: usize,
    pub(super) relationship_count: usize,
    pub(super) selected_graph_obligation_count: usize,
    pub(super) graph_child_row_count: usize,
    pub(super) caller_owned_recursive_walk_count: usize,
    pub(super) caller_owned_scan_count: usize,
    pub(super) source_reparse_count: usize,
    pub(super) renderer_parse_count: usize,
}

impl WorthUiAccessibilityAssociationReceipt {
    pub(crate) fn new(
        kind: WorthUiAccessibilityAssociationKind,
        source_node_id: impl Into<String>,
        target_node_id: impl Into<String>,
    ) -> Self {
        let source_node_id = source_node_id.into();
        let target_node_id = target_node_id.into();
        let association_digest = digest_parts([
            "accessibility_association",
            kind.token(),
            source_node_id.as_str(),
            target_node_id.as_str(),
        ]);
        Self {
            kind,
            source_node_id,
            target_node_id,
            association_digest,
        }
    }

    pub fn kind(&self) -> WorthUiAccessibilityAssociationKind {
        self.kind
    }

    pub fn source_node_id(&self) -> &str {
        &self.source_node_id
    }

    pub fn target_node_id(&self) -> &str {
        &self.target_node_id
    }

    pub fn association_digest(&self) -> u64 {
        self.association_digest
    }
}

impl WorthUiAccessibilityNodeParticipationReceipt {
    pub(crate) fn new(
        node_id: impl Into<String>,
        role: impl Into<String>,
        name: Option<String>,
        description_node_ids: Vec<String>,
        error_node_ids: Vec<String>,
        posture: WorthUiAccessibilityParticipationPosture,
    ) -> Self {
        let node_id = node_id.into();
        let role = role.into();
        let receipt_digest = digest_parts(
            [
                "accessibility_node".to_owned(),
                node_id.clone(),
                role.clone(),
                name.clone().unwrap_or_default(),
                posture.token().to_owned(),
            ]
            .into_iter()
            .chain(description_node_ids.iter().cloned())
            .chain(error_node_ids.iter().cloned()),
        );
        Self {
            node_id,
            role,
            name,
            description_node_ids,
            error_node_ids,
            posture,
            receipt_digest,
        }
    }

    pub fn node_id(&self) -> &str {
        &self.node_id
    }

    pub fn role(&self) -> &str {
        &self.role
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn description_node_ids(&self) -> &[String] {
        &self.description_node_ids
    }

    pub fn error_node_ids(&self) -> &[String] {
        &self.error_node_ids
    }

    pub fn posture(&self) -> WorthUiAccessibilityParticipationPosture {
        self.posture
    }

    pub fn receipt_digest(&self) -> u64 {
        self.receipt_digest
    }
}

impl WorthUiFocusNodeParticipationReceipt {
    pub(crate) fn new(
        node_id: impl Into<String>,
        focus_scope_id: impl Into<String>,
        graph_order: u32,
        posture: WorthUiFocusParticipationPosture,
    ) -> Self {
        let node_id = node_id.into();
        let focus_scope_id = focus_scope_id.into();
        let receipt_digest = digest_parts([
            "focus_node",
            node_id.as_str(),
            focus_scope_id.as_str(),
            graph_order.to_string().as_str(),
            posture.token(),
        ]);
        Self {
            node_id,
            focus_scope_id,
            graph_order,
            posture,
            receipt_digest,
        }
    }

    pub fn node_id(&self) -> &str {
        &self.node_id
    }

    pub fn focus_scope_id(&self) -> &str {
        &self.focus_scope_id
    }

    pub fn graph_order(&self) -> u32 {
        self.graph_order
    }

    pub fn posture(&self) -> WorthUiFocusParticipationPosture {
        self.posture
    }

    pub fn receipt_digest(&self) -> u64 {
        self.receipt_digest
    }
}

impl WorthUiCompositionParticipationReceipt {
    pub(crate) fn new(
        root_id: impl Into<String>,
        accessibility_nodes: Vec<WorthUiAccessibilityNodeParticipationReceipt>,
        focus_nodes: Vec<WorthUiFocusNodeParticipationReceipt>,
        focus_scopes: Vec<WorthUiFocusScopeParticipationReceipt>,
        associations: Vec<WorthUiAccessibilityAssociationReceipt>,
        relationships: Vec<WorthUiAccessibilityRelationshipReceipt>,
        traversal: WorthUiCompositionParticipationTraversalReceipt,
        mut consumed_facts: Vec<WorthUiRuntimeFactId>,
        query_graph_execution: WorthUiQueryGraphExecutionReceipt,
    ) -> Self {
        let root_id = root_id.into();
        consumed_facts.sort();
        consumed_facts.dedup();
        let counters = WorthUiCompositionParticipationCounters {
            accessibility_node_count: accessibility_nodes.len(),
            focus_node_count: focus_nodes.len(),
            focus_scope_count: focus_scopes.len(),
            association_count: associations.len(),
            relationship_count: relationships.len(),
            selected_graph_obligation_count: query_graph_execution.selected_obligation_count(),
            graph_child_row_count: traversal.counters().graph_child_row_count(),
            caller_owned_recursive_walk_count: traversal
                .counters()
                .caller_owned_recursive_walk_count(),
            caller_owned_scan_count: traversal.counters().caller_owned_scan_count(),
            source_reparse_count: traversal.counters().source_reparse_count(),
            renderer_parse_count: traversal.counters().renderer_parse_count(),
        };
        let receipt_digest = digest_parts(
            [
                "composition_participation".to_owned(),
                root_id.clone(),
                query_graph_execution.execution_digest().to_string(),
                traversal.receipt_digest().to_string(),
            ]
            .into_iter()
            .chain(
                accessibility_nodes
                    .iter()
                    .map(|node| node.receipt_digest().to_string()),
            )
            .chain(
                focus_nodes
                    .iter()
                    .map(|node| node.receipt_digest().to_string()),
            )
            .chain(
                focus_scopes
                    .iter()
                    .map(|scope| scope.receipt_digest().to_string()),
            )
            .chain(
                associations
                    .iter()
                    .map(|association| association.association_digest().to_string()),
            )
            .chain(
                relationships
                    .iter()
                    .map(|relationship| relationship.receipt_digest().to_string()),
            )
            .chain(consumed_facts.iter().map(|fact| fact.identity().to_owned())),
        );
        Self {
            root_id,
            accessibility_nodes,
            focus_nodes,
            focus_scopes,
            associations,
            relationships,
            traversal,
            consumed_facts,
            query_graph_execution,
            counters,
            receipt_digest,
        }
    }

    pub fn root_id(&self) -> &str {
        &self.root_id
    }

    pub fn accessibility_nodes(&self) -> &[WorthUiAccessibilityNodeParticipationReceipt] {
        &self.accessibility_nodes
    }

    pub fn focus_nodes(&self) -> &[WorthUiFocusNodeParticipationReceipt] {
        &self.focus_nodes
    }

    pub fn focus_scopes(&self) -> &[WorthUiFocusScopeParticipationReceipt] {
        &self.focus_scopes
    }

    pub fn associations(&self) -> &[WorthUiAccessibilityAssociationReceipt] {
        &self.associations
    }

    pub fn relationships(&self) -> &[WorthUiAccessibilityRelationshipReceipt] {
        &self.relationships
    }

    pub fn traversal(&self) -> &WorthUiCompositionParticipationTraversalReceipt {
        &self.traversal
    }

    pub fn consumed_facts(&self) -> &[WorthUiRuntimeFactId] {
        &self.consumed_facts
    }

    pub fn query_graph_execution(&self) -> &WorthUiQueryGraphExecutionReceipt {
        &self.query_graph_execution
    }

    pub fn counters(&self) -> WorthUiCompositionParticipationCounters {
        self.counters
    }

    pub fn receipt_digest(&self) -> u64 {
        self.receipt_digest
    }
}
