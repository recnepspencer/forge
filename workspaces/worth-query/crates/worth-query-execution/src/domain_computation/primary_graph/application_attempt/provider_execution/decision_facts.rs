use super::super::super::provider::WorthQueryPrimaryGraphApplicationDecisionFact;
use super::super::WorthQueryApplicationObservedFact;
use crate::domain_computation::primary_graph::authorization::WorthQueryAuthorizationCommitDependency;
use crate::domain_computation::{WorthQueryDecisionFactLocator, WorthQueryDecisionFactRequest};
use worth_query_installation::facade::{
    APPLICATION_AUTHORIZATION_FACT_FAMILY, APPLICATION_DECISION_FACT_FAMILY,
};

pub(super) fn bind_provider_decision_facts(
    application: Vec<WorthQueryApplicationObservedFact>,
    authorization: Vec<WorthQueryAuthorizationCommitDependency>,
) -> Result<
    (
        Vec<WorthQueryPrimaryGraphApplicationDecisionFact>,
        Vec<WorthQueryDecisionFactRequest>,
    ),
    (),
> {
    let application_count = application.len();
    let facts: Vec<WorthQueryPrimaryGraphApplicationDecisionFact> = application
        .into_iter()
        .map(WorthQueryPrimaryGraphApplicationDecisionFact::application)
        .chain(
            authorization
                .into_iter()
                .enumerate()
                .map(|(ordinal, observation)| {
                    WorthQueryPrimaryGraphApplicationDecisionFact::authorization(
                        ordinal,
                        observation,
                    )
                }),
        )
        .collect::<Vec<_>>();
    let requests = facts
        .iter()
        .enumerate()
        .map(|(index, fact)| {
            let family = if index < application_count {
                APPLICATION_DECISION_FACT_FAMILY
            } else {
                APPLICATION_AUTHORIZATION_FACT_FAMILY
            };
            WorthQueryDecisionFactLocator::structural_proof(fact.locator_identity())
                .map_err(|_| ())
                .and_then(|locator| {
                    WorthQueryDecisionFactRequest::new(family, locator).map_err(|_| ())
                })
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok((facts, requests))
}
