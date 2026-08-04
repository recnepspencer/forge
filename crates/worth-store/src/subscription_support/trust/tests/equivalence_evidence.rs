use super::super::{SupportTrustEquivalenceContract, SupportTrustEquivalenceLane};
use super::operational_basis::basis;
use crate::subscription_support::{
    SubscriptionResumeClassification, SubscriptionSupportArtifactId, SubscriptionSupportFamilyId,
    SubscriptionSupportOperationalVerdict, SubscriptionSupportRole,
};

pub(super) fn exact_equivalence_contract(
    lane: SupportTrustEquivalenceLane,
) -> SupportTrustEquivalenceContract {
    let source_basis = basis();
    SupportTrustEquivalenceContract::new(
        lane,
        source_basis,
        SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
        SubscriptionSupportRole::ExactContinuation,
        SubscriptionSupportArtifactId("artifact:trust:phase-1".into()),
        "basis:trust",
        "cursor:trust",
        "checkpoint:trust",
        "compatibility:trust",
        "portability:trust",
        SubscriptionResumeClassification::Exact,
        SubscriptionSupportOperationalVerdict::ExactResumePreserved,
        "equivalence:exact",
    )
    .unwrap()
}
