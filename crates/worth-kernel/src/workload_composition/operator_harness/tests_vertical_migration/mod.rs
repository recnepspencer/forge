mod support;

use super::{OperatorOutcomeKind, OperatorWorkloadError, WorkloadOperator, WorkloadOperatorFamily};
use crate::workload_composition::{
    BatchAdmissionFamilyPosture, WorkloadCatalog, WorkloadStageRequirement,
};

use self::support::{
    compatible_aspect_parallel_spatial_batch_execution_slice,
    denied_same_participant_spatial_batch_execution_slice,
    disjoint_parallel_spatial_batch_execution_slice, operator_context_and_bundle,
    run_stack_heavy_test,
};

#[test]
fn migrated_coplanar_overlap_slice_executes_through_real_batch_execution_chain() {
    run_stack_heavy_test(|| {
        let built = WorkloadCatalog::coplanar_overlap_storm()
            .declared("phase10 migrated coplanar overlap slice")
            .build()
            .expect("vertical migration workload should build");
        let batch_execution = disjoint_parallel_spatial_batch_execution_slice();
        let migrated = WorkloadOperator::for_family(WorkloadOperatorFamily::CoplanarOverlap)
            .requiring(WorkloadStageRequirement::BatchAdmissionExecution)
            .declared_by_query("phase10 migrated grouped coplanar overlap execution")
            .admit_for_batch_execution(built.workload(), batch_execution.execution_receipt())
            .expect("migrated grouped workload should admit through batch execution");
        let (context, bundle) = operator_context_and_bundle(
            "phase10-migrated-coplanar-overlap-slice",
            built.projected_workload(),
            built.transform_receipts(),
        );

        let outcome = migrated
            .execute_coplanar_overlap(&context, &bundle)
            .expect("migrated grouped slice should execute");

        assert_eq!(
            batch_execution.independence_disposition(),
            crate::workload_composition::ConflictIndependenceDisposition::Disjoint
        );
        assert_eq!(
            migrated.batch_execution().posture(),
            BatchAdmissionFamilyPosture::ParallelAdmit
        );
        assert_eq!(
            migrated
                .batch_execution()
                .selected_conflict_plan_identities()
                .len(),
            2
        );
        assert_eq!(
            migrated
                .batch_execution()
                .independence_proof_identities()
                .len(),
            1
        );
        assert_eq!(
            migrated
                .batch_execution()
                .counters()
                .spatial_supporting_conflict_family_row_count(),
            2
        );
        assert_eq!(
            migrated
                .batch_execution()
                .counters()
                .topology_supporting_conflict_family_row_count(),
            0
        );
        assert_eq!(outcome.kind(), OperatorOutcomeKind::Admitted);
    });
}

#[test]
fn legacy_overlap_requirement_cannot_admit_migrated_slice_after_cutover() {
    run_stack_heavy_test(|| {
        let built = WorkloadCatalog::coplanar_overlap_storm()
            .declared("phase10 legacy overlap helper cutover denial")
            .build()
            .expect("vertical migration workload should build");
        let batch_execution = disjoint_parallel_spatial_batch_execution_slice();
        let (context, bundle) = operator_context_and_bundle(
            "phase10-legacy-overlap-helper-cutover-denial",
            built.projected_workload(),
            built.transform_receipts(),
        );

        let legacy = WorkloadOperator::for_family(WorkloadOperatorFamily::CoplanarOverlap)
            .requiring(WorkloadStageRequirement::RetainedReplay)
            .declared_by_query("phase10 legacy overlap helper should not satisfy migrated slice")
            .admit_for(built.workload())
            .expect("legacy retained-replay operator still admits its own old boundary");
        let error = legacy
            .execute_coplanar_overlap_with_batch_execution(
                built.workload(),
                batch_execution.execution_receipt(),
                &context,
                &bundle,
            )
            .expect_err(
                "legacy retained-replay run must not execute the migrated batch-execution slice",
            );

        assert_eq!(error, OperatorWorkloadError::MissingBatchAdmissionExecution);
    });
}

