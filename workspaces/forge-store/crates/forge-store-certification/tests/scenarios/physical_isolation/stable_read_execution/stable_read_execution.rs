#[path = "../../security/stable_read_execution/security_scope.rs"]
mod execution_security_scope;
#[path = "../../../support/physical_isolation/stable_read_execution/support.rs"]
mod execution_support;
use forge_store_test_support::harness::physical_isolation::epoch_scope as support;
use forge_store_test_support::harness::physical_isolation::read_plan as plan_admission;

use execution_security_scope::logical_decode_entry_for_handle;
use execution_support::{
    admit_payload_frame_for_reference, bounded_copy_for_record, bounded_copy_for_reference,
    payload_admission_for_frame, payload_admission_for_reference, resident_frame_table,
};
use forge_foundational::{
    FoundationalBoundaryEvidenceReceiptKind, FoundationalDiagnosticOutcomeKind,
    FoundationalDiagnosticRowFamily,
};
use forge_proof::TransitionOutcome;
use forge_store_physical_isolation::{
    PhysicalByteGuard, PhysicalByteGuardDenial, PhysicalByteGuardScope,
    PhysicalReadExecutionDenial, PhysicalReadIoAttempt, PhysicalReadPlanRetryPosture,
    StablePhysicalReadExecution,
};
use plan_admission::{admit_plan, protected_set};
use support::{
    current_generation_page_reference, current_root_from_authority,
    physical_authority_from_complete_closeout, physical_authority_from_operation_digest_closeout,
};

#[test]
fn execution_consumes_handle_admits_guard_and_releases_plan() {
    let authority = physical_authority_from_complete_closeout();
    let root = current_root_from_authority(&authority);
    let reference = current_generation_page_reference(101);
    let plan = admit_plan(&authority, root, protected_set([reference], 4), 8, 4);
    let footprint_basis = plan.footprint().declared_footprint_basis();
    let admitted_plan_allocations = plan.counters().allocation_events();
    let handle = plan.into_execution_ready_handle();
    let scope = PhysicalByteGuardScope::for_owned_read_buffer(reference);
    let decode_entry = logical_decode_entry_for_handle(&handle, scope, "execution-consumes-handle");
    let mut execution = StablePhysicalReadExecution::from_execution_ready_handle(handle);
    let guard_admission = execution.admit_byte_guard(scope).unwrap();
    let guard = PhysicalByteGuard::from_bounded_copy(
        guard_admission,
        bounded_copy_for_reference(reference, b"copy"),
    )
    .unwrap();

    let guarded = execution
        .read_guarded_bytes_with_security_scope(&guard, decode_entry)
        .unwrap();
    assert_eq!(guarded.scope(), scope);
    assert_eq!(guarded.physical_bytes(), b"copy");

    let receipt = match execution.complete_with_proof() {
        TransitionOutcome::Success(receipt) => receipt,
        other => panic!("execution should complete without retry: {other:?}"),
    };
    assert_eq!(
        receipt.read_plan_release().footprint_basis(),
        footprint_basis
    );
    assert_eq!(receipt.counters().guard_admissions(), 1);
    assert_eq!(receipt.counters().guarded_byte_reads(), 1);
    assert_eq!(receipt.counters().guarded_bytes(), 4);
    assert_eq!(receipt.counters().compact_footprint_checks(), 1);
    assert_eq!(receipt.counters().broad_footprint_scans(), 0);
    assert_eq!(
        receipt.counters().plan_allocations(),
        admitted_plan_allocations
    );
    assert_eq!(receipt.counters().diagnostic_materializations(), 0);
    let foundational = receipt.lower_to_foundational_evidence();
    assert_eq!(
        foundational.executed_receipt().receipt_kind(),
        FoundationalBoundaryEvidenceReceiptKind::Execution
    );
    assert!(foundational.executed_receipt().did_execute());
    assert_eq!(foundational.diagnostic_rows().len(), 1);
    assert_eq!(
        foundational.diagnostic_rows()[0].family(),
        FoundationalDiagnosticRowFamily::ProvenanceReady
    );
    assert_eq!(
        foundational.diagnostic_rows()[0].outcome_kind(),
        FoundationalDiagnosticOutcomeKind::Accepted
    );
}

