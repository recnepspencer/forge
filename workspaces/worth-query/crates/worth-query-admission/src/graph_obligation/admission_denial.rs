use worth_foundational::facade::CanonicalDigestDerivationDenial;

use crate::domain_computation::execution_resource_admission::WorthQueryExecutionResourceAdmissionDenial;
use crate::graph_read_access::WorthQueryGraphReadPlanReviewDenialKind;

#[derive(Debug)]
pub enum WorthQueryGraphWorkAdmissionDenial {
    IntentMismatch,
    GraphReadRequirementMismatch,
    GraphReadPlan(WorthQueryGraphReadPlanReviewDenialKind),
    ExecutionResource(WorthQueryExecutionResourceAdmissionDenial),
    ProviderSupportUnavailable,
    CapacityUnavailable,
    CanonicalIdentity(CanonicalDigestDerivationDenial),
}
