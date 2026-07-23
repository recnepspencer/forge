use worth_store_physical_backend::ArtifactRangeWriteDurabilityRequirement;
use worth_store_physical_format::{RecordArtifactFile, RecordFrameCoordinate};

use super::{
    effect_contract::require_effect_contract, PhysicalWorkDeclarationDenial,
    PhysicalWorkDurabilityRequirement, PhysicalWorkEffectClass, PhysicalWorkOperationFamily,
    PhysicalWorkRecoveryDisposition, PhysicalWorkResourceDemand, PhysicalWorkScope,
};

#[test]
fn batch_scope_rejects_empty_and_duplicate_member_sets() {
    assert_eq!(
        PhysicalWorkScope::batch([]),
        Err(PhysicalWorkDeclarationDenial::EmptyScope)
    );
    let coordinate = RecordFrameCoordinate::new(RecordArtifactFile::BootstrapCatalog, 0, 8)
        .expect("fixture coordinate is non-empty");
    assert_eq!(
        PhysicalWorkScope::batch([coordinate]),
        Err(PhysicalWorkDeclarationDenial::BatchRequiresMultipleMembers)
    );
    assert_eq!(
        PhysicalWorkScope::batch([coordinate, coordinate]),
        Err(PhysicalWorkDeclarationDenial::DuplicateScopeMember)
    );
}

#[test]
fn batch_scope_stops_at_the_physical_member_bound() {
    let coordinates = (0..=256).map(|offset| {
        RecordFrameCoordinate::new(RecordArtifactFile::BootstrapCatalog, offset * 8, 8)
            .expect("fixture coordinate is non-empty")
    });
    assert_eq!(
        PhysicalWorkScope::batch(coordinates),
        Err(PhysicalWorkDeclarationDenial::ScopeCapacityExceeded)
    );
}

#[test]
fn batch_scope_rejects_overlapping_ranges_but_accepts_touching_ranges() {
    let first = RecordFrameCoordinate::new(RecordArtifactFile::BootstrapCatalog, 0, 8).unwrap();
    let overlap = RecordFrameCoordinate::new(RecordArtifactFile::BootstrapCatalog, 7, 8).unwrap();
    let touching = RecordFrameCoordinate::new(RecordArtifactFile::BootstrapCatalog, 8, 8).unwrap();
    assert_eq!(
        PhysicalWorkScope::batch([first, overlap]),
        Err(PhysicalWorkDeclarationDenial::OverlappingScopeMembers)
    );
    assert!(PhysicalWorkScope::batch([first, touching]).is_ok());
}

#[test]
fn batch_scope_canonicalizes_the_same_member_set_across_input_order() {
    let first = RecordFrameCoordinate::new(RecordArtifactFile::BootstrapCatalog, 0, 8).unwrap();
    let second = RecordFrameCoordinate::new(RecordArtifactFile::BootstrapCatalog, 16, 8).unwrap();
    let forward = PhysicalWorkScope::batch([first, second]).unwrap();
    let reversed = PhysicalWorkScope::batch([second, first]).unwrap();

    assert_eq!(forward, reversed);
    assert_eq!(reversed.coordinates(), &[first, second]);
}

#[test]
fn physical_demand_is_derived_from_the_exact_scope() {
    let scope = PhysicalWorkScope::one(
        RecordFrameCoordinate::new(RecordArtifactFile::BootstrapCatalog, 0, 8).unwrap(),
    );
    let demand = PhysicalWorkResourceDemand::derive(
        &scope,
        PhysicalWorkOperationFamily::ArtifactRangeRead,
        PhysicalWorkDurabilityRequirement::ReadOnly,
    )
    .queue_shape();
    assert_eq!(demand.queue_slots(), 1);
    assert_eq!(demand.worker_permits(), 1);
    assert_eq!(demand.bandwidth_tokens(), 8);
}

