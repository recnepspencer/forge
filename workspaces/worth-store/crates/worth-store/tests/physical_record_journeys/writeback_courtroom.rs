use std::io::Write;
use std::path::Path;

use sha2::{Digest, Sha256};
use worth_foundational::{
    performance, FoundationalPerformanceAccessPatternPosture, FoundationalPerformanceBoundary,
    FoundationalPerformanceBreadthLocalityPosture, FoundationalPerformanceBudgetKind,
    FoundationalPerformanceEvidenceStrength, FoundationalPerformanceExecutionTemperature,
    FoundationalPerformanceFallbackDebtPosture, FoundationalPerformanceFreshnessRetentionPosture,
    FoundationalPerformanceWorkClass,
};
use worth_store::physical_runtime::{PhysicalScheduledWritebackOutcome, ServingPhysicalRuntime};
use worth_store_buffer_pool::BufferPoolQueueGroupingScope;
use worth_store_contracts::QueueProducerResourceShape;
use worth_store_io_scheduler::{
    admit_queue_execution_plan, admit_queue_policy_receipt, admit_secure_io_scope_for_scheduler,
    admit_security_scope_for_scheduler, lower_buffer_pool_queue_declaration,
    BackgroundResourceBudget, QueueExecutionAdmissionRequest, QueueExecutionOutcome,
    QueueExecutionReadyPlan, SecureIoOperation, SecureIoPostureRequirement,
    SecureIoPreservationRequest,
};
use worth_store_physical_backend::BackendQueueExecutionAdaptation;
use worth_store_physical_format::{RecordArtifactFile, RecordFrameCoordinate};

const RANGE_BYTES: usize = 64;

#[test]
fn physical_writeback_survives_process_exit_and_fresh_store_admission() {
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("store");
    let writer = super::run_child("c6_writeback_writer", &root, None);
    let digest = writer
        .lines()
        .find_map(|line| line.strip_prefix("C6_WRITEBACK "))
        .expect("writer must publish the predeclared digest");
    let observer = super::run_child("c6_writeback_observer", &root, Some(digest));
    assert!(observer.lines().any(|line| line == "C6_WRITEBACK_OBSERVED"));
    let reopener = super::run_child("c6_writeback_reopener", &root, None);
    assert!(reopener.lines().any(|line| line == "C6_WRITEBACK_REOPENED"));
}

pub(super) fn writer(root: &Path) {
    let serving = super::serving_from_initialization(root);
    let target = root.join("families/records/bootstrap.catalog");
    let bytes = std::fs::read(target).unwrap()[..RANGE_BYTES].to_vec();
    let coordinate =
        RecordFrameCoordinate::new(RecordArtifactFile::BootstrapCatalog, 0, RANGE_BYTES as u32)
            .unwrap();
    serving
        .certification_admit_dirty_frame(coordinate, bytes.clone())
        .unwrap();
    let outcome = serving
        .execute_scheduled_writeback(
            writeback_plan(&serving, coordinate),
            BackendQueueExecutionAdaptation::None,
        )
        .unwrap();
    assert!(matches!(
        outcome,
        PhysicalScheduledWritebackOutcome::Applied {
            execution: QueueExecutionOutcome::Executed(_),
            ..
        }
    ));
    println!("C6_WRITEBACK {}", hex(&Sha256::digest(bytes)));
    std::io::stdout().flush().unwrap();
    std::process::exit(0);
}

pub(super) fn observer(root: &Path, expected_digest: &str) {
    let bytes = std::fs::read(root.join("families/records/bootstrap.catalog")).unwrap();
    assert_eq!(hex(&Sha256::digest(&bytes[..RANGE_BYTES])), expected_digest);
    println!("C6_WRITEBACK_OBSERVED");
    std::io::stdout().flush().unwrap();
}

