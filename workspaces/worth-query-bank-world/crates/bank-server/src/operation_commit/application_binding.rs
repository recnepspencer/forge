//! Query application bindings derived from one admitted Bank proposal.

use super::BankCommitPreparationDenial;

pub(super) fn application_idempotency(
    proposal: &bank_domain::proposals::BankInvariantApprovedProposal,
) -> worth_query_host::facade::primary_graph::WorthQueryApplicationIdempotencyBinding {
    worth_query_host::facade::primary_graph::WorthQueryApplicationIdempotencyBinding::new(
        proposal.idempotency_key_identity().bytes(),
        proposal.idempotency_intent().bytes(),
    )
}

pub(super) fn entity_key<Entity>(
    value: String,
) -> Result<
    worth_query_host::facade::primary_graph::WorthQueryApplicationEntityKey<
        bank_domain::schema::BankSchema,
        Entity,
    >,
    BankCommitPreparationDenial,
> {
    worth_query_host::facade::primary_graph::WorthQueryApplicationEntityKey::new(value)
        .map_err(|_| BankCommitPreparationDenial::InvalidProposalShape)
}
