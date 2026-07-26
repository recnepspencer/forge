use std::num::NonZeroU32;

use sha2::{Digest, Sha256};
use worth_proof::TransitionOutcome;
use worth_store::physical_runtime::{
    C6PhysicalWorkHandoff, C6PhysicalWorkHandoffFailure, CertificationFrameReadFailure,
    CertificationFrameWorkFailure, PhysicalEffectObligation, PhysicalMutationWorkRequest,
    PhysicalReadWorkRequest, PhysicalSignalSettlementOutcome, PhysicalWorkArtifactBinding,
    PhysicalWorkCourtroomFinding, PhysicalWorkCourtroomRunBinding, PhysicalWorkEffectFate,
    PhysicalWorkEffectFateEvidence, PhysicalWorkEvidenceDigest, PhysicalWorkExecutionContext,
    PhysicalWorkOracleEvidence, PhysicalWorkPreEffectDenial, PhysicalWorkProcessEvidence,
    PhysicalWorkReadiness, PhysicalWorkRecoveryDisposition, PhysicalWorkRunEnvironmentEvidence,
    PhysicalWorkSourceBinding, PhysicalWorkSubmissionOutcome, PhysicalWorkSubmissionReceipt,
    PhysicalWorkSubmissionStale, ReadyPhysicalWork, ServingPhysicalRuntime,
};
use worth_store_contracts::QueueProducerResourceShape;
use worth_store_physical_backend::MediaOperationRole;
use worth_store_physical_format::{RecordArtifactFile, RecordFrameCoordinate};

use super::{serving_from_initialization_with_work_profile, work_fixture};

const EXPECTED_WRITEBACK: [u8; 8] = [0xc6, 0x51, 0x16, 0xa5, 0x3c, 0x7e, 0x91, 0x42];

#[test]
fn c6_handoff_joins_submission_writeback_and_lifecycle_fencing() {
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("store");
    let (profile, read_request, mutation_request) = work_fixture();
    let serving = serving_from_initialization_with_work_profile(&root, profile);
    let filesystem = serving.observer().media_snapshot().unwrap();
    let environment = super::courtroom_environment::for_test(
        filesystem.backend_profile(),
        "physical_work::c6_handoff::c6_handoff_joins_submission_writeback_and_lifecycle_fencing",
    );
    let evidence_binding = serving.certification_physical_work_courtroom_binding();
    let handoff = serving.c6_physical_work_handoff();
    let handoff_identity = handoff.identity();

    let mutation =
        prove_identity_and_cancellation(&serving, &handoff, read_request, mutation_request.clone());
    prove_exact_writeback(&root, &serving, &handoff, mutation);
    let residency_certification = serving.certification_physical_residency();
    prove_close_fencing(
        serving,
        &handoff,
        &residency_certification,
        mutation_request,
    );
    prove_bound_courtroom_evidence(evidence_binding, handoff_identity, &root, environment);
}

#[test]
fn c6_dirty_admission_rejects_a_foreign_store_lease() {
    let parent = tempfile::tempdir().unwrap();
    let (first_profile, _, _) = work_fixture();
    let (second_profile, _, mutation) = work_fixture();
    let first =
        serving_from_initialization_with_work_profile(&parent.path().join("first"), first_profile);
    let second = serving_from_initialization_with_work_profile(
        &parent.path().join("second"),
        second_profile,
    );
    let first_residency = first.certification_physical_residency();
    let second_handoff = second.c6_physical_work_handoff();
    let second_residency = second_handoff.residency_work();
    let ready = ready_from_receipt(
        &second_handoff,
        successful_receipt(second_handoff.mutation_submission().submit(mutation)),
    );
    let coordinate =
        RecordFrameCoordinate::new(RecordArtifactFile::BootstrapCatalog, 8, 8).unwrap();
    let foreign = first_residency.pin_exact(coordinate).unwrap();
    let bytes = foreign.bytes().to_vec();
    let writes_before = second
        .media_counters()
        .attempts_for(MediaOperationRole::PositionedWrite);

    assert!(matches!(
        second_residency.admit_dirty_frame(&ready, foreign, move |_, target| {
            target.copy_from_slice(&bytes);
        }),
        Err(C6PhysicalWorkHandoffFailure::StaleOrForeignIdentity)
    ));
    assert_eq!(
        second
            .media_counters()
            .attempts_for(MediaOperationRole::PositionedWrite),
        writes_before
    );
    drop(ready);
    assert!(!first.close().residency().requires_inspection());
    assert!(!second.close().residency().requires_inspection());
}

