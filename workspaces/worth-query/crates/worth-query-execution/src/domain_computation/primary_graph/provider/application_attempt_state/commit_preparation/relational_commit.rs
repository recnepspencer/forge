//! Typed progression for one owner-validated Relational commit.

mod commit_execution;
mod evidence_seal;

pub(in crate::domain_computation::primary_graph) use commit_execution::WorthQueryPrimaryGraphCommittedApplication;

pub(in crate::domain_computation::primary_graph) use evidence_seal::{
    WorthQueryMutationWorkCommitSeal, WorthQueryPrimaryGraphCommitEvidence,
};

use super::super::super::WorthQueryPrimaryGraphProvider;
use super::WorthQueryPreparedApplicationCommit;
pub(super) struct WorthQueryCommitProgressionMint {
    _private: (),
}

impl WorthQueryCommitProgressionMint {
    fn witness() -> Self {
        Self { _private: () }
    }
}

pub(super) fn commit_owner_validated(
    provider: &WorthQueryPrimaryGraphProvider,
    prepared: WorthQueryPreparedApplicationCommit,
) -> Result<
    crate::domain_computation::WorthQueryProviderTerminalDescription,
    crate::domain_computation::WorthQueryProviderSessionCommitStop,
> {
    provider.graph.with_runtime_mut(|runtime| {
        let performed = commit_execution::commit(
            runtime,
            prepared,
            WorthQueryCommitProgressionMint::witness(),
        )?;
        let evidence = evidence_seal::seal(performed.committed());
        let (committed, settlement_deferred) = performed.into_parts();
        let publication = committed.publish_and_encode(provider, runtime, evidence);
        match (publication, settlement_deferred) {
            (Ok(_), Some(deferred)) => Err(
                crate::domain_computation::WorthQueryProviderSessionCommitStop::SettlementDeferred(
                    deferred,
                ),
            ),
            (Err(failure), Some(deferred)) => Err(
                crate::domain_computation::WorthQueryProviderSessionCommitStop::SettlementDeferred(
                    deferred.with_publication_failure(&failure),
                ),
            ),
            (Ok(terminal), None) => Ok(terminal),
            (Err(failure), None) => {
                Err(crate::domain_computation::WorthQueryProviderSessionCommitStop::Denied(failure))
            }
        }
    })
}
