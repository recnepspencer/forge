use crate::runtime::{
    graph::{WorthUiGraphDependencyEdge, WorthUiGraphInvalidationRequest},
    WorthUiRuntimeFactSet, WorthUiRuntimeFactSetDigest,
};

use super::{registry_selection::registries_for_request, WorthUiGraphInvalidationCounters};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiGraphInvalidationReceipt {
    authoritative_changed_facts: WorthUiRuntimeFactSet,
    affected_facts: WorthUiRuntimeFactSet,
    traversed_edges: Vec<WorthUiGraphDependencyEdge>,
    counters: WorthUiGraphInvalidationCounters,
    receipt_digest: WorthUiRuntimeFactSetDigest,
}

impl WorthUiGraphInvalidationReceipt {
    pub(crate) fn plan(request: WorthUiGraphInvalidationRequest) -> Self {
        let registries = registries_for_request(&request);
        let mut affected_facts = request.authoritative_changed_facts().clone();
        let mut traversed_edges = Vec::new();
        for registry in &registries {
            let starting_facts = affected_facts.clone();
            let reachable = registry.facts_reachable_from(&starting_facts);
            for edge in registry.edges() {
                if reachable.contains(edge.source()) && reachable.contains(edge.target()) {
                    traversed_edges.push(edge.clone());
                }
            }
            affected_facts.extend(reachable.facts().cloned());
        }
        let authoritative_changed_facts = request.into_authoritative_changed_facts();
        let derived_fact_count = affected_facts
            .facts()
            .filter(|fact| !authoritative_changed_facts.contains(fact))
            .count();
        let counters = WorthUiGraphInvalidationCounters::new(
            authoritative_changed_facts.len(),
            derived_fact_count,
            traversed_edges.len(),
            registries.len(),
        );
        let receipt_digest = affected_facts.digest();
        Self {
            authoritative_changed_facts,
            affected_facts,
            traversed_edges,
            counters,
            receipt_digest,
        }
    }

    pub fn authoritative_changed_facts(&self) -> &WorthUiRuntimeFactSet {
        &self.authoritative_changed_facts
    }

    pub fn affected_facts(&self) -> &WorthUiRuntimeFactSet {
        &self.affected_facts
    }

    pub fn traversed_edges(&self) -> &[WorthUiGraphDependencyEdge] {
        &self.traversed_edges
    }

    pub fn counters(&self) -> WorthUiGraphInvalidationCounters {
        self.counters
    }

    pub fn receipt_digest(&self) -> WorthUiRuntimeFactSetDigest {
        self.receipt_digest
    }
}
