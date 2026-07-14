#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CorruptionDenial {
    ReadmissionInputDoesNotMatchRequiredOutcome {
        required: super::classification::LayoutCorruptionClass,
    },
    FamilyBoundReadmissionWitnessRequired {
        family: crate::PhysicalArtifactFamily,
        source: super::readmission::LayoutReadmissionSource,
    },
    QuarantineRecordBackedReadmissionEvidenceRequired {
        family: crate::PhysicalArtifactFamily,
    },
    AdmittedFamilyReadmissionAuthorityRequired {
        family: crate::PhysicalArtifactFamily,
    },
    SecurityScopeReadmissionMismatch {
        family: crate::PhysicalArtifactFamily,
        required: forge_store_security::StoreSecurityScopeIdentity,
        current: forge_store_security::StoreSecurityScopeIdentity,
    },
    QuarantineReadmissionRequired {
        family: crate::PhysicalArtifactFamily,
    },
    ImportReadmissionRequired {
        family: crate::PhysicalArtifactFamily,
    },
    NoForegroundReadAuthority {
        family: crate::PhysicalArtifactFamily,
    },
    UnexpectedOfflineReadmissionClass {
        family: crate::PhysicalArtifactFamily,
        class: forge_store_recovery_physics::RecoveryLayoutReadmissionClass,
    },
    UnexpectedQuarantineReadmissionClass {
        family: crate::PhysicalArtifactFamily,
        class: forge_store_recovery_physics::RecoveryLayoutReadmissionClass,
    },
}
