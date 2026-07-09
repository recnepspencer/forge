use worth_store_security::{
    admitted_wrong_s6_io_qos_security_scope_for_test, StoreAuthenticityRequirement,
    StoreAuthenticityRequirementClass, StoreKeyScope, StoreTenantScope,
};

use super::test_support::{
    backend_for, grouping_for, point_read_budget, policy_receipt, secure_io_for_work,
};
use crate::foreground_reservation::{
    admitted_point_read_reservation_for_certification_test, ForegroundIoLaneKind,
};
use crate::{
    admit_queue_execution_plan, group_ready_queue_pair, QueueExecutionAdmissionDenial,
    QueueExecutionAdmissionRequest, QueueGroupingBasis, QueueGroupingDenial, QueueGroupingOutcome,
    QueueRecoveryOrdering, QueueWorkClass, QueueWorkDeclaration, QueueWritebackPolicy,
    S6QueueDurabilityClass,
};

#[test]
fn grouping_basis_compatibility_rejects_every_declared_axis() {
    let base = base_grouping_basis();
    for (mutated, expected) in [
        (
            basis_with_security_scope(base),
            QueueGroupingDenial::SecurityScopeMismatch,
        ),
        (
            basis_with_tenant_scope(base),
            QueueGroupingDenial::TenantScopeMismatch,
        ),
        (
            basis_with_key_scope(base),
            QueueGroupingDenial::KeyScopeMismatch,
        ),
        (
            basis_with_authenticity(base),
            QueueGroupingDenial::AuthenticityRequirementMismatch,
        ),
        (
            basis_with_durability(base),
            QueueGroupingDenial::DurabilityClassMismatch,
        ),
        (
            basis_with_flush_epoch(base),
            QueueGroupingDenial::FlushEpochMismatch,
        ),
        (
            basis_with_work_class(base),
            QueueGroupingDenial::WorkClassMismatch,
        ),
        (
            basis_with_recovery_ordering(base),
            QueueGroupingDenial::RecoveryOrderingMismatch,
        ),
        (
            basis_with_writeback_policy(base),
            QueueGroupingDenial::WritebackPolicyMismatch,
        ),
    ] {
        assert_eq!(base.compatible_with(mutated), Err(expected));
    }
}

#[test]
fn queue_admission_rejects_basis_axes_that_must_match_declared_work() {
    for (basis, expected) in [
        (
            basis_with_security_scope(base_grouping_basis()),
            QueueGroupingDenial::SecurityScopeMismatch,
        ),
        (
            basis_with_tenant_scope(base_grouping_basis()),
            QueueGroupingDenial::TenantScopeMismatch,
        ),
        (
            basis_with_key_scope(base_grouping_basis()),
            QueueGroupingDenial::KeyScopeMismatch,
        ),
        (
            basis_with_authenticity(base_grouping_basis()),
            QueueGroupingDenial::AuthenticityRequirementMismatch,
        ),
        (
            basis_with_durability(base_grouping_basis()),
            QueueGroupingDenial::DurabilityClassMismatch,
        ),
        (
            basis_with_work_class(base_grouping_basis()),
            QueueGroupingDenial::WorkClassMismatch,
        ),
    ] {
        let denial = admit_plan_with_basis(basis).expect_err("basis mismatch must deny admission");
        assert_eq!(
            denial,
            QueueExecutionAdmissionDenial::GroupingDenied(expected)
        );
    }
}

#[test]
fn ready_grouping_rejects_derived_flush_recovery_and_writeback_axes() {
    for (basis, expected) in [
        (
            basis_with_flush_epoch(base_grouping_basis()),
            QueueGroupingDenial::FlushEpochMismatch,
        ),
        (
            basis_with_recovery_ordering(base_grouping_basis()),
            QueueGroupingDenial::RecoveryOrderingMismatch,
        ),
        (
            basis_with_writeback_policy(base_grouping_basis()),
            QueueGroupingDenial::WritebackPolicyMismatch,
        ),
    ] {
        let first = admit_plan_with_basis(base_grouping_basis()).expect("base plan should admit");
        let second = admit_plan_with_basis(basis).expect("derived-axis plan should admit");
        let QueueGroupingOutcome::Denied(denied) = group_ready_queue_pair(first, second) else {
            panic!("derived grouping-axis mismatch must deny grouping");
        };
        assert_eq!(denied.denial(), expected);
    }
}

