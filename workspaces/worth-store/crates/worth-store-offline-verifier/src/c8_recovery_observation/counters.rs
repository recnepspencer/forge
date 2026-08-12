#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RecoveryObserverCounters {
    directories_admitted: u64,
    directories_opened: u64,
    directory_entries_observed: u64,
    artifacts_admitted: u64,
    artifacts_observed: u64,
    files_opened: u64,
    bytes_read: u64,
}

impl RecoveryObserverCounters {
    pub(super) const fn from_parts(
        directories_admitted: u64,
        directories_opened: u64,
        directory_entries_observed: u64,
        artifacts_admitted: u64,
        artifacts_observed: u64,
        files_opened: u64,
        bytes_read: u64,
    ) -> Self {
        Self {
            directories_admitted,
            directories_opened,
            directory_entries_observed,
            artifacts_admitted,
            artifacts_observed,
            files_opened,
            bytes_read,
        }
    }

    pub(super) const fn with_root_admitted() -> Self {
        Self {
            directories_admitted: 1,
            directories_opened: 0,
            directory_entries_observed: 0,
            artifacts_admitted: 0,
            artifacts_observed: 0,
            files_opened: 0,
            bytes_read: 0,
        }
    }

    pub const fn directories_admitted(self) -> u64 {
        self.directories_admitted
    }

    pub const fn directories_opened(self) -> u64 {
        self.directories_opened
    }

    pub const fn directory_entries_observed(self) -> u64 {
        self.directory_entries_observed
    }

    pub const fn artifacts_admitted(self) -> u64 {
        self.artifacts_admitted
    }

    pub const fn artifacts_observed(self) -> u64 {
        self.artifacts_observed
    }

    pub const fn files_opened(self) -> u64 {
        self.files_opened
    }

    pub const fn bytes_read(self) -> u64 {
        self.bytes_read
    }

    pub(super) fn record_directory_admitted(&mut self) -> Option<u64> {
        self.directories_admitted = self.directories_admitted.checked_add(1)?;
        Some(self.directories_admitted)
    }

    pub(super) fn record_directory_opened(&mut self) -> Option<u64> {
        self.directories_opened = self.directories_opened.checked_add(1)?;
        Some(self.directories_opened)
    }

    pub(super) fn record_directory_entry(&mut self) -> Option<u64> {
        self.directory_entries_observed = self.directory_entries_observed.checked_add(1)?;
        Some(self.directory_entries_observed)
    }

    pub(super) fn record_artifact_admitted(&mut self) -> Option<u64> {
        self.artifacts_admitted = self.artifacts_admitted.checked_add(1)?;
        Some(self.artifacts_admitted)
    }

    pub(super) fn record_artifact_observed(&mut self) -> Option<u64> {
        self.artifacts_observed = self.artifacts_observed.checked_add(1)?;
        Some(self.artifacts_observed)
    }

    pub(super) fn record_file_opened(&mut self) -> Option<u64> {
        self.files_opened = self.files_opened.checked_add(1)?;
        Some(self.files_opened)
    }

    pub(super) fn record_bytes_read(&mut self, bytes: u64) -> Option<u64> {
        self.bytes_read = self.bytes_read.checked_add(bytes)?;
        Some(self.bytes_read)
    }
}
