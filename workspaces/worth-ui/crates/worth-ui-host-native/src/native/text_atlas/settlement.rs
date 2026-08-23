//! Public settlement receipts for the native text-atlas transaction.

use super::recovery::{
    UiNativeTextAtlasDenial, UiNativeTextAtlasGeneration, UiNativeTextAtlasRecovery,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct UiNativeTextAtlasSnapshot {
    pub generation: UiNativeTextAtlasGeneration,
    pub alpha_pages: u32,
    pub color_pages: u32,
    pub alpha_entries: u32,
    pub color_entries: u32,
    pub pins: u32,
    pub staging_bytes: u64,
    pub reservation_active: bool,
}

#[derive(Debug, PartialEq)]
pub(crate) enum UiNativeTextAtlasCommitOutcome {
    Committed(UiNativeTextAtlasCommitReceipt),
    Denied(UiNativeTextAtlasDenial),
    EffectsIndeterminate(UiNativeTextAtlasRecovery),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct UiNativeTextAtlasCommitReceipt {
    pub generation: UiNativeTextAtlasGeneration,
    pub misses: u32,
    pub hits: u32,
    pub evictions: u32,
    pub committed_pins: u32,
    pub staged_bytes: u64,
    pub physical_staged_bytes: u64,
    pub peak_entries: u32,
    pub peak_texel_bytes: u64,
}
