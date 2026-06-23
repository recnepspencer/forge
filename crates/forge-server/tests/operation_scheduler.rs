use forge_server::{
    request_context::DiagnosticRichnessProfile, ForgeServer, ForgeServerLoweredOperationPlan,
    ForgeServerOperationPlannerInput, ForgeServerOperationRequestInput,
    ForgeServerQueryHandoffOperation, ForgeServerRequestContextInput,
    ForgeServerSchedulerCancellationDirective, ForgeServerSchedulerCancellationPosture,
    ForgeServerSchedulerCertificationSabotage, ForgeServerSchedulerConflictDenialCode,
    ForgeServerSchedulerFailurePosture, ForgeServerSurfaceFamily, ForgeServerTransportClass,
};

#[path = "support/operation_scheduler/runtime.rs"]
mod operation_scheduler_runtime;
#[path = "support/query_handoff/fixture.rs"]
mod query_handoff_fixture;

use operation_scheduler_runtime::{
    SchedulerWorkspaceProvider, SelectiveSharedReadWorkspaceProvider,
};
use query_handoff_fixture::{admit_read_posture, resolve_request_context, test_server};

#[test]
fn concurrent_shared_read_scheduler_matches_serialized_replay() {
    let server = test_server(SchedulerWorkspaceProvider, false);

    let concurrent = server
        .operation_scheduler()
        .schedule_shared_read_batch(build_shared_read_plans(&server))
        .expect("shared-read plans should schedule")
        .execute();
    let serialized = server
        .operation_scheduler()
        .schedule_shared_read_batch(build_shared_read_plans(&server))
        .expect("shared-read plans should schedule")
        .execute_serialized_replay();
    let repeated = server
        .operation_scheduler()
        .schedule_shared_read_batch(build_shared_read_plans(&server))
        .expect("shared-read plans should schedule")
        .execute();

    assert_scheduler_equivalence(&concurrent, &serialized);
    assert_scheduler_equivalence(&concurrent, &repeated);
}

#[test]
fn shared_read_hot_path_reports_exact_zero_global_lock_acquisitions() {
    let server = test_server(SchedulerWorkspaceProvider, false);
    let executed = server
        .operation_scheduler()
        .schedule_shared_read_batch(build_shared_read_plans(&server))
        .expect("shared-read plans should schedule")
        .execute();

    assert_eq!(
        executed
            .counters()
            .forbidden_global_lock_acquisition_count(),
        0
    );
    assert_eq!(executed.counters().planned_batch_width(), 3);
    assert_eq!(executed.counters().completed_read_slot_count(), 3);
    for outcome in executed.outcomes() {
        assert_eq!(
            outcome
                .scheduler_counters()
                .forbidden_global_lock_acquisition_count(),
            0
        );
    }
}

#[test]
fn shared_read_lock_accounting_sabotage_reopens_exact_zero_posture() {
    let server = test_server(SchedulerWorkspaceProvider, false);
    let executed = server
        .operation_scheduler()
        .schedule_shared_read_batch(build_shared_read_plans(&server))
        .expect("shared-read plans should schedule")
        .execute_with_certification_sabotage([
            ForgeServerSchedulerCertificationSabotage::ForbiddenGlobalLockAfterAdmission {
                slot_ordinal: 1,
            },
        ]);

    assert_eq!(executed.counters().planned_batch_width(), 3);
    assert_eq!(executed.counters().completed_read_slot_count(), 3);
    assert_eq!(
        executed
            .counters()
            .forbidden_global_lock_acquisition_count(),
        1
    );
}

#[test]
fn scheduler_failures_are_isolated_by_declared_dependency() {
    let server = test_server(SelectiveSharedReadWorkspaceProvider, false);
    let executed = server
        .operation_scheduler()
        .schedule_shared_read_batch(vec![
            lower_shared_read_plan(
                &server,
                "workspace-shared-read-denied",
                ForgeServerSurfaceFamily::ForgeNative,
                ForgeServerTransportClass::ForgeNativeInProcess,
                ForgeServerQueryHandoffOperation::direct_read("users.profile"),
                DiagnosticRichnessProfile::Standard,
            ),
            lower_shared_read_plan(
                &server,
                "workspace-shared-read-denied",
                ForgeServerSurfaceFamily::CompatHttp,
                ForgeServerTransportClass::CompatHttp,
                ForgeServerQueryHandoffOperation::direct_state("users.profile"),
                DiagnosticRichnessProfile::Standard,
            ),
            lower_shared_read_plan(
                &server,
                "workspace-42",
                ForgeServerSurfaceFamily::ForgeNative,
                ForgeServerTransportClass::ForgeNativeInProcess,
                ForgeServerQueryHandoffOperation::direct_inspection("users.profile"),
                DiagnosticRichnessProfile::Standard,
            ),
        ])
        .expect("shared-read plans should schedule")
        .execute();

    assert_eq!(executed.counters().isolated_failure_count(), 1);
    assert_eq!(executed.counters().dependent_failure_count(), 1);
    assert!(executed.outcomes()[2].response_envelope().is_some());
    match executed.outcomes()[0]
        .failure_posture()
        .expect("first slot should fail in isolation")
    {
        ForgeServerSchedulerFailurePosture::IsolatedRuntimeFailure { runtime_failure } => {
            assert!(runtime_failure.detail().contains("shared-read"));
        }
        other => panic!("expected isolated runtime failure, got {other:?}"),
    }
    match executed.outcomes()[1]
        .failure_posture()
        .expect("second slot should fail as dependent")
    {
        ForgeServerSchedulerFailurePosture::DependentSharedBasisFailure {
            shared_basis_key,
            failed_slot_ordinal,
        } => {
            assert!(shared_basis_key.contains("basis-users-profile"));
            assert_eq!(*failed_slot_ordinal, 0);
        }
        other => panic!("expected dependent shared-basis failure, got {other:?}"),
    }
}

