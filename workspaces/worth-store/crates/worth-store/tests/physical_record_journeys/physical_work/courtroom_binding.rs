use std::num::NonZeroU32;

use sha2::{Digest, Sha256};
use worth_store::physical_runtime::{
    CertificationFrameReadFailure, CertificationFrameWorkFailure, LifecycleGeneration,
    PhysicalDirtyTransitionFailure, PhysicalSignalSettlementOutcome, PhysicalWorkArtifactBinding,
    PhysicalWorkCourtroomFinding, PhysicalWorkCourtroomRunBinding, PhysicalWorkEffectFate,
    PhysicalWorkEffectFateEvidence, PhysicalWorkEvidenceDigest, PhysicalWorkExecutionContext,
    PhysicalWorkOracleEvidence, PhysicalWorkPreEffectDenial, PhysicalWorkProcessEvidence,
    PhysicalWorkRecoveryDisposition, PhysicalWorkRunEnvironmentEvidence, PhysicalWorkScheduleSeed,
    PhysicalWorkSourceBinding, PhysicalWorkWorkloadSeed, PhysicalWritebackExecution,
    RuntimeIdentity, ServingPhysicalRuntime,
};
use worth_store_physical_backend::{ArtifactRangeWriteDurabilityRequirement, MediaOperationRole};
use worth_store_physical_format::{
    store_namespace::StableStoreIdentity, RecordArtifactFile, RecordFrameCoordinate,
};

use super::{serving_from_initialization_with_work_profile, work_fixture};

const EXPECTED_WRITEBACK: [u8; 8] = [0xc6, 0x51, 0x16, 0xa5, 0x3c, 0x7e, 0x91, 0x42];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ExpectedCourtroomIdentity {
    store: StableStoreIdentity,
    runtime: RuntimeIdentity,
    generation: LifecycleGeneration,
}

#[test]
fn courtroom_binding_joins_exact_writeback_and_lifecycle_fencing() {
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("store");
    let (profile, _, _) = work_fixture();
    let serving = serving_from_initialization_with_work_profile(&root, profile);
    let filesystem = serving.observer().media_snapshot().unwrap();
    let environment = super::courtroom_environment::for_test(
        filesystem.backend_profile(),
        "physical_work::courtroom_binding::courtroom_binding_joins_exact_writeback_and_lifecycle_fencing",
    );
    let expected = ExpectedCourtroomIdentity {
        store: serving.store_identity(),
        runtime: serving.runtime_identity(),
        generation: serving.residency_observation().store_generation(),
    };
    let evidence_binding = serving.certification_physical_work_courtroom_binding();
    let media_before = serving.media_counters();

    prove_exact_writeback(&root, &serving);
    let media_after = serving.media_counters();
    let residency = serving.certification_physical_residency();
    prove_close_fencing(serving, &residency);
    prove_bound_courtroom_evidence(
        evidence_binding,
        expected,
        &root,
        environment,
        media_before,
        media_after,
    );
}

#[test]
fn dirty_admission_rejects_a_foreign_store_lease() {
    let parent = tempfile::tempdir().unwrap();
    let (first_profile, _, _) = work_fixture();
    let (second_profile, _, _) = work_fixture();
    let first =
        serving_from_initialization_with_work_profile(&parent.path().join("first"), first_profile);
    let second = serving_from_initialization_with_work_profile(
        &parent.path().join("second"),
        second_profile,
    );
    let first_residency = first.certification_physical_residency();
    let second_residency = second.certification_physical_residency();
    let coordinate =
        RecordFrameCoordinate::new(RecordArtifactFile::BootstrapCatalog, 8, 8).unwrap();
    let foreign = first_residency.pin_exact(coordinate).unwrap();
    let bytes = foreign.bytes().to_vec();
    let writes_before = second
        .media_counters()
        .attempts_for(MediaOperationRole::PositionedWrite);

    assert!(matches!(
        second_residency.admit_dirty_frame(foreign, move |_, target| {
            target.copy_from_slice(&bytes);
        }),
        Err(PhysicalDirtyTransitionFailure::StaleOrForeignFrame)
    ));
    assert_eq!(
        second
            .media_counters()
            .attempts_for(MediaOperationRole::PositionedWrite),
        writes_before
    );
    assert!(!first.close().residency().requires_inspection());
    assert!(!second.close().residency().requires_inspection());
}

