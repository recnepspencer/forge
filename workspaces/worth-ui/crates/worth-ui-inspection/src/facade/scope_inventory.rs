use crate::{UiInspectionScope, UiInspectionSupportStatus};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiInspectionScopeSupportRow {
    subsystem: &'static str,
    scope: UiInspectionScope,
    status: UiInspectionSupportStatus,
}

impl UiInspectionScopeSupportRow {
    pub fn new(
        subsystem: &'static str,
        scope: UiInspectionScope,
        status: UiInspectionSupportStatus,
    ) -> Self {
        Self {
            subsystem,
            scope,
            status,
        }
    }

    pub fn scope(self) -> UiInspectionScope {
        self.scope
    }

    pub fn status(self) -> UiInspectionSupportStatus {
        self.status
    }

    pub fn subsystem(self) -> &'static str {
        self.subsystem
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiInspectionScopeInventoryFields {
    pub dsl_package: UiInspectionScopeSupportRow,
    pub inspection: UiInspectionScopeSupportRow,
    pub query_binding: UiInspectionScopeSupportRow,
    pub host_contract: UiInspectionScopeSupportRow,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiInspectionScopeInventory {
    rows: [UiInspectionScopeSupportRow; 4],
}

impl UiInspectionScopeInventory {
    pub fn new(fields: UiInspectionScopeInventoryFields) -> Self {
        Self {
            rows: [
                fields.dsl_package,
                fields.inspection,
                fields.query_binding,
                fields.host_contract,
            ],
        }
    }

    pub fn phase3_runtime_defaults() -> Self {
        Self::new(UiInspectionScopeInventoryFields {
            dsl_package: UiInspectionScopeSupportRow::new(
                "dsl_package",
                UiInspectionScope::Graph,
                UiInspectionSupportStatus::Unsupported,
            ),
            inspection: UiInspectionScopeSupportRow::new(
                "inspection",
                UiInspectionScope::Graph,
                UiInspectionSupportStatus::Unsupported,
            ),
            query_binding: UiInspectionScopeSupportRow::new(
                "query_binding",
                UiInspectionScope::Graph,
                UiInspectionSupportStatus::Unsupported,
            ),
            host_contract: UiInspectionScopeSupportRow::new(
                "host_contract",
                UiInspectionScope::Graph,
                UiInspectionSupportStatus::Unsupported,
            ),
        })
    }

    pub fn rows(&self) -> &[UiInspectionScopeSupportRow] {
        &self.rows
    }
}
