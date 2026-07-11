use forge_store_security::{
    StoreAdmittedSecurityScope, StoreKeyVersionPosture, StoreLegacySecurityPosture,
    StoreSecurityMetadata,
};
use forge_store_wal::{
    CheckpointRecordSecurityMetadataEnvelope, StoreCheckpointRecordIdentity,
    StoreWalRecordIdentity, WalRecordSecurityMetadataEnvelope,
};

use crate::{RecoveryEntryAdmission, RecoveryEntryIdentity};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecoveryWalRecordSecurityMetadataIdentity {
    sequence: u64,
}

impl RecoveryWalRecordSecurityMetadataIdentity {
    pub const fn new(sequence: u64) -> Self {
        Self { sequence }
    }

    pub const fn from_store_wal_record(identity: StoreWalRecordIdentity) -> Self {
        Self::new(identity.sequence())
    }

    pub const fn sequence(self) -> u64 {
        self.sequence
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecoveryCheckpointRecordSecurityMetadataIdentity {
    checkpoint_epoch: u64,
}

impl RecoveryCheckpointRecordSecurityMetadataIdentity {
    pub const fn new(checkpoint_epoch: u64) -> Self {
        Self { checkpoint_epoch }
    }

    pub const fn from_store_checkpoint_record(identity: StoreCheckpointRecordIdentity) -> Self {
        Self::new(identity.checkpoint_epoch())
    }

    pub const fn checkpoint_epoch(self) -> u64 {
        self.checkpoint_epoch
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryWalRecordSecurityMetadataEnvelope {
    identity: RecoveryWalRecordSecurityMetadataIdentity,
    security_metadata: StoreSecurityMetadata,
}

impl RecoveryWalRecordSecurityMetadataEnvelope {
    pub fn from_wal_record_envelope(record: &WalRecordSecurityMetadataEnvelope) -> Self {
        Self::new(
            RecoveryWalRecordSecurityMetadataIdentity::from_store_wal_record(*record.record()),
            record.security_metadata().physical_metadata(),
        )
    }

    pub fn from_admitted_scope(
        identity: RecoveryWalRecordSecurityMetadataIdentity,
        admitted_scope: &StoreAdmittedSecurityScope,
        key_version_posture: StoreKeyVersionPosture,
        legacy_posture: StoreLegacySecurityPosture,
    ) -> Self {
        Self::new(
            identity,
            StoreSecurityMetadata::from_current_security_scope(
                admitted_scope.witnesses(),
                key_version_posture,
                legacy_posture,
            ),
        )
    }

    const fn new(
        identity: RecoveryWalRecordSecurityMetadataIdentity,
        security_metadata: StoreSecurityMetadata,
    ) -> Self {
        Self {
            identity,
            security_metadata,
        }
    }

    pub const fn identity(&self) -> RecoveryWalRecordSecurityMetadataIdentity {
        self.identity
    }

    pub const fn security_metadata(&self) -> StoreSecurityMetadata {
        self.security_metadata
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryCheckpointRecordSecurityMetadataEnvelope {
    identity: RecoveryCheckpointRecordSecurityMetadataIdentity,
    security_metadata: StoreSecurityMetadata,
}

impl RecoveryCheckpointRecordSecurityMetadataEnvelope {
    pub fn from_checkpoint_record_envelope(
        record: &CheckpointRecordSecurityMetadataEnvelope,
    ) -> Self {
        Self::new(
            RecoveryCheckpointRecordSecurityMetadataIdentity::from_store_checkpoint_record(
                *record.record(),
            ),
            record.security_metadata().physical_metadata(),
        )
    }

    pub fn from_admitted_scope(
        identity: RecoveryCheckpointRecordSecurityMetadataIdentity,
        admitted_scope: &StoreAdmittedSecurityScope,
        key_version_posture: StoreKeyVersionPosture,
        legacy_posture: StoreLegacySecurityPosture,
    ) -> Self {
        Self::new(
            identity,
            StoreSecurityMetadata::from_current_security_scope(
                admitted_scope.witnesses(),
                key_version_posture,
                legacy_posture,
            ),
        )
    }

    const fn new(
        identity: RecoveryCheckpointRecordSecurityMetadataIdentity,
        security_metadata: StoreSecurityMetadata,
    ) -> Self {
        Self {
            identity,
            security_metadata,
        }
    }

    pub const fn identity(&self) -> RecoveryCheckpointRecordSecurityMetadataIdentity {
        self.identity
    }

    pub const fn security_metadata(&self) -> StoreSecurityMetadata {
        self.security_metadata
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryRootSecurityMetadataEnvelope {
    entry_identity: RecoveryEntryIdentity,
    security_metadata: StoreSecurityMetadata,
}

impl RecoveryRootSecurityMetadataEnvelope {
    pub fn from_recovery_entry(
        recovery_entry: &RecoveryEntryAdmission,
        admitted_scope: &StoreAdmittedSecurityScope,
        key_version_posture: StoreKeyVersionPosture,
        legacy_posture: StoreLegacySecurityPosture,
    ) -> Self {
        Self::new(
            recovery_entry.entry_identity().clone(),
            StoreSecurityMetadata::from_current_security_scope(
                admitted_scope.witnesses(),
                key_version_posture,
                legacy_posture,
            ),
        )
    }

    const fn new(
        entry_identity: RecoveryEntryIdentity,
        security_metadata: StoreSecurityMetadata,
    ) -> Self {
        Self {
            entry_identity,
            security_metadata,
        }
    }

    pub const fn entry_identity(&self) -> &RecoveryEntryIdentity {
        &self.entry_identity
    }

    pub const fn security_metadata(&self) -> StoreSecurityMetadata {
        self.security_metadata
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoverySecurityScopePropagationInput {
    wal_record_identity: RecoveryWalRecordSecurityMetadataIdentity,
    checkpoint_record_identity: RecoveryCheckpointRecordSecurityMetadataIdentity,
    root_artifact_present: bool,
    wal_metadata: StoreSecurityMetadata,
    checkpoint_metadata: StoreSecurityMetadata,
    root_metadata: StoreSecurityMetadata,
    entry_identity: RecoveryEntryIdentity,
}

impl RecoverySecurityScopePropagationInput {
    pub fn new(
        wal_record: &RecoveryWalRecordSecurityMetadataEnvelope,
        checkpoint_record: &RecoveryCheckpointRecordSecurityMetadataEnvelope,
        recovery_root: &RecoveryRootSecurityMetadataEnvelope,
        recovery_entry: &RecoveryEntryAdmission,
    ) -> Self {
        Self {
            wal_record_identity: wal_record.identity(),
            checkpoint_record_identity: checkpoint_record.identity(),
            root_artifact_present: recovery_root.entry_identity()
                == recovery_entry.entry_identity(),
            wal_metadata: wal_record.security_metadata(),
            checkpoint_metadata: checkpoint_record.security_metadata(),
            root_metadata: recovery_root.security_metadata(),
            entry_identity: recovery_entry.entry_identity().clone(),
        }
    }

    pub const fn wal_record_identity(&self) -> RecoveryWalRecordSecurityMetadataIdentity {
        self.wal_record_identity
    }

    pub const fn checkpoint_record_identity(
        &self,
    ) -> RecoveryCheckpointRecordSecurityMetadataIdentity {
        self.checkpoint_record_identity
    }

    pub const fn root_artifact_present(&self) -> bool {
        self.root_artifact_present
    }

    pub const fn wal_metadata(&self) -> StoreSecurityMetadata {
        self.wal_metadata
    }

    pub const fn checkpoint_metadata(&self) -> StoreSecurityMetadata {
        self.checkpoint_metadata
    }

    pub const fn root_metadata(&self) -> StoreSecurityMetadata {
        self.root_metadata
    }

    pub const fn entry_identity(&self) -> &RecoveryEntryIdentity {
        &self.entry_identity
    }
}