#[test]
fn execution_reports_epoch_retry_without_readmission_or_replanning() {
    let authority = physical_authority_from_complete_closeout();
    let root = current_root_from_authority(&authority);
    let reference = current_generation_page_reference(102);
    let plan = admit_plan(&authority, root, protected_set([reference], 4), 8, 4);
    let admitted_plan_allocations = plan.counters().allocation_events();
    let drifted_authority =
        physical_authority_from_operation_digest_closeout("s5-phase6-execution-time-drift");
    let drifted_root = current_root_from_authority(&drifted_authority);
    let mut execution = StablePhysicalReadExecution::from_execution_ready_handle(
        plan.into_execution_ready_handle(),
    );

    let outcome = execution.observe_epoch_freshness(drifted_root);

    match outcome {
        TransitionOutcome::Stale(receipt) => {
            assert_eq!(receipt.admitted_root_epoch().get(), root.epoch().get());
            assert_eq!(
                receipt.observed_root_epoch().get(),
                drifted_root.epoch().get()
            );
            assert_eq!(receipt.retry_posture(), PhysicalReadPlanRetryPosture::Retry);
            assert_eq!(receipt.counters().retry_decisions(), 1);
        }
        TransitionOutcome::RebindRequired(receipt) => {
            assert_eq!(
                receipt.admitted_manifest_epoch().get(),
                root.manifest_epoch().get()
            );
            assert_eq!(
                receipt.observed_manifest_epoch().get(),
                drifted_root.manifest_epoch().get()
            );
            assert_eq!(
                receipt.retry_posture(),
                PhysicalReadPlanRetryPosture::RebindRequired
            );
            assert_eq!(receipt.counters().retry_decisions(), 1);
        }
        other => panic!("root drift must stay typed as retry/rebind evidence: {other:?}"),
    }
    assert_eq!(execution.counters().broad_footprint_scans(), 0);
    assert_eq!(
        execution.counters().plan_allocations(),
        admitted_plan_allocations
    );
}

#[test]
fn owned_read_buffer_guard_rejects_mismatched_copy_provenance() {
    let authority = physical_authority_from_complete_closeout();
    let root = current_root_from_authority(&authority);
    let reference = current_generation_page_reference(103);
    let plan = admit_plan(&authority, root, protected_set([reference], 4), 8, 4);
    let mut execution = StablePhysicalReadExecution::from_execution_ready_handle(
        plan.into_execution_ready_handle(),
    );
    let admission = execution
        .admit_byte_guard(PhysicalByteGuardScope::for_owned_read_buffer(reference))
        .unwrap();

    let denial = PhysicalByteGuard::from_bounded_copy(admission, bounded_copy_for_record(b"copy"))
        .unwrap_err();

    assert!(matches!(
        denial,
        PhysicalByteGuardDenial::ByteProvenanceMismatch { .. }
    ));
}

#[test]
fn execution_denies_reference_discovery_before_guard_construction() {
    let authority = physical_authority_from_complete_closeout();
    let root = current_root_from_authority(&authority);
    let protected = current_generation_page_reference(107);
    let discovered = current_generation_page_reference(109);
    let plan = admit_plan(&authority, root, protected_set([protected], 4), 8, 4);
    let mut execution = StablePhysicalReadExecution::from_execution_ready_handle(
        plan.into_execution_ready_handle(),
    );
    let denial = execution
        .admit_byte_guard(PhysicalByteGuardScope::for_owned_read_buffer(discovered))
        .unwrap_err();

    assert!(matches!(
        denial,
        PhysicalReadExecutionDenial::ReadPlanDenied(
            forge_store_physical_isolation::PhysicalReadPlanAdmissionDenial::ExecutionTimeReferenceDiscovery
        )
    ));
    assert_eq!(
        execution.counters().execution_time_reference_discoveries(),
        1
    );
}

