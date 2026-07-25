use super::policy_receipts::background_policy_receipt;
use super::test_support::{read_pressure_budget, World};

use crate::{
    admit_background_pacing, admit_queue_execution_plan, lower_background_queue_lease,
    BackgroundIoPressureShape, BackgroundPacingDenial, BackgroundPacingOutcome,
    IoSchedulerBackendCapabilityRequirement, QueueExecutionAdmissionRequest,
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

fn admitted_background_queue_plan(
    world: &World,
    pressure: BackgroundIoPressureShape,
) -> QueueExecutionReadyPlan {
    let BackgroundPacingOutcome::AdmittedWithDebt(admitted) =
        admit_background_pacing(world.request(pressure))
    else {
        panic!("secure background pressure should admit before queue lowering");
    };
    let work = lower_background_queue_lease(admitted.lease());
    let policy = crate::admit_queue_policy_receipt(
        work.clone(),
        background_policy_receipt(work.requested_budget(), work.requested_budget()),
    )
    .expect("background policy receipt should bind exact work");
    admit_queue_execution_plan(QueueExecutionAdmissionRequest::new(
        work,
        world.backend(),
        policy,
    ))
    .expect("background lease should lower into queue admission with secure-I/O intact")
}
