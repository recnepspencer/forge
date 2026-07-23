use super::*;
use worth_foundational::{
    performance, FoundationalPerformanceAccessPatternPosture, FoundationalPerformanceBoundary,
    FoundationalPerformanceBreadthLocalityPosture, FoundationalPerformanceBudgetKind,
    FoundationalPerformanceEvidenceStrength, FoundationalPerformanceExecutionTemperature,
    FoundationalPerformanceFallbackDebtPosture, FoundationalPerformanceFreshnessRetentionPosture,
    FoundationalPerformanceWorkClass,
};
use worth_store_buffer_pool::{
    BufferPoolQueueExecutionDeclaration, BufferPoolQueueGroupingScope, PhysicalFrameKey,
    PhysicalResidencyLimits, PhysicalResidencyPool,
};
use worth_store_contracts::QueueProducerResourceShape;
use worth_store_io_scheduler::{
    admit_backend_capability_for_scheduler_claim, admit_queue_execution_plan,
    admit_queue_policy_receipt,
    admit_secure_io_scope_for_scheduler, admit_security_scope_for_scheduler,
    lower_buffer_pool_queue_declaration, BackgroundResourceBudget, QueueExecutionAdmissionRequest,
    QueueExecutionOutcome, SecureIoOperation, SecureIoPostureRequirement,
    SecureIoPreservationRequest,
};
use worth_store_physical_backend::{
    BackendCapabilityAdmissionRequest, BackendCapabilityEvidenceBasis, BackendCapabilitySupportSet,
    BackendMediaAssumptionSet, BackendQueueExecutionAdaptation, BackendRebindTriggers,
    BackendTargetProfile, FilesystemAccessPosture, MediaFaultDirective, MediaOperationRole,
    PhysicalBackendCapabilityAdmissionAuthority,
};
use worth_store_physical_format::{
    store_namespace::{ProposedStoreIdentity, StoreNamespaceIdentityRecord, StoreNamespaceVersion},
    RecordArtifactFile, RecordFrameCoordinate,
};

#[test]
fn exact_backend_completion_applies_to_the_same_claimed_pool_frame() {
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("writeback-store");
    let serving = initialized_store(&root, None);
    let coordinate =
        RecordFrameCoordinate::new(RecordArtifactFile::BootstrapCatalog, 0, 64).unwrap();
    serving
        .certification_admit_dirty_frame(coordinate, vec![19; 64])
        .unwrap();
    let plan = admitted_runtime_writeback_plan(&serving, coordinate);
    let outcome = serving
        .execute_scheduled_writeback(plan, BackendQueueExecutionAdaptation::None)
        .unwrap();
    assert!(matches!(
        outcome,
        PhysicalScheduledWritebackOutcome::Applied {
            execution: QueueExecutionOutcome::Executed(_),
            ..
        }
    ));
    assert_eq!(
        serving.certification_read_artifact_range(coordinate),
        [19; 64]
    );
    let counters = serving.residency_counters();
    assert_eq!(counters.dirty_frames(), 0);
    assert_eq!(counters.writebacks(), 1);
}

#[test]
fn short_physical_write_stays_dirty_and_revokes_serving_health() {
    let parent = tempfile::tempdir().unwrap();
    let prior_positioned_writes = fresh_initialization_positioned_writes(parent.path());
    let root = parent.path().join("short-write-store");
    let admission = crate::physical_runtime::FilesystemMediaAdmission::production(
        FilesystemAccessPosture::CoordinatedServiceAccount,
    );
    let authority = admission.fault_schedule_authority();
    let schedule = authority
        .schedule(vec![authority.rule(
            MediaOperationRole::PositionedWrite,
            prior_positioned_writes + 1,
            MediaFaultDirective::AllowPrefix { bytes: 17 },
        )])
        .unwrap();
    let serving = initialized_store(&root, Some(admission.with_fault_schedule(schedule)));
    let coordinate =
        RecordFrameCoordinate::new(RecordArtifactFile::BootstrapCatalog, 0, 64).unwrap();
    let before = serving.certification_read_artifact_range(coordinate);
    serving
        .certification_admit_dirty_frame(coordinate, vec![31; 64])
        .unwrap();
    let outcome = serving
        .execute_scheduled_writeback(
            admitted_runtime_writeback_plan(&serving, coordinate),
            BackendQueueExecutionAdaptation::None,
        )
        .unwrap();
    assert!(
        matches!(
            &outcome,
            PhysicalScheduledWritebackOutcome::InspectionRequired(failure)
                if failure.completed_bytes() == 17
        ),
        "unexpected short-write outcome: {outcome:?}"
    );
    let after = serving.certification_read_artifact_range(coordinate);
    assert_eq!(&after[..17], &[31; 17]);
    assert_eq!(&after[17..64], &before[17..64]);
    assert_eq!(serving.residency_counters().dirty_frames(), 1);
    assert_eq!(
        serving.close().records().posture(),
        crate::physical_runtime::RecordServingTerminalPosture::InspectionRequired
    );
}

