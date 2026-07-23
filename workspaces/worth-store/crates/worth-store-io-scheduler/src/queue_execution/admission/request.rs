use worth_foundational::{
    FoundationalPerformanceBudgetKind, FoundationalPolicyAdmissionReceipt,
};

use crate::{IoResourceUnitKind, IoSchedulerBackendCapabilityAdmission};
use crate::{
    IoSchedulerBackendCapabilityRequirement, SecureIoOperation, SecureIoPreservationDenial,
};

use super::{
    QueueExecutionAdmissionDenial, QueueExecutionReadyPlan, QueuePolicyAdmissionReceipt,
    QueueWorkDeclaration,
};

#[derive(Clone, Debug)]
pub struct QueueExecutionAdmissionRequest<'a> {
    work: QueueWorkDeclaration,
    backend: &'a IoSchedulerBackendCapabilityAdmission,
    policy_receipt: QueuePolicyAdmissionReceipt,
}

impl<'a> QueueExecutionAdmissionRequest<'a> {
    pub fn new(
        work: QueueWorkDeclaration,
        backend: &'a IoSchedulerBackendCapabilityAdmission,
        policy_receipt: QueuePolicyAdmissionReceipt,
    ) -> Self {
        Self {
            work,
            backend,
            policy_receipt,
        }
    }
}

pub fn admit_queue_execution_plan(
    request: QueueExecutionAdmissionRequest<'_>,
) -> Result<QueueExecutionReadyPlan, QueueExecutionAdmissionDenial> {
    if request.policy_receipt.work() != request.work {
        return Err(QueueExecutionAdmissionDenial::PolicyReceiptContextMismatch {
            expected_work: super::policy_receipt::expected_work_class(request.work),
        });
    }
    let budget = request.work.requested_budget();
    if budget.is_empty() {
        return Err(QueueExecutionAdmissionDenial::MissingQueueWorkBudget);
    }
    let grouping_basis = request
        .work
        .grouping_basis()
        .ok_or(QueueExecutionAdmissionDenial::MissingGroupingBasis)?;
    let security_identity = request.work.security_scope_identity();
    if grouping_basis.security_scope_identity() != request.work.security_scope_identity() {
        return Err(QueueExecutionAdmissionDenial::GroupingDenied(
            super::QueueGroupingDenial::SecurityScopeMismatch,
        ));
    }
    if grouping_basis.tenant_scope() != security_identity.tenant_scope() {
        return Err(QueueExecutionAdmissionDenial::GroupingDenied(
            super::QueueGroupingDenial::TenantScopeMismatch,
        ));
    }
    if grouping_basis.key_scope() != security_identity.key_scope() {
        return Err(QueueExecutionAdmissionDenial::GroupingDenied(
            super::QueueGroupingDenial::KeyScopeMismatch,
        ));
    }
    if grouping_basis.authenticity_requirement() != security_identity.authenticity_requirement() {
        return Err(QueueExecutionAdmissionDenial::GroupingDenied(
            super::QueueGroupingDenial::AuthenticityRequirementMismatch,
        ));
    }
    if grouping_basis.durability_class() != request.work.durability_class() {
        return Err(QueueExecutionAdmissionDenial::GroupingDenied(
            super::QueueGroupingDenial::DurabilityClassMismatch,
        ));
    }
    if grouping_basis.work_class() != request.work.class() {
        return Err(QueueExecutionAdmissionDenial::GroupingDenied(
            super::QueueGroupingDenial::WorkClassMismatch,
        ));
    }
    if request.backend.requirement() != request.work.backend_requirement() {
        return Err(QueueExecutionAdmissionDenial::BackendRequirementMismatch {
            required: request.work.backend_requirement(),
            admitted: request.backend.requirement(),
        });
    }
    require_secure_io_preservation(&request)?;
    require_policy_receipt(request.policy_receipt.foundational(), budget)?;
    let policy_receipt = request.policy_receipt.into_foundational();
    Ok(super::AdmittedQueueExecutionPlan::new(
        request.work,
        request.backend.profile(),
        request.backend.evidence_class(),
        policy_receipt,
        grouping_basis,
        budget,
    )
    .into_execution_ready())
}

fn require_secure_io_preservation(
    request: &QueueExecutionAdmissionRequest<'_>,
) -> Result<(), QueueExecutionAdmissionDenial> {
    let expected = expected_secure_operation(request);
    if !queue_work_requires_secure_io(request) {
        return Ok(());
    }
    let Some(secure_io) = request.work.secure_io() else {
        return Err(QueueExecutionAdmissionDenial::MissingSecureIoPreservation);
    };
    if secure_io.identity() != request.work.security_scope_identity() {
        return Err(QueueExecutionAdmissionDenial::SecureIoDenied(
            SecureIoPreservationDenial::ScopeMismatch {
                operation: secure_io.operation(),
            },
        ));
    }
    if secure_io.backend_requirement() != request.backend.requirement() {
        return Err(QueueExecutionAdmissionDenial::SecureIoDenied(
            SecureIoPreservationDenial::BackendRequirementMismatch {
                required: secure_io.backend_requirement(),
                admitted: request.backend.requirement(),
            },
        ));
    }
    if secure_io.operation() != expected {
        return Err(QueueExecutionAdmissionDenial::SecureIoDenied(
            SecureIoPreservationDenial::OperationMismatch {
                expected,
                actual: secure_io.operation(),
            },
        ));
    }
    Ok(())
}