#[test]
fn dependency_grouping_isolated_for_basis_and_footprint_near_misses() {
    let server = test_server(SelectiveSharedReadWorkspaceProvider, false);
    let executed = server
        .operation_scheduler()
        .schedule_shared_read_batch(vec![
            lower_shared_read_plan(
                &server,
                "workspace-shared-read-denied",
                ForgeServerSurfaceFamily::ForgeNative,
                ForgeServerTransportClass::ForgeNativeInProcess,
                ForgeServerQueryHandoffOperation::direct_read("users.profile"),
                DiagnosticRichnessProfile::Standard,
            ),
            lower_shared_read_plan_with_basis(
                &server,
                "workspace-shared-read-denied",
                "basis-users-profile-other",
                ForgeServerSurfaceFamily::CompatHttp,
                ForgeServerTransportClass::CompatHttp,
                ForgeServerQueryHandoffOperation::direct_state("users.profile"),
                DiagnosticRichnessProfile::Standard,
            ),
            lower_shared_read_plan_with_basis(
                &server,
                "workspace-43",
                "basis-users-profile",
                ForgeServerSurfaceFamily::ForgeNative,
                ForgeServerTransportClass::ForgeNativeInProcess,
                ForgeServerQueryHandoffOperation::direct_inspection("users.profile"),
                DiagnosticRichnessProfile::Standard,
            ),
        ])
        .expect("shared-read plans should schedule")
        .execute();

    assert_eq!(executed.counters().isolated_failure_count(), 2);
    assert_eq!(executed.counters().dependent_failure_count(), 0);
    assert!(matches!(
        executed.outcomes()[0].failure_posture(),
        Some(ForgeServerSchedulerFailurePosture::IsolatedRuntimeFailure { .. })
    ));
    assert!(matches!(
        executed.outcomes()[1].failure_posture(),
        Some(ForgeServerSchedulerFailurePosture::IsolatedRuntimeFailure { .. })
    ));
    assert!(executed.outcomes()[2].response_envelope().is_some());
}

#[test]
fn scheduler_cancellation_records_only_after_admission() {
    let server = test_server(SchedulerWorkspaceProvider, false);
    let executed = server
        .operation_scheduler()
        .schedule_shared_read_batch(build_shared_read_plans(&server))
        .expect("shared-read plans should schedule")
        .execute_with_cancellation([
            ForgeServerSchedulerCancellationDirective::BeforeAdmission { slot_ordinal: 0 },
            ForgeServerSchedulerCancellationDirective::AfterAdmissionBeforeExecution {
                slot_ordinal: 1,
            },
            ForgeServerSchedulerCancellationDirective::DuringExecution { slot_ordinal: 2 },
        ]);

    assert_eq!(executed.counters().cancelled_before_admission_count(), 1);
    assert_eq!(
        executed
            .counters()
            .cancelled_after_admission_before_execution_count(),
        1
    );
    assert_eq!(executed.counters().cancelled_during_execution_count(), 1);
    assert_eq!(executed.counters().admitted_read_slot_count(), 2);
    assert_eq!(
        executed.outcomes()[0].cancellation_posture(),
        Some(ForgeServerSchedulerCancellationPosture::BeforeAdmission)
    );
    assert_eq!(
        executed.outcomes()[1].cancellation_posture(),
        Some(ForgeServerSchedulerCancellationPosture::AfterAdmissionBeforeExecution)
    );
    assert_eq!(
        executed.outcomes()[2].cancellation_posture(),
        Some(ForgeServerSchedulerCancellationPosture::DuringExecution)
    );
}

