use std::collections::BTreeMap;

use crate::runtime::{WorthUiChangedRuntimeFacts, WorthUiRuntimeFactId, WorthUiRuntimeFactSet};

use super::receipt::WorthUiLiveViewConditionalProjectionReceipt;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiLiveViewConditionalProjectionRebindCounters {
    prior_conditional_count: usize,
    next_conditional_count: usize,
    changed_conditional_count: usize,
    source_reparse_count: usize,
    renderer_parse_count: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiLiveViewConditionalProjectionRebindReceipt {
    changed_facts: WorthUiChangedRuntimeFacts,
    counters: WorthUiLiveViewConditionalProjectionRebindCounters,
}

impl WorthUiLiveViewConditionalProjectionRebindReceipt {
    pub fn from_conditional_projection_receipts(
        prior: &[WorthUiLiveViewConditionalProjectionReceipt],
        next: &[WorthUiLiveViewConditionalProjectionReceipt],
    ) -> Self {
        let changed_facts = changed_conditional_projection_facts(prior, next);
        let counters = WorthUiLiveViewConditionalProjectionRebindCounters {
            prior_conditional_count: prior.len(),
            next_conditional_count: next.len(),
            changed_conditional_count: changed_facts.len(),
            source_reparse_count: 0,
            renderer_parse_count: 0,
        };
        Self {
            changed_facts,
            counters,
        }
    }

    pub fn changed_facts(&self) -> &WorthUiChangedRuntimeFacts {
        &self.changed_facts
    }

    pub fn counters(&self) -> WorthUiLiveViewConditionalProjectionRebindCounters {
        self.counters
    }
}

impl WorthUiLiveViewConditionalProjectionRebindCounters {
    pub fn prior_conditional_count(self) -> usize {
        self.prior_conditional_count
    }

    pub fn next_conditional_count(self) -> usize {
        self.next_conditional_count
    }

    pub fn changed_conditional_count(self) -> usize {
        self.changed_conditional_count
    }

    pub fn source_reparse_count(self) -> usize {
        self.source_reparse_count
    }

    pub fn renderer_parse_count(self) -> usize {
        self.renderer_parse_count
    }
}

fn changed_conditional_projection_facts(
    prior: &[WorthUiLiveViewConditionalProjectionReceipt],
    next: &[WorthUiLiveViewConditionalProjectionReceipt],
) -> WorthUiChangedRuntimeFacts {
    let prior_digests = prior
        .iter()
        .map(|conditional| {
            (
                conditional.control().control_id(),
                conditional.conditional_projection_digest(),
            )
        })
        .collect::<BTreeMap<_, _>>();
    let mut changed = WorthUiRuntimeFactSet::empty();
    for conditional in next {
        if prior_digests.get(conditional.control().control_id())
            != Some(&conditional.conditional_projection_digest())
        {
            changed.insert(conditional_projection_fact(conditional));
            changed.insert(participation_fact(conditional));
        }
    }
    for conditional in prior {
        if !next
            .iter()
            .any(|candidate| candidate.control().control_id() == conditional.control().control_id())
        {
            changed.insert(conditional_projection_fact(conditional));
            changed.insert(participation_fact(conditional));
        }
    }
    WorthUiChangedRuntimeFacts::from_runtime(changed)
}

fn conditional_projection_fact(
    conditional: &WorthUiLiveViewConditionalProjectionReceipt,
) -> WorthUiRuntimeFactId {
    WorthUiRuntimeFactId::live_view_conditional_projection(format!(
        "{}:{}",
        conditional.live_view_id(),
        conditional.control().control_id()
    ))
}

fn participation_fact(
    conditional: &WorthUiLiveViewConditionalProjectionReceipt,
) -> WorthUiRuntimeFactId {
    WorthUiRuntimeFactId::live_view_participation(format!(
        "{}:{}",
        conditional.live_view_id(),
        conditional.control().control_id()
    ))
}