#[test]
fn execution_rejects_guard_admitted_by_different_protected_footprint() {
    let authority = physical_authority_from_complete_closeout();
    let root = current_root_from_authority(&authority);
    let left = current_generation_page_reference(111);
    let right = current_generation_page_reference(112);
    let left_plan = admit_plan(&authority, root, protected_set([left], 4), 8, 4);
    let right_plan = admit_plan(&authority, root, protected_set([right], 4), 8, 4);
    let admitted_plan_allocations = right_plan.counters().allocation_events();
    let mut left_execution = StablePhysicalReadExecution::from_execution_ready_handle(
        left_plan.into_execution_ready_handle(),
    );
    let right_handle = right_plan.into_execution_ready_handle();
    let right_decode_entry = logical_decode_entry_for_handle(
        &right_handle,
        PhysicalByteGuardScope::for_owned_read_buffer(right),
        "different-protected-footprint",
    );
    let mut right_execution =
        StablePhysicalReadExecution::from_execution_ready_handle(right_handle);
    let admission = left_execution
        .admit_byte_guard(PhysicalByteGuardScope::for_owned_read_buffer(left))
        .unwrap();
    let guard =
        PhysicalByteGuard::from_bounded_copy(admission, bounded_copy_for_reference(left, b"left"))
            .unwrap();

    let denial = right_execution
        .read_guarded_bytes_with_security_scope(&guard, right_decode_entry)
        .unwrap_err();

    assert!(matches!(
        denial,
        PhysicalReadExecutionDenial::LogicalDecodeScopeFootprintMismatch { .. }
    ));
    assert_eq!(right_execution.counters().guarded_byte_reads(), 0);
    assert_eq!(right_execution.counters().broad_footprint_scans(), 0);
    assert_eq!(
        right_execution.counters().plan_allocations(),
        admitted_plan_allocations
    );
}

#[test]
fn owned_read_buffer_guard_rejects_extent_scope() {
    let authority = physical_authority_from_complete_closeout();
    let root = current_root_from_authority(&authority);
    let reference = current_generation_page_reference(113);
    let plan = admit_plan(&authority, root, protected_set([reference], 4), 8, 4);
    let mut execution = StablePhysicalReadExecution::from_execution_ready_handle(
        plan.into_execution_ready_handle(),
    );
    let admission = execution
        .admit_byte_guard(PhysicalByteGuardScope::for_extent_window(reference))
        .unwrap();

    let denial =
        PhysicalByteGuard::from_bounded_copy(admission, bounded_copy_for_record(b"wrong-scope"))
            .unwrap_err();

    assert_eq!(
        denial,
        PhysicalByteGuardDenial::GuardScopeKindMismatch {
            expected: forge_store_physical_isolation::PhysicalByteGuardScopeKind::OwnedReadBuffer,
            observed: forge_store_physical_isolation::PhysicalByteGuardScopeKind::ExtentWindow
        }
    );
}

#[test]
fn extent_and_mmap_guards_require_matching_scope_kind() {
    let authority = physical_authority_from_complete_closeout();
    let root = current_root_from_authority(&authority);
    let reference = current_generation_page_reference(119);
    let plan = admit_plan(&authority, root, protected_set([reference], 4), 8, 4);
    let handle = plan.into_execution_ready_handle();
    let extent_scope = PhysicalByteGuardScope::for_extent_window(reference);
    let mmap_scope = PhysicalByteGuardScope::for_mmap_view(reference);
    let extent_decode_entry =
        logical_decode_entry_for_handle(&handle, extent_scope, "extent-and-mmap");
    let mmap_decode_entry = logical_decode_entry_for_handle(&handle, mmap_scope, "extent-and-mmap");
    let mut execution = StablePhysicalReadExecution::from_execution_ready_handle(handle);

    let extent = execution.admit_byte_guard(extent_scope).unwrap();
    let extent_guard = PhysicalByteGuard::from_extent_window(
        extent,
        payload_admission_for_reference(reference, b"extent"),
    )
    .unwrap();
    assert_eq!(
        execution
            .read_guarded_bytes_with_security_scope(&extent_guard, extent_decode_entry)
            .unwrap()
            .physical_bytes(),
        b"extent"
    );

    let mmap = execution.admit_byte_guard(mmap_scope).unwrap();
    let mmap_guard = PhysicalByteGuard::from_mmap_view(
        mmap,
        payload_admission_for_reference(reference, b"mmap"),
    )
    .unwrap();
    let denial = execution
        .read_guarded_bytes_with_security_scope(&mmap_guard, extent_decode_entry)
        .unwrap_err();
    assert!(matches!(
        denial,
        PhysicalReadExecutionDenial::ByteGuardScopeMismatch { .. }
    ));
    assert_eq!(
        execution
            .read_guarded_bytes_with_security_scope(&mmap_guard, mmap_decode_entry)
            .unwrap()
            .physical_bytes(),
        b"mmap"
    );
}

