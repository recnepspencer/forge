mod binding_compaction;
mod dirty_basis;
mod footer;
mod identity;
mod record;
mod source;
mod stream;

pub use dirty_basis::{CheckpointDirtyFrameBasis, CHECKPOINT_DIRTY_FRAME_RECORD_BYTES};
pub use footer::{CheckpointStreamFooter, CHECKPOINT_STREAM_FOOTER_RECORD_BYTES};
pub use identity::PhysicalCheckpointIdentity;
pub use record::CheckpointStreamDecodeDenial;
pub use source::{
    CheckpointRootBasis, CheckpointWalSourceRange, PhysicalCheckpointSource,
    CHECKPOINT_STREAM_HEADER_RECORD_BYTES,
};
pub use stream::{
    CheckpointBindingCompactionDecoder, CheckpointBindingCompactionEncoder,
    CheckpointStreamDecoder, CheckpointStreamEncoder,
};

#[cfg(test)]
mod tests;
pub use binding_compaction::{
    decode_checkpoint_binding_record, CheckpointBindingCompactionHeader,
    CheckpointBindingRecordFrameLength, CHECKPOINT_BINDING_COMPACTION_HEADER_RECORD_BYTES,
    CHECKPOINT_BINDING_RECORD_PREFIX_BYTES, MAX_CHECKPOINT_BINDING_RECORD_BYTES,
};
