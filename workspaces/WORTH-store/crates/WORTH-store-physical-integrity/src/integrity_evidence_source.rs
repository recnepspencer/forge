use crate::{
    CheckpointRecordIntegrityReport, DerivedDamageClassification, IndexPageIntegrityReport,
    ManifestIntegrityReport, PageIntegrityReport, PhysicalIntegrityEvidenceDenial,
    QuarantineRecord, WalFrameIntegrityReport,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegrityEvidenceMaterializationPath {
    ExecutedAuthorityReport,
    ExecutedAuthorityBoundary,
    ExecutedWalFrameReport,
    ExecutedCheckpointRecordReport,
    ExecutedManifestReport,
    ExecutedDerivedReport,
    ExecutedDerivedBoundary,
    ExecutedReceipt,
    ExecutedReceiptBoundary,
    ExecutedSupportDiagnostic,
}

#[derive(Debug, Clone, Copy)]
pub enum StoreExecutedIntegrityEvidence<'a> {
    AuthoritativePage {
        report: &'a PageIntegrityReport,
        path: IntegrityEvidenceMaterializationPath,
    },
    AuthoritativeWalFrame {
        report: &'a WalFrameIntegrityReport,
    },
    AuthoritativeCheckpointRecord {
        report: &'a CheckpointRecordIntegrityReport,
    },
    AuthoritativeManifest {
        report: &'a ManifestIntegrityReport,
    },
    RebuildableDerived {
        report: &'a IndexPageIntegrityReport,
        path: IntegrityEvidenceMaterializationPath,
    },
    ReceiptEvidence {
        record: &'a QuarantineRecord,
        path: IntegrityEvidenceMaterializationPath,
    },
    SupportDiagnostic {
        diagnostic: &'a crate::IntegrityDiagnosticReport,
    },
}

impl<'a> StoreExecutedIntegrityEvidence<'a> {
    pub fn authoritative_page(report: &'a PageIntegrityReport) -> Self {
        Self::AuthoritativePage {
            report,
            path: IntegrityEvidenceMaterializationPath::ExecutedAuthorityReport,
        }
    }

    pub fn authoritative_page_boundary(report: &'a PageIntegrityReport) -> Self {
        Self::AuthoritativePage {
            report,
            path: IntegrityEvidenceMaterializationPath::ExecutedAuthorityBoundary,
        }
    }

    pub fn authoritative_wal_frame(report: &'a WalFrameIntegrityReport) -> Self {
        Self::AuthoritativeWalFrame { report }
    }

    pub fn authoritative_checkpoint_record(report: &'a CheckpointRecordIntegrityReport) -> Self {
        Self::AuthoritativeCheckpointRecord { report }
    }

    pub fn authoritative_manifest(report: &'a ManifestIntegrityReport) -> Self {
        Self::AuthoritativeManifest { report }
    }

    pub fn rebuildable_derived_report(
        report: &'a IndexPageIntegrityReport,
    ) -> Result<Self, PhysicalIntegrityEvidenceDenial> {
        require_rebuildable(report)?;
        Ok(Self::RebuildableDerived {
            report,
            path: IntegrityEvidenceMaterializationPath::ExecutedDerivedReport,
        })
    }

    pub fn rebuildable_derived_boundary(
        report: &'a IndexPageIntegrityReport,
    ) -> Result<Self, PhysicalIntegrityEvidenceDenial> {
        require_rebuildable(report)?;
        Ok(Self::RebuildableDerived {
            report,
            path: IntegrityEvidenceMaterializationPath::ExecutedDerivedBoundary,
        })
    }

    pub fn receipt_evidence(record: &'a QuarantineRecord) -> Self {
        Self::ReceiptEvidence {
            record,
            path: IntegrityEvidenceMaterializationPath::ExecutedReceipt,
        }
    }

    pub fn receipt_evidence_boundary(record: &'a QuarantineRecord) -> Self {
        Self::ReceiptEvidence {
            record,
            path: IntegrityEvidenceMaterializationPath::ExecutedReceiptBoundary,
        }
    }

    pub fn support_diagnostic(diagnostic: &'a crate::IntegrityDiagnosticReport) -> Self {
        Self::SupportDiagnostic { diagnostic }
    }

    pub const fn materialization_path(self) -> IntegrityEvidenceMaterializationPath {
        match self {
            Self::AuthoritativePage { path, .. }
            | Self::RebuildableDerived { path, .. }
            | Self::ReceiptEvidence { path, .. } => path,
            Self::AuthoritativeWalFrame { .. } => {
                IntegrityEvidenceMaterializationPath::ExecutedWalFrameReport
            }
            Self::AuthoritativeCheckpointRecord { .. } => {
                IntegrityEvidenceMaterializationPath::ExecutedCheckpointRecordReport
            }
            Self::AuthoritativeManifest { .. } => {
                IntegrityEvidenceMaterializationPath::ExecutedManifestReport
            }
            Self::SupportDiagnostic { .. } => {
                IntegrityEvidenceMaterializationPath::ExecutedSupportDiagnostic
            }
        }
    }
}

fn require_rebuildable(
    report: &IndexPageIntegrityReport,
) -> Result<(), PhysicalIntegrityEvidenceDenial> {
    if matches!(
        report.damage_classification(),
        DerivedDamageClassification::RebuildableDerived(_)
    ) {
        Ok(())
    } else {
        Err(PhysicalIntegrityEvidenceDenial::DerivedReportIsNotRebuildable)
    }
}
