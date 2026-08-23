//! Deterministic unpinned atlas eviction policy.

use std::collections::{HashMap, HashSet};

use worth_ui_host_contract::UiGlyphRasterKey;

use super::entry::UiAtlasEntry;
use super::key::canonical_raster_key_bytes;
use super::ownership::AtlasStore;
use super::UiAtlasEntryIdentity;

pub(crate) fn evict_one(
    alpha: &mut AtlasStore,
    color: &mut AtlasStore,
    protected: &HashSet<UiGlyphRasterKey>,
) -> Option<UiGlyphRasterKey> {
    let alpha_candidate = first_candidate(alpha, protected);
    let color_candidate = first_candidate(color, protected);
    let key = match (alpha_candidate, color_candidate) {
        (Some(left), Some(right)) => {
            if (left.0, left.1) <= (right.0, right.1) {
                left.2
            } else {
                right.2
            }
        }
        (Some(left), None) => left.2,
        (None, Some(right)) => right.2,
        (None, None) => return None,
    };
    if alpha.remove(key).is_none() {
        color.remove(key);
    }
    Some(key)
}

fn first_candidate(
    store: &AtlasStore,
    protected: &HashSet<UiGlyphRasterKey>,
) -> Option<(u64, Vec<u8>, UiGlyphRasterKey)> {
    ordered_candidates(&store.entries, protected)
        .into_iter()
        .next()
        .and_then(|identity| {
            store
                .entries
                .values()
                .find(|entry| entry.identity == identity)
        })
        .map(|entry| {
            (
                entry.completed_use_epoch,
                canonical_raster_key_bytes(entry.key),
                entry.key,
            )
        })
}

pub(crate) fn ordered_candidates(
    entries: &HashMap<UiGlyphRasterKey, UiAtlasEntry>,
    protected: &HashSet<UiGlyphRasterKey>,
) -> Vec<UiAtlasEntryIdentity> {
    let mut candidates = entries
        .values()
        .filter(|entry| !entry.pinned() && !protected.contains(&entry.key))
        .collect::<Vec<_>>();
    candidates.sort_by_key(|entry| {
        (
            entry.completed_use_epoch,
            canonical_raster_key_bytes(entry.key),
        )
    });
    candidates.into_iter().map(|entry| entry.identity).collect()
}
