use worth_store_authority::StoreCurrentAuthorityWitness;
use worth_store_physical_integrity::QuarantineRecord;

use crate::corruption_readmission::build_recovery_readmission_handoff;
use crate::{
    admit_recovery_corruption_readmission, RecoveryCorruptionReadmissionDenial,
    RecoveryCorruptionReadmissionHandoff,
};

use super::{IntegrityDamageMap, QuarantineSummary};

impl IntegrityDamageMap {
    pub(crate) fn build_corruption_readmission_handoffs(
        &self,
    ) -> Vec<RecoveryCorruptionReadmissionHandoff> {
        self.quarantine_summaries()
            .iter()
            .map(build_recovery_readmission_handoff)
            .collect()
    }

    pub fn admit_corruption_readmission(
        &self,
        summary: &QuarantineSummary,
        record: &QuarantineRecord,
        current_store_authority: &StoreCurrentAuthorityWitness,
    ) -> Result<RecoveryCorruptionReadmissionHandoff, RecoveryCorruptionReadmissionDenial> {
        let _ = self;
        admit_recovery_corruption_readmission(summary, record, current_store_authority)
    }
}
