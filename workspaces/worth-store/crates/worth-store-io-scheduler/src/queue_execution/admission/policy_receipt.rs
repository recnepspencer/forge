use worth_foundational::{
    FoundationalPerformanceBoundary, FoundationalPerformanceEvidenceStrength,
    FoundationalPerformanceFallbackDebtPosture, FoundationalPerformanceWorkClass,
    FoundationalPolicyAdmissionReceipt,
};

use super::{
    QueueDurabilityClass, QueueExecutionAdmissionDenial, QueueWorkClass, QueueWorkDeclaration,
};

/// Policy admission sealed to one exact scheduler workload declaration.
#[derive(Clone, Debug)]
pub struct QueuePolicyAdmissionReceipt {
    work: QueueWorkDeclaration,
    foundational: FoundationalPolicyAdmissionReceipt,
}

impl QueuePolicyAdmissionReceipt {
    pub(super) fn work(&self) -> QueueWorkDeclaration {
        self.work.clone()
    }

    pub(super) fn into_foundational(self) -> FoundationalPolicyAdmissionReceipt {
        self.foundational
    }

    pub(super) fn foundational(&self) -> &FoundationalPolicyAdmissionReceipt {
        &self.foundational
    }
}

pub fn admit_queue_policy_receipt(
    work: QueueWorkDeclaration,
    receipt: FoundationalPolicyAdmissionReceipt,
) -> Result<QueuePolicyAdmissionReceipt, QueueExecutionAdmissionDenial> {
    let expected_work = expected_work_class(&work);
    let exact_policy_context = receipt.boundary()
        == FoundationalPerformanceBoundary::AuthoritativeExecution
        && receipt.evidence_strength()
            == FoundationalPerformanceEvidenceStrength::RuntimePolicyAdmission
        && receipt.claim().fallback_debt() == FoundationalPerformanceFallbackDebtPosture::Verified
        && receipt.stronger_evidence_still_required()
            == FoundationalPerformanceEvidenceStrength::CounterBackedExecutionReceipt
        && receipt.included_work() == [expected_work]
        && !receipt.excluded_work().contains(&expected_work)
        && receipt.denied_work().is_empty()
        && receipt.widened_work().is_empty();
    if !exact_policy_context {
        return Err(QueueExecutionAdmissionDenial::PolicyReceiptContextMismatch { expected_work });
    }
    Ok(QueuePolicyAdmissionReceipt {
        work,
        foundational: receipt,
    })
}

pub(super) const fn expected_work_class(
    work: &QueueWorkDeclaration,
) -> FoundationalPerformanceWorkClass {
    if matches!(work.class(), QueueWorkClass::Background(_)) {
        return FoundationalPerformanceWorkClass::ValidationPlanning;
    }
    match work.durability_class() {
        QueueDurabilityClass::ReadOnly => FoundationalPerformanceWorkClass::AuthoritativeRead,
        QueueDurabilityClass::BufferedWrite | QueueDurabilityClass::WalCommit => {
            FoundationalPerformanceWorkClass::AuthoritativeMutation
        }
        QueueDurabilityClass::PlatformDurable => {
            FoundationalPerformanceWorkClass::PublicationDelivery
        }
    }
}