pub(super) fn reopener(root: &Path) {
    let serving = super::serving_from_open(root);
    assert!(!serving.observed_non_authoritative_residue());
    println!("C6_WRITEBACK_REOPENED");
    std::io::stdout().flush().unwrap();
    serving.close();
}

fn writeback_plan(
    serving: &ServingPhysicalRuntime,
    coordinate: RecordFrameCoordinate,
) -> QueueExecutionReadyPlan {
    let reservation = worth_store_io_scheduler::foreground_reservation::
        admitted_page_write_reservation_for_certification_test();
    let security = reservation.security_scope_identity();
    let shape = QueueProducerResourceShape::new()
        .with_queue_slots(1)
        .with_bandwidth_tokens(u64::from(coordinate.length()))
        .with_write_back_windows(1)
        .with_worker_permits(1);
    let grouping = BufferPoolQueueGroupingScope::new(security);
    let declaration = serving
        .certification_writeback_declaration(coordinate, grouping, 7, shape)
        .unwrap();
    let work = lower_buffer_pool_queue_declaration(declaration, reservation).unwrap();
    let backend = serving
        .admit_physical_scheduler_capability(work.backend_requirement())
        .unwrap();
    let security_scope =
        worth_store_security::admitted_store_internal_security_scope_for_io_qos_test();
    let scope = admit_security_scope_for_scheduler(&security_scope).unwrap();
    let secure_io = admit_secure_io_scope_for_scheduler(
        SecureIoPreservationRequest::new(SecureIoOperation::WriteBack, &scope, &backend)
            .require_posture(SecureIoPostureRequirement::ScopePreserving),
    )
    .unwrap();
    let work = work.with_secure_io_scope(secure_io);
    let policy = admit_queue_policy_receipt(work, policy_receipt(work.requested_budget())).unwrap();
    admit_queue_execution_plan(QueueExecutionAdmissionRequest::new(
        work,
        &backend,
        policy,
    ))
    .unwrap()
}

fn policy_receipt(
    budget: BackgroundResourceBudget,
) -> worth_foundational::FoundationalPolicyAdmissionReceipt {
    let claim = performance()
        .claim()
        .policy_admission()
        .boundary(FoundationalPerformanceBoundary::AuthoritativeExecution)
        .evidence_strength(FoundationalPerformanceEvidenceStrength::RuntimePolicyAdmission)
        .breadth_locality(FoundationalPerformanceBreadthLocalityPosture::DeltaBound)
        .access_pattern(FoundationalPerformanceAccessPatternPosture::PointLookup)
        .execution_temperature(FoundationalPerformanceExecutionTemperature::HotPath)
        .freshness_retention(FoundationalPerformanceFreshnessRetentionPosture::ExactBasisCurrent)
        .fallback_debt(FoundationalPerformanceFallbackDebtPosture::Verified)
        .include_work(FoundationalPerformanceWorkClass::AuthoritativeMutation)
        .exclude_work(FoundationalPerformanceWorkClass::SupportReportAssembly)
        .finish()
        .unwrap();
    let amount = |kind| match kind {
        FoundationalPerformanceBudgetKind::Breadth => {
            budget.queue_slots() + budget.worker_permits()
        }
        FoundationalPerformanceBudgetKind::Density => {
            budget.bandwidth_tokens() + budget.cache_residency_hints()
        }
        FoundationalPerformanceBudgetKind::Locality => {
            budget.read_ahead_window() + budget.write_back_window() + budget.reclaim_permits()
        }
        FoundationalPerformanceBudgetKind::FreshnessSensitive => 0,
    } as u32;
    let receipt = performance().policy_admission_receipt(claim);
    [
        FoundationalPerformanceBudgetKind::Breadth,
        FoundationalPerformanceBudgetKind::Density,
        FoundationalPerformanceBudgetKind::Locality,
    ]
    .into_iter()
    .fold(receipt, |receipt, kind| {
        receipt.budget_decision(kind, amount(kind), amount(kind))
    })
    .finish()
    .unwrap()
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}
