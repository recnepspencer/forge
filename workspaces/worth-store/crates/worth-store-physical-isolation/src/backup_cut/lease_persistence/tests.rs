use worth_store_physical_format::PhysicalCellReuseDomain;

use super::binary_codec::{encode, OWNER_BYTES};
use super::protected_owner::BackupProtectedPhysicalOwner;
use super::{BackupReachabilityLeasePersistenceRecord, BackupReachabilityLeaseRecoveryDenial};

fn protected_page() -> BackupProtectedPhysicalOwner {
    BackupProtectedPhysicalOwner {
        domain: PhysicalCellReuseDomain::Page,
        segment: Some(7),
        page: Some(11),
        extent: None,
        slot: None,
        root: None,
        allocation: None,
        generation: 3,
    }
}

#[test]
fn sealed_lease_round_trips_exact_physical_ownership() {
    let encoded = encode([9; 32], &[protected_page()]).expect("encode");
    let recovered = BackupReachabilityLeasePersistenceRecord::recover(&encoded)
        .expect("canonical lease record");
    assert_eq!(recovered.cut_identity(), [9; 32]);
    assert_eq!(recovered.protection(), &[protected_page()]);
    assert_eq!(recovered.recovery_bytes(), encoded);
}

#[test]
fn malformed_or_duplicated_physical_ownership_fails_closed() {
    let mut malformed = encode([9; 32], &[protected_page()]).expect("encode");
    malformed[40] = 255;
    assert_eq!(
        BackupReachabilityLeasePersistenceRecord::recover(&malformed),
        Err(BackupReachabilityLeaseRecoveryDenial::InvalidOwnerCoordinate)
    );

    let mut duplicated = encode([9; 32], &[protected_page()]).expect("encode");
    let duplicate_row = duplicated[40..40 + OWNER_BYTES].to_vec();
    duplicated[36..40].copy_from_slice(&2u32.to_le_bytes());
    duplicated.extend_from_slice(&duplicate_row);
    assert_eq!(
        BackupReachabilityLeasePersistenceRecord::recover(&duplicated),
        Err(BackupReachabilityLeaseRecoveryDenial::DuplicateProtection)
    );
}

#[test]
fn noncanonical_wire_order_fails_closed_instead_of_rewriting_history() {
    let first = protected_page();
    let second = BackupProtectedPhysicalOwner {
        page: Some(12),
        ..first
    };
    let encoded = encode([9; 32], &[second, first]).expect("encode noncanonical wire order");

    assert_eq!(
        BackupReachabilityLeasePersistenceRecord::recover(&encoded),
        Err(BackupReachabilityLeaseRecoveryDenial::NonCanonicalProtectionOrder)
    );
}

#[test]
fn high_cardinality_lease_recovery_keeps_duplicate_detection_scalable() {
    const OWNERS: u64 = 20_000;
    let protected = (1..=OWNERS)
        .map(|page| BackupProtectedPhysicalOwner {
            page: Some(page),
            ..protected_page()
        })
        .collect::<Vec<_>>();
    let record =
        BackupReachabilityLeasePersistenceRecord::from_protected_owners([7; 32], protected)
            .expect("encode high-cardinality lease");
    let recovered = BackupReachabilityLeasePersistenceRecord::recover(record.recovery_bytes())
        .expect("high-cardinality lease");

    assert_eq!(recovered.protected_artifacts(), OWNERS as usize);
    assert_eq!(recovered.recovery_bytes(), record.recovery_bytes());
}