#[test]
fn effect_contracts_reject_read_write_category_substitution() {
    assert!(require_effect_contract(
        PhysicalWorkOperationFamily::ArtifactRangeRead,
        PhysicalWorkEffectClass::ReadOnly,
        PhysicalWorkDurabilityRequirement::ReadOnly,
        PhysicalWorkRecoveryDisposition::NoEffect,
    )
    .is_ok());
    assert_eq!(
        require_effect_contract(
            PhysicalWorkOperationFamily::ArtifactRangeWrite,
            PhysicalWorkEffectClass::IdempotentExactWrite,
            PhysicalWorkDurabilityRequirement::ReadOnly,
            PhysicalWorkRecoveryDisposition::RetryExact,
        ),
        Err(PhysicalWorkDeclarationDenial::EffectfulContractMismatch)
    );
}

#[test]
fn effect_contract_matrix_has_no_implicit_success_cells() {
    let operations = [
        PhysicalWorkOperationFamily::ArtifactRangeRead,
        PhysicalWorkOperationFamily::ArtifactRangeWrite,
        PhysicalWorkOperationFamily::ArtifactPublication,
    ];
    let effects = [
        PhysicalWorkEffectClass::ReadOnly,
        PhysicalWorkEffectClass::ReversibleBeforePublication,
        PhysicalWorkEffectClass::IdempotentExactWrite,
        PhysicalWorkEffectClass::PublicationBoundary,
    ];
    let durabilities = [
        PhysicalWorkDurabilityRequirement::ReadOnly,
        PhysicalWorkDurabilityRequirement::ArtifactRangeWrite(
            ArtifactRangeWriteDurabilityRequirement::BufferedWrite,
        ),
    ];
    let recoveries = [
        PhysicalWorkRecoveryDisposition::NoEffect,
        PhysicalWorkRecoveryDisposition::RetryExact,
        PhysicalWorkRecoveryDisposition::ContinueSettlement,
        PhysicalWorkRecoveryDisposition::InspectionRequired,
    ];
    for operation in operations {
        for effect in effects {
            for durability in durabilities {
                for recovery in recoveries {
                    let expected = expected_effect_cell(operation, effect, durability, recovery);
                    assert_eq!(
                        require_effect_contract(operation, effect, durability, recovery).is_ok(),
                        expected,
                        "unexpected effect-contract cell: {operation:?}/{effect:?}/{durability:?}/{recovery:?}"
                    );
                }
            }
        }
    }
}

fn expected_effect_cell(
    operation: PhysicalWorkOperationFamily,
    effect: PhysicalWorkEffectClass,
    durability: PhysicalWorkDurabilityRequirement,
    recovery: PhysicalWorkRecoveryDisposition,
) -> bool {
    let writes = matches!(
        durability,
        PhysicalWorkDurabilityRequirement::ArtifactRangeWrite(_)
    );
    match operation {
        PhysicalWorkOperationFamily::ArtifactRangeRead => {
            effect == PhysicalWorkEffectClass::ReadOnly
                && durability == PhysicalWorkDurabilityRequirement::ReadOnly
                && recovery == PhysicalWorkRecoveryDisposition::NoEffect
        }
        PhysicalWorkOperationFamily::ArtifactRangeWrite => {
            writes
                && ((effect == PhysicalWorkEffectClass::ReversibleBeforePublication
                    && recovery == PhysicalWorkRecoveryDisposition::InspectionRequired)
                    || (effect == PhysicalWorkEffectClass::IdempotentExactWrite
                        && matches!(
                            recovery,
                            PhysicalWorkRecoveryDisposition::RetryExact
                                | PhysicalWorkRecoveryDisposition::InspectionRequired
                        )))
        }
        PhysicalWorkOperationFamily::ArtifactPublication => {
            writes
                && effect == PhysicalWorkEffectClass::PublicationBoundary
                && matches!(
                    recovery,
                    PhysicalWorkRecoveryDisposition::ContinueSettlement
                        | PhysicalWorkRecoveryDisposition::InspectionRequired
                )
        }
    }
}
