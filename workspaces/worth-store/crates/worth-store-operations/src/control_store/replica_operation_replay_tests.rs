use super::*;

#[test]
fn bootstrap_transfer_survives_replay_with_exact_authority_bindings() {
    let operation = operation("bootstrap-crash-recovery");
    let mut replay = SelectedControlReplay::new(OperationalControlReplayBudget::default());
    replay
        .observe(0, authorization(operation.clone(), 10))
        .expect("bootstrap authorization should open a recovery journal");
    replay
        .observe(
            1,
            record(
                operation.clone(),
                "bootstrap-transfer",
                OperationalControlRecordKind::ReplicaBootstrapTransferRecorded {
                    authorization_plan_fingerprint: [2; 32],
                    execution_plan_fingerprint: [3; 32],
                    receipt_identity: [4; 32],
                    durable_target_identity: [5; 32],
                    source_lease_identity: [6; 32],
                    source_bytes_read: 1024,
                    output_bytes_written: 1024,
                    backend_requests: 4,
                    maximum_resident_buffer_bytes: 256,
                },
            ),
        )
        .expect("durable bootstrap transfer should extend the exact journal");

    let finished = replay.finish().expect("journal should replay");
    let [handle] = finished.replica_bootstraps.as_slice() else {
        panic!("one bootstrap recovery handle should remain");
    };
    assert_eq!(handle.authorization_identity(), [1; 32]);
    assert_eq!(handle.authorization_plan_fingerprint(), [2; 32]);
    assert_eq!(handle.execution_plan_fingerprint(), [3; 32]);
    let transfer = handle.transfer().expect("transfer should be durably known");
    assert_eq!(transfer.receipt_identity(), [4; 32]);
    assert_eq!(transfer.durable_target_identity(), [5; 32]);
    assert_eq!(transfer.source_lease_identity(), [6; 32]);
}

#[test]
fn bootstrap_completion_is_terminal_and_carries_independent_verification() {
    let operation = operation("bootstrap-terminal");
    let mut replay = SelectedControlReplay::new(OperationalControlReplayBudget::default());
    replay
        .observe(0, authorization(operation.clone(), 10))
        .unwrap();
    replay
        .observe(
            1,
            record(
                operation.clone(),
                "transfer",
                OperationalControlRecordKind::ReplicaBootstrapTransferRecorded {
                    authorization_plan_fingerprint: [2; 32],
                    execution_plan_fingerprint: [3; 32],
                    receipt_identity: [4; 32],
                    durable_target_identity: [5; 32],
                    source_lease_identity: [6; 32],
                    source_bytes_read: 1024,
                    output_bytes_written: 1024,
                    backend_requests: 4,
                    maximum_resident_buffer_bytes: 256,
                },
            ),
        )
        .unwrap();
    replay
        .observe(
            2,
            record(
                operation.clone(),
                "completed",
                OperationalControlRecordKind::ReplicaBootstrapCompleted {
                    receipt_identity: [4; 32],
                    verification_identity: [8; 32],
                    source_lease_identity: [6; 32],
                },
            ),
        )
        .unwrap();
    let denial = replay
        .observe(
            3,
            record(
                operation,
                "second-terminal",
                OperationalControlRecordKind::ReplicaBootstrapAbandoned {
                    receipt_identity: [4; 32],
                    reason: "late".into(),
                    source_lease_identity: [6; 32],
                },
            ),
        )
        .unwrap_err();
    assert!(
        matches!(denial, SelectedControlReplayDenial::Invalid(ref violation)
        if violation.kind() == &OperationalControlHistoryViolationKind::ReplicaRecordAfterTerminal)
    );
}

