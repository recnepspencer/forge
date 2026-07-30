use worth_foundational::FoundationalPolicyAdmissionReceipt;
use worth_store_physical_backend::{BackendTargetProfile, CapabilityEvidenceClass};

use crate::{BackgroundResourceBudget, IoSchedulerBackendCapabilityAdmission};

use super::{QueueGroupingBasis, QueuePolicyAdmissionReceipt, QueueWorkDeclaration};

#[derive(Debug)]
pub(crate) struct ValidatedQueueExecutionAdmission {
    pub(crate) work: QueueWorkDeclaration,
    pub(crate) backend_profile: BackendTargetProfile,
    pub(crate) backend_evidence_class: CapabilityEvidenceClass,
    pub(crate) policy_receipt: FoundationalPolicyAdmissionReceipt,
    pub(crate) grouping_basis: QueueGroupingBasis,
    pub(crate) admitted_budget: BackgroundResourceBudget,
}

impl ValidatedQueueExecutionAdmission {
    pub(crate) fn from_checked_request(
        policy_receipt: QueuePolicyAdmissionReceipt,
        backend: &IoSchedulerBackendCapabilityAdmission,
        grouping_basis: QueueGroupingBasis,
        admitted_budget: BackgroundResourceBudget,
    ) -> Self {
        let (work, policy_receipt) = policy_receipt.into_parts();
        Self {
            work,
            backend_profile: backend.profile(),
            backend_evidence_class: backend.evidence_class(),
            policy_receipt,
            grouping_basis,
            admitted_budget,
        }
    }
}
