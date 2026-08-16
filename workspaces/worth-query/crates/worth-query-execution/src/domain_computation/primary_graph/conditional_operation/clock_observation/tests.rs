use super::*;

struct TestClock;

fn granular_installation(
) -> crate::domain_computation::primary_graph::WorthQueryGranularInvalidationInstallation {
    let world =
        crate::domain_computation::primary_graph::tests::fixture::installed_authorization_world(
            true,
        );
    let integration = world
        .application
        .runtime
        .primary_graph()
        .expect("the fixture publishes one primary graph")
        .integration_handle();
    crate::domain_computation::primary_graph::WorthQueryGranularInvalidationInstallation::new(
        worth_query_installation::facade::ApplicationSchemaBindingIdentity::from_installed_parts(
            7,
            3,
            worth_foundational::facade::CanonicalDigestId::new([0x11; 32]),
            worth_foundational::facade::CanonicalDigestId::new([0x22; 32]),
        ),
        integration,
    )
}

fn receipt() -> ErasedClockObservationReceipt {
    ErasedClockObservationReceipt {
        sequence: 7,
        observed_coordinate: 41,
        due_wake_count: 2,
        due_work_remaining: true,
        authoritative_commit_count: 2,
        authoritative_work_remaining: true,
        retained_due_wake_count: 3,
        retained_eligible_wake_count: 1,
        retained_suppressed_wake_count: 1,
        retained_deferred_wake_count: 0,
        retained_failed_wake_count: 1,
        committed_operation_count: 2,
        already_committed_operation_count: 1,
        failed_operation_count: 3,
        indeterminate_operation_count: 1,
        execution_provenance: Vec::new(),
        granular_invalidations: Vec::new(),
    }
}

#[test]
fn typed_receipt_preserves_query_owned_clock_and_work_evidence() {
    let installation = granular_installation();
    let mut receipt = receipt().typed::<TestClock>(installation.clone());

    assert_eq!(receipt.sequence(), 7);
    assert_eq!(receipt.observed_time().nanoseconds(), 41);
    assert_eq!(receipt.due_wake_count(), 2);
    assert!(receipt.due_work_remaining());
    assert_eq!(receipt.authoritative_commit_count(), 2);
    assert!(receipt.authoritative_work_remaining());
    assert_eq!(receipt.retained_due_wake_count(), 3);
    assert_eq!(receipt.retained_eligible_wake_count(), 1);
    assert_eq!(receipt.retained_suppressed_wake_count(), 1);
    assert_eq!(receipt.retained_deferred_wake_count(), 0);
    assert_eq!(receipt.retained_failed_wake_count(), 1);
    assert_eq!(receipt.committed_operation_count(), 2);
    assert_eq!(receipt.already_committed_operation_count(), 1);
    assert_eq!(receipt.failed_operation_count(), 3);
    assert_eq!(receipt.indeterminate_operation_count(), 1);
    let granular = receipt.take_granular_invalidation_batch();
    assert!(installation.admits_batch(&granular));
    assert!(granular.is_empty());
    assert_eq!(granular.observation().direct_truth_delivery_count(), 0);
    assert_eq!(granular.observation().signal_performed_delivery_count(), 0);
}

#[test]
fn erased_postures_map_without_lower_runtime_evidence() {
    assert!(matches!(
        ErasedClockObservationOutcome::Accepted(receipt())
            .typed::<TestClock>(granular_installation()),
        WorthQueryConditionalClockObservationOutcome::Accepted(_)
    ));
    assert!(matches!(
        ErasedClockObservationOutcome::Duplicate(receipt())
            .typed::<TestClock>(granular_installation()),
        WorthQueryConditionalClockObservationOutcome::Duplicate(_)
    ));
    assert!(matches!(
        ErasedClockObservationOutcome::Stale.typed::<TestClock>(granular_installation()),
        WorthQueryConditionalClockObservationOutcome::Stale
    ));
    assert!(matches!(
        ErasedClockObservationOutcome::Reordered.typed::<TestClock>(granular_installation()),
        WorthQueryConditionalClockObservationOutcome::Reordered
    ));
    assert!(matches!(
        ErasedClockObservationOutcome::Closed.typed::<TestClock>(granular_installation()),
        WorthQueryConditionalClockObservationOutcome::Closed
    ));
    let failed = ErasedClockObservationOutcome::Failed {
        kind: WorthQueryConditionalClockObservationFailureKind::ObservationFailed,
        detail: "clock read failed".to_string(),
    }
    .typed::<TestClock>(granular_installation());
    let WorthQueryConditionalClockObservationOutcome::Failed(failed) = failed else {
        panic!("provider failure must remain a Query-owned failure posture");
    };
    assert_eq!(
        failed.kind(),
        WorthQueryConditionalClockObservationFailureKind::ObservationFailed
    );
    assert_eq!(failed.detail(), "clock read failed");
}
