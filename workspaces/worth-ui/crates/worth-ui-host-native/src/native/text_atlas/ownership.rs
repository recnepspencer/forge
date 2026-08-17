//! Host-owned bounded alpha/color atlas state.

#[cfg(test)]
use std::cell::Cell;
use std::cell::RefCell;
use std::collections::{BTreeSet, HashMap};
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};

use sha2::{Digest, Sha256};

use worth_ui_host_contract::{UiGlyphRasterKey, UiQualifiedTextLayoutIdentity};

use super::entry::UiAtlasEntry;
use super::key::canonical_raster_key_bytes;
use super::placement::UiAtlasPage;
#[cfg(test)]
use super::placement::UiAtlasRect;
use super::recovery::{UiNativeTextAtlasGeneration, UiNativeTextAtlasLineageIdentity};
use super::{alpha::AlphaAtlasStore, color::ColorAtlasStore};

static NEXT_ATLAS_LINEAGE: AtomicU64 = AtomicU64::new(1);

fn next_atlas_lineage() -> UiNativeTextAtlasLineageIdentity {
    let value = NEXT_ATLAS_LINEAGE.fetch_add(1, Ordering::Relaxed);
    UiNativeTextAtlasLineageIdentity::from_native_host(value)
        .unwrap_or_else(|| panic!("atlas lineage identity exhausted"))
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct AtlasStore {
    pub(crate) page_width: u32,
    pub(crate) page_height: u32,
    pub(crate) page_limit: u32,
    pub(crate) channels: u32,
    pub(crate) pages: Vec<UiAtlasPage>,
    pub(crate) entries: HashMap<UiGlyphRasterKey, UiAtlasEntry>,
}

impl AtlasStore {
    pub(crate) fn new(page_width: u32, page_height: u32, page_limit: u32, channels: u32) -> Self {
        Self {
            page_width,
            page_height,
            page_limit,
            channels,
            pages: Vec::new(),
            entries: HashMap::new(),
        }
    }

    #[cfg(test)]
    pub(crate) fn allocate(&mut self, width: u32, height: u32) -> Option<(u32, UiAtlasRect)> {
        for (page, slot) in self.pages.iter_mut().enumerate() {
            if let Some(rect) = slot.allocate(width, height) {
                return Some((u32::try_from(page).ok()?, rect));
            }
        }
        if u32::try_from(self.pages.len()).ok()? >= self.page_limit {
            return None;
        }
        self.pages
            .push(UiAtlasPage::new(self.page_width, self.page_height));
        let page = u32::try_from(self.pages.len() - 1).ok()?;
        self.pages
            .last_mut()?
            .allocate(width, height)
            .map(|rect| (page, rect))
    }

    #[cfg(test)]
    pub(crate) fn remove(&mut self, key: UiGlyphRasterKey) -> Option<UiAtlasEntry> {
        let entry = self.entries.remove(&key)?;
        let page = self.pages.get_mut(usize::try_from(entry.page).ok()?)?;
        page.release(entry.rect);
        Some(entry)
    }

    #[cfg(test)]
    pub(crate) fn insert(&mut self, entry: UiAtlasEntry) {
        self.entries.insert(entry.key, entry);
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) struct PinIdentity {
    layout: [u8; 32],
    key: [u8; 256],
    key_len: u16,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct UiNativeTextPinObservation {
    layout: [u8; 32],
    raster_key: [u8; 32],
}

impl PinIdentity {
    pub(crate) fn new(layout: UiQualifiedTextLayoutIdentity, key: UiGlyphRasterKey) -> Self {
        let bytes = canonical_raster_key_bytes(key);
        let mut key_buffer = [0; 256];
        let length = bytes.len().min(key_buffer.len());
        key_buffer[..length].copy_from_slice(&bytes[..length]);
        Self {
            layout: layout.digest(),
            key: key_buffer,
            key_len: u16::try_from(length).unwrap_or(u16::MAX),
        }
    }

    pub(crate) fn key_matches(self, key: UiGlyphRasterKey) -> bool {
        let bytes = canonical_raster_key_bytes(key);
        usize::from(self.key_len) == bytes.len() && self.key[..bytes.len()] == bytes
    }

    fn observation(self) -> UiNativeTextPinObservation {
        let length = usize::from(self.key_len);
        UiNativeTextPinObservation {
            layout: self.layout,
            raster_key: Sha256::digest(&self.key[..length]).into(),
        }
    }
}

impl UiNativeTextPinObservation {
    #[doc(hidden)]
    pub const fn layout_digest(self) -> [u8; 32] {
        self.layout
    }

    #[doc(hidden)]
    pub const fn raster_key_digest(self) -> [u8; 32] {
        self.raster_key
    }

    #[doc(hidden)]
    pub fn matches(self, request: worth_ui_host_contract::UiGlyphRasterPinRequest) -> bool {
        self == PinIdentity::new(request.layout_identity(), request.key()).observation()
    }
}

pub(crate) struct AtlasCore {
    pub(crate) alpha: AlphaAtlasStore,
    pub(crate) color: ColorAtlasStore,
    pub(crate) generation: UiNativeTextAtlasGeneration,
    pub(crate) reservation: Option<u64>,
    pub(crate) next_reservation: u64,
    pub(crate) next_entry: u64,
    pub(crate) completed_use_epoch: u64,
    pub(crate) committed_transactions: u64,
    pub(crate) pins: BTreeSet<PinIdentity>,
    pub(crate) quarantined: bool,
    pub(crate) lineage: UiNativeTextAtlasLineageIdentity,
}

impl AtlasCore {
    pub(crate) fn new(lineage: UiNativeTextAtlasLineageIdentity) -> Self {
        Self {
            alpha: AlphaAtlasStore::new(),
            color: ColorAtlasStore::new(),
            generation: UiNativeTextAtlasGeneration::new(1).expect("nonzero atlas generation"),
            reservation: None,
            next_reservation: 1,
            next_entry: 1,
            completed_use_epoch: 0,
            committed_transactions: 0,
            pins: BTreeSet::new(),
            quarantined: false,
            lineage,
        }
    }

    pub(crate) fn reset_for_reconstruction(&mut self, generation: UiNativeTextAtlasGeneration) {
        self.alpha = AlphaAtlasStore::new();
        self.color = ColorAtlasStore::new();
        self.generation = generation;
        self.reservation = None;
        self.pins.clear();
        self.quarantined = false;
    }
}

pub(crate) struct UiNativeTextAtlas {
    pub(crate) core: Rc<RefCell<AtlasCore>>,
    #[cfg(test)]
    pub(crate) plan_calls: Cell<usize>,
}

#[derive(Clone, Copy)]
pub(crate) struct UiNativeTextAtlasEntryView {
    pub(crate) kind: super::upload::UiNativeGpuAtlasKind,
    pub(crate) page: u32,
    pub(crate) origin: [u32; 2],
    pub(crate) extent: [u32; 2],
    pub(crate) bearing: worth_ui_host_contract::UiGlyphRasterBearing,
}

impl Default for UiNativeTextAtlas {
    fn default() -> Self {
        Self::new()
    }
}

impl UiNativeTextAtlas {
    pub(crate) fn new() -> Self {
        let lineage = next_atlas_lineage();
        Self {
            core: Rc::new(RefCell::new(AtlasCore::new(lineage))),
            #[cfg(test)]
            plan_calls: Cell::new(0),
        }
    }

    pub(crate) fn pin_observations(&self) -> Box<[UiNativeTextPinObservation]> {
        self.core
            .borrow()
            .pins
            .iter()
            .copied()
            .map(PinIdentity::observation)
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    pub(crate) fn committed_transactions(&self) -> u64 {
        self.core.borrow().committed_transactions
    }

    pub(crate) fn entry_view(&self, key: UiGlyphRasterKey) -> Option<UiNativeTextAtlasEntryView> {
        let core = self.core.borrow();
        let (entry, kind) = match key.source() {
            worth_ui_host_contract::UiGlyphRasterSource::AlphaOutline
            | worth_ui_host_contract::UiGlyphRasterSource::LastResort => (
                core.alpha.entries.get(&key)?,
                super::upload::UiNativeGpuAtlasKind::Alpha,
            ),
            worth_ui_host_contract::UiGlyphRasterSource::ColorOutline
            | worth_ui_host_contract::UiGlyphRasterSource::ColorBitmap => (
                core.color.entries.get(&key)?,
                super::upload::UiNativeGpuAtlasKind::Color,
            ),
        };
        Some(UiNativeTextAtlasEntryView {
            kind,
            page: entry.page,
            origin: [entry.rect.x, entry.rect.y],
            extent: entry.content_extent,
            bearing: entry.bearing,
        })
    }
}
