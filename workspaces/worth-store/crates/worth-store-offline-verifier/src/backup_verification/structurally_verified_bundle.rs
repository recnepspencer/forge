use worth_store_physical_format::MaterializedBackupBundle;

use super::BackupVerificationReport;
use crate::OperationalTruthReport;

#[derive(Debug)]
pub struct StructurallyVerifiedBackupBundle {
    materialized: MaterializedBackupBundle,
    verification_identity: [u8; 32],
    report: BackupVerificationReport,
    operational_truth: OperationalTruthReport,
}

impl StructurallyVerifiedBackupBundle {
    pub(crate) fn new(
        materialized: MaterializedBackupBundle,
        verification_identity: [u8; 32],
        report: BackupVerificationReport,
        operational_truth: OperationalTruthReport,
    ) -> Self {
        Self {
            materialized,
            verification_identity,
            report,
            operational_truth,
        }
    }
    pub const fn materialized(&self) -> &MaterializedBackupBundle {
        &self.materialized
    }
    pub const fn verification_identity(&self) -> [u8; 32] {
        self.verification_identity
    }
    pub const fn report(&self) -> &BackupVerificationReport {
        &self.report
    }
    pub const fn operational_truth(&self) -> &OperationalTruthReport {
        &self.operational_truth
    }
}
