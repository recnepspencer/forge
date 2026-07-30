use super::policy_receipts::background_policy_receipt;
use super::test_support::{read_pressure_budget, World};
use crate::queue_execution::test_support::{completion_for_plan, speculative_scope};

use crate::{
    admit_background_pacing, admit_queue_execution_plan, execute_ready_queue_plan,
    lower_background_queue_lease, BackgroundIoPressureShape, BackgroundPacingDenial,
    BackgroundPacingOutcome, IoSchedulerBackendCapabilityRequirement,
    QueueExecutionAdmissionRequest, QueueExecutionOutcome, QueueExecutionProgression,
    QueueExecutionReadyPlan, SecureIoOperation, SecureIoPreservationDenial,
};

#[test]
fn secure_scope_pressure_rejects_wrong_operation_receipts() {
    let world = World::new();
    let requested = read_pressure_budget();
    let cases = [
        (
            BackgroundIoPressureShape::backup_prep_read().requesting(requested),
            SecureIoOperation::VerificationPressure,
            SecureIoOperation::BackgroundLease,
        ),
        (
            BackgroundIoPressureShape::repair_scan().requesting(requested),
            SecureIoOperation::BackgroundLease,
            SecureIoOperation::RepairScan,
        ),
        (
            BackgroundIoPressureShape::verification_pressure().requesting(requested),
            SecureIoOperation::RepairScan,
            SecureIoOperation::VerificationPressure,
        ),
    ];

    for (pressure, actual, expected) in cases {
        assert_eq!(
            world.capacity_denial_with_secure_io(pressure, world.secure_io_for_operation(actual),),
            BackgroundPacingDenial::SecureIoDenied(SecureIoPreservationDenial::OperationMismatch {
                expected,
                actual
            })
        );
    }
}

#[test]
fn secure_scope_pressure_rejects_wrong_security_scope_receipts() {
    let security_scope = worth_store_security::admitted_wrong_io_qos_security_scope_for_test();
    let denial = crate::admit_security_scope_for_scheduler(&security_scope)
        .expect_err("wrong security scope must not enter scheduler use");
    assert!(matches!(
        denial,
        crate::IoSchedulerSecurityScopeAdmissionDenial::WrongKeyScope { .. }
    ));
}

#[test]
fn secure_scope_pressure_rejects_wrong_backend_receipts() {
    let world = World::new();
    let requested = read_pressure_budget();
    let cases = [
        (
            BackgroundIoPressureShape::backup_prep_read().requesting(requested),
            SecureIoOperation::BackgroundLease,
        ),
        (
            BackgroundIoPressureShape::repair_scan().requesting(requested),
            SecureIoOperation::RepairScan,
        ),
        (
            BackgroundIoPressureShape::verification_pressure().requesting(requested),
            SecureIoOperation::VerificationPressure,
        ),
    ];

    for (pressure, operation) in cases {
        assert_eq!(
            world.capacity_denial_with_secure_io(
                pressure,
                world.secure_io_for_backend_requirement(
                    operation,
                    IoSchedulerBackendCapabilityRequirement::Fsync,
                ),
            ),
            BackgroundPacingDenial::SecureIoDenied(
                SecureIoPreservationDenial::BackendRequirementMismatch {
                    required: IoSchedulerBackendCapabilityRequirement::Fsync,
                    admitted: IoSchedulerBackendCapabilityRequirement::DirectIo,
                }
            )
        );
    }
}

#[test]
fn secure_scope_background_leases_lower_into_queue_admission() {
    let world = World::new();
    let requested = read_pressure_budget();
    for pressure in [
        BackgroundIoPressureShape::backup_prep_read().requesting(requested),
        BackgroundIoPressureShape::repair_scan().requesting(requested),
        BackgroundIoPressureShape::verification_pressure().requesting(requested),
    ] {
        let plan = admitted_background_queue_plan(&world, pressure);
        assert_eq!(
            plan.work().class(),
            crate::QueueWorkClass::Background(pressure.class())
        );
        assert_eq!(
            plan.work().secure_io(),
            Some(world.secure_io_for_pressure(pressure))
        );
    }
}

#[test]
fn one_background_lease_progresses_into_one_executed_queue_plan() {
    let world = World::new();
    let pressure = BackgroundIoPressureShape::repair_scan().requesting(read_pressure_budget());
    let plan = admitted_background_queue_plan(&world, pressure);
    let scope = speculative_scope(&plan);
    let completion = completion_for_plan(&plan, 1, Some(scope), 0, None).complete();
    let QueueExecutionOutcome::Executed(executed) = execute_ready_queue_plan(plan, completion)
    else {
        panic!("one scheduler lease must progress through one executed queue plan");
    };
    assert_eq!(
        executed.plan().progression(),
        QueueExecutionProgression::Executed
    );
    assert_eq!(
        executed.plan().work().class(),
        crate::QueueWorkClass::Background(pressure.class())
    );
}

fn admitted_background_queue_plan(
    world: &World,
    pressure: BackgroundIoPressureShape,
) -> QueueExecutionReadyPlan {
    let BackgroundPacingOutcome::AdmittedWithDebt(admitted) =
        admit_background_pacing(world.request(pressure))
    else {
        panic!("secure background pressure should admit before queue lowering");
    };
    let work = lower_background_queue_lease(admitted.into_lease());
    let policy_budget = work.requested_budget();
    let policy = crate::admit_queue_policy_receipt(
        work,
        background_policy_receipt(policy_budget, policy_budget),
    )
    .expect("background policy receipt should bind exact work");
    admit_queue_execution_plan(QueueExecutionAdmissionRequest::new(policy, world.backend()))
        .expect("background lease should lower into queue admission with secure-I/O intact")
}
