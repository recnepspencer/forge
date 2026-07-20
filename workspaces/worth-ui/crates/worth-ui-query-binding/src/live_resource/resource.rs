use std::fmt;

use worth_query::facade::{domain, read, runtime};

use crate::{
    WorthUiDomainEntry, WorthUiQueryLiveCloseOutcome, WorthUiQueryLiveProjectionOutcome,
    WorthUiQueryLiveRead, WorthUiQueryViewDefinition,
};

#[must_use = "Query live resources remain active until admitted, closed, or abandoned"]
pub struct WorthUiQueryLiveResource {
    definition: WorthUiQueryViewDefinition,
    query_handle: Box<domain::WorthQueryInstalledDomainLiveHandle<WorthUiDomainEntry>>,
}

impl WorthUiQueryLiveResource {
    pub(crate) fn new(
        definition: WorthUiQueryViewDefinition,
        query_handle: domain::WorthQueryInstalledDomainLiveHandle<WorthUiDomainEntry>,
    ) -> Self {
        Self {
            definition,
            query_handle: Box::new(query_handle),
        }
    }

    pub fn definition(&self) -> &WorthUiQueryViewDefinition {
        &self.definition
    }

    pub fn read(
        &self,
        workspace: &mut runtime::WorthQueryWorkspace,
    ) -> Result<WorthUiQueryLiveRead, Box<domain::WorthQueryInstalledDomainLiveOperationError>>
    {
        self.query_handle
            .read(workspace)
            .map(|query_read| WorthUiQueryLiveRead { query_read })
            .map_err(Box::new)
    }

    pub fn project(
        &self,
        read: &WorthUiQueryLiveRead,
        declaration: read::WorthQueryProjectionDeclaration,
    ) -> WorthUiQueryLiveProjectionOutcome {
        WorthUiQueryLiveProjectionOutcome::from_installed(
            self.definition.clone(),
            self.query_handle.project(&read.query_read, declaration),
        )
    }

    pub fn observe(
        &self,
        workspace: &mut runtime::WorthQueryWorkspace,
    ) -> Result<
        worth_query::facade::live::WorthQueryManagedLiveLifecycleObservation,
        Box<domain::WorthQueryInstalledDomainLiveOperationError>,
    > {
        self.query_handle.observe(workspace).map_err(Box::new)
    }

    pub fn close(
        self,
        workspace: &mut runtime::WorthQueryWorkspace,
    ) -> WorthUiQueryLiveCloseOutcome {
        let Self {
            definition,
            query_handle,
        } = self;
        WorthUiQueryLiveCloseOutcome::from_query(definition, (*query_handle).close(workspace))
    }

    pub(crate) fn installed_authority(&self) -> &domain::WorthQueryInstalledDomainAuthorityWitness {
        self.query_handle
            .installation_receipt()
            .installed_authority()
    }

    pub(crate) fn matches_projection_resource(
        &self,
        receipt: &domain::WorthQueryInstalledDomainExecutionReceipt,
    ) -> bool {
        self.query_handle
            .installation_receipt()
            .shares_managed_live_resource_with(receipt)
    }
}

impl fmt::Debug for WorthUiQueryLiveResource {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("WorthUiQueryLiveResource")
            .field("definition", &self.definition)
            .field("query_resource", &"sealed")
            .finish()
    }
}
