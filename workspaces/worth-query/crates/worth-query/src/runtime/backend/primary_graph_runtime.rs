use worth_query_execution::facade::integration::WorthQueryPrimaryGraphIndexRefreshDenial;
use worth_query_execution::facade::integration::WorthQueryPrimaryGraphIntegrationHandle;
use worth_relational::facade::runtime::RelationalRuntime;

/// Move-only ownership transfer for the Relational runtime that will become
/// Query's primary logical graph.
///
/// The private field prevents custom backends from manufacturing support for
/// primary-graph composition. Only Query's bridge-backed backend may surrender
/// the unpublished runtime it already owns.
#[doc(hidden)]
pub struct WorthQueryUnpublishedPrimaryGraphRuntime {
    runtime: RelationalRuntime,
}

impl WorthQueryUnpublishedPrimaryGraphRuntime {
    pub(in crate::runtime) fn new(runtime: RelationalRuntime) -> Self {
        Self { runtime }
    }

    pub(in crate::runtime) fn into_runtime(self) -> RelationalRuntime {
        self.runtime
    }
}

/// Opaque backend capability for the one execution-owned primary graph.
///
/// Product consumers receive neither this handle nor raw Relational access.
/// It exists only to reconnect Query's ordinary backend after execution has
/// consumed installation authority and published the graph.
#[doc(hidden)]
#[derive(Clone)]
pub struct WorthQueryPrimaryGraphBackendHandle {
    integration: WorthQueryPrimaryGraphIntegrationHandle,
}

impl WorthQueryPrimaryGraphBackendHandle {
    pub(in crate::runtime) fn new(integration: WorthQueryPrimaryGraphIntegrationHandle) -> Self {
        Self { integration }
    }

    pub(in crate::runtime) fn with_runtime<T>(
        &self,
        read: impl FnOnce(&RelationalRuntime) -> T,
    ) -> T {
        self.integration.with_runtime(read)
    }

    pub(in crate::runtime) fn repair_deferred_publication_settlement(
        &self,
        settlement: &worth_relational::facade::publication::DeferredPublicationSettlement,
    ) -> Result<
        worth_relational::facade::history::RelationalCommitReceipt,
        crate::runtime::WorthQuerySettlementRepairError,
    > {
        self.integration
            .execute_mutation_with_index_refresh(|runtime| {
                runtime.repair_deferred_publication_settlement(settlement)
            })
            .map_err(crate::runtime::WorthQuerySettlementRepairError::PrimaryGraphIndexRefresh)?
            .map_err(Into::into)
    }

    pub(in crate::runtime) fn repair_pending_publication_settlement(
        &self,
        commit_id: worth_relational::facade::history::CommitId,
    ) -> Result<
        worth_relational::facade::history::RelationalCommitReceipt,
        crate::runtime::WorthQuerySettlementRepairError,
    > {
        self.integration
            .execute_mutation_with_index_refresh(|runtime| {
                runtime.repair_pending_publication_settlement(commit_id)
            })
            .map_err(crate::runtime::WorthQuerySettlementRepairError::PrimaryGraphIndexRefresh)?
            .map_err(Into::into)
    }

    pub(in crate::runtime) fn execute_mutation<T, E>(
        &self,
        mutate: impl FnOnce(&mut RelationalRuntime) -> Result<T, E>,
    ) -> Result<Result<T, E>, WorthQueryPrimaryGraphIndexRefreshDenial> {
        self.integration.execute_mutation_with_index_refresh(mutate)
    }
}
