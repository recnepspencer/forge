#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiRuntimeSupportRow {
    subsystem: &'static str,
}

impl WorthUiRuntimeSupportRow {
    pub const fn new(subsystem: &'static str) -> Self {
        Self { subsystem }
    }

    pub fn subsystem(&self) -> &'static str {
        self.subsystem
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiRuntimeSupportInventory {
    rows: [WorthUiRuntimeSupportRow; 4],
}

impl WorthUiRuntimeSupportInventory {
    pub fn rows(&self) -> &[WorthUiRuntimeSupportRow] {
        &self.rows
    }
}

pub const RUNTIME_SUPPORT_INVENTORY: WorthUiRuntimeSupportInventory =
    WorthUiRuntimeSupportInventory {
        rows: [
            WorthUiRuntimeSupportRow::new("dsl_package"),
            WorthUiRuntimeSupportRow::new("inspection"),
            WorthUiRuntimeSupportRow::new("query_binding"),
            WorthUiRuntimeSupportRow::new("host_contract"),
        ],
    };
