#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PhysicalBackupMaterializationCounters {
    pub(super) source_bytes_read: u64,
    pub(super) output_bytes_written: u64,
    pub(super) manifest_bytes_written: u64,
    pub(super) resume_validation_bytes: u64,
    pub(super) resumed_artifacts: u64,
    pub(super) resumed_bytes: u64,
    pub(super) rollback_bytes: u64,
    pub(super) resumed_sessions: u64,
    pub(super) sync_operations: u64,
    pub(super) artifact_sync_operations: u64,
    pub(super) peak_buffer_bytes: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalBackupMaterializationCounterScope {
    CompleteUninterruptedExecution,
    CurrentRecoveredExecution,
}

impl PhysicalBackupMaterializationCounters {
    pub const fn source_bytes_read(self) -> u64 {
        self.source_bytes_read
    }
    pub const fn output_bytes_written(self) -> u64 {
        self.output_bytes_written
    }
    pub const fn manifest_bytes_written(self) -> u64 {
        self.manifest_bytes_written
    }
    pub const fn total_output_bytes_written(self) -> Option<u64> {
        self.output_bytes_written
            .checked_add(self.manifest_bytes_written)
    }
    pub const fn logically_materialized_bytes(self) -> Option<u64> {
        self.output_bytes_written.checked_add(self.resumed_bytes)
    }
    pub const fn resume_validation_bytes(self) -> u64 {
        self.resume_validation_bytes
    }
    pub const fn resumed_artifacts(self) -> u64 {
        self.resumed_artifacts
    }
    pub const fn resumed_bytes(self) -> u64 {
        self.resumed_bytes
    }
    pub const fn rollback_bytes(self) -> u64 {
        self.rollback_bytes
    }
    pub const fn resumed_sessions(self) -> u64 {
        self.resumed_sessions
    }
    pub const fn sync_operations(self) -> u64 {
        self.sync_operations
    }
    pub const fn artifact_sync_operations(self) -> u64 {
        self.artifact_sync_operations
    }
    pub const fn peak_buffer_bytes(self) -> u64 {
        self.peak_buffer_bytes
    }
    pub const fn scope(self) -> PhysicalBackupMaterializationCounterScope {
        if self.resumed_sessions == 0 {
            PhysicalBackupMaterializationCounterScope::CompleteUninterruptedExecution
        } else {
            PhysicalBackupMaterializationCounterScope::CurrentRecoveredExecution
        }
    }
}