#[test]
fn migrated_slice_strengthens_old_parallel_posture_to_denied_without_proof() {
    run_stack_heavy_test(|| {
        let built = WorkloadCatalog::coplanar_overlap_storm()
            .declared("phase10 strengthened grouped posture")
            .build()
            .expect("vertical migration workload should build");
        let batch_execution = denied_same_participant_spatial_batch_execution_slice();
        let migrated = WorkloadOperator::for_family(WorkloadOperatorFamily::CoplanarOverlap)
            .requiring(WorkloadStageRequirement::BatchAdmissionExecution)
            .declared_by_query("phase10 denied grouped coplanar overlap execution")
            .admit_for_batch_execution(built.workload(), batch_execution.execution_receipt())
            .expect("migrated grouped workload should still admit the typed receipt boundary");
        let (context, bundle) = operator_context_and_bundle(
            "phase10-strengthened-grouped-posture",
            built.projected_workload(),
            built.transform_receipts(),
        );

        let error = migrated
            .execute_coplanar_overlap(&context, &bundle)
            .expect_err("non-parallel batch posture must block grouped overlap execution");

        assert_eq!(
            batch_execution.independence_disposition(),
            crate::workload_composition::ConflictIndependenceDisposition::Denied
        );
        assert_eq!(
            error,
            OperatorWorkloadError::GroupedExecutionRequiresParallelBatchPosture(
                BatchAdmissionFamilyPosture::Denied,
            )
        );
    });
}

#[test]
fn migrated_slice_preserves_spatial_aspect_local_distinctions() {
    let compatible = compatible_aspect_parallel_spatial_batch_execution_slice();
    let denied = denied_same_participant_spatial_batch_execution_slice();

    assert_eq!(
        compatible.independence_disposition(),
        crate::workload_composition::ConflictIndependenceDisposition::CompatibleAspectOverlap
    );
    assert_eq!(
        denied.independence_disposition(),
        crate::workload_composition::ConflictIndependenceDisposition::Denied
    );
    assert_eq!(
        compatible.authority_participant_identities(),
        denied.authority_participant_identities()
    );
    assert_eq!(compatible.authority_participant_identities().len(), 1);
    assert_eq!(
        compatible.execution_receipt().posture(),
        BatchAdmissionFamilyPosture::ParallelAdmit
    );
    assert_eq!(
        denied.execution_receipt().posture(),
        BatchAdmissionFamilyPosture::Denied
    );
    assert_eq!(
        compatible
            .execution_receipt()
            .participant_identities()
            .len(),
        2
    );
    assert_eq!(denied.execution_receipt().participant_identities().len(), 2);
    assert_eq!(
        compatible
            .execution_receipt()
            .counters()
            .spatial_supporting_conflict_family_row_count(),
        2
    );
    assert_eq!(
        denied
            .execution_receipt()
            .counters()
            .spatial_supporting_conflict_family_row_count(),
        2
    );
    assert_eq!(
        compatible
            .execution_receipt()
            .counters()
            .topology_supporting_conflict_family_row_count(),
        0
    );
    assert_eq!(
        denied
            .execution_receipt()
            .counters()
            .topology_supporting_conflict_family_row_count(),
        0
    );
}

#[test]
fn migrated_slice_rejects_mismatched_batch_execution_receipt() {
    run_stack_heavy_test(|| {
        let built = WorkloadCatalog::coplanar_overlap_storm()
            .declared("phase10 mismatched batch execution denial")
            .build()
            .expect("vertical migration workload should build");
        let admitted_execution = disjoint_parallel_spatial_batch_execution_slice();
        let mismatched_execution = denied_same_participant_spatial_batch_execution_slice();
        let migrated = WorkloadOperator::for_family(WorkloadOperatorFamily::CoplanarOverlap)
            .requiring(WorkloadStageRequirement::BatchAdmissionExecution)
            .declared_by_query("phase10 mismatched batch execution must deny")
            .admit_for_batch_execution(built.workload(), admitted_execution.execution_receipt())
            .expect("migrated grouped workload should admit through its own batch execution");
        let (context, bundle) = operator_context_and_bundle(
            "phase10-mismatched-batch-execution-denial",
            built.projected_workload(),
            built.transform_receipts(),
        );

        let error = migrated
            .run()
            .execute_coplanar_overlap_with_batch_execution(
                migrated.workload(),
                mismatched_execution.execution_receipt(),
                &context,
                &bundle,
            )
            .expect_err(
                "migrated grouped execution must reject a mismatched batch-execution receipt",
            );

        assert_eq!(
            error,
            OperatorWorkloadError::MismatchedBatchAdmissionExecution
        );
    });
}
