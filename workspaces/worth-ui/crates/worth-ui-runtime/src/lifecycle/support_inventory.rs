#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiRuntimeSupportRow {
    subsystem: &'static str,
}

impl WorthUiRuntimeSupportRow {
    pub fn new(subsystem: &'static str) -> Self {
        Self { subsystem }
    }

    pub fn subsystem(&self) -> &'static str {
        self.subsystem
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiRuntimeSupportInventoryFields {
    pub dsl_package: WorthUiRuntimeSupportRow,
    pub inspection: WorthUiRuntimeSupportRow,
    pub query_binding: WorthUiRuntimeSupportRow,
    pub host_contract: WorthUiRuntimeSupportRow,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiRuntimeSupportInventory {
    rows: [WorthUiRuntimeSupportRow; 4],
}

impl WorthUiRuntimeSupportInventory {
    pub fn new(fields: WorthUiRuntimeSupportInventoryFields) -> Self {
        Self {
            rows: [
                fields.dsl_package,
                fields.inspection,
                fields.query_binding,
                fields.host_contract,
            ],
        }
    }

    pub fn rows(&self) -> &[WorthUiRuntimeSupportRow] {
        &self.rows
    }
}
