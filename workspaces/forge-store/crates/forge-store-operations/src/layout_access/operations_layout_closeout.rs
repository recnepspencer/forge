use super::backup_family::BackupLayoutEvidenceReport;
use super::capsule_operation_family::CapsuleOperationLayoutReport;
use super::export_family::ExportLayoutEvidenceReport;
use super::import_family::ImportLayoutEvidenceReport;
use super::restore_family::RestoreLayoutEvidenceReport;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperationsLayoutCloseout {
    backup: BackupLayoutEvidenceReport,
    export: ExportLayoutEvidenceReport,
    capsule: CapsuleOperationLayoutReport,
    restore: RestoreLayoutEvidenceReport,
    import: ImportLayoutEvidenceReport,
}

impl OperationsLayoutCloseout {
    pub const fn new(
        backup: BackupLayoutEvidenceReport,
        export: ExportLayoutEvidenceReport,
        capsule: CapsuleOperationLayoutReport,
        restore: RestoreLayoutEvidenceReport,
        import: ImportLayoutEvidenceReport,
    ) -> Self {
        Self {
            backup,
            export,
            capsule,
            restore,
            import,
        }
    }

    pub const fn backup(&self) -> &BackupLayoutEvidenceReport {
        &self.backup
    }

    pub const fn export(&self) -> &ExportLayoutEvidenceReport {
        &self.export
    }

    pub const fn capsule(&self) -> &CapsuleOperationLayoutReport {
        &self.capsule
    }

    pub const fn restore(&self) -> &RestoreLayoutEvidenceReport {
        &self.restore
    }

    pub const fn import(&self) -> &ImportLayoutEvidenceReport {
        &self.import
    }
}
