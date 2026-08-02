use worth_proof::{NonEmpty, TransitionOutcome};
use worth_signal::facade::TemporalDuration;
use worth_store::physical_runtime::certification::CertificationPhysicalRecordSubmission;
use worth_store::physical_runtime::{
    AdmittedRecordPlacementPolicy, DataSettledPhysicalMutation, PhysicalDataDispatchOutcome,
    PhysicalDataSettlementOutcome, PhysicalDurabilityGroupBasis,
    PhysicalManifestCapacityTransition, PhysicalMutationDeadline,
    PhysicalMutationIdempotencyMaterial, PhysicalMutationPreparationOutcome,
    PhysicalMutationPreparationSuccess, PhysicalMutationRequest, PhysicalRecordSubmission,
    PhysicalWalGroupAppendOutcome, PhysicalWalGroupBarrierOutcome, RecordAppendBatch,
};

pub(crate) struct SettledSingleton {
    pub(crate) basis: PhysicalDurabilityGroupBasis,
    pub(crate) member: DataSettledPhysicalMutation,
}

pub(crate) fn settle_single(
    submission: &CertificationPhysicalRecordSubmission,
    placement: AdmittedRecordPlacementPolicy,
    material: PhysicalMutationIdempotencyMaterial,
    batch: RecordAppendBatch,
) -> SettledSingleton {
    settle_single_with_manifest_capacity_transition(
        submission,
        placement,
        PhysicalManifestCapacityTransition::PreserveCurrent,
        material,
        batch,
    )
}

pub(crate) fn prepare_single(
    submission: &PhysicalRecordSubmission,
    placement: AdmittedRecordPlacementPolicy,
    manifest_capacity_transition: PhysicalManifestCapacityTransition,
    material: PhysicalMutationIdempotencyMaterial,
    batch: RecordAppendBatch,
) -> PhysicalMutationPreparationOutcome {
    let key = submission
        .issue_idempotency_key(material)
        .expect("the explicit test identity must be admitted");
    submission.prepare_durable_append_with_manifest_capacity_transition(
        batch,
        placement,
        manifest_capacity_transition,
        PhysicalMutationRequest::platform_durable(
            key,
            PhysicalMutationDeadline::at(
                TemporalDuration::temporal_duration(1_000)
                    .expect("the fixed test deadline is nonzero"),
            ),
        ),
    )
}

pub(crate) fn settle_single_with_manifest_capacity_transition(
    submission: &CertificationPhysicalRecordSubmission,
    placement: AdmittedRecordPlacementPolicy,
    manifest_capacity_transition: PhysicalManifestCapacityTransition,
    material: PhysicalMutationIdempotencyMaterial,
    batch: RecordAppendBatch,
) -> SettledSingleton {
    let prepared = match prepare_single(
        submission,
        placement,
        manifest_capacity_transition,
        material,
        batch,
    )
    .into_raw()
    {
        TransitionOutcome::Success(PhysicalMutationPreparationSuccess::Prepared(prepared)) => {
            prepared
        }
        _ => panic!("the canonical durable preparation must succeed"),
    };
    let appended = match submission.append_prepared_wal_group(NonEmpty::new(prepared, Vec::new())) {
        PhysicalWalGroupAppendOutcome::Appended(appended) => appended,
        _ => panic!("the canonical WAL append must succeed"),
    };
    let basis = appended.basis();
    let durable = match submission.synchronize_appended_wal_group(appended) {
        PhysicalWalGroupBarrierOutcome::Durable(durable) => durable
            .into_members()
            .into_vec()
            .pop()
            .expect("a singleton group must derive one durable member"),
        _ => panic!("the canonical WAL barrier must succeed"),
    };
    let dispatched = match submission.dispatch_wal_durable_data(durable) {
        PhysicalDataDispatchOutcome::Dispatched(dispatched) => dispatched,
        _ => panic!("the exact WAL-durable member must dispatch"),
    };
    let member = match dispatched.settle_exact_effects() {
        PhysicalDataSettlementOutcome::Settled(settled) => settled,
        PhysicalDataSettlementOutcome::InspectionRequired { cause, .. } => {
            panic!("the exact data effects must settle: {cause:?}")
        }
    };
    SettledSingleton { basis, member }
}
