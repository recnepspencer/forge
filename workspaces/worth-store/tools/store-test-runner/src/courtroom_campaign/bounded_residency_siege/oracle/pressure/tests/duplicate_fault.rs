use super::super::verify_duplicate_fault;
use crate::courtroom_campaign::bounded_residency_siege::protocol::BoundedResidencyDuplicateFaultObservation;

const DENIAL: &str = "Courtroom C duplicate cold reads did not share one fault and source load";

#[test]
fn duplicate_fault_oracle_rejects_every_one_field_bypass() {
    let accepted = accepted_duplicate_fault();
    assert!(verify_duplicate_fault(accepted).is_ok());
    for hostile in [
        BoundedResidencyDuplicateFaultObservation {
            faults: 2,
            ..accepted
        },
        BoundedResidencyDuplicateFaultObservation {
            source_loads: 2,
            ..accepted
        },
        BoundedResidencyDuplicateFaultObservation {
            coalesced_waiters: 0,
            ..accepted
        },
        BoundedResidencyDuplicateFaultObservation {
            pinned_frames: 2,
            ..accepted
        },
        BoundedResidencyDuplicateFaultObservation {
            pin_leases: 1,
            ..accepted
        },
        BoundedResidencyDuplicateFaultObservation {
            positioned_reads: 2,
            ..accepted
        },
        BoundedResidencyDuplicateFaultObservation {
            owner_work: 0,
            ..accepted
        },
        BoundedResidencyDuplicateFaultObservation {
            waiter_work: 1,
            ..accepted
        },
        BoundedResidencyDuplicateFaultObservation {
            same_frame: false,
            ..accepted
        },
        BoundedResidencyDuplicateFaultObservation {
            same_prefix: false,
            ..accepted
        },
        BoundedResidencyDuplicateFaultObservation {
            waiter_created_work: true,
            ..accepted
        },
    ] {
        assert_eq!(verify_duplicate_fault(hostile).unwrap_err(), DENIAL);
    }
}

fn accepted_duplicate_fault() -> BoundedResidencyDuplicateFaultObservation {
    BoundedResidencyDuplicateFaultObservation {
        faults: 1,
        source_loads: 1,
        coalesced_waiters: 1,
        pinned_frames: 1,
        pin_leases: 2,
        positioned_reads: 1,
        owner_work: 1,
        waiter_work: 0,
        same_frame: true,
        same_prefix: true,
        waiter_created_work: false,
    }
}