#[test]
fn pre_effect_write_denial_releases_the_claim_for_an_exact_retry() {
    let parent = tempfile::tempdir().unwrap();
    let prior_positioned_writes = fresh_initialization_positioned_writes(parent.path());
    let root = parent.path().join("retryable-write-store");
    let admission = crate::physical_runtime::FilesystemMediaAdmission::production(
        FilesystemAccessPosture::CoordinatedServiceAccount,
    );
    let authority = admission.fault_schedule_authority();
    let schedule = authority
        .schedule(vec![authority.rule(
            MediaOperationRole::PositionedWrite,
            prior_positioned_writes + 1,
            MediaFaultDirective::FailBefore {
                kind: std::io::ErrorKind::PermissionDenied,
                raw_os_error: None,
            },
        )])
        .unwrap();
    let serving = initialized_store(&root, Some(admission.with_fault_schedule(schedule)));
    let coordinate =
        RecordFrameCoordinate::new(RecordArtifactFile::BootstrapCatalog, 0, 64).unwrap();
    let bytes = serving.certification_read_artifact_range(coordinate);
    serving
        .certification_admit_dirty_frame(coordinate, bytes)
        .unwrap();

    let first = serving
        .execute_scheduled_writeback(
            admitted_runtime_writeback_plan(&serving, coordinate),
            BackendQueueExecutionAdaptation::None,
        )
        .unwrap();
    assert!(matches!(
        first,
        PhysicalScheduledWritebackOutcome::RetryableBeforeEffect(_)
    ));
    assert_eq!(serving.residency_counters().dirty_frames(), 1);

    let retry = serving
        .execute_scheduled_writeback(
            admitted_runtime_writeback_plan(&serving, coordinate),
            BackendQueueExecutionAdaptation::None,
        )
        .unwrap();
    assert!(matches!(
        retry,
        PhysicalScheduledWritebackOutcome::Applied { .. }
    ));
    assert_eq!(serving.residency_counters().dirty_frames(), 0);
    assert_eq!(
        serving.close().records().posture(),
        crate::physical_runtime::RecordServingTerminalPosture::NoInspectionRequired
    );
}

#[test]
fn scheduler_plan_for_another_pool_cannot_consume_the_dirty_claim() {
    let store = StoreNamespaceIdentityRecord::new(
        StoreNamespaceVersion::CURRENT,
        ProposedStoreIdentity::from_nonzero_bytes([74; 16]).unwrap(),
    )
    .published_identity();
    let limits =
        PhysicalResidencyLimits::new_with_metadata_budget(256, 4096, 4, 4, 256, 4).unwrap();
    let declared_pool = PhysicalResidencyPool::open(store, limits).unwrap();
    let claimed_pool = PhysicalResidencyPool::open(store, limits).unwrap();
    let key = PhysicalFrameKey::new(
        store,
        RecordFrameCoordinate::new(RecordArtifactFile::BootstrapCatalog, 0, 64).unwrap(),
    );
    drop(claimed_pool.admit_dirty(key, vec![23; 64]).unwrap());
    let claim = claimed_pool.claim_writeback(vec![key]).unwrap();
    let declaration = writeback_declaration(&declared_pool, key);
    let plan = admitted_writeback_plan(declaration, None);

    assert_eq!(
        PhysicalScheduledWriteback::admit(claim, plan).unwrap_err(),
        PhysicalScheduledWritebackAdmissionDenial::PoolIncarnationMismatch
    );
    assert!(claimed_pool.claim_writeback(vec![key]).is_ok());
}

fn admitted_runtime_writeback_plan(
    serving: &crate::physical_runtime::ServingPhysicalRuntime,
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
    admitted_writeback_plan(declaration, Some(serving))
}

fn writeback_declaration(
    pool: &PhysicalResidencyPool,
    key: PhysicalFrameKey,
) -> BufferPoolQueueExecutionDeclaration {
    let reservation = worth_store_io_scheduler::foreground_reservation::
        admitted_page_write_reservation_for_certification_test();
    let security = reservation.security_scope_identity();
    let shape = QueueProducerResourceShape::new()
        .with_queue_slots(1)
        .with_bandwidth_tokens(u64::from(key.coordinate().length()))
        .with_write_back_windows(1)
        .with_worker_permits(1);
    let grouping = BufferPoolQueueGroupingScope::new(security);
    BufferPoolQueueExecutionDeclaration::write_back(pool, key, grouping, 7, shape).unwrap()
}

