use crate::basis_lifecycle::BasisOperationLane;

use super::super::WorthQueryBoundDomainOperation;
use super::denial::{
    WorthQueryCompatibilityCounters, WorthQueryCompatibilityDenial,
    WorthQueryCompatibilityDenialKind,
};

pub(super) fn require_distinct_capabilities<D, O, F, L: BasisOperationLane>(
    subject: &WorthQueryBoundDomainOperation<D, O, F, L>,
    candidate: &WorthQueryBoundDomainOperation<D, O, F, L>,
    counters: &mut WorthQueryCompatibilityCounters,
) -> Result<(), WorthQueryCompatibilityDenial> {
    counters.retained_authority_checks += 2;
    if subject.capability_identity() == candidate.capability_identity() {
        Err(WorthQueryCompatibilityDenial::plain(
            WorthQueryCompatibilityDenialKind::RelationshipRule,
            "this relationship requires two distinct bound capabilities",
            *counters,
        ))
    } else {
        Ok(())
    }
}
