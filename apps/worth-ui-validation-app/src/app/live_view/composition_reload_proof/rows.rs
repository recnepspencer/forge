use std::collections::BTreeMap;

use worth_ui::facade::{WorthUiMountedProductViewReceipt, WorthUiRuntimeFactId};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationLiveViewCompositionRebindRow {
    changed_fact: WorthUiRuntimeFactId,
    consumer_fact: WorthUiRuntimeFactId,
    semantic_slice: &'static str,
    decision: ValidationLiveViewCompositionRebindDecision,
    prior_row_digest: Option<u64>,
    next_row_digest: Option<u64>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ValidationLiveViewCompositionRebindDecision {
    Preserve,
    Rebind,
}

pub(super) fn composition_rebind_rows(
    prior: &WorthUiMountedProductViewReceipt,
    next: &WorthUiMountedProductViewReceipt,
) -> Vec<ValidationLiveViewCompositionRebindRow> {
    let prior_rows = child_rows_by_edge_fact(prior);
    let next_rows = child_rows_by_edge_fact(next);
    let consumer_fact = WorthUiRuntimeFactId::composition_topology(next.live_view_id());
    let mut edge_facts = prior_rows.keys().cloned().collect::<Vec<_>>();
    edge_facts.extend(
        next_rows
            .keys()
            .filter(|fact| !prior_rows.contains_key(*fact))
            .cloned(),
    );
    edge_facts
        .into_iter()
        .map(|changed_fact| {
            composition_rebind_row_for_edge_fact(
                changed_fact,
                &consumer_fact,
                &prior_rows,
                &next_rows,
            )
        })
        .collect()
}

impl ValidationLiveViewCompositionRebindRow {
    pub fn changed_fact(&self) -> &WorthUiRuntimeFactId {
        &self.changed_fact
    }

    pub fn consumer_fact(&self) -> &WorthUiRuntimeFactId {
        &self.consumer_fact
    }

    pub fn semantic_slice(&self) -> &'static str {
        self.semantic_slice
    }

    pub fn decision(&self) -> ValidationLiveViewCompositionRebindDecision {
        self.decision
    }

    pub fn prior_row_digest(&self) -> Option<u64> {
        self.prior_row_digest
    }

    pub fn next_row_digest(&self) -> Option<u64> {
        self.next_row_digest
    }
}

fn composition_rebind_row_for_edge_fact(
    changed_fact: WorthUiRuntimeFactId,
    consumer_fact: &WorthUiRuntimeFactId,
    prior_rows: &BTreeMap<WorthUiRuntimeFactId, u64>,
    next_rows: &BTreeMap<WorthUiRuntimeFactId, u64>,
) -> ValidationLiveViewCompositionRebindRow {
    let prior_row_digest = prior_rows.get(&changed_fact).copied();
    let next_row_digest = next_rows.get(&changed_fact).copied();
    let decision = if prior_row_digest == next_row_digest {
        ValidationLiveViewCompositionRebindDecision::Preserve
    } else {
        ValidationLiveViewCompositionRebindDecision::Rebind
    };
    ValidationLiveViewCompositionRebindRow {
        changed_fact,
        consumer_fact: consumer_fact.clone(),
        semantic_slice: "MountedCompositionTree",
        decision,
        prior_row_digest,
        next_row_digest,
    }
}

fn child_rows_by_edge_fact(
    product_view: &WorthUiMountedProductViewReceipt,
) -> BTreeMap<WorthUiRuntimeFactId, u64> {
    product_view
        .composition_tree()
        .graph_access()
        .child_rows()
        .iter()
        .map(|row| (row.edge().fact_id().clone(), row.row_digest()))
        .collect()
}
