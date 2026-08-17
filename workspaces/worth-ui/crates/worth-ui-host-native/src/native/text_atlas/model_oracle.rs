use std::collections::BTreeSet;

pub(super) use super::model_key::{ModelDemand, ModelKey, ModelPin, ModelSource};
use super::model_placement::ModelStore;
pub(super) use super::model_records::{
    ModelDenial, ModelPlacement, ModelReceipt, ModelRecovery, ModelSnapshot,
};
pub(super) const MAX_ENTRIES: usize = 8_192;
pub(super) const MAX_EXTENT: u32 = 512;
pub(super) const MAX_STAGING_BYTES: u64 = 8 * 1_024 * 1_024;
pub(super) const MAX_TEXEL_BYTES: u64 = 36 * 1_024 * 1_024;
#[derive(Clone, Debug)]
pub(super) struct IndependentAtlasModel {
    alpha: ModelStore,
    color: ModelStore,
    pins: BTreeSet<ModelPin>,
    generation: u64,
    completed_use_epoch: u64,
    staging_bytes: u64,
    quarantined: bool,
    lineage: u64,
}

impl IndependentAtlasModel {
    pub(super) fn new(lineage: u64) -> Self {
        Self {
            alpha: ModelStore::new(ModelSource::Alpha),
            color: ModelStore::new(ModelSource::Color),
            pins: BTreeSet::new(),
            generation: 1,
            completed_use_epoch: 0,
            staging_bytes: 0,
            quarantined: false,
            lineage,
        }
    }

    pub(super) fn snapshot(&self) -> ModelSnapshot {
        ModelSnapshot {
            generation: self.generation,
            alpha_pages: self.alpha.page_count(),
            color_pages: self.color.page_count(),
            alpha_entries: self.alpha.entries.len(),
            color_entries: self.color.entries.len(),
            pins: self.pins.len(),
            staging_bytes: self.staging_bytes,
            quarantined: self.quarantined,
        }
    }

    pub(super) fn admit(
        &mut self,
        demands: &[ModelDemand],
        additions: &[ModelPin],
        releases: &[ModelPin],
    ) -> Result<ModelReceipt, ModelDenial> {
        if self.quarantined {
            return Err(ModelDenial::Reconstruction);
        }
        let mut candidate = self.clone();
        apply_pin_transition(&mut candidate, additions, releases);
        apply_model_pin_counts(&mut candidate);
        let (staged_bytes, physical_staged_bytes) = staging_posture(demands)?;
        let predecessor_entries = candidate.alpha.entries.len() + candidate.color.entries.len();
        candidate.completed_use_epoch = candidate.completed_use_epoch.saturating_add(1);
        let work = apply_demands(&mut candidate, demands)?;
        apply_model_pin_counts(&mut candidate);
        validate_added_pins(&candidate, additions)?;
        let peak_texel_bytes = candidate.alpha.texel_bytes() + candidate.color.texel_bytes();
        if peak_texel_bytes > MAX_TEXEL_BYTES {
            return Err(ModelDenial::Texels);
        }
        candidate.generation = candidate.generation.saturating_add(1);
        candidate.staging_bytes = staged_bytes;
        let peak_entries =
            predecessor_entries.max(candidate.alpha.entries.len() + candidate.color.entries.len());
        *self = candidate;
        Ok(ModelReceipt {
            generation: self.generation,
            misses: work.misses,
            hits: work.hits,
            evictions: work.evicted_keys.len(),
            peak_entries,
            peak_texel_bytes,
            staged_bytes,
            physical_staged_bytes,
            evicted_keys: work.evicted_keys.into_boxed_slice(),
            placements: work.placements.into_boxed_slice(),
        })
    }

    pub(super) fn force_equal_epoch_for_test(&mut self, epoch: u64) {
        self.completed_use_epoch = epoch;
        for entry in self
            .alpha
            .entries
            .values_mut()
            .chain(self.color.entries.values_mut())
        {
            entry.last_use = epoch;
        }
    }

    pub(super) fn evict_one_for_test(&mut self) -> Option<ModelKey> {
        evict_model_entry(self)
    }

    pub(super) fn indeterminate(&mut self) -> ModelRecovery {
        self.quarantined = true;
        self.staging_bytes = 0;
        ModelRecovery {
            lineage: self.lineage,
            generation: self.generation.saturating_add(1),
        }
    }

    pub(super) fn recover(&mut self, recovery: ModelRecovery) -> Result<(), ModelDenial> {
        if !self.quarantined
            || recovery.lineage != self.lineage
            || recovery.generation != self.generation.saturating_add(1)
        {
            return Err(ModelDenial::RecoveryOwner);
        }
        let lineage = self.lineage;
        let completed_use_epoch = self.completed_use_epoch;
        *self = Self::new(lineage);
        self.generation = recovery.generation;
        self.completed_use_epoch = completed_use_epoch;
        Ok(())
    }
}

struct ModelWork {
    misses: usize,
    hits: usize,
    evicted_keys: Vec<ModelKey>,
    placements: Vec<ModelPlacement>,
}

