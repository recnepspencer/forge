//! Exact recovery of a co-committed aftermath causal fact.

use crate::domain_computation::application_aftermath::{
    WorthQueryCommittedAftermathCausality, WorthQueryPendingAftermathCausality,
};
use crate::domain_computation::primary_graph::provider::{
    WorthQueryPrimaryGraphCommittedApplication, WorthQueryPrimaryGraphProvider,
};

pub(super) fn resolve_exact_committed_aftermath(
    provider: &WorthQueryPrimaryGraphProvider,
    pending: Option<&WorthQueryPendingAftermathCausality>,
    receipt: &WorthQueryPrimaryGraphCommittedApplication,
) -> Result<
    Option<WorthQueryCommittedAftermathCausality>,
    crate::domain_computation::primary_graph::WorthQueryAftermathCausalityReadDenial,
> {
    let Some(pending) = pending else {
        return Ok(None);
    };
    provider
        .resolve_aftermath_causality(pending, receipt.application_outcome_identity())
        ?
        .filter(|causality| causality.child() == receipt.commit_reference())
        .map(Some)
        .ok_or(crate::domain_computation::primary_graph::WorthQueryAftermathCausalityReadDenial::Unavailable)
}
