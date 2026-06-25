use crate::runtime::composition_graph::digest::digest_parts;
use crate::runtime::{
    WorthUiCompositionEdgeReceipt, WorthUiCompositionNodeReceipt,
    WorthUiQueryGraphExecutionReceipt, WorthUiRuntimeFactId,
};

use super::super::{WorthUiAdmittedCompositionGraphReceipt, WorthUiCompositionRootReceipt};
use super::counters::WorthUiCompositionGraphAccessCounters;
use super::indexes::WorthUiCompositionGraphIndexes;
use super::request::WorthUiCompositionGraphAccessRequest;

mod materialization;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiCompositionGraphAccessPlanReceipt {
    root: WorthUiCompositionRootReceipt,
    request: WorthUiCompositionGraphAccessRequest,
    consumed_facts: Vec<WorthUiRuntimeFactId>,
    query_graph_execution: WorthUiQueryGraphExecutionReceipt,
    counters: WorthUiCompositionGraphAccessCounters,
    plan_digest: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiCompositionGraphAccessReceipt {
    plan: WorthUiCompositionGraphAccessPlanReceipt,
    root_children: Vec<WorthUiCompositionGraphChildAccessRow>,
    child_rows: Vec<WorthUiCompositionGraphChildAccessRow>,
    ancestor_rows: Vec<WorthUiCompositionGraphAncestorAccessRow>,
    participating_descendant_rows: Vec<WorthUiCompositionGraphChildAccessRow>,
    affected_consumer_rows: Vec<WorthUiCompositionGraphAffectedConsumerRow>,
    access_digest: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiCompositionGraphChildAccessRow {
    parent_id: String,
    order: u32,
    sizing_token: String,
    node: WorthUiCompositionNodeReceipt,
    edge: WorthUiCompositionEdgeReceipt,
    row_digest: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiCompositionGraphAncestorAccessRow {
    node_id: String,
    ancestor_id: String,
    depth: usize,
    row_digest: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiCompositionGraphAffectedConsumerRow {
    changed_fact: WorthUiRuntimeFactId,
    consumer_fact: WorthUiRuntimeFactId,
    semantic_slice: &'static str,
    row_digest: u64,
}

impl WorthUiCompositionGraphAccessPlanReceipt {
    pub(super) fn new(
        graph: &WorthUiAdmittedCompositionGraphReceipt,
        request: WorthUiCompositionGraphAccessRequest,
        consumed_facts: Vec<WorthUiRuntimeFactId>,
        query_graph_execution: WorthUiQueryGraphExecutionReceipt,
        counters: WorthUiCompositionGraphAccessCounters,
    ) -> Self {
        let plan_digest = digest_parts(
            [
                graph.receipt_digest().to_string(),
                request.token().to_owned(),
                request.identity(graph.root().root_id()).to_owned(),
                query_graph_execution.execution_digest().to_string(),
            ]
            .into_iter()
            .chain(consumed_facts.iter().map(|fact| fact.identity().to_owned())),
        );
        Self {
            root: graph.root().clone(),
            request,
            consumed_facts,
            query_graph_execution,
            counters,
            plan_digest,
        }
    }

    pub(super) fn execute(
        self,
        graph: &WorthUiAdmittedCompositionGraphReceipt,
        indexes: WorthUiCompositionGraphIndexes,
    ) -> WorthUiCompositionGraphAccessReceipt {
        WorthUiCompositionGraphAccessReceipt::from_plan(self, graph, indexes)
    }

    pub fn root(&self) -> &WorthUiCompositionRootReceipt {
        &self.root
    }

    pub fn request(&self) -> &WorthUiCompositionGraphAccessRequest {
        &self.request
    }

    pub fn consumed_facts(&self) -> &[WorthUiRuntimeFactId] {
        &self.consumed_facts
    }

    pub fn query_graph_execution(&self) -> &WorthUiQueryGraphExecutionReceipt {
        &self.query_graph_execution
    }

    pub fn counters(&self) -> WorthUiCompositionGraphAccessCounters {
        self.counters
    }

    pub fn plan_digest(&self) -> u64 {
        self.plan_digest
    }
}

impl WorthUiCompositionGraphAccessReceipt {
    fn from_plan(
        plan: WorthUiCompositionGraphAccessPlanReceipt,
        graph: &WorthUiAdmittedCompositionGraphReceipt,
        indexes: WorthUiCompositionGraphIndexes,
    ) -> Self {
        let root_id = graph.root().root_id().as_str();
        let root_children =
            materialization::root_child_rows_for_request(&plan.request, root_id, &indexes);
        let child_rows = materialization::child_rows_for_request(&plan.request, graph, &indexes);
        let ancestor_rows =
            materialization::ancestor_rows_for_request(&plan.request, graph, &indexes);
        let participating_descendant_rows =
            materialization::participating_descendant_rows_for_request(
                &plan.request,
                root_id,
                &indexes,
            );
        let affected_consumer_rows =
            materialization::affected_consumer_rows_for_request(&plan.request, graph);
        let access_digest = digest_parts(
            [plan.plan_digest().to_string()]
                .into_iter()
                .chain(root_children.iter().map(|row| row.row_digest().to_string()))
                .chain(child_rows.iter().map(|row| row.row_digest().to_string()))
                .chain(ancestor_rows.iter().map(|row| row.row_digest().to_string()))
                .chain(
                    participating_descendant_rows
                        .iter()
                        .map(|row| row.row_digest().to_string()),
                )
                .chain(
                    affected_consumer_rows
                        .iter()
                        .map(|row| row.row_digest().to_string()),
                ),
        );
        Self {
            plan,
            root_children,
            child_rows,
            ancestor_rows,
            participating_descendant_rows,
            affected_consumer_rows,
            access_digest,
        }
    }

    pub fn plan(&self) -> &WorthUiCompositionGraphAccessPlanReceipt {
        &self.plan
    }

    pub fn root_children(&self) -> &[WorthUiCompositionGraphChildAccessRow] {
        &self.root_children
    }

    pub fn child_rows(&self) -> &[WorthUiCompositionGraphChildAccessRow] {
        &self.child_rows
    }

    pub fn ordered_children(&self, parent_id: &str) -> Vec<&WorthUiCompositionGraphChildAccessRow> {
        self.child_rows
            .iter()
            .filter(move |row| row.parent_id() == parent_id)
            .collect()
    }

    pub fn ancestors_of(&self, node_id: &str) -> Vec<&WorthUiCompositionGraphAncestorAccessRow> {
        self.ancestor_rows
            .iter()
            .filter(move |row| row.node_id() == node_id)
            .collect()
    }

    pub fn parent_of(&self, node_id: &str) -> Option<&str> {
        self.ancestors_of(node_id)
            .first()
            .map(|row| row.ancestor_id())
    }

    pub fn participating_descendants(&self) -> &[WorthUiCompositionGraphChildAccessRow] {
        &self.participating_descendant_rows
    }

    pub fn affected_consumers(&self) -> &[WorthUiCompositionGraphAffectedConsumerRow] {
        &self.affected_consumer_rows
    }

    pub fn counters(&self) -> WorthUiCompositionGraphAccessCounters {
        self.plan.counters()
    }

    pub fn access_digest(&self) -> u64 {
        self.access_digest
    }
}

impl WorthUiCompositionGraphChildAccessRow {
    pub(super) fn new(
        parent_id: String,
        edge: WorthUiCompositionEdgeReceipt,
        node: WorthUiCompositionNodeReceipt,
    ) -> Self {
        let order = edge.order();
        let sizing_token = edge.sizing().token();
        let row_digest = digest_parts([
            "composition_child_access",
            &parent_id,
            &order.to_string(),
            sizing_token.as_str(),
            node.receipt_digest().to_string().as_str(),
            edge.receipt_digest().to_string().as_str(),
        ]);
        Self {
            parent_id,
            order,
            sizing_token,
            node,
            edge,
            row_digest,
        }
    }

    pub fn parent_id(&self) -> &str {
        &self.parent_id
    }

    pub fn order(&self) -> u32 {
        self.order
    }

    pub fn sizing_token(&self) -> &str {
        &self.sizing_token
    }

    pub fn node(&self) -> &WorthUiCompositionNodeReceipt {
        &self.node
    }

    pub fn edge(&self) -> &WorthUiCompositionEdgeReceipt {
        &self.edge
    }

    pub fn row_digest(&self) -> u64 {
        self.row_digest
    }
}

impl WorthUiCompositionGraphAncestorAccessRow {
    pub(super) fn new(node_id: &str, ancestor_id: String, depth: usize) -> Self {
        let row_digest = digest_parts([
            "composition_ancestor_access",
            node_id,
            &ancestor_id,
            &depth.to_string(),
        ]);
        Self {
            node_id: node_id.to_owned(),
            ancestor_id,
            depth,
            row_digest,
        }
    }

    pub fn node_id(&self) -> &str {
        &self.node_id
    }

    pub fn ancestor_id(&self) -> &str {
        &self.ancestor_id
    }

    pub fn depth(&self) -> usize {
        self.depth
    }

    pub fn row_digest(&self) -> u64 {
        self.row_digest
    }
}

impl WorthUiCompositionGraphAffectedConsumerRow {
    pub(super) fn new(
        changed_fact: WorthUiRuntimeFactId,
        consumer_fact: WorthUiRuntimeFactId,
    ) -> Self {
        let row_digest = digest_parts([
            "composition_affected_consumer",
            changed_fact.identity(),
            consumer_fact.identity(),
        ]);
        Self {
            changed_fact,
            consumer_fact,
            semantic_slice: "MountedCompositionTree",
            row_digest,
        }
    }

    pub fn changed_fact(&self) -> &WorthUiRuntimeFactId {
        &self.changed_fact
    }

    pub fn consumer_fact(&self) -> &WorthUiRuntimeFactId {
        &self.consumer_fact
    }

    pub fn semantic_slice(&self) -> &'static str {
        self.semantic_slice
    }

    pub fn row_digest(&self) -> u64 {
        self.row_digest
    }
}
