use super::receipts::{
    SupportBasisReceipt, SupportCompatibilityReceipt, SupportCursorCheckpointReceipt,
    SupportFamilyRoleReceipt, SupportImportAdmissionReceipt, SupportMaintenanceReceipt,
    SupportOperationalVerdictReceipt, SupportPortabilityReceipt,
    SupportResumeClassificationReceipt, SupportRetentionReceipt,
};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportTrustReceiptBundle {
    resume: SupportResumeClassificationReceipt,
    operational: SupportOperationalVerdictReceipt,
    family_role: SupportFamilyRoleReceipt,
    basis: SupportBasisReceipt,
    cursor_checkpoint: SupportCursorCheckpointReceipt,
    compatibility: SupportCompatibilityReceipt,
    portability: SupportPortabilityReceipt,
    retention: Option<SupportRetentionReceipt>,
    maintenance: Option<SupportMaintenanceReceipt>,
    import_admission: Option<SupportImportAdmissionReceipt>,
}

impl SupportTrustReceiptBundle {
    pub fn new(
        resume: SupportResumeClassificationReceipt,
        operational: SupportOperationalVerdictReceipt,
        family_role: SupportFamilyRoleReceipt,
        basis: SupportBasisReceipt,
        cursor_checkpoint: SupportCursorCheckpointReceipt,
        compatibility: SupportCompatibilityReceipt,
        portability: SupportPortabilityReceipt,
    ) -> Self {
        Self {
            resume,
            operational,
            family_role,
            basis,
            cursor_checkpoint,
            compatibility,
            portability,
            retention: None,
            maintenance: None,
            import_admission: None,
        }
    }

    pub fn with_retention(mut self, receipt: SupportRetentionReceipt) -> Self {
        self.retention = Some(receipt);
        self
    }

    pub fn with_maintenance(mut self, receipt: SupportMaintenanceReceipt) -> Self {
        self.maintenance = Some(receipt);
        self
    }

    pub fn with_import_admission(mut self, receipt: SupportImportAdmissionReceipt) -> Self {
        self.import_admission = Some(receipt);
        self
    }

    pub fn receipt_count(&self) -> u64 {
        7 + u64::from(self.retention.is_some())
            + u64::from(self.maintenance.is_some())
            + u64::from(self.import_admission.is_some())
    }

    pub fn receipt_bytes(&self) -> u64 {
        self.resume.receipt_bytes()
            + self.operational.receipt_bytes()
            + self.family_role.receipt_bytes()
            + self.basis.receipt_bytes()
            + self.cursor_checkpoint.receipt_bytes()
            + self.compatibility.receipt_bytes()
            + self.portability.receipt_bytes()
            + self
                .retention
                .as_ref()
                .map_or(0, |receipt| receipt.receipt_bytes())
            + self
                .maintenance
                .as_ref()
                .map_or(0, |receipt| receipt.receipt_bytes())
            + self
                .import_admission
                .as_ref()
                .map_or(0, |receipt| receipt.receipt_bytes())
    }

    pub(crate) fn resume(&self) -> &SupportResumeClassificationReceipt {
        &self.resume
    }

    pub(crate) fn operational(&self) -> &SupportOperationalVerdictReceipt {
        &self.operational
    }

    pub(crate) fn family_role(&self) -> &SupportFamilyRoleReceipt {
        &self.family_role
    }

    pub(crate) fn basis(&self) -> &SupportBasisReceipt {
        &self.basis
    }

    pub(crate) fn cursor_checkpoint(&self) -> &SupportCursorCheckpointReceipt {
        &self.cursor_checkpoint
    }

    pub(crate) fn compatibility(&self) -> &SupportCompatibilityReceipt {
        &self.compatibility
    }

    pub(crate) fn portability(&self) -> &SupportPortabilityReceipt {
        &self.portability
    }

    pub(crate) fn retention(&self) -> Option<&SupportRetentionReceipt> {
        self.retention.as_ref()
    }

    pub(crate) fn maintenance(&self) -> Option<&SupportMaintenanceReceipt> {
        self.maintenance.as_ref()
    }

    pub(crate) fn import_admission(&self) -> Option<&SupportImportAdmissionReceipt> {
        self.import_admission.as_ref()
    }
}
