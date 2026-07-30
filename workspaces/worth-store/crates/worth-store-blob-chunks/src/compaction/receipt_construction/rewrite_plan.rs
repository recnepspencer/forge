use crate::compaction::types::{
    BlobCompactionBasis, BlobCompactionIntent, BlobCompactionRewritePlan,
    BlobCompactionRewritePlanParts,
};
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
    let parts = intent.into_parts();
    let physical = parts
        .physical
        .into_admitted()
        .expect("admitted intent carries physical interlock plan");
    let reachability = parts
        .reachability
        .expect("admitted intent carries reachability proof");
    let physical_counters = physical.counters();
    let basis = BlobCompactionBasis::from_lifecycle(&parts.lifecycle);
    let pacing_yields = parts.pacing.counters().yield_events();
    BlobCompactionRewritePlan::new(BlobCompactionRewritePlanParts {
        basis,
        pacing: parts.pacing,
        physical,
        reachability,
        placement: parts.placement,
        old_canonical_basis: parts.uncompacted_publication.canonical_basis().clone(),
        dedupe_reference_identities: parts
            .dedupe_references
            .iter()
            .map(|reference| reference.reference_identity().clone())
            .collect(),
        counters: counters
            .with_physical(physical_counters)
            .preserve_dedupe_edges(parts.dedupe_references.len() as u64)
            .record_foreground_yields(pacing_yields),
    })
}
