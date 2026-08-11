//! Typed progression for one owner-validated Relational commit.

mod commit_execution;
mod evidence_seal;

pub(in crate::domain_computation::primary_graph::provider) use commit_execution::WorthQueryCommittedApplicationPublicationSeal;

pub(in crate::domain_computation::primary_graph) use evidence_seal::{
    WorthQueryMutationWorkCommitSeal, WorthQueryPrimaryGraphCommitEvidence,
};

use super::super::WorthQueryPrimaryGraphProvider;
use super::prepared_session::WorthQueryPreparedApplicationCommit;
use crate::domain_computation::WorthQueryProviderSessionFailure;

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
) -> Result<String, WorthQueryProviderSessionFailure> {
    provider.graph.with_runtime_mut(|runtime| {
        let committed = commit_execution::commit(
            runtime,
            prepared,
            WorthQueryCommitProgressionMint::witness(),
        )?;
        let evidence = evidence_seal::seal(&committed);
        let published = committed.publish(provider, runtime, evidence)?;
        commit_execution::encode(provider, published)
    })
}
