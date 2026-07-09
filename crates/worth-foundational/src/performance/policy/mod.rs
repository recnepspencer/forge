mod receipt;
mod vocabulary;

pub use receipt::{
    FoundationalPolicyAdmissionReceipt, FoundationalPolicyAdmissionReceiptBuilder,
    FoundationalPolicyAdmissionReceiptConstructionDenial,
};
pub use vocabulary::{
    foundational_performance_budget_definitions, FoundationalPerformanceBudgetDecision,
    FoundationalPerformanceBudgetDefinition, FoundationalPerformanceBudgetKind,
};

use crate::performance::FoundationalPolicyAdmissionPerformanceClaim;

pub fn policy_admission_receipt(
    claim: FoundationalPolicyAdmissionPerformanceClaim,
) -> FoundationalPolicyAdmissionReceiptBuilder {
    FoundationalPolicyAdmissionReceiptBuilder::new(claim)
}
