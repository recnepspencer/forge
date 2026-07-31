use worth_store::physical_runtime::{
    PhysicalRecordSubmission, PhysicalWalAppendFailureCause, PhysicalWalAppendOutcome,
    PhysicalWalReservationDenial, PreparedPhysicalMutation, WalAppendedPhysicalMutation,
    WalRangeReservedPhysicalMutation,
};
enum WalAppendDecision {
    Appended(WalAppendedPhysicalMutation),
    ReservationDenied {
        prepared: PreparedPhysicalMutation,
        cause: PhysicalWalReservationDenial,
    },
    ProvenNoEffect {
        prepared: PreparedPhysicalMutation,
        cause: PhysicalWalAppendFailureCause,
    },
    Inspect(WalRangeReservedPhysicalMutation),
}

fn append_one_wal_member(
    submission: &PhysicalRecordSubmission,
    prepared: PreparedPhysicalMutation,
) -> WalAppendDecision {
    match submission.append_prepared_wal(prepared) {
        PhysicalWalAppendOutcome::Appended(appended) => WalAppendDecision::Appended(appended),
        PhysicalWalAppendOutcome::ReservationDenied { prepared, cause } => {
            WalAppendDecision::ReservationDenied { prepared, cause }
        }
        PhysicalWalAppendOutcome::ProvenNoEffect { prepared, cause } => {
            WalAppendDecision::ProvenNoEffect { prepared, cause }
        }
        PhysicalWalAppendOutcome::Indeterminate { reserved } => {
            WalAppendDecision::Inspect(reserved)
        }
    }
}

use worth_store::physical_runtime::{
    PhysicalWalBarrierFailureCause, PhysicalWalBarrierOutcome, WalDurablePhysicalMutation,
};

enum WalBarrierDecision {
    Durable(WalDurablePhysicalMutation),
    Retry {
        appended: WalAppendedPhysicalMutation,
        cause: PhysicalWalBarrierFailureCause,
    },
    Inspect,
}

fn synchronize_one_wal_member(
    submission: &PhysicalRecordSubmission,
    appended: WalAppendedPhysicalMutation,
) -> WalBarrierDecision {
    match submission.synchronize_appended_wal(appended) {
        PhysicalWalBarrierOutcome::Durable(durable) => WalBarrierDecision::Durable(durable),
        PhysicalWalBarrierOutcome::BarrierNotStarted { appended, cause } => {
            WalBarrierDecision::Retry { appended, cause }
        }
        PhysicalWalBarrierOutcome::Indeterminate(_) => WalBarrierDecision::Inspect,
    }
}

fn blocked_dependency_keeps_its_signal_classification(
    cause: &PhysicalWalAppendFailureCause,
) -> bool {
    match cause {
        PhysicalWalAppendFailureCause::DependencyBlocked { class, condition } => {
            let _exact_signal_cause = (*class, *condition);
            true
        }
        _ => false,
    }
}

fn inspect_wal_append(
    submission: &PhysicalRecordSubmission,
    appended: &WalAppendedPhysicalMutation,
) -> Result<(), &'static str> {
    let declaration = appended.reserved().declaration();
    let settlement = appended.settlement();

    assert_eq!(settlement.range(), declaration.artifact_range());
    assert_eq!(
        settlement.payload_digest(),
        declaration.payload_digest(),
    );

    let wal = submission
        .wal_observation()
        .ok_or("the serving Store has released its publication authority")?;

    assert!(wal.appended_frames() >= 1);
    assert_eq!(
        wal.last_lsn_end(),
        Some(declaration.lsn_range().end_exclusive().get()),
    );
    assert!(!wal.sealed_for_inspection());
    Ok(())
}

fn main() {
    let _ = (
        append_one_wal_member,
        synchronize_one_wal_member,
        blocked_dependency_keeps_its_signal_classification,
        inspect_wal_append,
    );
}
