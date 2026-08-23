//! Independent atlas model observations used by the Gate-D oracle.

use super::model_key::ModelKey;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct ModelSnapshot {
    pub(super) generation: u64,
    pub(super) alpha_pages: usize,
    pub(super) color_pages: usize,
    pub(super) alpha_entries: usize,
    pub(super) color_entries: usize,
    pub(super) pins: usize,
    pub(super) staging_bytes: u64,
    pub(super) quarantined: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct ModelPlacement {
    pub(super) key: ModelKey,
    pub(super) page: usize,
    pub(super) origin: [u32; 2],
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct ModelReceipt {
    pub(super) generation: u64,
    pub(super) misses: usize,
    pub(super) hits: usize,
    pub(super) evictions: usize,
    pub(super) peak_entries: usize,
    pub(super) peak_texel_bytes: u64,
    pub(super) staged_bytes: u64,
    pub(super) physical_staged_bytes: u64,
    pub(super) evicted_keys: Box<[ModelKey]>,
    pub(super) placements: Box<[ModelPlacement]>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct ModelRecovery {
    pub(super) lineage: u64,
    pub(super) generation: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum ModelDenial {
    Extent,
    Staging,
    Texels,
    Pages,
    Pinned,
    Reconstruction,
    RecoveryOwner,
}
