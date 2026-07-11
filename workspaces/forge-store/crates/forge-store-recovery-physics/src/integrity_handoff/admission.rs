use super::{IntegrityHandoffDenial, IntegrityHandoffDenialKind, IntegrityHandoffPayload};
use crate::AdmittedRecoveryIntegrityInput;
use forge_store_contracts::PhysicalIntegrityReadinessPayload;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntegrityHandoffAdmission;

impl IntegrityHandoffAdmission {
    pub fn admit(
        s3_payload: PhysicalIntegrityReadinessPayload,
        payload: IntegrityHandoffPayload,
    ) -> Result<AdmittedRecoveryIntegrityInput, IntegrityHandoffDenial> {
        let _s3_entry_basis = S3HandoffEntryAdmissionBasis::from_payload(s3_payload)?;
        Ok(AdmittedRecoveryIntegrityInput::from_admitted_integrity_handoff(payload))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct S3HandoffEntryAdmissionBasis;

impl S3HandoffEntryAdmissionBasis {
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
        return denied(IntegrityHandoffDenialKind::MissingS3ProtectedViewCapability);
    }
    Ok(())
}

fn require_lease_scoped_integrity_inspection(
    payload: PhysicalIntegrityReadinessPayload,
) -> Result<(), IntegrityHandoffDenial> {
    if !payload.inspection_lifetime_law().is_lease_scoped() {
        return denied(IntegrityHandoffDenialKind::MissingS3InspectionLifetimeLaw);
    }
    Ok(())
}

fn require_no_materialization_entry_witness(
    payload: PhysicalIntegrityReadinessPayload,
) -> Result<(), IntegrityHandoffDenial> {
    let witness = payload.no_materialization_witness();
    if !witness.forbids_whole_store() || !witness.forbids_whole_object() {
        return denied(IntegrityHandoffDenialKind::MissingS3NoMaterializationWitness);
    }
    Ok(())
}

fn denied<T>(kind: IntegrityHandoffDenialKind) -> Result<T, IntegrityHandoffDenial> {
    Err(IntegrityHandoffDenial::new(kind))
}