fn queue_work_requires_secure_io(request: &QueueExecutionAdmissionRequest<'_>) -> bool {
    let budget = request.work.requested_budget();
    request.backend.requirement() == IoSchedulerBackendCapabilityRequirement::SecureFrameIo
        || budget.read_ahead_window() > 0
        || budget.write_back_window() > 0
        || request.work.secure_io().is_some()
        || matches!(
            request.work.class(),
            crate::QueueWorkClass::Background(
                crate::BackgroundIoPressureClass::BackupPrepRead
                    | crate::BackgroundIoPressureClass::RepairScan
                    | crate::BackgroundIoPressureClass::VerificationPressure
            )
        )
}

fn expected_secure_operation(request: &QueueExecutionAdmissionRequest<'_>) -> SecureIoOperation {
    if matches!(
        request.work.class(),
        crate::QueueWorkClass::Background(crate::BackgroundIoPressureClass::RepairScan)
    ) {
        return SecureIoOperation::RepairScan;
    }
    if matches!(
        request.work.class(),
        crate::QueueWorkClass::Background(crate::BackgroundIoPressureClass::VerificationPressure)
    ) {
        return SecureIoOperation::VerificationPressure;
    }
    if matches!(request.work.class(), crate::QueueWorkClass::Background(_)) {
        return SecureIoOperation::BackgroundLease;
    }
    let budget = request.work.requested_budget();
    if budget.read_ahead_window() > 0 {
        SecureIoOperation::ReadAhead
    } else if budget.write_back_window() > 0 {
        SecureIoOperation::WriteBack
    } else {
        SecureIoOperation::BatchedWrite
    }
}

fn require_policy_receipt(
    receipt: &FoundationalPolicyAdmissionReceipt,
    budget: crate::BackgroundResourceBudget,
) -> Result<(), QueueExecutionAdmissionDenial> {
    if receipt.budget_decisions().is_empty() {
        return Err(QueueExecutionAdmissionDenial::PolicyReceiptHasNoBudgetDecision);
    }
    require_kind(
        receipt,
        FoundationalPerformanceBudgetKind::Breadth,
        breadth(budget)?,
    )?;
    require_kind(
        receipt,
        FoundationalPerformanceBudgetKind::Density,
        density(budget)?,
    )?;
    require_kind(
        receipt,
        FoundationalPerformanceBudgetKind::Locality,
        locality(budget)?,
    )?;
    require_kind(
        receipt,
        FoundationalPerformanceBudgetKind::FreshnessSensitive,
        freshness(budget)?,
    )
}

fn require_kind(
    receipt: &FoundationalPolicyAdmissionReceipt,
    kind: FoundationalPerformanceBudgetKind,
    expected: u32,
) -> Result<(), QueueExecutionAdmissionDenial> {
    if expected == 0 {
        return Ok(());
    }
    let Some(decision) = receipt
        .budget_decisions()
        .iter()
        .find(|decision| decision.kind() == kind)
    else {
        return Err(QueueExecutionAdmissionDenial::PolicyReceiptBudgetMismatch {
            kind,
            expected_requested_units: expected,
            expected_admitted_units: expected,
        });
    };
    if decision.requested_units() != expected || decision.admitted_units() != expected {
        return Err(QueueExecutionAdmissionDenial::PolicyReceiptBudgetMismatch {
            kind,
            expected_requested_units: expected,
            expected_admitted_units: expected,
        });
    }
    Ok(())
}

fn breadth(budget: crate::BackgroundResourceBudget) -> Result<u32, QueueExecutionAdmissionDenial> {
    checked_units(
        IoResourceUnitKind::QueueSlot,
        budget.queue_slots().saturating_add(budget.worker_permits()),
    )
}

fn density(budget: crate::BackgroundResourceBudget) -> Result<u32, QueueExecutionAdmissionDenial> {
    checked_units(
        IoResourceUnitKind::BandwidthToken,
        budget
            .bandwidth_tokens()
            .saturating_add(budget.dirty_page_budget())
            .saturating_add(budget.cache_residency_hints()),
    )
}

fn locality(budget: crate::BackgroundResourceBudget) -> Result<u32, QueueExecutionAdmissionDenial> {
    checked_units(
        IoResourceUnitKind::ReadAheadWindow,
        budget
            .read_ahead_window()
            .saturating_add(budget.write_back_window())
            .saturating_add(budget.reclaim_permits()),
    )
}

fn freshness(
    budget: crate::BackgroundResourceBudget,
) -> Result<u32, QueueExecutionAdmissionDenial> {
    checked_units(
        IoResourceUnitKind::FlushPermit,
        budget.flush_permits().saturating_add(budget.sync_debt()),
    )
}

fn checked_units(
    unit: IoResourceUnitKind,
    value: u64,
) -> Result<u32, QueueExecutionAdmissionDenial> {
    u32::try_from(value).map_err(|_| QueueExecutionAdmissionDenial::ResourceBudgetOverflow(unit))
}
