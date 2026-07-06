use crate::compaction::types::{BlobCompactionBasis, BlobCompactionIntent, BlobCompactionRewritePlan};
use crate::BlobCompactionCounterSnapshot;

pub(crate) fn base_counters(intent: &BlobCompactionIntent) -> BlobCompactionCounterSnapshot {
    let chunks = intent.reachability().map_or(0, |reachability| {
        reachability.reachable_chunks().len() as u64
    });
    let references = intent.reachability().map_or(0, |reachability| {
        reachability.reference_edges().len() as u64
    });
    BlobCompactionCounterSnapshot::start(
        chunks,
        references,
        intent
            .physical()
            .admitted()
            .map_or(0, |physical| physical.counters().copied_pages()),
    )
}

pub(crate) fn construct_rewrite_plan(
    intent: BlobCompactionIntent,
    counters: BlobCompactionCounterSnapshot,
) -> BlobCompactionRewritePlan {
    let physical = intent
        .physical()
        .admitted()
        .expect("admitted intent carries physical interlock plan");
    let reachability = intent
        .reachability()
        .expect("admitted intent carries reachability proof");
    let physical_counters = physical.counters();
    BlobCompactionRewritePlan::new(
        BlobCompactionBasis::from_lifecycle(intent.lifecycle()),
        physical.clone(),
        reachability.clone(),
        intent.placement().clone(),
        intent.uncompacted_publication().canonical_basis().clone(),
        intent
            .dedupe_references()
            .iter()
            .map(|reference| reference.reference_identity().clone())
            .collect(),
        counters
            .with_physical(physical_counters)
            .preserve_dedupe_edges(intent.dedupe_references().len() as u64)
            .record_foreground_yields(intent.pacing().foreground_yields()),
    )
}