mod backup_artifact;
mod backup_artifact_decode;
mod binding_compaction;
mod compaction_cutover_record;
mod dirty_basis;
mod footer;
mod identity;
mod inspection;
mod record;
mod source;
mod stream;

pub use dirty_basis::{CheckpointDirtyFrameBasis, CHECKPOINT_DIRTY_FRAME_RECORD_BYTES};
pub use footer::{CheckpointStreamFooter, CHECKPOINT_STREAM_FOOTER_RECORD_BYTES};
pub use identity::PhysicalCheckpointIdentity;
pub use inspection::{inspect_checkpoint_stream, VerifiedCheckpointStream};
pub use record::CheckpointStreamDecodeDenial;
pub use source::{
    CheckpointRootBasis, CheckpointWalSourceRange, PhysicalCheckpointSecurityBinding,
    PhysicalCheckpointSource, CHECKPOINT_STREAM_HEADER_RECORD_BYTES,
};
pub use stream::{
    CheckpointBindingCompactionDecoder, CheckpointBindingCompactionEncoder,
    CheckpointStreamDecoder, CheckpointStreamEncoder,
};

#[cfg(test)]
mod tests;
pub use backup_artifact::{CheckpointBackupArtifact, CheckpointBackupArtifactInput};
pub use backup_artifact_decode::{
    decode_checkpoint_backup_artifact_from_reader, CheckpointBackupArtifactDecodeDenial,
    CheckpointBackupArtifactDecodeObservation, CheckpointBackupArtifactDecodeRequest,
    DecodedCheckpointBackupArtifact,
};
pub use binding_compaction::{
    decode_checkpoint_binding_record, CheckpointBindingCompactionHeader,
    CheckpointBindingRecordFrameLength, CHECKPOINT_BINDING_COMPACTION_HEADER_RECORD_BYTES,
    CHECKPOINT_BINDING_RECORD_PREFIX_BYTES, MAX_CHECKPOINT_BINDING_RECORD_BYTES,
    PHYSICAL_MUTATION_BINDING_COMPACTION_RECORD_DOMAIN,
};
pub use compaction_cutover_record::{
    PersistedCompactionCutoverRecord, PersistedCompactionProductRole,
};
