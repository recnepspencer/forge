//! Native-owned committed atlas transaction observation.

#[derive(Default)]
pub(crate) struct UiNativeTextAtlasActivity {
    committed_transactions: u64,
}

impl UiNativeTextAtlasActivity {
    pub(crate) fn record_committed_transaction(&mut self) {
        self.committed_transactions = self.committed_transactions.saturating_add(1);
    }

    pub(crate) const fn committed_transactions(&self) -> u64 {
        self.committed_transactions
    }
}
