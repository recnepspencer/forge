#[cfg(test)]
use super::accuracy_class::require_exact_accuracy_claim as require_phase_three_exact_accuracy_claim;
use super::{
    accuracy_class::declare_derived_accuracy_class as declare_phase_three_accuracy_class,
    authority_role::declare_authority_role as declare_phase_three_authority_role,
    scope_partition::require_scope_partition as require_phase_three_scope_partition,
    ArtifactAuthorityRoleWitness, ArtifactDerivedAccuracyWitness,
    ArtifactFamilyAuthorityDisposition, ArtifactFamilyAuthorityWitness,
    ArtifactFamilyClassification, ArtifactFamilyDenial, ArtifactFamilyLifecycleAdmission,
    ArtifactFamilyLifecycleDisposition, ArtifactFamilyStrategyLane, ArtifactScopePartitionWitness,
    PhysicalArtifactFamilyDeclaration,
};
use forge_store_contracts::{
    ArtifactFamilyAccessLane, ArtifactFamilyAuthorityClass, ArtifactFamilyLifecycleClass,
    DurableArtifactMigrationPosture, DurableArtifactRebuildPosture,
};
use forge_store_security::StoreCurrentSecurityScopeWitnessSet;

pub(crate) fn classify_family(
    declaration: &'static PhysicalArtifactFamilyDeclaration,
) -> ArtifactFamilyClassification {
    let authority = match declaration.authority() {
        ArtifactFamilyAuthorityClass::Authoritative => {
            ArtifactFamilyAuthorityDisposition::Authoritative
        }
        ArtifactFamilyAuthorityClass::Derived => ArtifactFamilyAuthorityDisposition::Derived,
        ArtifactFamilyAuthorityClass::Diagnostic => ArtifactFamilyAuthorityDisposition::Diagnostic,
        ArtifactFamilyAuthorityClass::Terminal => ArtifactFamilyAuthorityDisposition::Terminal,
        ArtifactFamilyAuthorityClass::CertificationEvidence => {
            ArtifactFamilyAuthorityDisposition::Certification
        }
    };

    let lifecycle = if declaration.migration_posture()
        == DurableArtifactMigrationPosture::OfflineImportOnly
    {
        ArtifactFamilyLifecycleDisposition::OfflineImportOnly
    } else if declaration.rebuild_posture() == DurableArtifactRebuildPosture::QuarantineOnly
        || declaration.migration_posture() == DurableArtifactMigrationPosture::VersionedReadmission
    {
        ArtifactFamilyLifecycleDisposition::ReadmissionRequired
    } else if declaration.lifecycle() == ArtifactFamilyLifecycleClass::TransferBoundary {
        ArtifactFamilyLifecycleDisposition::TransferBoundaryOnly
    } else {
        match declaration.access_lane() {
            ArtifactFamilyAccessLane::HotPath => {
                ArtifactFamilyLifecycleDisposition::StrategyHotPath
            }
            ArtifactFamilyAccessLane::MaintenancePath => {
                ArtifactFamilyLifecycleDisposition::StrategyMaintenancePath
            }
            ArtifactFamilyAccessLane::VerifierPath => {
                ArtifactFamilyLifecycleDisposition::VerifierOnly
            }
            ArtifactFamilyAccessLane::TerminalPath => {
                ArtifactFamilyLifecycleDisposition::TerminalOnly
            }
        }
    };

    ArtifactFamilyClassification::new(declaration, authority, lifecycle)
}

pub(crate) fn require_production_authority(
    classification: ArtifactFamilyClassification,
) -> Result<ArtifactFamilyAuthorityWitness, ArtifactFamilyDenial> {
    match classification.authority() {
        ArtifactFamilyAuthorityDisposition::Authoritative => {
            Ok(ArtifactFamilyAuthorityWitness::new(classification))
        }
        ArtifactFamilyAuthorityDisposition::Derived => {
            Err(ArtifactFamilyDenial::DerivedFamilyCannotMintProductionAuthority)
        }
        ArtifactFamilyAuthorityDisposition::Diagnostic => {
            Err(ArtifactFamilyDenial::DiagnosticFamilyCannotMintProductionAuthority)
        }
        ArtifactFamilyAuthorityDisposition::Terminal => {
            Err(ArtifactFamilyDenial::TerminalProjectionCannotMintAuthority)
        }
        ArtifactFamilyAuthorityDisposition::Certification => {
            Err(ArtifactFamilyDenial::CourtroomCannotMintAuthority)
        }
    }
}

pub(crate) fn require_strategy_lifecycle(
    authority: ArtifactFamilyAuthorityWitness,
) -> Result<ArtifactFamilyLifecycleAdmission, ArtifactFamilyDenial> {
    match authority.classification().lifecycle() {
        ArtifactFamilyLifecycleDisposition::StrategyHotPath => Ok(
            ArtifactFamilyLifecycleAdmission::new(authority, ArtifactFamilyStrategyLane::HotPath),
        ),
        ArtifactFamilyLifecycleDisposition::StrategyMaintenancePath => {
            Ok(ArtifactFamilyLifecycleAdmission::new(
                authority,
                ArtifactFamilyStrategyLane::MaintenancePath,
            ))
        }
        ArtifactFamilyLifecycleDisposition::VerifierOnly => {
            Err(ArtifactFamilyDenial::VerifierLaneCannotEnterStrategyAdmission)
        }
        ArtifactFamilyLifecycleDisposition::ReadmissionRequired => {
            Err(ArtifactFamilyDenial::ReadmissionFamilyCannotEnterStrategyAdmission)
        }
        ArtifactFamilyLifecycleDisposition::TransferBoundaryOnly => {
            Err(ArtifactFamilyDenial::TransferBoundaryFamilyCannotEnterStrategyAdmission)
        }
        ArtifactFamilyLifecycleDisposition::OfflineImportOnly => {
            Err(ArtifactFamilyDenial::OfflineImportOnlyFamilyCannotEnterStrategyAdmission)
        }
        ArtifactFamilyLifecycleDisposition::TerminalOnly => {
            Err(ArtifactFamilyDenial::TerminalProjectionCannotMintAuthority)
        }
    }
}

pub(crate) fn declare_authority_role(
    classification: ArtifactFamilyClassification,
) -> ArtifactAuthorityRoleWitness {
    declare_phase_three_authority_role(classification)
}

pub(crate) fn declare_derived_accuracy_class(
    role: ArtifactAuthorityRoleWitness,
) -> ArtifactDerivedAccuracyWitness {
    declare_phase_three_accuracy_class(role)
}

#[cfg(test)]
pub(crate) fn require_exact_accuracy_claim(
    accuracy: ArtifactDerivedAccuracyWitness,
) -> Result<ArtifactDerivedAccuracyWitness, ArtifactFamilyDenial> {
    require_phase_three_exact_accuracy_claim(accuracy)
}

pub(crate) fn require_scope_partition(
    accuracy: ArtifactDerivedAccuracyWitness,
    security_scope: &StoreCurrentSecurityScopeWitnessSet,
) -> Result<ArtifactScopePartitionWitness, ArtifactFamilyDenial> {
    require_phase_three_scope_partition(accuracy, security_scope)
}
