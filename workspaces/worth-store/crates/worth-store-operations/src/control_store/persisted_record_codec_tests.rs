use worth_store_physical_backend::MAX_OPERATIONAL_CONTROL_PAYLOAD_BYTES;

use super::{
    decode_control_record, encode_control_record, OperationalControlEncodingDenial,
    OperationalControlRecord, OperationalControlRecordKind, OperationalOperationId,
    OperationalTransitionId, OperationalWorkflowKind, PersistedControlRecordDecodeDenial,
};

#[test]
fn implemented_control_records_round_trip_without_a_parallel_schema() {
    let workflows = [
        OperationalWorkflowKind::OfflineInspection,
        OperationalWorkflowKind::Backup,
        OperationalWorkflowKind::Restore,
        OperationalWorkflowKind::PointInTimeRecovery,
        OperationalWorkflowKind::Rollback,
        OperationalWorkflowKind::Repair,
        OperationalWorkflowKind::ReplicaBootstrap,
        OperationalWorkflowKind::ReplicaPromotion,
        OperationalWorkflowKind::ForensicAcquisition,
    ];
    let mut records = workflows
        .into_iter()
        .enumerate()
        .map(|(index, workflow)| {
            record(
                index,
                OperationalControlRecordKind::WorkflowOpened { workflow },
            )
        })
        .collect::<Vec<_>>();
    records.extend([
        record(
            19,
            OperationalControlRecordKind::BackupMaterializationOpened {
                plan: super::BackupMaterializationRecoveryPlan::from_persisted(
                    [0x41; 32],
                    std::env::current_dir()
                        .expect("current directory")
                        .join("materialization-target"),
                    4096,
                )
                .expect("persisted materialization plan"),
            },
        ),
        record(
            20,
            OperationalControlRecordKind::BackupMaterializationRecorded {
                manifest_digest: [0x51; 32],
            },
        ),
        record(
            44,
            OperationalControlRecordKind::BackupAbandoned {
                reason: "operator cancellation".into(),
                released_source_lease: release([0x44; 32]),
            },
        ),
        record(
            45,
            OperationalControlRecordKind::AuthorizationConsumed {
                authorization_identity: [1; 32],
                plan_fingerprint: [2; 32],
                operation_tag: 1,
                execution_plan_fingerprint: Some([7; 32]),
                assertion_identity: [3; 32],
                expires_at: 500,
                replay_same_operation_identity: false,
            },
        ),
        record(
            51,
            OperationalControlRecordKind::RecoveryStagingCompleted {
                authorization_identity: [1; 32],
                plan_fingerprint: [2; 32],
                execution_plan_fingerprint: [7; 32],
                staged_media_identity: [8; 32],
            },
        ),
        record(
            46,
            OperationalControlRecordKind::RepairExecutionOpened {
                authorization_identity: [1; 32],
                plan_fingerprint: [4; 32],
                owner_node_count: 3,
                topology_tag: 1,
            },
        ),
        record(
            47,
            OperationalControlRecordKind::RepairOwnerEffectStarted {
                plan_fingerprint: [4; 32],
                node_fingerprint: [5; 32],
                owner_tag: 2,
            },
        ),
        record(
            48,
            OperationalControlRecordKind::RepairOwnerReceiptPersisted {
                plan_fingerprint: [4; 32],
                node_fingerprint: [5; 32],
                receipt_fingerprint: [6; 32],
                owner_tag: 2,
            },
        ),
        record(
            49,
            OperationalControlRecordKind::RepairDispositionRecorded {
                plan_fingerprint: [4; 32],
                disposition_tag: 1,
                disposition_basis: [15; 32],
            },
        ),
    ]);

    for expected in records {
        let encoded = encode_control_record(&expected).expect("canonical binary record");
        let decoded = decode_control_record(&encoded)
            .expect("decode")
            .into_domain(|_| panic!("simple record cannot request a recovery object"))
            .expect("domain admission");
        assert_eq!(decoded, expected);
    }
}

#[test]
fn hostile_binary_control_records_fail_closed_at_every_structural_boundary() {
    let canonical = encode_control_record(&record(
        1,
        OperationalControlRecordKind::WorkflowOpened {
            workflow: OperationalWorkflowKind::Backup,
        },
    ))
    .expect("canonical binary record");

    for boundary in 0..canonical.len() {
        assert!(decode_control_record(&canonical[..boundary]).is_err());
    }

    let mut trailing = canonical.clone();
    trailing.push(0);
    assert!(decode_control_record(&trailing).is_err());

    let mut unknown_version = canonical.clone();
    unknown_version[7] = b'9';
    assert!(decode_control_record(&unknown_version).is_err());

    let mut unknown_kind = canonical.clone();
    *unknown_kind.last_mut().expect("kind tag") = u8::MAX;
    assert!(decode_control_record(&unknown_kind).is_err());

    let mut impossible_length = canonical.clone();
    impossible_length[40..44].copy_from_slice(&u32::MAX.to_le_bytes());
    assert!(decode_control_record(&impossible_length).is_err());

    let mut invalid_utf8 = canonical;
    invalid_utf8[44] = u8::MAX;
    assert!(decode_control_record(&invalid_utf8).is_err());

    let oversized = vec![0; MAX_OPERATIONAL_CONTROL_PAYLOAD_BYTES + 1];
    assert!(matches!(
        decode_control_record(&oversized),
        Err(PersistedControlRecordDecodeDenial::InvalidEncoding)
    ));
}

#[test]
fn control_encoder_denies_payloads_the_physical_store_cannot_append() {
    let oversized = record(
        1,
        OperationalControlRecordKind::BackupAbandoned {
            reason: "x".repeat(MAX_OPERATIONAL_CONTROL_PAYLOAD_BYTES),
            released_source_lease: release([0x51; 32]),
        },
    );
    assert!(matches!(
        encode_control_record(&oversized),
        Err(OperationalControlEncodingDenial::RecordTooLarge)
    ));
}

#[test]
fn abandonment_without_a_durable_source_release_is_not_a_valid_record() {
    let valid = record(
        3,
        OperationalControlRecordKind::BackupAbandoned {
            reason: "operator cancellation".into(),
            released_source_lease: release([0x61; 32]),
        },
    );
    let mut encoded = encode_control_record(&valid).expect("valid abandonment");
    let release_presence = encoded.len() - 41;
    encoded[release_presence] = 0;

    assert!(matches!(
        decode_control_record(&encoded),
        Err(PersistedControlRecordDecodeDenial::InvalidEncoding)
    ));
}

fn release(
    cut_identity: [u8; 32],
) -> worth_store_physical_isolation::BackupReachabilityLeaseReleaseRecord {
    let mut encoded = [0; 36];
    encoded[..4].copy_from_slice(b"WBR1");
    encoded[4..].copy_from_slice(&cut_identity);
    worth_store_physical_isolation::BackupReachabilityLeaseReleaseRecord::recover(&encoded)
        .expect("canonical release record")
}

fn record(index: usize, kind: OperationalControlRecordKind) -> OperationalControlRecord {
    OperationalControlRecord::from_persisted_parts(
        worth_store_authority::StoreCurrentAuthorityIdentity::from_persisted_fingerprint(
            [index as u8; 32],
        ),
        OperationalOperationId::new(format!("operation-{index}")).expect("operation"),
        OperationalTransitionId::new(format!("operation-{index}:transition")).expect("transition"),
        kind,
    )
}
