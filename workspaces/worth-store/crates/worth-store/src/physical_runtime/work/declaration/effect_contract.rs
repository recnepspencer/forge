use super::{
    PhysicalWorkDeclarationDenial, PhysicalWorkDurabilityRequirement, PhysicalWorkEffectClass,
    PhysicalWorkOperationFamily, PhysicalWorkRecoveryDisposition,
};

pub(super) fn require_effect_contract(
    operation: PhysicalWorkOperationFamily,
    effect: PhysicalWorkEffectClass,
    durability: PhysicalWorkDurabilityRequirement,
    recovery: PhysicalWorkRecoveryDisposition,
) -> Result<(), PhysicalWorkDeclarationDenial> {
    match operation {
        PhysicalWorkOperationFamily::ArtifactMetadataRead
        | PhysicalWorkOperationFamily::ArtifactRangeRead => {
            require_read_contract(effect, durability, recovery)
        }
        PhysicalWorkOperationFamily::ArtifactRangeWrite => {
            require_write_contract(effect, durability, recovery)
        }
        PhysicalWorkOperationFamily::ArtifactPublication => {
            require_publication_contract(effect, durability, recovery)
        }
        PhysicalWorkOperationFamily::WalAppend => {
            require_wal_append_contract(effect, durability, recovery)
        }
        PhysicalWorkOperationFamily::DurabilityBarrier => {
            require_wal_barrier_contract(effect, durability, recovery)
        }
    }
}

fn require_wal_barrier_contract(
    effect: PhysicalWorkEffectClass,
    durability: PhysicalWorkDurabilityRequirement,
    recovery: PhysicalWorkRecoveryDisposition,
) -> Result<(), PhysicalWorkDeclarationDenial> {
    matches!(
        (effect, durability, recovery),
        (
            PhysicalWorkEffectClass::PublicationBoundary,
            PhysicalWorkDurabilityRequirement::WalDurabilityBarrier,
            PhysicalWorkRecoveryDisposition::InspectionRequired,
        )
    )
    .then_some(())
    .ok_or(PhysicalWorkDeclarationDenial::EffectfulContractMismatch)
}

fn require_wal_append_contract(
    effect: PhysicalWorkEffectClass,
    durability: PhysicalWorkDurabilityRequirement,
    recovery: PhysicalWorkRecoveryDisposition,
) -> Result<(), PhysicalWorkDeclarationDenial> {
    matches!(
        (effect, durability, recovery),
        (
            PhysicalWorkEffectClass::ReversibleBeforePublication,
            PhysicalWorkDurabilityRequirement::WalAppend,
            PhysicalWorkRecoveryDisposition::InspectionRequired,
        )
    )
    .then_some(())
    .ok_or(PhysicalWorkDeclarationDenial::EffectfulContractMismatch)
}

fn require_read_contract(
    effect: PhysicalWorkEffectClass,
    durability: PhysicalWorkDurabilityRequirement,
    recovery: PhysicalWorkRecoveryDisposition,
) -> Result<(), PhysicalWorkDeclarationDenial> {
    matches!(
        (effect, durability, recovery),
        (
            PhysicalWorkEffectClass::ReadOnly,
            PhysicalWorkDurabilityRequirement::ReadOnly,
            PhysicalWorkRecoveryDisposition::NoEffect,
        )
    )
    .then_some(())
    .ok_or(PhysicalWorkDeclarationDenial::ReadOnlyContractMismatch)
}

fn require_write_contract(
    effect: PhysicalWorkEffectClass,
    durability: PhysicalWorkDurabilityRequirement,
    recovery: PhysicalWorkRecoveryDisposition,
) -> Result<(), PhysicalWorkDeclarationDenial> {
    let write_durability = matches!(
        durability,
        PhysicalWorkDurabilityRequirement::ArtifactRangeWrite(_)
    );
    let reversible = effect == PhysicalWorkEffectClass::ReversibleBeforePublication
        && recovery == PhysicalWorkRecoveryDisposition::InspectionRequired;
    let exact = effect == PhysicalWorkEffectClass::IdempotentExactWrite
        && matches!(
            recovery,
            PhysicalWorkRecoveryDisposition::RetryExact
                | PhysicalWorkRecoveryDisposition::InspectionRequired
        );
    (write_durability && (reversible || exact))
        .then_some(())
        .ok_or(PhysicalWorkDeclarationDenial::EffectfulContractMismatch)
}

fn require_publication_contract(
    effect: PhysicalWorkEffectClass,
    durability: PhysicalWorkDurabilityRequirement,
    recovery: PhysicalWorkRecoveryDisposition,
) -> Result<(), PhysicalWorkDeclarationDenial> {
    let write_durability = matches!(
        durability,
        PhysicalWorkDurabilityRequirement::ArtifactRangeWrite(_)
    );
    let publication = effect == PhysicalWorkEffectClass::PublicationBoundary
        && matches!(
            recovery,
            PhysicalWorkRecoveryDisposition::ContinueSettlement
                | PhysicalWorkRecoveryDisposition::InspectionRequired
        );
    (write_durability && publication)
        .then_some(())
        .ok_or(PhysicalWorkDeclarationDenial::EffectfulContractMismatch)
}
