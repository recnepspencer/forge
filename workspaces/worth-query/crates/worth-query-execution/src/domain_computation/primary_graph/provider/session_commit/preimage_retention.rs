//! Fail-closed retention from the admitted decision read-set.

use crate::domain_computation::application_aftermath::{
    WorthQueryPreImageRetentionDenial, WorthQueryRetainedPreImage,
};
use crate::domain_computation::{
    WorthQueryProviderSessionFailure, WorthQueryProviderSessionProtocolStage,
};

use super::super::WorthQueryPrimaryGraphApplicationAttempt;

pub(super) fn retain_attempt_preimage(
    attempt: &WorthQueryPrimaryGraphApplicationAttempt,
    footprint: &worth_relational::facade::transactions::ValidatedMutationFootprint,
) -> Result<Option<WorthQueryRetainedPreImage>, WorthQueryProviderSessionFailure> {
    let Some(demand) = attempt.preimage_demand.as_ref() else {
        return Ok(None);
    };
    crate::domain_computation::application_aftermath::retain_preimage_from_observed_facts(
        demand,
        &retention_candidates(attempt),
        footprint,
    )
    .map(Some)
    .map_err(retention_failure)
}

fn retention_failure(
    denial: WorthQueryPreImageRetentionDenial,
) -> WorthQueryProviderSessionFailure {
    super::provider_failure(
        WorthQueryProviderSessionProtocolStage::Commit,
        match denial {
            WorthQueryPreImageRetentionDenial::MissingDemandedField => {
                "recorded inverse demands an unobserved exact mutated field"
            }
            WorthQueryPreImageRetentionDenial::ExceedsByteBound => {
                "retained pre-image exceeds the installed demand byte bound"
            }
            WorthQueryPreImageRetentionDenial::EmptyDemand => {
                "installed recorded inverse declares an empty pre-image demand"
            }
            WorthQueryPreImageRetentionDenial::AmbiguousDemandedField => {
                "recorded inverse has several admitted observations for one exact mutated field"
            }
            WorthQueryPreImageRetentionDenial::NoMutatedRecord => {
                "recorded inverse has no existing mutated field to retain"
            }
        },
    )
}

fn retention_candidates(
    attempt: &WorthQueryPrimaryGraphApplicationAttempt,
) -> Vec<crate::domain_computation::application_aftermath::WorthQueryObservedPreImageCandidate> {
    attempt
        .facts
        .values()
        .filter_map(observed_field_candidate)
        .collect()
}

fn observed_field_candidate(
    fact: &super::super::WorthQueryPrimaryGraphApplicationDecisionFact,
) -> Option<crate::domain_computation::application_aftermath::WorthQueryObservedPreImageCandidate> {
    use crate::domain_computation::primary_graph::application_attempt::WorthQueryApplicationObservedFact;

    let super::super::WorthQueryPrimaryGraphApplicationDecisionFact::Application(
        WorthQueryApplicationObservedFact::Field {
            locator,
            value,
            entity_id,
            kind,
            ..
        },
    ) = fact
    else {
        return None;
    };
    crate::domain_computation::application_aftermath::demanded_field_slot(locator.field_path())?;
    Some(
        crate::domain_computation::application_aftermath::WorthQueryObservedPreImageCandidate::from_observed_field(
            locator.clone(),
            value.clone(),
            *entity_id,
            *kind,
        ),
    )
}
