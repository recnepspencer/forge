use super::s4_physical_record_grammar::{
    S4CheckpointManifestRecord, S4CheckpointPageImageRecord, S4PersistedPhysicalRecord,
    S4WalRedoFrameRecord,
};
use super::PersistedRecoveryArtifacts;

pub(super) struct DecodedS4RecoveryRecords<'a> {
    checkpoint: Option<&'a S4CheckpointManifestRecord>,
    wal_frame: Option<&'a S4WalRedoFrameRecord>,
    checkpoint_page: Option<&'a S4CheckpointPageImageRecord>,
    ambiguous_role: bool,
    semantic_decode_attempts: u32,
}

impl<'a> DecodedS4RecoveryRecords<'a> {
    pub(super) fn from_artifacts(artifacts: &'a PersistedRecoveryArtifacts) -> Self {
        let mut decoded = Self {
            checkpoint: None,
            wal_frame: None,
            checkpoint_page: None,
            ambiguous_role: false,
            semantic_decode_attempts: 0,
        };
        for record in artifacts.records() {
            decoded.semantic_decode_attempts += 1;
            match record.physical_record() {
                S4PersistedPhysicalRecord::CheckpointManifest(record) => {
                    decoded.ambiguous_role |= decoded.checkpoint.is_some();
                    decoded.checkpoint = Some(record);
                }
                S4PersistedPhysicalRecord::WalRedoFrame(record) => {
                    decoded.ambiguous_role |= decoded.wal_frame.is_some();
                    decoded.wal_frame = Some(record);
                }
                S4PersistedPhysicalRecord::CheckpointPageImage(record) => {
                    decoded.ambiguous_role |= decoded.checkpoint_page.is_some();
                    decoded.checkpoint_page = Some(record);
                }
            }
        }
        decoded
    }

    pub(super) const fn checkpoint(&self) -> Option<&'a S4CheckpointManifestRecord> {
        self.checkpoint
    }

    pub(super) const fn wal_frame(&self) -> Option<&'a S4WalRedoFrameRecord> {
        self.wal_frame
    }

    pub(super) const fn checkpoint_page(&self) -> Option<&'a S4CheckpointPageImageRecord> {
        self.checkpoint_page
    }

    pub(super) const fn has_ambiguous_role(&self) -> bool {
        self.ambiguous_role
    }

    pub(super) const fn semantic_decode_attempts(&self) -> u32 {
        self.semantic_decode_attempts
    }
}
