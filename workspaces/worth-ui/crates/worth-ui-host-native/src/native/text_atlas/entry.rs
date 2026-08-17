//! Native-owned atlas entry identity, placement, and use epoch.

use worth_ui_host_contract::UiGlyphRasterKey;

use super::placement::UiAtlasRect;
use super::UiAtlasEntryIdentity;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct UiAtlasEntry {
    pub(crate) identity: UiAtlasEntryIdentity,
    pub(crate) key: UiGlyphRasterKey,
    pub(crate) page: u32,
    pub(crate) rect: UiAtlasRect,
    pub(crate) staged_bytes: u64,
    pub(crate) digest: [u8; 32],
    pub(crate) pin_count: u32,
    pub(crate) completed_use_epoch: u64,
}

impl UiAtlasEntry {
    #[cfg(test)]
    pub(crate) fn pinned(&self) -> bool {
        self.pin_count != 0
    }
}