#[test]
fn unsupported_shared_read_operations_are_denied_at_schedule_boundary() {
    let server = test_server(SchedulerWorkspaceProvider, false);
    for (surface_family, transport_class) in [
        (
            ForgeServerSurfaceFamily::ForgeNative,
            ForgeServerTransportClass::ForgeNativeInProcess,
        ),
        (
            ForgeServerSurfaceFamily::CompatHttp,
            ForgeServerTransportClass::CompatHttp,
        ),
    ] {
        let denial = server
            .operation_scheduler()
            .schedule_shared_read_batch([lower_shared_read_plan(
                &server,
                "workspace-42",
                surface_family,
                transport_class,
                ForgeServerQueryHandoffOperation::direct_projection("users.profile"),
                DiagnosticRichnessProfile::Standard,
            )])
            .expect_err("unsupported projection operations must deny at scheduling");

        assert_eq!(
            denial.code(),
            ForgeServerSchedulerConflictDenialCode::UnsupportedSharedReadOperation
        );
    }
}

fn assert_scheduler_equivalence(
    left: &forge_server::ForgeServerExecutedOperationBatch,
    right: &forge_server::ForgeServerExecutedOperationBatch,
) {
    assert_eq!(left.outcomes().len(), right.outcomes().len());
    for (left, right) in left.outcomes().iter().zip(right.outcomes()) {
        assert_eq!(
            left.response_envelope()
                .expect("equivalence lane should succeed")
                .canonical_digest(),
            right
                .response_envelope()
                .expect("equivalence lane should succeed")
                .canonical_digest()
        );
        assert_eq!(
            left.shared_read_basis_identity(),
            right.shared_read_basis_identity()
        );
        assert_eq!(left.execution_digest(), right.execution_digest());
        assert!(
            left.execution_digest().is_some(),
            "shared-read success should carry a real execution digest"
        );
        assert_eq!(
            left.plan_proof().receipt().expected_scheduler_lane(),
            right.plan_proof().receipt().expected_scheduler_lane()
        );
    }
}

fn build_shared_read_plans(server: &ForgeServer) -> Vec<ForgeServerLoweredOperationPlan> {
    vec![
        lower_shared_read_plan(
            server,
            "workspace-42",
            ForgeServerSurfaceFamily::ForgeNative,
            ForgeServerTransportClass::ForgeNativeInProcess,
            ForgeServerQueryHandoffOperation::direct_read("users.profile"),
            DiagnosticRichnessProfile::Standard,
        ),
        lower_shared_read_plan(
            server,
            "workspace-42",
            ForgeServerSurfaceFamily::CompatHttp,
            ForgeServerTransportClass::CompatHttp,
            ForgeServerQueryHandoffOperation::direct_state("users.profile"),
            DiagnosticRichnessProfile::Standard,
        ),
        lower_shared_read_plan(
            server,
            "workspace-42",
            ForgeServerSurfaceFamily::ForgeNative,
            ForgeServerTransportClass::ForgeNativeInProcess,
            ForgeServerQueryHandoffOperation::direct_inspection("users.profile"),
            DiagnosticRichnessProfile::Forensic,
        ),
    ]
}

fn lower_shared_read_plan(
    server: &ForgeServer,
    workspace_id: &str,
    surface_family: ForgeServerSurfaceFamily,
    transport_class: ForgeServerTransportClass,
    operation: ForgeServerQueryHandoffOperation,
    diagnostics_profile: DiagnosticRichnessProfile,
) -> ForgeServerLoweredOperationPlan {
    lower_shared_read_plan_with_basis(
        server,
        workspace_id,
        "basis-users-profile",
        surface_family,
        transport_class,
        operation,
        diagnostics_profile,
    )
}

fn lower_shared_read_plan_with_basis(
    server: &ForgeServer,
    workspace_id: &str,
    basis_digest: &str,
    surface_family: ForgeServerSurfaceFamily,
    transport_class: ForgeServerTransportClass,
    operation: ForgeServerQueryHandoffOperation,
    diagnostics_profile: DiagnosticRichnessProfile,
) -> ForgeServerLoweredOperationPlan {
    let resolved = resolve_request_context(
        server,
        ForgeServerRequestContextInput::builder()
            .with_surface_family(surface_family)
            .with_transport_class(transport_class)
            .with_authenticated_principal_id("principal-7")
            .with_tenant_id("tenant-a")
            .with_workspace_id(workspace_id)
            .with_diagnostics_profile(diagnostics_profile)
            .build()
            .expect("scheduler test request context should validate"),
    );
    let admission = admit_read_posture(server, resolved);
    let operation_request = server
        .operation_requests()
        .admit_from_forge_native_admission(
            admission.authorization_proof().admission(),
            ForgeServerOperationRequestInput::builder()
                .with_operation_family(forge_server::ForgeServerOperationFamily::QueryDirectRead)
                .with_operation_name("users.profile")
                .with_basis_digest(basis_digest)
                .build(),
        )
        .expect("read operation request should admit");
    let operation_admission = server
        .operation_admissions()
        .admit_declared(
            admission.authorization_proof().admission(),
            &operation_request,
        )
        .expect("read operation admission should admit");
    server
        .operation_planner()
        .lower(ForgeServerOperationPlannerInput::new(
            operation_admission,
            operation,
        ))
        .expect("shared-read plan should lower")
}
