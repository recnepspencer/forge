use crate::{
    S4IntegrityHandoffDenial, S4IntegrityHandoffDenialKind, S4IntegrityHandoffPayload,
    S4RecoveryPhysicsIntegrityReadiness,
};
use forge_store_readiness::{S3PhysicalIntegrityReadiness, S3PhysicalIntegrityReadinessPayload};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S4IntegrityHandoffAdmission;

impl S4IntegrityHandoffAdmission {
    pub fn admit(
        s3_readiness: S3PhysicalIntegrityReadiness,
        payload: S4IntegrityHandoffPayload,
    ) -> Result<S4RecoveryPhysicsIntegrityReadiness, S4IntegrityHandoffDenial> {
        let _s3_entry_basis = S3HandoffEntryAdmissionBasis::from_readiness(s3_readiness)?;
        Ok(S4RecoveryPhysicsIntegrityReadiness::from_admitted_s3_handoff(payload))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct S3HandoffEntryAdmissionBasis;

impl S3HandoffEntryAdmissionBasis {
    fn from_readiness(
        s3_readiness: S3PhysicalIntegrityReadiness,
    ) -> Result<Self, S4IntegrityHandoffDenial> {
        let payload = s3_readiness.payload();
        require_protected_integrity_view(payload)?;
        require_lease_scoped_integrity_inspection(payload)?;
        require_no_materialization_entry_witness(payload)?;
        Ok(Self)
    }
}

fn require_protected_integrity_view(
    payload: S3PhysicalIntegrityReadinessPayload,
) -> Result<(), S4IntegrityHandoffDenial> {
    if !payload.protected_view_capability().is_concrete() {
        return denied(S4IntegrityHandoffDenialKind::MissingS3ProtectedViewCapability);
    }
    Ok(())
}

fn require_lease_scoped_integrity_inspection(
    payload: S3PhysicalIntegrityReadinessPayload,
) -> Result<(), S4IntegrityHandoffDenial> {
    if !payload.inspection_lifetime_law().is_lease_scoped() {
        return denied(S4IntegrityHandoffDenialKind::MissingS3InspectionLifetimeLaw);
    }
    Ok(())
}

fn require_no_materialization_entry_witness(
    payload: S3PhysicalIntegrityReadinessPayload,
) -> Result<(), S4IntegrityHandoffDenial> {
    let witness = payload.no_materialization_witness();
    if !witness.forbids_whole_store() || !witness.forbids_whole_object() {
        return denied(S4IntegrityHandoffDenialKind::MissingS3NoMaterializationWitness);
    }
    Ok(())
}

fn denied<T>(kind: S4IntegrityHandoffDenialKind) -> Result<T, S4IntegrityHandoffDenial> {
    Err(S4IntegrityHandoffDenial::new(kind))
}
