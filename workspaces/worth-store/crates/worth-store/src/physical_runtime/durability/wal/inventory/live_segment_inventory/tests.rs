use worth_store_wal::{
    LogSequenceNumber, WalLsnRange, WalSegmentArtifactIdentity, WalSegmentGeneration, WalSegmentId,
};

use super::{PhysicalWalSegmentInventory, PhysicalWalSegmentInventoryUpdateDenial};

fn identity(segment: u64, generation: u64) -> WalSegmentArtifactIdentity {
    WalSegmentArtifactIdentity::new(
        WalSegmentId::new(segment).unwrap(),
        WalSegmentGeneration::new(generation).unwrap(),
    )
}

fn range(start: u64, end: u64) -> WalLsnRange {
    WalLsnRange::new(LogSequenceNumber::new(start), LogSequenceNumber::new(end)).unwrap()
}

#[test]
fn completed_appends_extend_one_segment_and_rotation_preserves_exact_inventory() {
    let mut inventory = PhysicalWalSegmentInventory::empty();
    inventory
        .record_completed_append(identity(1, 7), range(1, 3), 80)
        .unwrap();
    inventory
        .record_completed_append(identity(1, 7), range(3, 4), 50)
        .unwrap();
    inventory
        .record_completed_append(identity(2, 7), range(4, 6), 90)
        .unwrap();

    assert_eq!(inventory.entries.len(), 2);
    assert_eq!(inventory.entries[0].identity, identity(1, 7));
    assert_eq!(inventory.entries[0].lsn_range, range(1, 4));
    assert_eq!(inventory.entries[0].byte_count, 130);
    assert_eq!(inventory.entries[1].identity, identity(2, 7));
    assert_eq!(inventory.entries[1].lsn_range, range(4, 6));
}

#[test]
fn inventory_rejects_noncanonical_rotation_truth() {
    let mut inventory = PhysicalWalSegmentInventory::empty();
    inventory
        .record_completed_append(identity(2, 7), range(1, 3), 80)
        .unwrap();
    assert_eq!(
        inventory.record_completed_append(identity(1, 7), range(3, 4), 50),
        Err(PhysicalWalSegmentInventoryUpdateDenial::ArtifactOrder)
    );
    assert_eq!(
        inventory.record_completed_append(identity(3, 8), range(3, 4), 50),
        Err(PhysicalWalSegmentInventoryUpdateDenial::GenerationMismatch)
    );
    assert_eq!(
        inventory.record_completed_append(identity(3, 7), range(4, 5), 50),
        Err(PhysicalWalSegmentInventoryUpdateDenial::LsnDiscontinuity)
    );
}

#[test]
fn reclamation_consumes_only_the_exact_oldest_inventory_entry() {
    let mut inventory = PhysicalWalSegmentInventory::empty();
    inventory
        .record_completed_append(identity(1, 7), range(1, 3), 80)
        .unwrap();
    inventory
        .record_completed_append(identity(2, 7), range(3, 5), 90)
        .unwrap();
    let oldest = inventory.entries()[0];
    let successor = inventory.entries()[1];

    assert_eq!(
        inventory.consume_reclaimed_head(successor),
        Err(PhysicalWalSegmentInventoryUpdateDenial::ArtifactOrder)
    );
    assert_eq!(inventory.entries().len(), 2);
    assert_eq!(inventory.consume_reclaimed_head(oldest), Ok(oldest));
    assert_eq!(inventory.entries(), &[successor]);
}
