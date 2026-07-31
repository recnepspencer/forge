use std::num::NonZeroU32;

use sha2::{Digest, Sha256};
use worth_proof::TransitionOutcome;
use worth_signal::facade::TemporalDuration;
use worth_store::physical_runtime::{
    PhysicalMutationDeadline, PhysicalMutationIdempotencyMaterial, PhysicalMutationRequest,
    PhysicalRecordInitialization, PhysicalWalAppendOutcome, RecordAppendBatch,
};
use worth_store_wal::artifact_store::{
    verify_bounded_wal_segment, BoundedWalSegmentVerificationRequest,
};

use super::super::{configuration, durability_with_group_limit, media, success};
use super::independent_wal_oracle::{
    independent_canonical_redo, independent_frame_payload, independent_target_claim,
    inspect_attempt_binding, inspect_member_payload, split_member_payload, BindingField,
    BindingInspectionDenial, ExpectedAttemptBinding,
};

#[test]
fn real_wal_attempt_binding_is_independently_decoded_and_substitution_hostile() {
    let parent = tempfile::tempdir().unwrap();
    let store_root = parent.path().join("store");
    let media = media(&store_root);
    let policy = durability_with_group_limit(&media, NonZeroU32::new(32).unwrap());
    let (format, placement, access) = configuration();
    let serving = success(
        media.initialize_record_store(PhysicalRecordInitialization::new(
            format, placement, access, policy,
        )),
    );
    let submission = serving.record_submission();
    let key = submission
        .issue_idempotency_key(PhysicalMutationIdempotencyMaterial::new([88; 32]))
        .unwrap();
    let source_redo = [
        b"binding-first-redo".as_slice(),
        b"binding-second-redo".as_slice(),
    ];
    let prepared = match submission
        .prepare_durable_append(
            RecordAppendBatch::try_from_iter(source_redo).unwrap(),
            placement,
            PhysicalMutationRequest::platform_durable(
                key.clone(),
                PhysicalMutationDeadline::at(TemporalDuration::temporal_duration(1_000).unwrap()),
            ),
        )
        .into_raw()
    {
        TransitionOutcome::Success(prepared) => prepared,
        _ => panic!("binding inspection fixture must prepare"),
    };
    let appended = match submission.append_prepared_wal(prepared) {
        PhysicalWalAppendOutcome::Appended(appended) => appended,
        _ => panic!("binding inspection fixture must append"),
    };
    let reserved = appended.reserved();
    let declaration = reserved.declaration();
    let mutation = reserved.mutation_identity();
    let member = reserved.member_basis();
    let targets = reserved
        .redo()
        .records()
        .iter()
        .map(|record| {
            assert_eq!(
                record.lsn().get(),
                member.lsn_range().start().get() + u64::from(record.ordinal())
            );
            assert!(!record.targets().is_empty());
            record
                .targets()
                .iter()
                .copied()
                .map(independent_target_claim)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    assert_eq!(targets[0], targets[1]);
    let expected_redo =
        independent_canonical_redo(&source_redo, member.lsn_range().start().get(), &targets);
    let expected = ExpectedAttemptBinding {
        key: key.identity().bytes(),
        issuance: key.lease().issuance_generation().get(),
        expiry: key.lease().expiry_generation().get(),
        fingerprint: reserved.request_fingerprint().bytes(),
        store: mutation.store_identity().bytes(),
        runtime: mutation.runtime_identity().get(),
        operation: mutation.operation_identity().get(),
        member: member.member_identity().bytes(),
        lsn_start: member.lsn_range().start().get(),
        lsn_end: member.lsn_range().end_exclusive().get(),
        redo_digest: Sha256::digest(&expected_redo).into(),
    };
    assert_eq!(reserved.redo().encoded(), expected_redo);
    assert_eq!(reserved.bound_redo_digest(), expected.redo_digest);
    assert_ne!(
        independent_canonical_redo(
            &[source_redo[1], source_redo[0]],
            member.lsn_range().start().get(),
            &targets,
        ),
        expected_redo,
        "the independent oracle must preserve the declared redo order"
    );
    serving.close();

    let artifact = store_root
        .join("families")
        .join("wal")
        .join("segment-1-generation-1.wal");
    let bytes = std::fs::read(&artifact).unwrap();
    let request = BoundedWalSegmentVerificationRequest::new(
        declaration.segment().get(),
        declaration.generation().get(),
        declaration.lsn_range().start().get(),
        declaration.lsn_range().end_exclusive().get(),
        bytes.len() as u64,
        Sha256::digest(&bytes).into(),
        4 * 1024,
    )
    .unwrap();
    assert_eq!(
        verify_bounded_wal_segment(&artifact, request)
            .unwrap()
            .frame_count(),
        1
    );

    let payload = independent_frame_payload(&bytes).unwrap();
    let inspected = inspect_member_payload(payload, &expected, &expected_redo).unwrap();
    let (binding, redo) = split_member_payload(payload).unwrap();
    assert_eq!(inspected.value, expected);

    for (field, span) in &inspected.spans {
        let mut substituted = binding.to_vec();
        substituted[span.start] ^= 0x01;
        assert!(
            inspect_attempt_binding(&substituted, &expected).is_err(),
            "{field:?} substitution survived independent inspection"
        );
    }

    for field in [
        BindingField::Domain,
        BindingField::Key,
        BindingField::Fingerprint,
        BindingField::Store,
        BindingField::Member,
        BindingField::RedoDigest,
    ] {
        let span = inspected.span(field);
        let mut wrong_length = binding.to_vec();
        wrong_length[span.start - 8..span.start].copy_from_slice(&0_u64.to_le_bytes());
        assert_eq!(
            inspect_attempt_binding(&wrong_length, &expected),
            Err(BindingInspectionDenial::InvalidFieldLength(field))
        );
    }

    let mut reordered = binding.to_vec();
    let key_span = inspected.span(BindingField::Key);
    let fingerprint_span = inspected.span(BindingField::Fingerprint);
    let key_bytes = reordered[key_span.clone()].to_vec();
    let fingerprint_bytes = reordered[fingerprint_span.clone()].to_vec();
    reordered[key_span].copy_from_slice(&fingerprint_bytes);
    reordered[fingerprint_span].copy_from_slice(&key_bytes);
    assert!(inspect_attempt_binding(&reordered, &expected).is_err());

    assert!(inspect_attempt_binding(&binding[..binding.len() - 1], &expected).is_err());
    let mut trailing = binding.to_vec();
    trailing.push(0);
    assert_eq!(
        inspect_attempt_binding(&trailing, &expected),
        Err(BindingInspectionDenial::TrailingBytes)
    );

    let mut wrong_redo = redo.to_vec();
    wrong_redo[0] ^= 0x01;
    let redo_offset = payload.len() - redo.len();
    let mut wrong_payload = payload.to_vec();
    wrong_payload[redo_offset..].copy_from_slice(&wrong_redo);
    assert_eq!(
        inspect_member_payload(&wrong_payload, &expected, &expected_redo),
        Err(BindingInspectionDenial::FieldMismatch(
            BindingField::RedoPayload
        ))
    );
}