#[test]
fn borrowed_payload_guards_reject_mismatched_payload_provenance() {
    let authority = physical_authority_from_complete_closeout();
    let root = current_root_from_authority(&authority);
    let reference = current_generation_page_reference(121);
    let plan = admit_plan(&authority, root, protected_set([reference], 4), 8, 4);
    let mut execution = StablePhysicalReadExecution::from_execution_ready_handle(
        plan.into_execution_ready_handle(),
    );
    let admission = execution
        .admit_byte_guard(PhysicalByteGuardScope::for_extent_window(reference))
        .unwrap();

    let denial =
        PhysicalByteGuard::from_extent_window(admission, payload_admission_for_frame(8, b"extent"))
            .unwrap_err();

    assert!(matches!(
        denial,
        PhysicalByteGuardDenial::ByteProvenanceMismatch { .. }
    ));
}

#[test]
fn resident_frame_guard_rejects_mismatched_lease_provenance() {
    let authority = physical_authority_from_complete_closeout();
    let root = current_root_from_authority(&authority);
    let protected = current_generation_page_reference(123);
    let observed = current_generation_page_reference(124);
    let plan = admit_plan(&authority, root, protected_set([protected], 4), 8, 4);
    let mut execution = StablePhysicalReadExecution::from_execution_ready_handle(
        plan.into_execution_ready_handle(),
    );
    let mut table = resident_frame_table(8192, 2);
    admit_payload_frame_for_reference(&mut table, protected, b"protected");
    let observed_admission = admit_payload_frame_for_reference(&mut table, observed, b"observed");
    let lease = table
        .lease_page(observed_admission.resident_frame_token())
        .unwrap();
    let pinned = lease.pin().unwrap();
    let admission = execution
        .admit_byte_guard(PhysicalByteGuardScope::for_resident_frame(
            protected,
            pinned.resident_frame_token(),
        ))
        .unwrap();

    let denial = PhysicalByteGuard::from_pinned_frame(admission, &pinned).unwrap_err();

    assert!(matches!(
        denial,
        PhysicalByteGuardDenial::ByteProvenanceMismatch { .. }
    ));
}

#[test]
fn ordinary_execution_denies_hidden_structural_latch_io() {
    let authority = physical_authority_from_complete_closeout();
    let root = current_root_from_authority(&authority);
    let reference = current_generation_page_reference(127);
    let plan = admit_plan(&authority, root, protected_set([reference], 4), 8, 4);
    let handle = plan.into_execution_ready_handle();
    let scope = PhysicalByteGuardScope::for_owned_read_buffer(reference);
    let decode_entry = logical_decode_entry_for_handle(&handle, scope, "blocking-io");
    let mut execution = StablePhysicalReadExecution::from_execution_ready_handle(handle);
    let admission = execution.admit_byte_guard(scope).unwrap();
    let guard = PhysicalByteGuard::from_bounded_copy(
        admission,
        bounded_copy_for_reference(reference, b"copy"),
    )
    .unwrap();
    let denial = execution
        .read_guarded_bytes_after_io_attempt(
            PhysicalReadIoAttempt::blocking_storage_io_while_structural_latch_held(),
            &guard,
            decode_entry,
        )
        .unwrap_err();

    assert!(matches!(
        denial,
        PhysicalReadExecutionDenial::HiddenStructuralLatchIoWithoutDeclaredCost { .. }
    ));
    assert_eq!(execution.counters().hidden_latch_io_denials(), 1);
    assert_eq!(execution.counters().blocking_io_events(), 1);
    assert_eq!(execution.counters().guarded_byte_reads(), 0);
}
