#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiStateQueryResidueScan {
    scanned_state_receipts: usize,
    scanned_query_bindings: usize,
    orphan_durable_state_count: usize,
    stale_live_binding_count: usize,
    ui_local_query_status_residue_count: usize,
}

impl WorthUiStateQueryResidueScan {
    pub(crate) fn clean(scanned_state_receipts: usize, scanned_query_bindings: usize) -> Self {
        Self {
            scanned_state_receipts,
            scanned_query_bindings,
            orphan_durable_state_count: 0,
            stale_live_binding_count: 0,
            ui_local_query_status_residue_count: 0,
        }
    }

    pub fn scanned_state_receipts(&self) -> usize {
        self.scanned_state_receipts
    }

    pub fn scanned_query_bindings(&self) -> usize {
        self.scanned_query_bindings
    }

    pub fn orphan_durable_state_count(&self) -> usize {
        self.orphan_durable_state_count
    }

    pub fn stale_live_binding_count(&self) -> usize {
        self.stale_live_binding_count
    }

    pub fn ui_local_query_status_residue_count(&self) -> usize {
        self.ui_local_query_status_residue_count
    }

    pub fn is_clean(&self) -> bool {
        self.orphan_durable_state_count == 0
            && self.stale_live_binding_count == 0
            && self.ui_local_query_status_residue_count == 0
    }
}