fn prove_identity_and_cancellation(
    serving: &ServingPhysicalRuntime,
    handoff: &C6PhysicalWorkHandoff,
    read_request: PhysicalReadWorkRequest,
    mutation_request: PhysicalMutationWorkRequest,
) -> PhysicalWorkSubmissionReceipt {
    assert_eq!(handoff.identity().store(), serving.store_identity());
    assert_eq!(handoff.identity().runtime(), serving.runtime_identity());
    assert_eq!(
        handoff.record_reads().store_identity(),
        serving.store_identity()
    );

    let before_cancellation = serving.media_counters();
    let read = successful_receipt(handoff.read_submission().submit(read_request));
    let mutation = successful_receipt(
        handoff
            .mutation_submission()
            .submit(mutation_request.clone()),
    );
    assert!(handoff.identity().admits(read.identity()));
    assert!(handoff.identity().admits(mutation.identity()));
    assert_ne!(read.identity(), mutation.identity());

    let ready_read = ready_from_receipt(handoff, read);
    let cancelled = handoff.cancel_work(ready_read.consumer_handle()).unwrap();
    assert_eq!(
        cancelled.obligation(),
        PhysicalEffectObligation::NotDispatched
    );
    drop(ready_read);
    assert_eq!(serving.media_counters(), before_cancellation);
    mutation
}

fn prove_exact_writeback(
    root: &std::path::Path,
    serving: &ServingPhysicalRuntime,
    handoff: &C6PhysicalWorkHandoff,
    mutation: PhysicalWorkSubmissionReceipt,
) {
    let ready_write = ready_from_receipt(handoff, mutation);
    let coordinate =
        RecordFrameCoordinate::new(RecordArtifactFile::BootstrapCatalog, 8, 8).unwrap();
    let residency = handoff.residency_work();
    let lease = serving
        .certification_physical_residency()
        .pin_exact(coordinate)
        .unwrap();
    let dirty = residency
        .admit_dirty_frame(&ready_write, lease, |_, target| {
            target.copy_from_slice(&EXPECTED_WRITEBACK);
        })
        .unwrap();
    assert_eq!(dirty.identity(), ready_write.intent().identity());
    let reservation = residency.reserve_writeback(&ready_write, &dirty).expect(
        "C5_PREDICATE:c6-local-scheduler canonical C.6 writeback progression \
             cannot be intercepted by a C.6-local pending registry",
    );
    assert_eq!(reservation.identity(), ready_write.intent().identity());
    let prepared = residency
        .prepare_writeback(ready_write, reservation, 7, writeback_shape())
        .unwrap();
    let write_identity = prepared.identity();
    let admitted = residency.admit_writeback(prepared, dirty).unwrap();
    assert_eq!(admitted.identity(), write_identity);
    let before = serving.media_counters();
    let settlement = residency
        .execute_writeback(admitted)
        .unwrap()
        .settled()
        .expect("unfaulted C.6 writeback must settle");
    let after = serving.media_counters();
    assert_eq!(settlement.identity(), write_identity);
    assert_eq!(
        settlement.effect_fate(),
        PhysicalWorkEffectFate::WriteCompleted
    );
    assert_eq!(
        settlement.recovery(),
        PhysicalWorkRecoveryDisposition::ContinueSettlement
    );
    assert_eq!(
        settlement.signal(),
        PhysicalSignalSettlementOutcome::Committed
    );
    assert!(settlement.effect().is_some());
    assert_eq!(
        after.attempts_for(MediaOperationRole::PositionedWrite)
            - before.attempts_for(MediaOperationRole::PositionedWrite),
        1
    );
    assert_eq!(residency.counters().dirty_frames(), 0);
    assert_eq!(
        &std::fs::read(root.join("families/records/bootstrap.catalog")).unwrap()[8..16],
        EXPECTED_WRITEBACK
    );
}

fn prove_close_fencing(
    serving: ServingPhysicalRuntime,
    handoff: &C6PhysicalWorkHandoff,
    residency_certification: &worth_store::physical_runtime::PhysicalResidencyCertification,
    mutation_request: PhysicalMutationWorkRequest,
) {
    let stale_submission = handoff.mutation_submission();
    let stale_request = mutation_request.clone();
    let _stale_ready = ready_from_receipt(
        handoff,
        successful_receipt(stale_submission.submit(mutation_request)),
    );
    let shutdown = serving.close();
    assert_eq!(shutdown.records().counters().reader_acquisitions(), 1);
    assert_eq!(shutdown.records().counters().readers_live(), 1);
    assert!(matches!(
        stale_submission.submit(stale_request).into_raw(),
        TransitionOutcome::Stale(PhysicalWorkSubmissionStale::OwnerReleased)
    ));
    assert!(matches!(
        residency_certification.pin_exact(
            RecordFrameCoordinate::new(RecordArtifactFile::BootstrapCatalog, 8, 8).unwrap()
        ),
        Err(CertificationFrameReadFailure::PhysicalWork(
            CertificationFrameWorkFailure::PreEffect(PhysicalWorkPreEffectDenial::AdmissionStopped)
        ))
    ));
    assert!(matches!(
        handoff.recovery_obligations(),
        Err(C6PhysicalWorkHandoffFailure::RuntimeReleased)
    ));
}