fn prove_exact_writeback(root: &std::path::Path, serving: &ServingPhysicalRuntime) {
    let coordinate =
        RecordFrameCoordinate::new(RecordArtifactFile::BootstrapCatalog, 8, 8).unwrap();
    let residency = serving.certification_physical_residency();
    let lease = residency.pin_exact(coordinate).unwrap();
    let dirty = residency
        .admit_dirty_frame(lease, |_, target| {
            target.copy_from_slice(&EXPECTED_WRITEBACK);
        })
        .unwrap();
    let prepared = residency
        .prepare_writeback(
            dirty,
            ArtifactRangeWriteDurabilityRequirement::BufferedWrite,
        )
        .unwrap();
    let write_identity = prepared.identity();
    let ready = residency.request_writeback(prepared).unwrap();
    assert_eq!(ready.identity(), write_identity);
    let admitted = residency.admit_writeback(ready).unwrap();
    assert_eq!(admitted.identity(), write_identity);
    let before = serving.media_counters();
    let settlement = match residency.execute_writeback(admitted).unwrap() {
        PhysicalWritebackExecution::Clean(settlement) => settlement,
        PhysicalWritebackExecution::Retryable(_) => {
            panic!("unfaulted writeback unexpectedly required retry")
        }
        PhysicalWritebackExecution::InspectionRequired(_) => {
            panic!("unfaulted writeback unexpectedly required inspection")
        }
    };
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
    residency: &worth_store::physical_runtime::PhysicalResidencyCertification,
) {
    let shutdown = serving.close();
    assert!(matches!(
        residency.pin_exact(
            RecordFrameCoordinate::new(RecordArtifactFile::BootstrapCatalog, 8, 8).unwrap()
        ),
        Err(CertificationFrameReadFailure::PhysicalWork(
            CertificationFrameWorkFailure::PreEffect(PhysicalWorkPreEffectDenial::AdmissionStopped)
        ))
    ));
    assert!(!shutdown.residency().requires_inspection());
}

fn prove_bound_courtroom_evidence(
    binding: worth_store::physical_runtime::PhysicalWorkCourtroomBinding,
    expected: ExpectedCourtroomIdentity,
    root: &std::path::Path,
    environment: PhysicalWorkRunEnvironmentEvidence,
    media_before: worth_store_physical_backend::MediaCounterSnapshot,
    media_after: worth_store_physical_backend::MediaCounterSnapshot,
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

    assert_eq!(evidence.store(), expected.store.bytes());
    assert_eq!(evidence.runtime(), expected.runtime.get());
    assert_eq!(evidence.generation(), expected.generation.get());
    assert!(evidence.backend_profile().is_some());
    let causal = evidence.causal();
    assert_eq!(causal.len(), 2);
    let positioned_reads = media_after
        .identified_operation_attempts_for(MediaOperationRole::PositionedRead)
        - media_before.identified_operation_attempts_for(MediaOperationRole::PositionedRead);
    let positioned_writes = media_after
        .identified_operation_attempts_for(MediaOperationRole::PositionedWrite)
        - media_before.identified_operation_attempts_for(MediaOperationRole::PositionedWrite);
    let settled_reads = causal
        .iter()
        .filter(|record| record.effect_fate() == PhysicalWorkEffectFateEvidence::ReadCompleted)
        .count() as u64;
    let settled_writes = causal
        .iter()
        .filter(|record| record.effect_fate() == PhysicalWorkEffectFateEvidence::WriteCompleted)
        .count() as u64;
    assert_eq!(
        (positioned_reads, positioned_writes),
        (settled_reads, settled_writes),
        "MUTANT_PREDICATE:physical-work-topology-bypass"
    );
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
        .join("tests/physical_record_journeys/physical_work/courtroom_binding.rs");
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
        PhysicalWorkWorkloadSeed::new(0xc651),
        PhysicalWorkScheduleSeed::new(0xc651),
        "physical-work-writeback-close",
        [PhysicalWorkProcessEvidence::active_evidence_producer(
            "physical-work-evidence-producer",
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
