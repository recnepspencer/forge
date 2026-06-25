use std::collections::BTreeMap;

use super::super::digest::digest_parts;
use super::receipt::WorthUiCompositionContextPropagationReceipt;
use crate::runtime::{WorthUiRuntimeFactId, WorthUiRuntimeGraphAuthority};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiCompositionContextDeltaReceipt {
    changed_context_facts: Vec<WorthUiRuntimeFactId>,
    preserved_context_facts: Vec<WorthUiRuntimeFactId>,
    affected_descendant_nodes: Vec<String>,
    preserved_sibling_nodes: Vec<String>,
    consumer_intersections: Vec<WorthUiCompositionContextConsumerIntersectionRow>,
    query_graph_execution_digest: u64,
    counters: WorthUiCompositionContextDeltaCounters,
    receipt_digest: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiCompositionContextConsumerIntersectionRow {
    changed_fact: WorthUiRuntimeFactId,
    consumer_fact: WorthUiRuntimeFactId,
    node_id: String,
    row_digest: u64,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiCompositionContextDeltaCounters {
    compared_context_count: usize,
    changed_context_count: usize,
    preserved_context_count: usize,
    source_reparse_count: usize,
    renderer_parse_count: usize,
}

pub fn compare_composition_context_propagation(
    prior: &WorthUiCompositionContextPropagationReceipt,
    next: &WorthUiCompositionContextPropagationReceipt,
) -> WorthUiCompositionContextDeltaReceipt {
    let prior_by_node = prior_contexts_by_node(prior);
    let next_by_node = prior_contexts_by_node(next);
    let mut changed_context_facts = Vec::new();
    let mut preserved_context_facts = Vec::new();
    let mut affected_descendant_nodes = Vec::new();
    let mut preserved_sibling_nodes = Vec::new();
    let mut consumer_intersections = Vec::new();
    for (node_id, next_digest) in &next_by_node {
        let fact = WorthUiRuntimeFactId::composition_context(node_id);
        if prior_by_node.get(node_id) == Some(next_digest) {
            preserved_context_facts.push(fact);
            preserved_sibling_nodes.push(node_id.clone());
            continue;
        }
        changed_context_facts.push(fact.clone());
        affected_descendant_nodes.push(node_id.clone());
        consumer_intersections.push(WorthUiCompositionContextConsumerIntersectionRow::new(
            fact,
            WorthUiRuntimeFactId::composition_context_propagation("context_delta"),
            node_id.clone(),
        ));
    }
    let query_graph_execution_digest = WorthUiRuntimeGraphAuthority::new()
        .plan_composition_context_graph_operation(
            "context_delta",
            changed_context_facts
                .iter()
                .cloned()
                .chain(preserved_context_facts.iter().cloned()),
        )
        .into_execution_receipt()
        .execution_digest();
    WorthUiCompositionContextDeltaReceipt::new(
        changed_context_facts,
        preserved_context_facts,
        affected_descendant_nodes,
        preserved_sibling_nodes,
        consumer_intersections,
        query_graph_execution_digest,
    )
}

impl WorthUiCompositionContextDeltaReceipt {
    fn new(
        changed_context_facts: Vec<WorthUiRuntimeFactId>,
        preserved_context_facts: Vec<WorthUiRuntimeFactId>,
        affected_descendant_nodes: Vec<String>,
        preserved_sibling_nodes: Vec<String>,
        consumer_intersections: Vec<WorthUiCompositionContextConsumerIntersectionRow>,
        query_graph_execution_digest: u64,
    ) -> Self {
        let counters = WorthUiCompositionContextDeltaCounters {
            compared_context_count: changed_context_facts.len() + preserved_context_facts.len(),
            changed_context_count: changed_context_facts.len(),
            preserved_context_count: preserved_context_facts.len(),
            source_reparse_count: 0,
            renderer_parse_count: 0,
        };
        let receipt_digest = digest_parts(
            ["composition_context_delta".to_owned()]
                .into_iter()
                .chain(
                    changed_context_facts
                        .iter()
                        .map(|fact| fact.identity().to_owned()),
                )
                .chain(
                    preserved_context_facts
                        .iter()
                        .map(|fact| fact.identity().to_owned()),
                )
                .chain(affected_descendant_nodes.iter().cloned())
                .chain(preserved_sibling_nodes.iter().cloned())
                .chain(
                    consumer_intersections
                        .iter()
                        .map(|row| row.row_digest().to_string()),
                )
                .chain(std::iter::once(query_graph_execution_digest.to_string())),
        );
        Self {
            changed_context_facts,
            preserved_context_facts,
            affected_descendant_nodes,
            preserved_sibling_nodes,
            consumer_intersections,
            query_graph_execution_digest,
            counters,
            receipt_digest,
        }
    }

    pub fn changed_context_facts(&self) -> &[WorthUiRuntimeFactId] {
        &self.changed_context_facts
    }

    pub fn preserved_context_facts(&self) -> &[WorthUiRuntimeFactId] {
        &self.preserved_context_facts
    }

    pub fn affected_descendant_nodes(&self) -> &[String] {
        &self.affected_descendant_nodes
    }

    pub fn preserved_sibling_nodes(&self) -> &[String] {
        &self.preserved_sibling_nodes
    }

    pub fn consumer_intersections(&self) -> &[WorthUiCompositionContextConsumerIntersectionRow] {
        &self.consumer_intersections
    }

    pub fn query_graph_execution_digest(&self) -> u64 {
        self.query_graph_execution_digest
    }

    pub fn counters(&self) -> WorthUiCompositionContextDeltaCounters {
        self.counters
    }

    pub fn receipt_digest(&self) -> u64 {
        self.receipt_digest
    }
}

impl WorthUiCompositionContextConsumerIntersectionRow {
    fn new(
        changed_fact: WorthUiRuntimeFactId,
        consumer_fact: WorthUiRuntimeFactId,
        node_id: String,
    ) -> Self {
        let row_digest = digest_parts([
            "composition_context_consumer_intersection",
            changed_fact.identity(),
            consumer_fact.identity(),
            &node_id,
        ]);
        Self {
            changed_fact,
            consumer_fact,
            node_id,
            row_digest,
        }
    }

    pub fn changed_fact(&self) -> &WorthUiRuntimeFactId {
        &self.changed_fact
    }

    pub fn consumer_fact(&self) -> &WorthUiRuntimeFactId {
        &self.consumer_fact
    }

    pub fn node_id(&self) -> &str {
        &self.node_id
    }

    pub fn row_digest(&self) -> u64 {
        self.row_digest
    }
}

impl WorthUiCompositionContextDeltaCounters {
    pub fn compared_context_count(self) -> usize {
        self.compared_context_count
    }

    pub fn changed_context_count(self) -> usize {
        self.changed_context_count
    }

    pub fn preserved_context_count(self) -> usize {
        self.preserved_context_count
    }

    pub fn source_reparse_count(self) -> usize {
        self.source_reparse_count
    }

    pub fn renderer_parse_count(self) -> usize {
        self.renderer_parse_count
    }
}

fn prior_contexts_by_node(
    receipt: &WorthUiCompositionContextPropagationReceipt,
) -> BTreeMap<String, u64> {
    receipt
        .node_contexts()
        .iter()
        .map(|context| {
            (
                context.node_id().as_str().to_owned(),
                context.receipt_digest(),
            )
        })
        .collect()
}
