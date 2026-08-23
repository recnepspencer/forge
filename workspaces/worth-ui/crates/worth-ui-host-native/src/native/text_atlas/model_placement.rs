//! Independent bounded page placement and entry store for the atlas oracle.

use std::collections::BTreeMap;

use super::model_key::{ModelDemand, ModelKey, ModelSource};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct ModelRect {
    pub(super) x: u32,
    pub(super) y: u32,
    pub(super) width: u32,
    pub(super) height: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct ModelEntry {
    pub(super) rect: ModelRect,
    pub(super) page: usize,
    pub(super) pin_count: u32,
    pub(super) last_use: u64,
}

#[derive(Clone, Debug)]
struct ModelPage {
    free: Vec<ModelRect>,
}

impl ModelPage {
    fn new(width: u32, height: u32) -> Self {
        Self {
            free: vec![ModelRect {
                x: 0,
                y: 0,
                width,
                height,
            }],
        }
    }

    fn allocate(&mut self, width: u32, height: u32) -> Option<ModelRect> {
        let index = self
            .free
            .iter()
            .enumerate()
            .filter(|(_, rect)| rect.width >= width && rect.height >= height)
            .min_by_key(|(_, rect)| {
                (
                    u64::from(rect.width) * u64::from(rect.height),
                    rect.y,
                    rect.x,
                    rect.width,
                    rect.height,
                )
            })
            .map(|(index, _)| index)?;
        let block = self.free.swap_remove(index);
        let placed = ModelRect {
            x: block.x,
            y: block.y,
            width,
            height,
        };
        if block.width > width {
            self.free.push(ModelRect {
                x: block.x + width,
                y: block.y,
                width: block.width - width,
                height,
            });
        }
        if block.height > height {
            self.free.push(ModelRect {
                x: block.x,
                y: block.y + height,
                width: block.width,
                height: block.height - height,
            });
        }
        normalize_free(&mut self.free);
        Some(placed)
    }

    fn release(&mut self, rect: ModelRect) {
        self.free.push(rect);
        normalize_free(&mut self.free);
    }
}

#[derive(Clone, Debug)]
pub(super) struct ModelStore {
    source: ModelSource,
    page_width: u32,
    page_height: u32,
    channels: u64,
    page_limit: usize,
    pages: Vec<ModelPage>,
    pub(super) entries: BTreeMap<ModelKey, ModelEntry>,
}

impl ModelStore {
    pub(super) fn new(source: ModelSource) -> Self {
        let (page_width, page_height, channels, page_limit) = match source {
            ModelSource::Alpha => (1_024, 1_024, 1, 4),
            ModelSource::Color => (2_048, 2_048, 4, 2),
        };
        Self {
            source,
            page_width,
            page_height,
            channels,
            page_limit,
            pages: Vec::new(),
            entries: BTreeMap::new(),
        }
    }

    pub(super) fn texel_bytes(&self) -> u64 {
        self.pages.len() as u64
            * u64::from(self.page_width)
            * u64::from(self.page_height)
            * self.channels
    }

    pub(super) fn page_count(&self) -> usize {
        self.pages.len()
    }

    pub(super) fn remove(&mut self, key: ModelKey) -> Option<ModelEntry> {
        let entry = self.entries.remove(&key)?;
        self.pages[entry.page].release(entry.rect);
        Some(entry)
    }

    pub(super) fn allocate(&mut self, demand: ModelDemand, now: u64) -> Option<ModelEntry> {
        debug_assert_eq!(demand.key.source(), self.source);
        let page = self
            .pages
            .iter_mut()
            .enumerate()
            .find_map(|(index, page)| {
                page.allocate(demand.width, demand.height)
                    .map(|rect| (index, rect))
            })
            .or_else(|| {
                if self.pages.len() == self.page_limit {
                    None
                } else {
                    self.pages
                        .push(ModelPage::new(self.page_width, self.page_height));
                    let index = self.pages.len() - 1;
                    self.pages[index]
                        .allocate(demand.width, demand.height)
                        .map(|rect| (index, rect))
                }
            })?;
        Some(ModelEntry {
            rect: page.1,
            page: page.0,
            pin_count: 0,
            last_use: now,
        })
    }

    pub(super) fn oldest_unpinned(&self) -> Option<ModelKey> {
        self.entries
            .iter()
            .filter(|(_, entry)| entry.pin_count == 0)
            .min_by_key(|(key, entry)| (entry.last_use, key.canonical()))
            .map(|(key, _)| *key)
    }

    pub(super) fn eviction_order(&self, key: ModelKey) -> Option<(u64, Vec<u8>, ModelKey)> {
        self.entries
            .get(&key)
            .map(|entry| (entry.last_use, key.canonical(), key))
    }
}

fn normalize_free(free: &mut Vec<ModelRect>) {
    free.sort_by_key(|rect| (rect.x, rect.y, rect.width, rect.height));
    free.dedup();
    let mut changed = true;
    while changed {
        changed = false;
        'pairs: for left in 0..free.len() {
            for right in left + 1..free.len() {
                if let Some(merged) = merge_rects(free[left], free[right]) {
                    free[left] = merged;
                    free.swap_remove(right);
                    free.sort_by_key(|rect| (rect.x, rect.y, rect.width, rect.height));
                    changed = true;
                    break 'pairs;
                }
            }
        }
    }
}

fn merge_rects(left: ModelRect, right: ModelRect) -> Option<ModelRect> {
    if left.y == right.y
        && left.height == right.height
        && (left.x + left.width == right.x || right.x + right.width == left.x)
    {
        return Some(ModelRect {
            x: left.x.min(right.x),
            y: left.y,
            width: left.width + right.width,
            height: left.height,
        });
    }
    if left.x == right.x
        && left.width == right.width
        && (left.y + left.height == right.y || right.y + right.height == left.y)
    {
        return Some(ModelRect {
            x: left.x,
            y: left.y.min(right.y),
            width: left.width,
            height: left.height + right.height,
        });
    }
    None
}
