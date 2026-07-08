use forge_proof::TransitionOutcome;
use forge_store_authority::StoreCurrentAuthorityWitness;
use forge_store_offline_verifier::OfflineCustodyCapsuleObservation;
use forge_store_security::{
    accept_s5_1_admitted_security_scope_readiness, admit_store_security_scope,
    readmit_trust_boundary_security_scope_declaration, S51SecurityScopeReadinessReservation,
    StoreCustodyPosture, StoreKeyScope, StoreKeyVersionPosture,
    StoreSecurityScopeAdmissionExpectation, StoreSecurityScopeAdmissionRequest, StoreTenantScope,
};

use crate::{
    backup_export_custody_declaration::backup_capsule_authenticity, BackupExportCustodyAdmission,
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
        let readmitted = readmit_trust_boundary_security_scope_declaration(
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
        let request = StoreSecurityScopeAdmissionRequest::from_raw_declaration(
            current_authority,
            readmitted,
            expectation,
        );
        let admitted = match admit_store_security_scope(request) {
            TransitionOutcome::Success(admitted) => admitted,
            TransitionOutcome::Denied(source) => {
                return Err(BackupExportCustodyDenial::SecurityScopeAdmissionDenied {
                    source,
                    counters: map_security_scope_denial_counters(source, counters),
                })
            }
            TransitionOutcome::Stale(_) | TransitionOutcome::RebindRequired(_) => {
                return Err(BackupExportCustodyDenial::ReadmissionNonCurrentKeyVersion {
                    posture: StoreKeyVersionPosture::Stale,
                    counters: counters.record_stale_key_version().denied(),
                })
            }
            TransitionOutcome::Deferred(_) | TransitionOutcome::Failed(_) => {
                return Err(BackupExportCustodyDenial::SecurityScopeAdmissionDenied {
                    source:
                        forge_store_security::StoreSecurityScopeAdmissionDenial::DeniedKeyVersionPosture,
                    counters: counters.denied(),
                })
            }
        };
        let readiness = accept_s5_1_admitted_security_scope_readiness(
            S51SecurityScopeReadinessReservation::backup_export_custody(),
            admitted,
        );
        Ok(
            BackupExportCustodyAdmission::from_trust_boundary_readmission(
                readiness,
                counters.readmitted().record_custody_admitted(),
            ),
        )
    }
}

fn map_trust_boundary_readmission_denial_counters(
    source: forge_store_security::StoreSecurityScopeAdmissionDenial,
    counters: BackupExportCustodyCounterSnapshot,
) -> BackupExportCustodyCounterSnapshot {
    match source {
        forge_store_security::StoreSecurityScopeAdmissionDenial::ExportedCustodyRequiresReadmission
        | forge_store_security::StoreSecurityScopeAdmissionDenial::ImportedCustodyRequiresReadmission
        | forge_store_security::StoreSecurityScopeAdmissionDenial::DeserializedSecurityScopeRequiresReadmission => {
            counters.record_readmission_required().denied()
        }
        forge_store_security::StoreSecurityScopeAdmissionDenial::MissingCustodyPosture
        | forge_store_security::StoreSecurityScopeAdmissionDenial::WrongCustodyPosture
        | forge_store_security::StoreSecurityScopeAdmissionDenial::DeniedCustodyPosture => {
            counters.record_custody_denied().denied()
        }
        forge_store_security::StoreSecurityScopeAdmissionDenial::UnavailableCustodyPosture => {
            counters.record_unavailable_custody_evidence().denied()
        }
        forge_store_security::StoreSecurityScopeAdmissionDenial::UnsupportedCustodyPosture
        | forge_store_security::StoreSecurityScopeAdmissionDenial::UnsupportedKeyVersionPosture
        | forge_store_security::StoreSecurityScopeAdmissionDenial::UnsupportedAuthenticityRequirement => {
            counters.record_unsupported_secure_posture().denied()
        }
        _ => counters.denied(),
    }
}

fn map_security_scope_denial_counters(
    source: forge_store_security::StoreSecurityScopeAdmissionDenial,
    counters: BackupExportCustodyCounterSnapshot,
) -> BackupExportCustodyCounterSnapshot {
    map_trust_boundary_readmission_denial_counters(source, counters)
}
