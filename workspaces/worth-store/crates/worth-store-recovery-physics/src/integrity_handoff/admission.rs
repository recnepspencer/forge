use super::{IntegrityHandoffDenial, IntegrityHandoffDenialKind, IntegrityHandoffPayload};
use crate::AdmittedRecoveryIntegrityInput;
use worth_store_contracts::PhysicalIntegrityReadinessPayload;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntegrityHandoffAdmission;

impl IntegrityHandoffAdmission {
    pub fn admit(
        s3_payload: PhysicalIntegrityReadinessPayload,
        payload: IntegrityHandoffPayload,
    ) -> Result<AdmittedRecoveryIntegrityInput, IntegrityHandoffDenial> {
        let _integrity_entry_basis = IntegrityHandoffEntryAdmissionBasis::from_payload(s3_payload)?;
        Ok(AdmittedRecoveryIntegrityInput::from_admitted_integrity_handoff(payload))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct IntegrityHandoffEntryAdmissionBasis;

impl IntegrityHandoffEntryAdmissionBasis {
    fn from_payload(
        payload: PhysicalIntegrityReadinessPayload,
    ) -> Result<Self, IntegrityHandoffDenial> {
        require_protected_integrity_view(payload)?;
        require_lease_scoped_integrity_inspection(payload)?;
        require_no_materialization_entry_witness(payload)?;
        Ok(Self)
    }
}

fn require_protected_integrity_view(
    payload: PhysicalIntegrityReadinessPayload,
) -> Result<(), IntegrityHandoffDenial> {
    if !payload.protected_view_capability().is_concrete() {
        return denied(IntegrityHandoffDenialKind::MissingProtectedViewCapability);
    }
    Ok(())
}

fn require_lease_scoped_integrity_inspection(
    payload: PhysicalIntegrityReadinessPayload,
) -> Result<(), IntegrityHandoffDenial> {
    if !payload.inspection_lifetime_law().is_lease_scoped() {
        return denied(IntegrityHandoffDenialKind::MissingInspectionLifetimeLaw);
    }
    Ok(())
}

fn require_no_materialization_entry_witness(
    payload: PhysicalIntegrityReadinessPayload,
) -> Result<(), IntegrityHandoffDenial> {
    let witness = payload.no_materialization_witness();
    if !witness.forbids_whole_store() || !witness.forbids_whole_object() {
        return denied(IntegrityHandoffDenialKind::MissingNoMaterializationWitness);
    }
    Ok(())
}

fn denied<T>(kind: IntegrityHandoffDenialKind) -> Result<T, IntegrityHandoffDenial> {
    Err(IntegrityHandoffDenial::new(kind))
}
