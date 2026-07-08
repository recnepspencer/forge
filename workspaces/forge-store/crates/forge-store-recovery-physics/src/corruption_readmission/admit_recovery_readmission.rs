use forge_store_authority::StoreCurrentAuthorityWitness;

use super::{verify_quarantine_handoff_for_readmission, verify_store_authority_for_readmission};
use crate::{
    classify_recovery_repair_capability, QuarantineSummary, RecoveryCorruptionReadmissionDenial,
    RecoveryCorruptionReadmissionHandoff,
};

pub(crate) fn build_recovery_readmission_handoff(
    summary: &QuarantineSummary,
) -> RecoveryCorruptionReadmissionHandoff {
    let primary_damage_case = summary.damage_case();
    RecoveryCorruptionReadmissionHandoff::new(
        primary_damage_case,
        classify_recovery_repair_capability(primary_damage_case, summary.handoff_posture()),
    )
}

pub fn admit_recovery_corruption_readmission(
    summary: &QuarantineSummary,
    record: &forge_store_physical_integrity::QuarantineRecord,
    current_store_authority: &StoreCurrentAuthorityWitness,
) -> Result<RecoveryCorruptionReadmissionHandoff, RecoveryCorruptionReadmissionDenial> {
    verify_quarantine_handoff_for_readmission(record, summary.receipt())
        .map_err(RecoveryCorruptionReadmissionDenial::QuarantineHandoff)?;
    verify_store_authority_for_readmission(current_store_authority)
        .map_err(RecoveryCorruptionReadmissionDenial::StoreAuthority)?;
    Ok(build_recovery_readmission_handoff(summary))
}