fn admitted_writeback_plan(
    declaration: BufferPoolQueueExecutionDeclaration,
    serving: Option<&crate::physical_runtime::ServingPhysicalRuntime>,
) -> QueueExecutionReadyPlan {
    let reservation = worth_store_io_scheduler::foreground_reservation::
        admitted_page_write_reservation_for_certification_test();
    let work = lower_buffer_pool_queue_declaration(declaration, reservation).unwrap();
    let backend = match serving {
        Some(serving) => serving
            .admit_physical_scheduler_capability(work.backend_requirement())
            .unwrap(),
        None => admit_backend_capability_for_scheduler_claim(
            &backend_witness(),
            work.backend_requirement(),
        )
        .unwrap(),
    };
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

fn initialized_store(
    root: &std::path::Path,
    admission: Option<crate::physical_runtime::FilesystemMediaAdmission>,
) -> crate::physical_runtime::ServingPhysicalRuntime {
    use crate::physical_runtime::{
        AdmittedPhysicalRecordFormat, FilesystemMediaAdmission, PhysicalRecordAccessPolicy,
        PhysicalRecordFormatDeclaration, PhysicalRecordInitialization,
        PhysicalRecordPlacementPolicy, PhysicalRuntimeAdmission, PhysicalStore,
    };
    let runtime = PhysicalStore::admit(PhysicalRuntimeAdmission::new(root).unwrap()).unwrap();
    let admission = admission.unwrap_or_else(|| {
        FilesystemMediaAdmission::production(FilesystemAccessPosture::CoordinatedServiceAccount)
    });
    let outcome = runtime.try_admit_filesystem_media(admission);
    let kind = outcome.kind();
    let media = match outcome.into_raw() {
        worth_proof::TransitionOutcome::Success(media) => media,
        _ => panic!("physical media admission failed: {kind:?}"),
    };
    let format = AdmittedPhysicalRecordFormat::admit(
        PhysicalRecordFormatDeclaration::builder().admit().unwrap(),
    );
    let placement = PhysicalRecordPlacementPolicy::builder()
        .admit(format)
        .unwrap();
    let access = PhysicalRecordAccessPolicy::builder().admit(format).unwrap();
    match media
        .initialize_record_store(PhysicalRecordInitialization::new(format, placement, access))
        .into_raw()
    {
        worth_proof::TransitionOutcome::Success(serving) => serving,
        _ => panic!("record initialization failed"),
    }
}

fn fresh_initialization_positioned_writes(parent: &std::path::Path) -> u64 {
    let baseline = initialized_store(&parent.join("writeback-baseline"), None);
    let attempts = baseline
        .media_counters()
        .attempts_for(MediaOperationRole::PositionedWrite);
    baseline.close();
    attempts
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
    performance()
        .policy_admission_receipt(claim)
        .budget_decision(
            FoundationalPerformanceBudgetKind::Breadth,
            (budget.queue_slots() + budget.worker_permits()) as u32,
            (budget.queue_slots() + budget.worker_permits()) as u32,
        )
        .budget_decision(
            FoundationalPerformanceBudgetKind::Density,
            (budget.bandwidth_tokens() + budget.cache_residency_hints()) as u32,
            (budget.bandwidth_tokens() + budget.cache_residency_hints()) as u32,
        )
        .budget_decision(
            FoundationalPerformanceBudgetKind::Locality,
            (budget.read_ahead_window() + budget.write_back_window() + budget.reclaim_permits())
                as u32,
            (budget.read_ahead_window() + budget.write_back_window() + budget.reclaim_permits())
                as u32,
        )
        .finish()
        .unwrap()
}

fn backend_witness() -> worth_store_physical_backend::AdmittedBackendCapabilityWitness {
    #[cfg(windows)]
    let profile = BackendTargetProfile::WindowsFlushFileBuffers;
    #[cfg(not(windows))]
    let profile = BackendTargetProfile::PosixFileFsyncDirSync;
    let request = BackendCapabilityAdmissionRequest::new(
        profile,
        BackendCapabilityEvidenceBasis::certified_backend_profile(),
        BackendCapabilitySupportSet::all_supported(),
        BackendMediaAssumptionSet::platform_file_defaults()
            .with_direct_io_alignment()
            .with_sector_atomicity()
            .with_page_cache_policy()
            .with_mmap_coherence()
            .with_async_ordering()
            .with_secure_frame_io()
            .with_flush_ordering()
            .with_fdatasync_durability(),
        BackendRebindTriggers::kernel_filesystem_mount_firmware_and_backend(),
    );
    PhysicalBackendCapabilityAdmissionAuthority::store_owned()
        .admit_backend_capability(request)
        .unwrap()
}