fn apply_pin_transition(
    candidate: &mut IndependentAtlasModel,
    additions: &[ModelPin],
    releases: &[ModelPin],
) {
    for pin in releases {
        candidate.pins.remove(pin);
    }
    candidate.pins.extend(additions.iter().copied());
}

fn staging_posture(demands: &[ModelDemand]) -> Result<(u64, u64), ModelDenial> {
    let sum = |physical: bool| {
        demands.iter().try_fold(0_u64, |total, demand| {
            total.checked_add(if physical {
                demand.physical_bytes
            } else {
                demand.logical_bytes
            })
        })
    };
    let staged = sum(false).ok_or(ModelDenial::Staging)?;
    let physical = sum(true).ok_or(ModelDenial::Staging)?;
    if staged > MAX_STAGING_BYTES || physical > MAX_STAGING_BYTES {
        return Err(ModelDenial::Staging);
    }
    Ok((staged, physical))
}

fn apply_demands(
    candidate: &mut IndependentAtlasModel,
    demands: &[ModelDemand],
) -> Result<ModelWork, ModelDenial> {
    let mut work = ModelWork {
        misses: 0,
        hits: 0,
        evicted_keys: Vec::new(),
        placements: Vec::new(),
    };
    for demand in demands {
        apply_demand(candidate, *demand, &mut work)?;
    }
    Ok(work)
}

fn apply_demand(
    candidate: &mut IndependentAtlasModel,
    demand: ModelDemand,
    work: &mut ModelWork,
) -> Result<(), ModelDenial> {
    let existing = match demand.key.source() {
        ModelSource::Alpha => candidate.alpha.entries.get_mut(&demand.key),
        ModelSource::Color => candidate.color.entries.get_mut(&demand.key),
    };
    if let Some(entry) = existing {
        entry.last_use = candidate.completed_use_epoch;
        work.hits += 1;
        return Ok(());
    }
    if demand.width == 0
        || demand.height == 0
        || demand.width > MAX_EXTENT
        || demand.height > MAX_EXTENT
    {
        return Err(ModelDenial::Extent);
    }
    if candidate.alpha.entries.len() + candidate.color.entries.len() >= MAX_ENTRIES {
        work.evicted_keys
            .push(evict_model_entry(candidate).ok_or(ModelDenial::Pinned)?);
    }
    let store = match demand.key.source() {
        ModelSource::Alpha => &mut candidate.alpha,
        ModelSource::Color => &mut candidate.color,
    };
    let entry = allocate_model_entry(
        store,
        demand,
        &mut work.evicted_keys,
        candidate.completed_use_epoch,
    )?;
    work.placements.push(ModelPlacement {
        key: demand.key,
        page: entry.page,
        origin: [entry.rect.x, entry.rect.y],
    });
    store.entries.insert(demand.key, entry);
    work.misses += 1;
    Ok(())
}

fn allocate_model_entry(
    store: &mut ModelStore,
    demand: ModelDemand,
    evicted: &mut Vec<ModelKey>,
    epoch: u64,
) -> Result<super::model_placement::ModelEntry, ModelDenial> {
    if let Some(entry) = store.allocate(demand, epoch) {
        return Ok(entry);
    }
    let oldest = store.oldest_unpinned().ok_or(ModelDenial::Pinned)?;
    store.remove(oldest);
    evicted.push(oldest);
    store.allocate(demand, epoch).ok_or(ModelDenial::Pages)
}

fn validate_added_pins(
    candidate: &IndependentAtlasModel,
    additions: &[ModelPin],
) -> Result<(), ModelDenial> {
    additions
        .iter()
        .all(|pin| {
            let key = pin.key();
            candidate.alpha.entries.contains_key(&key) || candidate.color.entries.contains_key(&key)
        })
        .then_some(())
        .ok_or(ModelDenial::Pinned)
}

fn evict_model_entry(candidate: &mut IndependentAtlasModel) -> Option<ModelKey> {
    let alpha = candidate
        .alpha
        .oldest_unpinned()
        .and_then(|key| candidate.alpha.eviction_order(key));
    let color = candidate
        .color
        .oldest_unpinned()
        .and_then(|key| candidate.color.eviction_order(key));
    let key = match (alpha, color) {
        (Some(left), Some(right)) => left.min(right).2,
        (Some(left), None) => left.2,
        (None, Some(right)) => right.2,
        (None, None) => return None,
    };
    if candidate.alpha.remove(key).is_none() {
        candidate.color.remove(key);
    }
    Some(key)
}

fn apply_model_pin_counts(candidate: &mut IndependentAtlasModel) {
    for (key, entry) in candidate
        .alpha
        .entries
        .iter_mut()
        .chain(candidate.color.entries.iter_mut())
    {
        entry.pin_count = u32::try_from(
            candidate
                .pins
                .iter()
                .filter(|pin| pin.key() == *key)
                .count(),
        )
        .unwrap_or(u32::MAX);
    }
}
