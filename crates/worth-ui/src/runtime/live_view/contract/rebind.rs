use std::collections::BTreeMap;

use crate::runtime::{WorthUiChangedRuntimeFacts, WorthUiRuntimeFactId, WorthUiRuntimeFactSet};

use super::WorthUiLiveViewDeclarationReceipt;
use crate::runtime::live_view::digest::digest_parts;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiLiveViewDeclarationRebindCounters {
    prior_binding_count: usize,
    next_binding_count: usize,
    changed_binding_count: usize,
    source_reparse_count: usize,
    renderer_parse_count: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiLiveViewDeclarationRebindReceipt {
    live_view_id: String,
    prior_declaration_digest: u64,
    next_declaration_digest: u64,
    changed_facts: WorthUiChangedRuntimeFacts,
    counters: WorthUiLiveViewDeclarationRebindCounters,
    receipt_digest: u64,
}

impl WorthUiLiveViewDeclarationRebindReceipt {
    pub(crate) fn from_admitted_declarations(
        prior: &WorthUiLiveViewDeclarationReceipt,
        next: &WorthUiLiveViewDeclarationReceipt,
    ) -> Self {
        let changed_facts = changed_live_view_binding_facts(prior, next);
        let counters = WorthUiLiveViewDeclarationRebindCounters {
            prior_binding_count: prior.bindings().len(),
            next_binding_count: next.bindings().len(),
            changed_binding_count: changed_facts.len(),
            source_reparse_count: 0,
            renderer_parse_count: 0,
        };
        let receipt_digest = digest_parts([
            prior.live_view_id().to_owned(),
            prior.declaration_digest().to_string(),
            next.declaration_digest().to_string(),
            changed_facts.digest().value().to_string(),
        ]);
        Self {
            live_view_id: next.live_view_id().to_owned(),
            prior_declaration_digest: prior.declaration_digest(),
            next_declaration_digest: next.declaration_digest(),
            changed_facts,
            counters,
            receipt_digest,
        }
    }

    pub fn live_view_id(&self) -> &str {
        &self.live_view_id
    }

    pub fn prior_declaration_digest(&self) -> u64 {
        self.prior_declaration_digest
    }

    pub fn next_declaration_digest(&self) -> u64 {
        self.next_declaration_digest
    }

    pub fn changed_facts(&self) -> &WorthUiChangedRuntimeFacts {
        &self.changed_facts
    }

    pub fn counters(&self) -> WorthUiLiveViewDeclarationRebindCounters {
        self.counters
    }

    pub fn receipt_digest(&self) -> u64 {
        self.receipt_digest
    }
}

impl WorthUiLiveViewDeclarationRebindCounters {
    pub fn prior_binding_count(self) -> usize {
        self.prior_binding_count
    }

    pub fn next_binding_count(self) -> usize {
        self.next_binding_count
    }

    pub fn changed_binding_count(self) -> usize {
        self.changed_binding_count
    }

    pub fn source_reparse_count(self) -> usize {
        self.source_reparse_count
    }

    pub fn renderer_parse_count(self) -> usize {
        self.renderer_parse_count
    }
}

fn changed_live_view_binding_facts(
    prior: &WorthUiLiveViewDeclarationReceipt,
    next: &WorthUiLiveViewDeclarationReceipt,
) -> WorthUiChangedRuntimeFacts {
    let prior_bindings = prior_binding_digests(prior);
    let mut changed = WorthUiRuntimeFactSet::empty();
    for binding in next.bindings() {
        if prior_bindings.get(binding.binding_id()) != Some(&binding.binding_digest()) {
            changed.insert(live_view_binding_fact(
                next.live_view_id(),
                binding.binding_id(),
            ));
        }
    }
    for binding in prior.bindings() {
        if next.binding(binding.binding_id()).is_none() {
            changed.insert(live_view_binding_fact(
                prior.live_view_id(),
                binding.binding_id(),
            ));
        }
    }
    WorthUiChangedRuntimeFacts::from_runtime(changed)
}

fn prior_binding_digests(receipt: &WorthUiLiveViewDeclarationReceipt) -> BTreeMap<&str, u64> {
    receipt
        .bindings()
        .iter()
        .map(|binding| (binding.binding_id(), binding.binding_digest()))
        .collect()
}

fn live_view_binding_fact(live_view_id: &str, binding_id: &str) -> WorthUiRuntimeFactId {
    WorthUiRuntimeFactId::live_view_state_binding(format!("{live_view_id}:{binding_id}"))
}
