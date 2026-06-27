#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiRuntimeSupportRow {
    subsystem: &'static str,
}

impl WorthUiRuntimeSupportRow {
    pub(crate) fn new(subsystem: &'static str) -> Self {
        Self { subsystem }
    }

    pub fn subsystem(&self) -> &'static str {
        self.subsystem
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiRuntimeSupportInventoryFields {
    pub(crate) dsl_package: WorthUiRuntimeSupportRow,
    pub(crate) inspection: WorthUiRuntimeSupportRow,
    pub(crate) query_binding: WorthUiRuntimeSupportRow,
    pub(crate) host_contract: WorthUiRuntimeSupportRow,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiRuntimeSupportInventory {
    rows: [WorthUiRuntimeSupportRow; 4],
}

impl WorthUiRuntimeSupportInventory {
    pub(crate) fn new(fields: WorthUiRuntimeSupportInventoryFields) -> Self {
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

pub const PHASE3_RUNTIME_SUPPORT_INVENTORY: WorthUiRuntimeSupportInventory =
    WorthUiRuntimeSupportInventory {
        rows: [
            WorthUiRuntimeSupportRow {
                subsystem: "dsl_package",
            },
            WorthUiRuntimeSupportRow {
                subsystem: "inspection",
            },
            WorthUiRuntimeSupportRow {
                subsystem: "query_binding",
            },
            WorthUiRuntimeSupportRow {
                subsystem: "host_contract",
            },
        ],
    };
