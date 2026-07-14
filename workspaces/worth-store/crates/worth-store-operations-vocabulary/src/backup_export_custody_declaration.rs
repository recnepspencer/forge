use worth_proof::TransitionOutcome;
use worth_store_authority::StoreCurrentAuthorityWitness;
use worth_store_security::{
    admit_store_security_scope, StoreAuthenticityRequirement, StoreAuthenticityRequirementClass,
    StoreCustodyPosture, StoreKeyScope, StoreKeyVersionPosture, StoreRawSecurityScopeDeclaration,
    StoreSecurityScopeAdmissionDenial, StoreSecurityScopeAdmissionExpectation,
    StoreSecurityScopeAdmissionRequest, StoreTenantScope,
};

use crate::{
    BackupExportCustodyAdmission, BackupExportCustodyCounterSnapshot, BackupExportCustodyDenial,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackupExportCustodyMode {
    Backup,
    PointInTimeRecovery,
    Export,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BackupExportCustodyDeclaration {
    mode: BackupExportCustodyMode,
    raw_declaration: StoreRawSecurityScopeDeclaration,
    counters: BackupExportCustodyCounterSnapshot,
}

impl BackupExportCustodyDeclaration {
    pub fn native(
        current_authority: &StoreCurrentAuthorityWitness,
        mode: BackupExportCustodyMode,
        key_version_posture: StoreKeyVersionPosture,
    ) -> Result<Self, BackupExportCustodyDenial> {
        reject_non_current_key_version(key_version_posture, mode)?;

        Ok(Self {
            mode,
            raw_declaration: StoreRawSecurityScopeDeclaration::native(
                current_authority.physical_witness(),
                StoreKeyScope::BackupExportEnvelope,
                key_version_posture,
                StoreTenantScope::BackupRestoreBoundary,
                backup_capsule_authenticity(),
                StoreCustodyPosture::ExportPrepared,
            ),
            counters: BackupExportCustodyCounterSnapshot::for_declaration(mode),
        })
    }

    pub const fn mode(self) -> BackupExportCustodyMode {
        self.mode
    }

    pub const fn raw_declaration(self) -> StoreRawSecurityScopeDeclaration {
        self.raw_declaration
    }

    pub const fn counters(self) -> BackupExportCustodyCounterSnapshot {
        self.counters
    }

    pub fn admit_with_current_authority(
        self,
        current_authority: &StoreCurrentAuthorityWitness,
    ) -> Result<BackupExportCustodyAdmission, BackupExportCustodyDenial> {
        let request = StoreSecurityScopeAdmissionRequest::from_raw_declaration(
            current_authority,
            self.raw_declaration,
            backup_export_expectation(),
        );

        match admit_store_security_scope(request) {
            TransitionOutcome::Success(admitted) => {
                Ok(BackupExportCustodyAdmission::from_outbound_declaration(
                    self.mode,
                    admitted,
                    self.counters.record_custody_admitted(),
                ))
            }
            TransitionOutcome::Denied(source) => {
                Err(BackupExportCustodyDenial::SecurityScopeAdmissionDenied {
                    source,
                    counters: self.counters.denied(),
                })
            }
            TransitionOutcome::Stale(_) => Err(BackupExportCustodyDenial::NonCurrentKeyVersion {
                mode: self.mode,
                posture: StoreKeyVersionPosture::Stale,
                counters: self.counters.record_stale_key_version().denied(),
            }),
            TransitionOutcome::RebindRequired(_) => {
                Err(BackupExportCustodyDenial::NonCurrentKeyVersion {
                    mode: self.mode,
                    posture: StoreKeyVersionPosture::RebindRequired,
                    counters: self.counters.denied(),
                })
            }
            TransitionOutcome::Deferred(_) | TransitionOutcome::Failed(_) => {
                Err(BackupExportCustodyDenial::SecurityScopeAdmissionDenied {
                    source: StoreSecurityScopeAdmissionDenial::DeniedKeyVersionPosture,
                    counters: self.counters.denied(),
                })
            }
        }
    }
}

pub const fn backup_capsule_authenticity() -> StoreAuthenticityRequirement {
    StoreAuthenticityRequirement::required(
        StoreAuthenticityRequirementClass::AuthenticatedBackupCapsule,
    )
}

pub(crate) const fn backup_export_expectation() -> StoreSecurityScopeAdmissionExpectation {
    StoreSecurityScopeAdmissionExpectation::new(
        StoreKeyScope::BackupExportEnvelope,
        StoreTenantScope::BackupRestoreBoundary,
        backup_capsule_authenticity(),
        StoreCustodyPosture::ExportPrepared,
    )
}

fn reject_non_current_key_version(
    posture: StoreKeyVersionPosture,
    mode: BackupExportCustodyMode,
) -> Result<(), BackupExportCustodyDenial> {
    if posture == StoreKeyVersionPosture::Current {
        Ok(())
    } else {
        let counters = match posture {
            StoreKeyVersionPosture::Stale => {
                BackupExportCustodyCounterSnapshot::for_declaration(mode).record_stale_key_version()
            }
            StoreKeyVersionPosture::Unsupported => {
                BackupExportCustodyCounterSnapshot::for_declaration(mode)
                    .record_unsupported_secure_posture()
            }
            _ => BackupExportCustodyCounterSnapshot::for_declaration(mode),
        };
        Err(BackupExportCustodyDenial::NonCurrentKeyVersion {
            mode,
            posture,
            counters: counters.denied(),
        })
    }
}
