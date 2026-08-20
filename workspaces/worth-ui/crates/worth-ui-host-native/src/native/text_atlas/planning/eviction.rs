//! Deterministic candidate eviction during effect-free atlas planning.

use std::collections::HashSet;

use worth_ui_host_contract::UiGlyphRasterKey;

use super::{bounded_u32, Candidate};
use crate::native::text_atlas::{
    candidate_store::CandidateAtlasStore,
    ownership::{AtlasCore, AtlasStore},
    placement::UiAtlasRect,
    recovery::UiNativeTextAtlasDenial,
    UiNativeTextAtlasDemand,
};

pub(super) fn allocate_candidate_with_eviction(
    candidate: &mut CandidateAtlasStore,
    predecessor: &AtlasStore,
    demand: UiNativeTextAtlasDemand,
    protected: &HashSet<UiGlyphRasterKey>,
    evictions: &mut Vec<UiGlyphRasterKey>,
    page_probes: &mut u32,
    placement_probes: &mut u32,
    eviction_candidates: &mut u32,
) -> Result<(u32, UiAtlasRect), UiNativeTextAtlasDenial> {
    loop {
        let allocation = candidate.allocate_observed(predecessor, demand.width(), demand.height());
        *page_probes = page_probes.saturating_add(bounded_u32(allocation.page_probes));
        *placement_probes =
            placement_probes.saturating_add(bounded_u32(allocation.placement_probes));
        if let Some(placement) = allocation.placement {
            return Ok(placement);
        }
        let (oldest, candidates) = candidate.oldest_unprotected_observed(predecessor, protected);
        *eviction_candidates = eviction_candidates.saturating_add(bounded_u32(candidates));
        let Some((_, _, key)) = oldest else {
            return Err(
                if predecessor
                    .entries
                    .keys()
                    .any(|key| protected.contains(key))
                {
                    UiNativeTextAtlasDenial::PinnedCapacityExceeded
                } else {
                    UiNativeTextAtlasDenial::PageCapacityExceeded
                },
            );
        };
        candidate.remove_existing(predecessor, key);
        evictions.push(key);
    }
}

pub(super) fn evict_candidate_one(
    candidate: &mut Candidate,
    core: &AtlasCore,
    protected: &HashSet<UiGlyphRasterKey>,
) -> Option<UiGlyphRasterKey> {
    let (alpha, alpha_candidates) = candidate
        .alpha
        .oldest_unprotected_observed(&core.alpha, protected);
    let (color, color_candidates) = candidate
        .color
        .oldest_unprotected_observed(&core.color, protected);
    candidate.eviction_candidates = candidate.eviction_candidates.saturating_add(bounded_u32(
        alpha_candidates.saturating_add(color_candidates),
    ));
    let key = match (alpha, color) {
        (Some(left), Some(right)) if (left.0, &left.1) <= (right.0, &right.1) => left.2,
        (Some(_), Some(right)) => right.2,
        (Some(left), None) => left.2,
        (None, Some(right)) => right.2,
        (None, None) => return None,
    };
    if candidate.alpha.remove_existing(&core.alpha, key).is_none() {
        candidate.color.remove_existing(&core.color, key);
    }
    Some(key)
}
