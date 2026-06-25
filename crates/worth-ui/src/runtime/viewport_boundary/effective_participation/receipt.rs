use crate::runtime::WorthUiRuntimeFactId;

use super::super::digest::digest_parts;
use super::counters::WorthUiEffectiveViewportParticipationCounters;
use super::row::WorthUiEffectiveViewportParticipationRow;

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiEffectiveViewportParticipationReceipt {
    rows: Vec<WorthUiEffectiveViewportParticipationRow>,
    consumed_facts: Vec<WorthUiRuntimeFactId>,
    counters: WorthUiEffectiveViewportParticipationCounters,
    receipt_digest: u64,
}

impl WorthUiEffectiveViewportParticipationReceipt {
    pub(super) fn new(
        rows: Vec<WorthUiEffectiveViewportParticipationRow>,
        mut consumed_facts: Vec<WorthUiRuntimeFactId>,
    ) -> Self {
        let counters = WorthUiEffectiveViewportParticipationCounters::new(
            rows.len(),
            rows.iter().filter(|row| !row.visible()).count(),
            rows.iter()
                .map(|row| row.governing_boundary_count())
                .sum::<usize>(),
        );
        consumed_facts.sort();
        consumed_facts.dedup();
        let receipt_digest = digest_parts(
            ["effective_viewport_participation".to_owned()]
                .into_iter()
                .chain(rows.iter().map(|row| row.receipt_digest().to_string()))
                .chain(consumed_facts.iter().map(|fact| fact.identity().to_owned())),
        );
        Self {
            rows,
            consumed_facts,
            counters,
            receipt_digest,
        }
    }

    pub fn rows(&self) -> &[WorthUiEffectiveViewportParticipationRow] {
        &self.rows
    }

    pub fn row_for_node(&self, node_id: &str) -> Option<&WorthUiEffectiveViewportParticipationRow> {
        self.rows.iter().find(|row| row.node_id() == node_id)
    }

    pub fn consumed_facts(&self) -> &[WorthUiRuntimeFactId] {
        &self.consumed_facts
    }

    pub fn counters(&self) -> WorthUiEffectiveViewportParticipationCounters {
        self.counters
    }

    pub fn receipt_digest(&self) -> u64 {
        self.receipt_digest
    }
}
