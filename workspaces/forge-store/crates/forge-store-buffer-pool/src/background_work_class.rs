use crate::AllocationScope;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BackgroundWorkClass {
    RecoveryPlanning,
    CompactionPlanning,
    ScrubPlanning,
    ImportExport,
    LargeRecordStreaming,
}

impl BackgroundWorkClass {
    pub const ALL: [Self; 5] = [
        Self::RecoveryPlanning,
        Self::CompactionPlanning,
        Self::ScrubPlanning,
        Self::ImportExport,
        Self::LargeRecordStreaming,
    ];

    pub const fn allocation_scope(self) -> AllocationScope {
        match self {
            Self::RecoveryPlanning => AllocationScope::Recovery,
            Self::CompactionPlanning => AllocationScope::Maintenance,
            Self::ScrubPlanning => AllocationScope::Scrub,
            Self::ImportExport => AllocationScope::ImportExport,
            Self::LargeRecordStreaming => AllocationScope::Streaming,
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RecoveryPlanning => "recovery_planning",
            Self::CompactionPlanning => "compaction_planning",
            Self::ScrubPlanning => "scrub_planning",
            Self::ImportExport => "import_export",
            Self::LargeRecordStreaming => "large_record_streaming",
        }
    }
}
