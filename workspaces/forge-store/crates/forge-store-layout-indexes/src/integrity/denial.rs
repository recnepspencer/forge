#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CorruptionDenial {
    ReadmissionInputDoesNotMatchRequiredOutcome {
        required: super::classification::LayoutCorruptionClass,
    },
    FamilyBoundReadmissionWitnessRequired {
        family: crate::PhysicalArtifactFamily,
        source: super::classification::LayoutReadmissionSource,
    },
    QuarantineRecordBackedReadmissionEvidenceRequired {
        family: crate::PhysicalArtifactFamily,
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
}
