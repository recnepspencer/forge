/// One read borrowed from an exact Query-owned compatibility resource.
pub struct WorthUiQueryLiveRead {
    pub(super) query_read: worth_query::facade::domain::WorthQueryInstalledDomainLiveRead,
}

impl WorthUiQueryLiveRead {
    pub fn result(&self) -> &worth_query::facade::runtime::WorthQueryLiveReadResult {
        self.query_read.result()
    }

    pub fn installation_receipt(
        &self,
    ) -> &worth_query::facade::domain::WorthQueryInstalledDomainExecutionReceipt {
        self.query_read.receipt()
    }
}