fn prove_bound_courtroom_evidence(
    binding: worth_store::physical_runtime::PhysicalWorkCourtroomBinding,
    identity: worth_store::physical_runtime::C6PhysicalWorkHandoffIdentity,
    root: &std::path::Path,
    environment: PhysicalWorkRunEnvironmentEvidence,
) {
    let artifact_path = root.join("families/records/bootstrap.catalog");
    let artifact_bytes = std::fs::read(&artifact_path).unwrap();
    let artifact = PhysicalWorkArtifactBinding::new(
        artifact_path.display().to_string(),
        artifact_bytes.len() as u64,
        digest(&artifact_bytes),
    )
    .unwrap();
    let oracle = PhysicalWorkOracleEvidence::new(
        "independent-exact-writeback-bytes",
        artifact_bytes[8..16] == EXPECTED_WRITEBACK,
        digest(&EXPECTED_WRITEBACK),
    )
    .unwrap();
    let evidence = binding
        .finish(run_binding(environment), [artifact], oracle, [])
        .unwrap();

    assert_eq!(evidence.store(), identity.store().bytes());
    assert_eq!(evidence.runtime(), identity.runtime().get());
    assert_eq!(evidence.generation(), identity.generation().get());
    assert!(evidence.backend_profile().is_some());
    let causal = evidence.causal();
    assert_eq!(causal.len(), 2);
    assert_eq!(
        causal[0].effect_fate(),
        PhysicalWorkEffectFateEvidence::ReadCompleted
    );
    assert_eq!(
        causal[1].effect_fate(),
        PhysicalWorkEffectFateEvidence::WriteCompleted
    );
    assert!(causal
        .iter()
        .all(|record| record.counters().len() == 4 * 6 * 7));
    assert_eq!(
        evidence.verdict().findings(),
        &[PhysicalWorkCourtroomFinding::MissingMutantLocalization]
    );
}

fn run_binding(environment: PhysicalWorkRunEnvironmentEvidence) -> PhysicalWorkCourtroomRunBinding {
    let source_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/physical_record_journeys/physical_work/c6_handoff.rs");
    let binary_path = std::env::current_exe().unwrap();
    let source = PhysicalWorkSourceBinding::new(
        source_path.display().to_string(),
        digest(&std::fs::read(&source_path).unwrap()),
    )
    .unwrap();
    let binary = PhysicalWorkSourceBinding::new(
        binary_path.display().to_string(),
        digest(&std::fs::read(binary_path).unwrap()),
    )
    .unwrap();
    let execution = PhysicalWorkExecutionContext::new(
        0xc651,
        "c6-handoff-writeback-close",
        [PhysicalWorkProcessEvidence::active_evidence_producer(
            "c6-handoff-evidence-producer",
            NonZeroU32::new(std::process::id()).unwrap(),
        )
        .unwrap()],
    )
    .unwrap();
    PhysicalWorkCourtroomRunBinding::new(source, binary, execution, environment)
}

fn digest(bytes: &[u8]) -> PhysicalWorkEvidenceDigest {
    PhysicalWorkEvidenceDigest::new(Sha256::digest(bytes).into()).unwrap()
}

fn writeback_shape() -> QueueProducerResourceShape {
    QueueProducerResourceShape::new()
        .with_queue_slots(1)
        .with_bandwidth_tokens(8)
        .with_write_back_windows(1)
        .with_worker_permits(1)
}

fn successful_receipt(outcome: PhysicalWorkSubmissionOutcome) -> PhysicalWorkSubmissionReceipt {
    match outcome.into_raw() {
        TransitionOutcome::Success(receipt) => receipt,
        other => panic!("C.6 handoff submission should succeed: {other:?}"),
    }
}

fn ready_from_receipt(
    handoff: &C6PhysicalWorkHandoff,
    receipt: PhysicalWorkSubmissionReceipt,
) -> ReadyPhysicalWork {
    let admitted = handoff.admit_submitted_work(receipt).unwrap();
    match handoff.request_work(admitted).unwrap() {
        PhysicalWorkReadiness::Ready(ready) => ready,
        PhysicalWorkReadiness::Blocked(blocked) => {
            panic!(
                "C.6 handoff work unexpectedly blocked: {:?}",
                blocked.condition()
            )
        }
    }
}