#[test]
fn promotion_fence_is_durable_before_the_promotion_receipt() {
    let operation = operation("promotion-crash-recovery");
    let mut replay = SelectedControlReplay::new(OperationalControlReplayBudget::default());
    replay
        .observe(0, authorization(operation.clone(), 11))
        .expect("promotion authorization should open a recovery journal");
    replay
        .observe(
            1,
            record(operation.clone(), "external-fence", promotion_fence_kind()),
        )
        .expect("external fence should persist independently");
    replay
        .observe(
            2,
            record(
                operation,
                "promotion-receipt",
                OperationalControlRecordKind::ReplicaPromotionRecorded {
                    authorization_plan_fingerprint: [2; 32],
                    execution_plan_fingerprint: [3; 32],
                    receipt_identity: [8; 32],
                    fence_identity: [7; 32],
                    promoted_epoch: 12,
                },
            ),
        )
        .expect("promotion receipt should require the matching durable fence");

    let finished = replay.finish().expect("journal should replay");
    let [handle] = finished.replica_promotions.as_slice() else {
        panic!("one promotion recovery handle should remain");
    };
    let fence = handle.fence().expect("fence must remain recoverable");
    assert_eq!(fence.fence_identity(), [7; 32]);
    assert_eq!(fence.promoted_epoch(), 12);
    let receipt = handle
        .receipt()
        .expect("promotion receipt must remain recoverable");
    assert_eq!(receipt.receipt_identity(), [8; 32]);
    assert_eq!(receipt.fence_identity(), fence.fence_identity());
    assert_eq!(receipt.promoted_epoch(), fence.promoted_epoch());
}

#[test]
fn promotion_publication_readmission_and_rejoin_replay_as_one_bound_chain() {
    let operation = operation("promotion-full-chain");
    let mut replay = SelectedControlReplay::new(OperationalControlReplayBudget::default());
    replay
        .observe(0, authorization(operation.clone(), 11))
        .unwrap();
    replay
        .observe(
            1,
            record(operation.clone(), "fence", promotion_fence_kind()),
        )
        .unwrap();
    replay
        .observe(
            2,
            record(
                operation.clone(),
                "promotion",
                OperationalControlRecordKind::ReplicaPromotionRecorded {
                    authorization_plan_fingerprint: [2; 32],
                    execution_plan_fingerprint: [3; 32],
                    receipt_identity: [8; 32],
                    fence_identity: [7; 32],
                    promoted_epoch: 12,
                },
            ),
        )
        .unwrap();
    replay
        .observe(
            3,
            record(
                operation.clone(),
                "publication",
                OperationalControlRecordKind::ReplicaPromotionPublished {
                    receipt_identity: [8; 32],
                    verification_identity: [10; 32],
                    publication_identity: [11; 32],
                    target_identity: [12; 32],
                    promoted_epoch: 12,
                },
            ),
        )
        .unwrap();
    replay
        .observe(
            4,
            record(
                operation.clone(),
                "readmission",
                OperationalControlRecordKind::ReplicaPromotionReadmitted {
                    publication_identity: [11; 32],
                    serve_lease_identity: [13; 32],
                    serving_epoch: 12,
                },
            ),
        )
        .unwrap();
    replay
        .observe(
            5,
            record(
                operation.clone(),
                "rejoin",
                OperationalControlRecordKind::OldPrimaryRejoinPlanned {
                    promotion_receipt_identity: [8; 32],
                    rejoin_plan_fingerprint: [14; 32],
                    disposition_tag: 0,
                },
            ),
        )
        .unwrap();
    replay
        .observe(
            6,
            record(
                operation,
                "rejoin-completed",
                OperationalControlRecordKind::OldPrimaryRejoinCompleted {
                    rejoin_plan_fingerprint: [14; 32],
                    rejoin_receipt_identity: [15; 32],
                    forensic_retention_identity: [16; 32],
                    rebootstrap_target_identity: [0; 32],
                    disposition_tag: 0,
                },
            ),
        )
        .unwrap();

    let finished = replay.finish().unwrap();
    let handle = &finished.replica_promotions[0];
    assert_eq!(
        handle.publication().unwrap().publication_identity(),
        [11; 32]
    );
    assert_eq!(
        handle.readmission().unwrap().serve_lease_identity(),
        [13; 32]
    );
    assert_eq!(handle.rejoin_plan_fingerprint(), Some([14; 32]));
    let completed = handle.completed_rejoin().unwrap();
    assert_eq!(completed.receipt_identity(), [15; 32]);
    assert_eq!(completed.forensic_retention_identity(), Some([16; 32]));
}

