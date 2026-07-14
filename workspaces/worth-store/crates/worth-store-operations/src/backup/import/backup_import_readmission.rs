use worth_store_authority::StoreCurrentAuthorityWitness;
use worth_store_offline_verifier::OfflineCustodyCapsuleObservation;
use worth_store_security::{
    admit_readmitted_trust_boundary_security_scope, StoreCustodyPosture, StoreKeyScope,
    StoreKeyVersionPosture, StoreSecurityScopeAdmissionExpectation, StoreTenantScope,
};

use crate::{
    backup::export::backup_capsule_authenticity, BackupExportCustodyAdmission,
    BackupExportCustodyCounterSnapshot, BackupExportCustodyDenial,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackupImportCustodyReadmission {
    observation: OfflineCustodyCapsuleObservation,
}

impl BackupImportCustodyReadmission {
    pub const fn new(observation: OfflineCustodyCapsuleObservation) -> Self {
        Self { observation }
    }

    pub const fn observation(&self) -> &OfflineCustodyCapsuleObservation {
        &self.observation
    }

    pub fn readmit_with_current_authority(
        self,
        current_authority: &StoreCurrentAuthorityWitness,
    ) -> Result<BackupExportCustodyAdmission, BackupExportCustodyDenial> {
        let counters =
            BackupExportCustodyCounterSnapshot::from_readiness().crossed_trust_boundary();
        let expectation = StoreSecurityScopeAdmissionExpectation::new(
            StoreKeyScope::BackupExportEnvelope,
            StoreTenantScope::ImportReadmissionBoundary,
            backup_capsule_authenticity(),
            StoreCustodyPosture::Readmitted,
        );
        let admitted = admit_readmitted_trust_boundary_security_scope(
            current_authority,
            self.observation.raw_declaration(),
            StoreKeyVersionPosture::Current,
            expectation,
            self.observation.readmission_trigger(),
        )
        .map_err(
            |source| BackupExportCustodyDenial::TrustBoundaryReadmissionDenied {
                source,
                counters: map_trust_boundary_readmission_denial_counters(source, counters),
            },
        )?;
        Ok(
            BackupExportCustodyAdmission::from_trust_boundary_readmission(
                admitted,
                counters.readmitted().record_custody_admitted(),
            ),
        )
    }
}

fn map_trust_boundary_readmission_denial_counters(
    source: worth_store_security::StoreSecurityScopeAdmissionDenial,
    counters: BackupExportCustodyCounterSnapshot,
) -> BackupExportCustodyCounterSnapshot {
    match source {
        worth_store_security::StoreSecurityScopeAdmissionDenial::ExportedCustodyRequiresReadmission
        | worth_store_security::StoreSecurityScopeAdmissionDenial::ImportedCustodyRequiresReadmission
        | worth_store_security::StoreSecurityScopeAdmissionDenial::DeserializedSecurityScopeRequiresReadmission => {
            counters.record_readmission_required().denied()
        }
        worth_store_security::StoreSecurityScopeAdmissionDenial::MissingCustodyPosture
        | worth_store_security::StoreSecurityScopeAdmissionDenial::WrongCustodyPosture
        | worth_store_security::StoreSecurityScopeAdmissionDenial::DeniedCustodyPosture => {
            counters.record_custody_denied().denied()
        }
        worth_store_security::StoreSecurityScopeAdmissionDenial::UnavailableCustodyPosture => {
            counters.record_unavailable_custody_evidence().denied()
        }
        worth_store_security::StoreSecurityScopeAdmissionDenial::UnsupportedCustodyPosture
        | worth_store_security::StoreSecurityScopeAdmissionDenial::UnsupportedKeyVersionPosture
        | worth_store_security::StoreSecurityScopeAdmissionDenial::UnsupportedAuthenticityRequirement => {
            counters.record_unsupported_secure_posture().denied()
        }
        _ => counters.denied(),
    }
}
