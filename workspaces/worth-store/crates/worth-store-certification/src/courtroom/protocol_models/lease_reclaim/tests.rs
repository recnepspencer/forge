use worth_store_formal_models::{
    map_active_lease, map_expiry, map_release, LeaseReclaimAction, LeaseReclaimActionKind,
};
use worth_store_physical_isolation::{
    HazardLeaseDenial, HazardLeaseTable, HazardLeaseTableCapacity, LeaseExpiryPosture,
};
use worth_store_test_support::harness::physical_isolation::reclaim::ReclaimFixture;

use super::scenario::execute_ordinary_lease_lifecycle;

#[test]
fn ordinary_owner_execution_covers_every_lease_reclaim_action_kind() {
    let mut observed = execute_ordinary_lease_lifecycle()
        .into_iter()
        .map(LeaseReclaimAction::kind)
        .collect::<Vec<_>>();
    observed.sort_unstable();
    observed.dedup();

    assert_eq!(observed, LeaseReclaimActionKind::all());
}

#[test]
fn real_lease_release_preserves_slot_and_generation_mapping() {
    let fixture = ReclaimFixture::new(4);
    let mut table =
        HazardLeaseTable::with_capacity(HazardLeaseTableCapacity::bounded_slots(1).unwrap());
    let active = table.acquire(fixture.root, fixture.lease).unwrap();
    let acquired = map_active_lease(active);
    let release = table.release(active).unwrap();
    let released = map_release(release);

    assert!(matches!(
        (acquired, released),
        (
            LeaseReclaimAction::LeaseAcquired {
                slot: 0,
                generation: 1
            },
            LeaseReclaimAction::LeaseReleased {
                slot: 0,
                generation: 1
            }
        )
    ));
    assert!(matches!(
        table.release(active),
        Err(HazardLeaseDenial::StaleLeaseGeneration { .. })
    ));
}

#[test]
fn expiry_without_owner_receipt_never_becomes_reclaim_authority() {
    let fixture = ReclaimFixture::new(5);
    let mut table =
        HazardLeaseTable::with_capacity(HazardLeaseTableCapacity::bounded_slots(1).unwrap());
    let active = table.acquire(fixture.root, fixture.lease).unwrap();
    let expiry = LeaseExpiryPosture::expired_without_authority(active.slot(), active.generation());

    assert!(expiry.require_reclaim_authority().is_err());
    assert_eq!(
        map_expiry(expiry),
        LeaseReclaimAction::LeaseExpiredWithoutAuthority {
            slot: active.slot().get(),
            generation: active.generation().get(),
        }
    );
}
