use worth_store_operations::{
    derive_operational_audit_records, AuditCompletenessReceipt, OperationalAuditDerivationDenial,
    OperationalAuditRecord, OperationalControlRecord, OperationalEvidenceExport,
    OperationalEvidenceExportDenial,
};

use crate::{
    DrivenOperationalTransition, OperationalRecoveryProductionDriver, OperationalRecoveryYieldpoint,
};

impl OperationalRecoveryProductionDriver {
    pub fn derive_audit(
        &self,
        records: &[OperationalControlRecord],
    ) -> Result<
        DrivenOperationalTransition<Vec<OperationalAuditRecord>>,
        OperationalAuditDerivationDenial,
    > {
        if self.before(OperationalRecoveryYieldpoint::BeforeAuditDerivation) {
            return Ok(DrivenOperationalTransition::InterruptedBefore);
        }
        let audit = derive_operational_audit_records(records)?;
        Ok(self.after(OperationalRecoveryYieldpoint::AfterAuditDerivation, audit))
    }

    pub fn export_audit(
        &self,
        completeness: &AuditCompletenessReceipt,
        records: &[OperationalAuditRecord],
    ) -> Result<
        DrivenOperationalTransition<OperationalEvidenceExport>,
        OperationalEvidenceExportDenial,
    > {
        if self.before(OperationalRecoveryYieldpoint::BeforeAuditExport) {
            return Ok(DrivenOperationalTransition::InterruptedBefore);
        }
        let export = OperationalEvidenceExport::from_complete_audit(completeness, records)?;
        Ok(self.after(OperationalRecoveryYieldpoint::AfterAuditExport, export))
    }
}