fn admit_plan_with_basis(
    basis: QueueGroupingBasis,
) -> Result<crate::QueueExecutionReadyPlan, QueueExecutionAdmissionDenial> {
    let reservation = admitted_point_read_reservation_for_certification_test();
    let budget = point_read_budget();
    let work = QueueWorkDeclaration::foreground(
        reservation.execution_ready(),
        S6QueueDurabilityClass::ReadOnly,
        budget,
    )
    .with_grouping_basis(basis);
    let backend = backend_for(work);
    let work = work.with_secure_io_scope(secure_io_for_work(work, &backend));
    admit_queue_execution_plan(QueueExecutionAdmissionRequest::new(
        work,
        &backend,
        policy_receipt(budget),
    ))
}

fn base_grouping_basis() -> QueueGroupingBasis {
    let reservation = admitted_point_read_reservation_for_certification_test();
    grouping_for(reservation.security_scope_identity())
}

fn basis_with_security_scope(base: QueueGroupingBasis) -> QueueGroupingBasis {
    let identity = admitted_wrong_s6_io_qos_security_scope_for_test().identity();
    QueueGroupingBasis::new(
        identity,
        identity.tenant_scope(),
        identity.key_scope(),
        identity.authenticity_requirement(),
        base.durability_class(),
        base.flush_epoch(),
        base.work_class(),
        base.recovery_ordering(),
        base.writeback_policy(),
    )
}

fn basis_with_tenant_scope(base: QueueGroupingBasis) -> QueueGroupingBasis {
    rebuild_basis(base, BasisAxis::Tenant)
}

fn basis_with_key_scope(base: QueueGroupingBasis) -> QueueGroupingBasis {
    rebuild_basis(base, BasisAxis::Key)
}

fn basis_with_authenticity(base: QueueGroupingBasis) -> QueueGroupingBasis {
    rebuild_basis(base, BasisAxis::Authenticity)
}

fn basis_with_durability(base: QueueGroupingBasis) -> QueueGroupingBasis {
    rebuild_basis(base, BasisAxis::Durability)
}

fn basis_with_flush_epoch(base: QueueGroupingBasis) -> QueueGroupingBasis {
    rebuild_basis(base, BasisAxis::FlushEpoch)
}

fn basis_with_work_class(base: QueueGroupingBasis) -> QueueGroupingBasis {
    rebuild_basis(base, BasisAxis::WorkClass)
}

fn basis_with_recovery_ordering(base: QueueGroupingBasis) -> QueueGroupingBasis {
    rebuild_basis(base, BasisAxis::RecoveryOrdering)
}

fn basis_with_writeback_policy(base: QueueGroupingBasis) -> QueueGroupingBasis {
    rebuild_basis(base, BasisAxis::WritebackPolicy)
}

fn rebuild_basis(base: QueueGroupingBasis, axis: BasisAxis) -> QueueGroupingBasis {
    QueueGroupingBasis::new(
        base.security_scope_identity(),
        if matches!(axis, BasisAxis::Tenant) {
            StoreTenantScope::TenantPhysicalBoundary
        } else {
            base.tenant_scope()
        },
        if matches!(axis, BasisAxis::Key) {
            StoreKeyScope::BackupExportEnvelope
        } else {
            base.key_scope()
        },
        if matches!(axis, BasisAxis::Authenticity) {
            StoreAuthenticityRequirement::required(
                StoreAuthenticityRequirementClass::AuthenticatedFrame,
            )
        } else {
            base.authenticity_requirement()
        },
        if matches!(axis, BasisAxis::Durability) {
            S6QueueDurabilityClass::PlatformDurable
        } else {
            base.durability_class()
        },
        if matches!(axis, BasisAxis::FlushEpoch) {
            base.flush_epoch() + 1
        } else {
            base.flush_epoch()
        },
        if matches!(axis, BasisAxis::WorkClass) {
            QueueWorkClass::Foreground(ForegroundIoLaneKind::RangeRead)
        } else {
            base.work_class()
        },
        if matches!(axis, BasisAxis::RecoveryOrdering) {
            QueueRecoveryOrdering::WalBeforeData
        } else {
            base.recovery_ordering()
        },
        if matches!(axis, BasisAxis::WritebackPolicy) {
            QueueWritebackPolicy::Immediate
        } else {
            base.writeback_policy()
        },
    )
}

#[derive(Clone, Copy)]
enum BasisAxis {
    Tenant,
    Key,
    Authenticity,
    Durability,
    FlushEpoch,
    WorkClass,
    RecoveryOrdering,
    WritebackPolicy,
}
