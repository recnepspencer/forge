use super::super::verify_read_pressure;
use crate::courtroom_campaign::bounded_residency_siege::{
    protocol::BoundedResidencyReadObservation, world::RESIDENT_BYTES,
};

#[test]
fn read_pressure_oracle_accepts_exact_causal_reconciliation() {
    assert!(verify_read_pressure(accepted_reads(), 128_000).is_ok());
}

#[test]
fn read_pressure_oracle_rejects_cold_hot_and_refault_drift() {
    let accepted = accepted_reads();
    assert_read_denied_at(
        [
            BoundedResidencyReadObservation {
                cold_effects: 0,
                ..accepted
            },
            BoundedResidencyReadObservation {
                cold_work: 5,
                ..accepted
            },
            BoundedResidencyReadObservation {
                cold_metadata_effects: 2,
                ..accepted
            },
        ],
        "Courtroom C cold read work did not reconcile",
    );
    assert_read_denied_at(
        [
            BoundedResidencyReadObservation {
                hot_effects: 1,
                ..accepted
            },
            BoundedResidencyReadObservation {
                hot_metadata_effects: 1,
                ..accepted
            },
            BoundedResidencyReadObservation {
                hot_work: 1,
                ..accepted
            },
        ],
        "Courtroom C hot read created physical work or effects",
    );
    assert_read_denied_at(
        [
            BoundedResidencyReadObservation {
                refault_effects: 0,
                ..accepted
            },
            BoundedResidencyReadObservation {
                refault_work: 5,
                ..accepted
            },
            BoundedResidencyReadObservation {
                refault_metadata_effects: 2,
                ..accepted
            },
        ],
        "Courtroom C refault work did not reconcile",
    );
}

#[test]
fn read_pressure_oracle_rejects_work_identity_drift() {
    let accepted = accepted_reads();
    assert_read_denied_at(
        [
            BoundedResidencyReadObservation {
                physical_work: 9,
                ..accepted
            },
            BoundedResidencyReadObservation {
                first_operation: 0,
                ..accepted
            },
            BoundedResidencyReadObservation {
                last_operation: 19,
                ..accepted
            },
            BoundedResidencyReadObservation {
                runtime_bound: false,
                ..accepted
            },
        ],
        "Courtroom C read work lifecycle did not reconcile",
    );
}

#[test]
fn read_pressure_oracle_rejects_work_state_drift() {
    let accepted = accepted_reads();
    assert_read_denied_at(
        [
            BoundedResidencyReadObservation {
                positioned_read_effects: 3,
                ..accepted
            },
            BoundedResidencyReadObservation {
                metadata_read_effects: 5,
                ..accepted
            },
            BoundedResidencyReadObservation {
                metadata_read_work_declared: 1,
                ..accepted
            },
            BoundedResidencyReadObservation {
                metadata_read_work_dispatched: 1,
                ..accepted
            },
            BoundedResidencyReadObservation {
                metadata_read_work_terminal: 5,
                ..accepted
            },
            BoundedResidencyReadObservation {
                range_read_work_declared: 1,
                ..accepted
            },
            BoundedResidencyReadObservation {
                range_read_work_dispatched: 1,
                ..accepted
            },
            BoundedResidencyReadObservation {
                range_read_work_terminal: 3,
                ..accepted
            },
        ],
        "Courtroom C read work lifecycle did not reconcile",
    );
}

#[test]
fn read_pressure_oracle_rejects_capacity_and_residency_drift() {
    let accepted = accepted_reads();
    assert_read_denied_at(
        [
            BoundedResidencyReadObservation {
                peak_resident_bytes: RESIDENT_BYTES + 1,
                ..accepted
            },
            BoundedResidencyReadObservation {
                peak_admitted_bytes: 128_001,
                ..accepted
            },
        ],
        "Courtroom C read residency exceeded admitted capacity",
    );
    assert_read_denied_at(
        [
            BoundedResidencyReadObservation {
                faults: 3,
                ..accepted
            },
            BoundedResidencyReadObservation {
                source_loads: 3,
                ..accepted
            },
            BoundedResidencyReadObservation {
                hits: 0,
                ..accepted
            },
            BoundedResidencyReadObservation {
                evictions: 0,
                ..accepted
            },
        ],
        "Courtroom C read residency lifecycle did not reconcile",
    );
}

#[test]
fn read_pressure_oracle_rejects_copy_accounting_drift() {
    let accepted = accepted_reads();
    assert_read_denied_at(
        [
            BoundedResidencyReadObservation {
                caller_copy_operations: 0,
                ..accepted
            },
            BoundedResidencyReadObservation {
                store_copy_operations: 9,
                ..accepted
            },
            BoundedResidencyReadObservation {
                caller_copied_bytes: 0,
                ..accepted
            },
            BoundedResidencyReadObservation {
                store_copied_bytes: 999,
                ..accepted
            },
            BoundedResidencyReadObservation {
                peak_copy_width: 0,
                ..accepted
            },
            BoundedResidencyReadObservation {
                store_maximum_copy_width: 65,
                ..accepted
            },
            BoundedResidencyReadObservation {
                peak_copy_width: 129,
                store_maximum_copy_width: 129,
                ..accepted
            },
            BoundedResidencyReadObservation {
                streaming_scratch_bytes: 1024,
                ..accepted
            },
        ],
        "Courtroom C read copy accounting did not reconcile",
    );
}

fn assert_read_denied_at<const N: usize>(
    hostiles: [BoundedResidencyReadObservation; N],
    expected: &str,
) {
    for hostile in hostiles {
        assert_eq!(
            verify_read_pressure(hostile, 128_000).unwrap_err(),
            expected,
            "{hostile:?}"
        );
    }
}

fn accepted_reads() -> BoundedResidencyReadObservation {
    BoundedResidencyReadObservation {
        cold_effects: 2,
        hot_effects: 0,
        refault_effects: 2,
        cold_metadata_effects: 3,
        hot_metadata_effects: 0,
        refault_metadata_effects: 3,
        cold_work: 3,
        hot_work: 0,
        refault_work: 3,
        physical_work: 10,
        positioned_read_effects: 4,
        metadata_read_effects: 10,
        metadata_read_work_declared: 0,
        metadata_read_work_dispatched: 0,
        metadata_read_work_terminal: 6,
        range_read_work_declared: 0,
        range_read_work_dispatched: 0,
        range_read_work_terminal: 4,
        first_operation: 11,
        last_operation: 20,
        runtime_bound: true,
        peak_resident_bytes: RESIDENT_BYTES,
        peak_admitted_bytes: 128_000,
        faults: 4,
        source_loads: 4,
        hits: 1,
        evictions: 1,
        caller_copy_operations: 10,
        caller_copied_bytes: 1_000,
        store_copy_operations: 10,
        store_copied_bytes: 1_000,
        peak_copy_width: 64,
        store_maximum_copy_width: 64,
        streaming_scratch_bytes: 128,
        largest_record_bytes: 1_024,
    }
}
