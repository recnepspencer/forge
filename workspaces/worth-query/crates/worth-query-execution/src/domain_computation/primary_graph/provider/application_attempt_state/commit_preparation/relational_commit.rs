//! Typed progression for one owner-validated Relational commit.

mod commit_execution;
mod evidence_seal;

pub(in crate::domain_computation::primary_graph) use commit_execution::WorthQueryPrimaryGraphCommittedApplication;

pub(in crate::domain_computation::primary_graph) use evidence_seal::{
    WorthQueryMutationWorkCommitSeal, WorthQueryPrimaryGraphCommitEvidence,
};

use super::super::super::WorthQueryPrimaryGraphProvider;
use super::WorthQueryPreparedApplicationCommit;
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
) -> Result<
    crate::domain_computation::WorthQueryProviderTerminalDescription,
    WorthQueryProviderSessionFailure,
> {
    provider.graph.with_runtime_mut(|runtime| {
        let committed = commit_execution::commit(
            runtime,
            prepared,
            WorthQueryCommitProgressionMint::witness(),
        )?;
        let evidence = evidence_seal::seal(&committed);
        committed.publish_and_encode(provider, runtime, evidence)
    })
}
