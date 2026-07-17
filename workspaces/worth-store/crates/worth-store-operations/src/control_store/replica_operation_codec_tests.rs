use super::{
    decode_control_record, encode_control_record, OperationalControlRecord,
    OperationalControlRecordKind, OperationalOperationId, OperationalTransitionId,
};

#[test]
fn replica_transition_codec_preserves_the_crash_recovery_evidence() {
    for (transition, kind) in [
        (
            "fence",
            OperationalControlRecordKind::ReplicaPromotionFenceRecorded {
                authorization_plan_fingerprint: [2; 32],
                execution_plan_fingerprint: [3; 32],
                fence_identity: [7; 32],
                promoted_epoch: 12,
            },
        ),
        (
            "rejoin-completed",
            OperationalControlRecordKind::OldPrimaryRejoinCompleted {
                rejoin_plan_fingerprint: [8; 32],
                rejoin_receipt_identity: [9; 32],
                forensic_retention_identity: [10; 32],
                rebootstrap_target_identity: [11; 32],
                disposition_tag: 2,
            },
        ),
    ] {
        let original = OperationalControlRecord::from_persisted_parts(
            worth_store_authority::StoreCurrentAuthorityIdentity::from_persisted_fingerprint(
                [9; 32],
            ),
            OperationalOperationId::new("replica-codec").expect("valid operation"),
            OperationalTransitionId::new(transition).expect("valid transition"),
            kind,
        );
        let encoded = encode_control_record(&original).expect("record should encode");
        let decoded = decode_control_record(&encoded)
            .expect("record should decode")
            .into_domain(|_| panic!("replica records have no detached recovery object"))
            .expect("record should return to its domain form");
        assert_eq!(decoded.kind(), original.kind());
    }
}
