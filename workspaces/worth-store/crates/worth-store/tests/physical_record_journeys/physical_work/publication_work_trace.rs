use std::{
    collections::{BTreeSet, HashSet},
    sync::mpsc,
    time::Duration,
};

use worth_store::physical_runtime::{
    PhysicalWorkEffectFate, PhysicalWorkRecoveryDisposition, RecordAppendBatch, RecordAppendError,
    RecordPublicationRecoveryBasis, RecordPublicationStage, RecordStreamFailureKind,
    RecordWriteSource, RecordWriteSourceError, UnpublishedRecordBatchCause,
};

use super::{configuration, serving_from_initialization};

#[test]
fn successful_publication_exposes_each_causal_work_identity_once() {
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("store");
    let (_, placement, _) = configuration();
    let serving = serving_from_initialization(&root);
    let payload = vec![0xA5; 20_000];

    let published = serving
        .record_submission()
        .append_batch(
            RecordAppendBatch::try_from_iter([payload.as_slice()]).unwrap(),
            placement,
        )
        .unwrap();

    let effects = published.physical_work().effects();
    let work = effects
        .iter()
        .map(|effect| effect.identity())
        .collect::<Vec<_>>();
    assert!(!work.is_empty());
    assert_eq!(
        work.iter().copied().collect::<HashSet<_>>().len(),
        work.len(),
        "a publication trace must contain each physical effect identity once"
    );
    let stages = effects
        .iter()
        .map(|effect| effect.stage())
        .collect::<Vec<_>>();
    for required in [
        RecordPublicationStage::CandidateDataWrite,
        RecordPublicationStage::DataSynchronization,
        RecordPublicationStage::PayloadManifestSynchronization,
        RecordPublicationStage::ManifestSynchronization,
        RecordPublicationStage::CatalogCandidateSynchronization,
        RecordPublicationStage::CatalogReplacement,
        RecordPublicationStage::NamespaceSynchronization,
    ] {
        assert!(
            stages.contains(&required),
            "C5_PREDICATE:publication-stage: missing {required:?}"
        );
    }
    assert!(work
        .iter()
        .all(|identity| identity.store() == serving.store_identity()));
    let causal = serving.physical_work_observer().causal().records();
    let publication_causal = causal
        .iter()
        .filter(|record| work.contains(&record.identity()))
        .collect::<Vec<_>>();
    assert_eq!(
        publication_causal.len(),
        work.len(),
        "C5_PREDICATE:settlement: every publication effect needs causal settlement"
    );
    let mut saw_publication = false;
    let mut signal_requests = BTreeSet::new();
    for identity in &work {
        let matches = publication_causal
            .iter()
            .filter(|record| record.identity() == *identity)
            .collect::<Vec<_>>();
        assert_eq!(
            matches.len(),
            1,
            "C5_PREDICATE:settlement: each publication effect needs one causal settlement"
        );
        assert!(
            signal_requests.insert(matches[0].signal_request()),
            "C5_PREDICATE:signal-readiness: each effect needs its own Signal request"
        );
        let scheduler = matches[0].scheduler_binding();
        assert_eq!(
            scheduler.secondary(),
            None,
            "C5_PREDICATE:scheduler-admission: publication effects retain one primary plan"
        );
        assert_eq!(
            scheduler.grouped_writes(),
            0,
            "C5_PREDICATE:scheduler-admission: ordinary publication effects remain ungrouped"
        );
        assert!(
            matches[0].backend_operation().is_some(),
            "C5_PREDICATE:backend-dispatch: every traced publication effect reaches media"
        );
        assert!(
            matches[0].derived_completion().is_some(),
            "C5_PREDICATE:derived-completion: settlement must advance Signal"
        );
        match matches[0].effect_fate() {
            PhysicalWorkEffectFate::WriteCompleted => {}
            PhysicalWorkEffectFate::PublicationCompleted => saw_publication = true,
            fate => panic!("successful publication retained non-successful work fate: {fate:?}"),
        }
        assert_eq!(
            matches[0].recovery(),
            PhysicalWorkRecoveryDisposition::ContinueSettlement
        );
    }
    assert!(saw_publication);
    assert!(!serving.close_plan().execute().requires_inspection());
}

#[test]
fn partial_payload_failure_keeps_preparation_basis_after_disjoint_root_advances() {
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("store");
    let (_, placement, _) = configuration();
    let serving = serving_from_initialization(&root);
    let (entered_tx, entered_rx) = mpsc::sync_channel(1);
    let (release_tx, release_rx) = mpsc::sync_channel(1);
    let failing = serving
        .record_submission()
        .prepare_append(
            RecordAppendBatch::builder()
                .push_source(PausedOverlongExtentSource {
                    declared: 20_000,
                    emitted: 0,
                    entered: entered_tx,
                    release: release_rx,
                })
                .build()
                .unwrap(),
            placement,
        )
        .unwrap();
    let failing_thread = std::thread::spawn(move || failing.publish());
    entered_rx
        .recv_timeout(Duration::from_secs(3))
        .expect("the overlong probe must pause after settled payload chunks");

    let disjoint = serving
        .record_submission()
        .append_batch(
            RecordAppendBatch::try_from_iter([b"disjoint publication".as_slice()]).unwrap(),
            placement,
        )
        .unwrap();
    assert_eq!(disjoint.root_generation(), 2);
    release_tx.send(()).unwrap();

    let RecordAppendError::Unpublished(failure) = failing_thread.join().unwrap().unwrap_err()
    else {
        panic!("settled payload followed by an overlong source must remain unpublished")
    };
    let UnpublishedRecordBatchCause::Stream(stream) = failure.cause() else {
        panic!("the publication failure must retain its stream cause")
    };
    assert_eq!(
        stream.kind(),
        RecordStreamFailureKind::SourceExceededDeclaredLength
    );
    assert_eq!(
        failure.recovery_locator().basis(),
        RecordPublicationRecoveryBasis::Preparation { root_generation: 1 }
    );
    assert_eq!(failure.recovery_locator().candidate_root_generation(), None);
    assert!(!failure.physical_work().effects().is_empty());
    let disjoint_work = disjoint
        .physical_work()
        .effects()
        .iter()
        .map(|effect| effect.identity())
        .collect::<Vec<_>>();
    assert!(failure
        .physical_work()
        .effects()
        .iter()
        .all(|effect| !disjoint_work.contains(&effect.identity())));
    assert!(serving.close_plan().execute().requires_inspection());
}

struct PausedOverlongExtentSource {
    declared: u64,
    emitted: u64,
    entered: mpsc::SyncSender<()>,
    release: mpsc::Receiver<()>,
}

impl RecordWriteSource for PausedOverlongExtentSource {
    fn declared_length(&self) -> u64 {
        self.declared
    }

    fn read_next(&mut self, target: &mut [u8]) -> Result<usize, RecordWriteSourceError> {
        if self.emitted < self.declared {
            let remaining = usize::try_from(self.declared - self.emitted).unwrap();
            let length = remaining.min(target.len());
            target[..length].fill(0xA5);
            self.emitted += length as u64;
            return Ok(length);
        }
        self.entered.send(()).unwrap();
        self.release.recv().unwrap();
        target[0] = 0x5A;
        Ok(1)
    }
}
