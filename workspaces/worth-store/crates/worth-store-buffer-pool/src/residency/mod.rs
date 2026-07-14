pub(crate) mod resident_frame_record;
pub(crate) mod resident_frame_table;

mod entry_denials;
mod resident_frame_bytes;
mod resident_frame_counters;
mod resident_frame_denials;
mod resident_frame_dirty_table;
mod resident_frame_eviction_table;
mod resident_frame_identity;
mod resident_frame_lease_table;
mod resident_frame_report;
mod resident_frame_request;
mod resident_frame_source;

#[cfg(test)]
mod entry_tests;
#[cfg(test)]
mod resident_frame_tests;

pub use entry_denials::{BufferPoolEntryDenial, BufferPoolEntryDenialKind};
pub use resident_frame_bytes::ResidentFrameBytes;
pub use resident_frame_counters::ResidentFrameCounterSnapshot;
pub use resident_frame_denials::{
    ResidentFrameDenial, ResidentFrameDenialKind, ResidentFrameShortcutAttempt,
};
pub use resident_frame_identity::{
    ResidentFrameGeneration, ResidentFrameIdentity, ResidentFrameSlot, ResidentFrameToken,
};
pub use resident_frame_report::{
    ResidentFrameAdmission, ResidentFrameHitMissReport, ResidentFrameResidence,
    ResidentGenerationSeparationProof,
};
pub use resident_frame_request::{ResidentFrameLoadRequest, ResidentFrameSize};
pub use resident_frame_table::{ResidentFrameTable, ResidentFrameTableCapacity};
