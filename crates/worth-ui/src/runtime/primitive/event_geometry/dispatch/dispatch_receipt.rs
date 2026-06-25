use super::super::digest::event_dispatch_digest;
use super::super::receipt::WorthUiPrimitiveEventContainment;
use super::candidate_receipt::WorthUiPrimitiveEventDispatchCandidateReceipt;
use super::outcome_receipt::WorthUiPrimitiveEventDispatchOutcome;
use crate::runtime::WorthUiQueryGraphExecutionReceipt;

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiPrimitiveEventDispatchReceipt {
    outcome: WorthUiPrimitiveEventDispatchOutcome,
    candidates: Vec<WorthUiPrimitiveEventDispatchCandidateReceipt>,
    counters: WorthUiPrimitiveEventDispatchCounters,
    query_graph_execution: WorthUiQueryGraphExecutionReceipt,
    dispatch_digest: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiPrimitiveEventDispatchCounters {
    region_count: usize,
    hit_candidate_count: usize,
    cursor_candidate_count: usize,
    parent_chain_count: usize,
    emitted_surface_count: usize,
}

impl WorthUiPrimitiveEventDispatchCounters {
    pub(super) fn new(
        region_count: usize,
        hit_candidate_count: usize,
        cursor_candidate_count: usize,
        parent_chain_count: usize,
        emitted_surface_count: usize,
    ) -> Self {
        Self {
            region_count,
            hit_candidate_count,
            cursor_candidate_count,
            parent_chain_count,
            emitted_surface_count,
        }
    }

    pub fn region_count(self) -> usize {
        self.region_count
    }

    pub fn hit_candidate_count(self) -> usize {
        self.hit_candidate_count
    }

    pub fn cursor_candidate_count(self) -> usize {
        self.cursor_candidate_count
    }

    pub fn parent_chain_count(self) -> usize {
        self.parent_chain_count
    }

    pub fn emitted_surface_count(self) -> usize {
        self.emitted_surface_count
    }
}

impl WorthUiPrimitiveEventDispatchReceipt {
    pub(super) fn new(
        outcome: WorthUiPrimitiveEventDispatchOutcome,
        candidates: Vec<WorthUiPrimitiveEventDispatchCandidateReceipt>,
        counters: WorthUiPrimitiveEventDispatchCounters,
        query_graph_execution: WorthUiQueryGraphExecutionReceipt,
    ) -> Self {
        let dispatch_digest = event_dispatch_digest(
            outcome.primary_surface_id(),
            outcome.emitted_surface_ids(),
            outcome.cursor(),
            query_graph_execution.execution_digest(),
        );
        Self {
            outcome,
            candidates,
            counters,
            query_graph_execution,
            dispatch_digest,
        }
    }

    pub fn primary_surface_id(&self) -> Option<&str> {
        self.outcome.primary_surface_id()
    }

    pub fn emitted_surface_ids(&self) -> &[String] {
        self.outcome.emitted_surface_ids()
    }

    pub fn cursor(&self) -> super::super::super::WorthUiPrimitiveResolvedCursorPosture {
        self.outcome.cursor()
    }

    pub fn containment(&self) -> Option<WorthUiPrimitiveEventContainment> {
        self.outcome.containment()
    }

    pub fn outcome(&self) -> &WorthUiPrimitiveEventDispatchOutcome {
        &self.outcome
    }

    pub fn candidates(&self) -> &[WorthUiPrimitiveEventDispatchCandidateReceipt] {
        &self.candidates
    }

    pub fn counters(&self) -> WorthUiPrimitiveEventDispatchCounters {
        self.counters
    }

    pub fn query_graph_execution(&self) -> &WorthUiQueryGraphExecutionReceipt {
        &self.query_graph_execution
    }

    pub fn dispatch_digest(&self) -> u64 {
        self.dispatch_digest
    }
}