#[test]
fn promotion_readmission_cannot_bind_a_foreign_publication() {
    let operation = operation("promotion-foreign-publication");
    let mut replay = SelectedControlReplay::new(OperationalControlReplayBudget::default());
    replay
        .observe(0, authorization(operation.clone(), 11))
        .unwrap();
    replay
        .observe(
            1,
            record(operation.clone(), "fence", promotion_fence_kind()),
        )
        .unwrap();
    replay
        .observe(
            2,
            record(
                operation.clone(),
                "promotion",
                OperationalControlRecordKind::ReplicaPromotionRecorded {
                    authorization_plan_fingerprint: [2; 32],
                    execution_plan_fingerprint: [3; 32],
                    receipt_identity: [8; 32],
                    fence_identity: [7; 32],
                    promoted_epoch: 12,
                },
            ),
        )
        .unwrap();
    replay
        .observe(
            3,
            record(
                operation.clone(),
                "publication",
                OperationalControlRecordKind::ReplicaPromotionPublished {
                    receipt_identity: [8; 32],
                    verification_identity: [10; 32],
                    publication_identity: [11; 32],
                    target_identity: [12; 32],
                    promoted_epoch: 12,
                },
            ),
        )
        .unwrap();
    let denial = replay
        .observe(
            4,
            record(
                operation,
                "foreign-readmission",
                OperationalControlRecordKind::ReplicaPromotionReadmitted {
                    publication_identity: [99; 32],
                    serve_lease_identity: [13; 32],
                    serving_epoch: 12,
                },
            ),
        )
        .unwrap_err();
    assert!(
        matches!(denial, SelectedControlReplayDenial::Invalid(ref violation)
        if violation.kind() == &OperationalControlHistoryViolationKind::ReplicaOperationBindingMismatch)
    );
}

#[test]
fn promotion_receipt_without_a_durable_fence_is_indeterminate_history() {
    let operation = operation("promotion-skipped-fence");
    let mut replay = SelectedControlReplay::new(OperationalControlReplayBudget::default());
    replay
        .observe(0, authorization(operation.clone(), 11))
        .expect("promotion authorization should open a recovery journal");
    let denial = replay
        .observe(
            1,
            record(
                operation,
                "forged-promotion-receipt",
                OperationalControlRecordKind::ReplicaPromotionRecorded {
                    authorization_plan_fingerprint: [2; 32],
                    execution_plan_fingerprint: [3; 32],
                    receipt_identity: [8; 32],
                    fence_identity: [7; 32],
                    promoted_epoch: 12,
                },
            ),
        )
        .expect_err("promotion must not skip durable fence persistence");
    assert!(matches!(
        denial,
        SelectedControlReplayDenial::Invalid(ref violation)
            if violation.kind()
                == &OperationalControlHistoryViolationKind::ReplicaPromotionBeforeFence
    ));
}

fn authorization(
    operation_id: OperationalOperationId,
    operation_tag: u8,
) -> OperationalControlRecord {
    record(
        operation_id,
        "authorization",
        OperationalControlRecordKind::AuthorizationConsumed {
            authorization_identity: [1; 32],
            plan_fingerprint: [2; 32],
            operation_tag,
            execution_plan_fingerprint: Some([3; 32]),
            assertion_identity: [9; 32],
            expires_at: 100,
            replay_same_operation_identity: true,
        },
    )
}

fn promotion_fence_kind() -> OperationalControlRecordKind {
    OperationalControlRecordKind::ReplicaPromotionFenceRecorded {
        authorization_plan_fingerprint: [2; 32],
        execution_plan_fingerprint: [3; 32],
        fence_identity: [7; 32],
        promoted_epoch: 12,
    }
}

fn record(
    operation_id: OperationalOperationId,
    transition: &str,
    kind: OperationalControlRecordKind,
) -> OperationalControlRecord {
    OperationalControlRecord::from_persisted_parts(
        worth_store_authority::StoreCurrentAuthorityIdentity::from_persisted_fingerprint([9; 32]),
        operation_id,
        OperationalTransitionId::new(transition).expect("transition should be valid"),
        kind,
    )
}

fn operation(value: &str) -> OperationalOperationId {
    OperationalOperationId::new(value).expect("operation should be valid")
}
