use worth_store_authority::{
    BackupRestoreAdmissionAuthority, BackupRestoreAdmissionDenial, BackupRestoreAdmissionPolicy,
    BackupRestoreAdmissionReceipt, BackupRestoreAdmissionRequest, StoreCurrentAuthorityIdentity,
    StoreCurrentAuthorityWitness,
};
use worth_store_offline_verifier::StructurallyVerifiedBackupBundle;
use worth_store_physical_isolation::{
    AdmittedBackupCut, BackupReachabilityLeaseRegistry, BackupReachabilityLeaseRegistryDenial,
};
use worth_store_security::StoreSecurityScopeAdmissionReceipt;

use crate::{
    BackupExportCustodyReadiness, OperationalControlAppendDenial, OperationalControlRecord,
    OperationalControlStorePort, OperationalOperationId,
};

use super::transition;

#[derive(Debug)]
pub enum BackupVerificationJoinDenial {
    WrongCut(UnreleasedIndependentBackupVerification),
    Control {
        pending: UnreleasedIndependentBackupVerification,
        source: OperationalControlAppendDenial,
    },
    LeaseRegistry {
        pending: UnreleasedIndependentBackupVerification,
        source: BackupReachabilityLeaseRegistryDenial,
    },
}

#[derive(Debug)]
pub struct UnreleasedIndependentBackupVerification {
    operation_id: OperationalOperationId,
    structural: StructurallyVerifiedBackupBundle,
    cut: AdmittedBackupCut,
}

#[derive(Debug)]
pub struct IndependentlyVerifiedBackup {
    structural: StructurallyVerifiedBackupBundle,
    cut_authority: StoreCurrentAuthorityIdentity,
    cut_security_scope: StoreSecurityScopeAdmissionReceipt,
}

pub fn record_independent_backup_verification(
    operation_id: &OperationalOperationId,
    structural: StructurallyVerifiedBackupBundle,
    cut: AdmittedBackupCut,
    control: &impl OperationalControlStorePort,
    leases: &BackupReachabilityLeaseRegistry,
) -> Result<IndependentlyVerifiedBackup, BackupVerificationJoinDenial> {
    UnreleasedIndependentBackupVerification {
        operation_id: operation_id.clone(),
        structural,
        cut,
    }
    .record(control, leases)
}

impl UnreleasedIndependentBackupVerification {
    pub fn record(
        self,
        control: &impl OperationalControlStorePort,
        leases: &BackupReachabilityLeaseRegistry,
    ) -> Result<IndependentlyVerifiedBackup, BackupVerificationJoinDenial> {
        if self.structural.materialized().manifest().cut_identity() != self.cut.identity()
            || self
                .structural
                .materialized()
                .manifest()
                .artifact_closure_digest()
                != self.cut.manifest().artifact_closure_digest()
        {
            return Err(BackupVerificationJoinDenial::WrongCut(self));
        }
        let release_record = self.cut.lease().release_record();
        let holder = crate::control_store::backup_lease_holder_id(&self.operation_id);
        let release_reservation =
            match leases.reserve_release(holder, release_record.cut_identity()) {
                Ok(reservation) => reservation,
                Err(source) => {
                    return Err(BackupVerificationJoinDenial::LeaseRegistry {
                        pending: self,
                        source,
                    })
                }
            };
        let record = OperationalControlRecord::independent_backup_verification_recorded_and_source_lease_released(
            self.cut.authority_identity(),
            self.operation_id.clone(),
            transition(&self.operation_id, "independent-structural-verification"),
            &self.structural,
            release_record,
        );
        let receipt = match control.append(&record) {
            Ok(receipt) => receipt,
            Err(source) => {
                return Err(BackupVerificationJoinDenial::Control {
                    pending: self,
                    source,
                })
            }
        };
        if let Err(source) = release_reservation.acknowledge_durable_release(receipt) {
            return Err(BackupVerificationJoinDenial::LeaseRegistry {
                pending: self,
                source,
            });
        }
        Ok(IndependentlyVerifiedBackup {
            structural: self.structural,
            cut_authority: self.cut.authority_identity(),
            cut_security_scope: self.cut.security_scope(),
        })
    }

    pub fn into_parts(self) -> (StructurallyVerifiedBackupBundle, AdmittedBackupCut) {
        (self.structural, self.cut)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackupCustodyQualificationDenial {
    WrongSecurityScope,
}

#[derive(Debug)]
pub struct CustodyQualifiedBackupBundle {
    verified: IndependentlyVerifiedBackup,
    custody_receipt: StoreSecurityScopeAdmissionReceipt,
}

pub fn qualify_backup_custody(
    verified: IndependentlyVerifiedBackup,
    custody: &BackupExportCustodyReadiness,
) -> Result<CustodyQualifiedBackupBundle, BackupCustodyQualificationDenial> {
    let receipt = custody.receipt();
    if receipt != verified.cut_security_scope
        || receipt.receipt_id().security_scope_fingerprint()
            != verified
                .structural
                .materialized()
                .manifest()
                .security_scope_fingerprint()
    {
        return Err(BackupCustodyQualificationDenial::WrongSecurityScope);
    }
    Ok(CustodyQualifiedBackupBundle {
        verified,
        custody_receipt: receipt,
    })
}

impl CustodyQualifiedBackupBundle {
    pub const fn structural(&self) -> &StructurallyVerifiedBackupBundle {
        &self.verified.structural
    }
    pub const fn custody_receipt(&self) -> StoreSecurityScopeAdmissionReceipt {
        self.custody_receipt
    }
}

#[derive(Debug)]
pub struct ProductionRestoreAdmissibleBackupBundle {
    custody: CustodyQualifiedBackupBundle,
    admission: BackupRestoreAdmissionReceipt,
}

pub fn admit_backup_for_production_restore(
    custody: CustodyQualifiedBackupBundle,
    current_authority: &StoreCurrentAuthorityWitness,
    policy: BackupRestoreAdmissionPolicy,
) -> Result<ProductionRestoreAdmissibleBackupBundle, BackupRestoreAdmissionDenial> {
    let request = BackupRestoreAdmissionRequest::new(
        custody.structural().verification_identity(),
        custody
            .custody_receipt()
            .receipt_id()
            .security_scope_fingerprint(),
        custody.verified.cut_authority,
    );
    let admission = BackupRestoreAdmissionAuthority::for_current_store(current_authority)
        .admit(request, policy)?;
    Ok(ProductionRestoreAdmissibleBackupBundle { custody, admission })
}

impl ProductionRestoreAdmissibleBackupBundle {
    pub const fn custody(&self) -> &CustodyQualifiedBackupBundle {
        &self.custody
    }
    pub const fn admission(&self) -> BackupRestoreAdmissionReceipt {
        self.admission
    }
}
