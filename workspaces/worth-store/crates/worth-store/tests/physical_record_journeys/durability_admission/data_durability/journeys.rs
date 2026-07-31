use std::collections::BTreeSet;
use std::fs;

use super::super::super::{configuration, serving_from_initialization};
use super::mutation_world::{append, synchronize};
use super::oracle::{
    assert_encoded_wal_matches_claims, assert_targets_absent, frame_page_lsn_matches_basis,
    verify_effect,
};
use worth_store::physical_runtime::{
    CertifiedPriorPageImage, PhysicalDataDispatchOutcome, PhysicalDataEffectSource,
    PhysicalDataFrameSubject, PhysicalDataSettlementOutcome, PhysicalMutationIdempotencyMaterial,
    RecordAppendBatch,
};
use worth_store_physical_format::{encode_data_frame_page_lsn, DurableFrameKind, PhysicalPageLsn};

#[test]
fn materialized_inline_prior_advances_through_exact_wal_bound_copy_on_write() {
    let parent = tempfile::tempdir().unwrap();
    let store_root = parent.path().join("store");
    let serving = serving_from_initialization(&store_root);
    let (format, placement, _) = configuration();
    let submission = serving.record_submission();
    submission
        .append_batch(
            RecordAppendBatch::try_from_iter([b"published-prior".as_slice()]).unwrap(),
            placement,
        )
        .expect("the real ordinary path must seed one published prior page");
    let source_path = only_data_artifact(&store_root, "segments");
    let source_before = fs::read(&source_path).unwrap();

    let appended = append(
        &submission,
        placement,
        PhysicalMutationIdempotencyMaterial::new([101; 32]),
        RecordAppendBatch::try_from_iter([b"copy-on-write-delta".as_slice()]).unwrap(),
    );
    assert_encoded_wal_matches_claims(&store_root, &appended);
    assert_targets_absent(&store_root, &appended);
    let member = appended.reserved().member_basis();
    let durable = synchronize(&submission, appended);
    let dispatched = match submission.dispatch_wal_durable_data(durable) {
        PhysicalDataDispatchOutcome::Dispatched(dispatched) => dispatched,
        _ => panic!("the exact WAL-durable copy-on-write image must dispatch"),
    };
    assert_eq!(dispatched.effects().len(), 1);
    let effect = &dispatched.effects()[0];
    assert_eq!(effect.source(), PhysicalDataEffectSource::NewArtifact);
    let prior = effect.basis().prior();
    assert!(matches!(
        prior.image(),
        CertifiedPriorPageImage::MaterializedSource(_)
    ));
    assert_eq!(effect.basis().delta().len(), 1);
    let target_bytes = verify_effect(&store_root, format.declaration(), member, effect);
    let mut page_lsn_ahead = target_bytes.clone();
    encode_data_frame_page_lsn(
        &mut page_lsn_ahead,
        DurableFrameKind::InlinePage,
        PhysicalPageLsn::new(effect.basis().resulting_page_lsn().get() + 1),
    )
    .expect("controlled pageLSN-ahead frame remains structurally valid");
    assert!(
        !frame_page_lsn_matches_basis(effect, &page_lsn_ahead),
        "the independent oracle must reject a checksum-valid pageLSN-ahead mutation"
    );
    assert_ne!(target_bytes, source_before);
    assert_eq!(fs::read(&source_path).unwrap(), source_before);
    match dispatched.settle_exact_effects() {
        PhysicalDataSettlementOutcome::Settled(settled) => {
            assert_eq!(settled.mutation_identity(), member.mutation_identity())
        }
        PhysicalDataSettlementOutcome::InspectionRequired { cause, .. } => {
            panic!("exact copy-on-write effects must settle: {cause:?}")
        }
    }
    serving.close();
}

#[test]
fn multi_chunk_extent_uses_one_new_artifact_effect_then_real_c6_writebacks() {
    let parent = tempfile::tempdir().unwrap();
    let store_root = parent.path().join("store");
    let serving = serving_from_initialization(&store_root);
    let (format, placement, _) = configuration();
    let submission = serving.record_submission();
    let payload = vec![0x5a; format.declaration().page_size().bytes() as usize * 3];
    let appended = append(
        &submission,
        placement,
        PhysicalMutationIdempotencyMaterial::new([102; 32]),
        RecordAppendBatch::builder()
            .push_owned(payload)
            .build()
            .unwrap(),
    );
    assert_encoded_wal_matches_claims(&store_root, &appended);
    assert_targets_absent(&store_root, &appended);
    let member = appended.reserved().member_basis();
    let durable = synchronize(&submission, appended);
    let dispatched = match submission.dispatch_wal_durable_data(durable) {
        PhysicalDataDispatchOutcome::Dispatched(dispatched) => dispatched,
        _ => panic!("the exact WAL-durable extent must dispatch"),
    };
    assert!(
        dispatched.effects().len() >= 3,
        "the fixture must cross the real multi-chunk C6 join"
    );
    let mut artifacts = BTreeSet::new();
    for (index, effect) in dispatched.effects().iter().enumerate() {
        assert_eq!(
            effect.source(),
            if index == 0 {
                PhysicalDataEffectSource::NewArtifact
            } else {
                PhysicalDataEffectSource::C6Writeback
            }
        );
        assert!(matches!(
            effect.basis().target().subject(),
            PhysicalDataFrameSubject::ExtentChunk(_)
        ));
        assert!(matches!(
            effect.basis().prior().image(),
            CertifiedPriorPageImage::AbsentTarget(target)
                if target == effect.basis().target()
        ));
        assert_eq!(effect.basis().delta().len(), 1);
        artifacts.insert(effect.coordinate().artifact());
        verify_effect(&store_root, format.declaration(), member, effect);
    }
    assert_eq!(artifacts.len(), 1);
    match dispatched.settle_exact_effects() {
        PhysicalDataSettlementOutcome::Settled(settled) => {
            assert_eq!(settled.mutation_identity(), member.mutation_identity())
        }
        PhysicalDataSettlementOutcome::InspectionRequired { cause, .. } => {
            panic!("exact C6 writeback effects must settle: {cause:?}")
        }
    }
    serving.close();
}

fn only_data_artifact(store_root: &std::path::Path, family: &str) -> std::path::PathBuf {
    let mut artifacts = fs::read_dir(store_root.join("families").join("records").join(family))
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .collect::<Vec<_>>();
    artifacts.sort();
    assert_eq!(artifacts.len(), 1, "fixture requires one source artifact");
    artifacts.pop().unwrap()
}
