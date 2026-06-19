use std::collections::BTreeSet;

use crate::runtime::{ForgeQueryGraphObligationRegistration, ForgeQueryGraphTouchDescriptor};

use super::super::construction::GraphObligationBuckets;
use super::super::lookup::touch_lookup_keys_for_descriptor;
use super::operating_world_descriptor::ForgeQueryGraphObligationOperatingWorldDescriptor;
use super::selection::ForgeQueryGraphObligationSelection;
use super::selection_counters::{
    ForgeQueryGraphObligationSelectionCounterInput, ForgeQueryGraphObligationSelectionCounters,
};

pub(in crate::runtime::mutation::graph_composition::obligation::index) fn select_graph_obligations_from_buckets(
    index_digest: &str,
    buckets: &GraphObligationBuckets,
    touch_descriptor: &ForgeQueryGraphTouchDescriptor,
    operating_world: &ForgeQueryGraphObligationOperatingWorldDescriptor,
) -> ForgeQueryGraphObligationSelection {
    let touch_keys = touch_lookup_keys_for_descriptor(touch_descriptor);
    let operating_world_keys = operating_world.lookup_keys();
    let mut attempted_bucket_lookup_count = 0;
    let mut matched_bucket_count = 0;
    let mut candidates = Vec::new();
    for touch_key in &touch_keys {
        for operating_world_key in &operating_world_keys {
            attempted_bucket_lookup_count += 1;
            if let Some(bucket) = buckets.get(&(touch_key.clone(), *operating_world_key)) {
                matched_bucket_count += 1;
                candidates.extend(bucket.iter().cloned());
            }
        }
    }
    let candidate_registration_count = candidates.len();
    let matched_registrations = deduplicate_selection_candidates(candidates);
    let counters = ForgeQueryGraphObligationSelectionCounters::new(
        ForgeQueryGraphObligationSelectionCounterInput {
            touch_lookup_key_count: touch_keys.len(),
            operating_world_lookup_key_count: operating_world_keys.len(),
            attempted_bucket_lookup_count,
            matched_bucket_count,
            candidate_registration_count,
            deduplicated_candidate_count: matched_registrations.len(),
            matched_obligation_count: matched_registrations.len(),
            registration_full_scan_count: 0,
        },
    );
    ForgeQueryGraphObligationSelection::new(
        index_digest,
        touch_descriptor,
        operating_world,
        matched_registrations,
        counters,
    )
}

fn deduplicate_selection_candidates(
    mut candidates: Vec<ForgeQueryGraphObligationRegistration>,
) -> Vec<ForgeQueryGraphObligationRegistration> {
    candidates.sort_by(|left, right| left.registration_digest().cmp(right.registration_digest()));
    let mut seen = BTreeSet::new();
    candidates
        .into_iter()
        .filter(|registration| seen.insert(registration.registration_digest().to_string()))
        .collect()
}
