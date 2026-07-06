use crate::{
    UiInspectionClosureReport, UiInspectionMilestoneExpectation, UiInspectionScope,
    UiInspectionScopeSupportRow, UiInspectionSupportReport, UiInspectionSupportWorld,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct UiInspectionScopeInventoryFields {
    pub(crate) graph: [UiInspectionScopeSupportRow; 4],
    pub(crate) measurement: [UiInspectionScopeSupportRow; 4],
    pub(crate) planning: [UiInspectionScopeSupportRow; 4],
    pub(crate) mounting: [UiInspectionScopeSupportRow; 4],
    pub(crate) rebind: [UiInspectionScopeSupportRow; 4],
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiInspectionScopeInventory {
    graph: [UiInspectionScopeSupportRow; 4],
    measurement: [UiInspectionScopeSupportRow; 4],
    planning: [UiInspectionScopeSupportRow; 4],
    mounting: [UiInspectionScopeSupportRow; 4],
    rebind: [UiInspectionScopeSupportRow; 4],
    rows: [UiInspectionScopeSupportRow; 20],
}

impl UiInspectionScopeInventory {
    pub(crate) const fn new(fields: UiInspectionScopeInventoryFields) -> Self {
        Self {
            graph: fields.graph,
            measurement: fields.measurement,
            planning: fields.planning,
            mounting: fields.mounting,
            rebind: fields.rebind,
            rows: [
                fields.graph[0],
                fields.measurement[0],
                fields.planning[0],
                fields.mounting[0],
                fields.rebind[0],
                fields.graph[1],
                fields.measurement[1],
                fields.planning[1],
                fields.mounting[1],
                fields.rebind[1],
                fields.graph[2],
                fields.measurement[2],
                fields.planning[2],
                fields.mounting[2],
                fields.rebind[2],
                fields.graph[3],
                fields.measurement[3],
                fields.planning[3],
                fields.mounting[3],
                fields.rebind[3],
            ],
        }
    }

    pub const fn from_scope_rows(
        graph: [UiInspectionScopeSupportRow; 4],
        measurement: [UiInspectionScopeSupportRow; 4],
        planning: [UiInspectionScopeSupportRow; 4],
        mounting: [UiInspectionScopeSupportRow; 4],
        rebind: [UiInspectionScopeSupportRow; 4],
    ) -> Self {
        Self::new(UiInspectionScopeInventoryFields {
            graph,
            measurement,
            planning,
            mounting,
            rebind,
        })
    }

    pub fn rows(&self) -> &[UiInspectionScopeSupportRow] {
        &self.rows
    }

    pub fn support_report(&self, scope: UiInspectionScope) -> UiInspectionSupportReport {
        match scope {
            UiInspectionScope::Graph => {
                UiInspectionSupportReport::from_scope_rows(scope, &self.graph)
            }
            UiInspectionScope::Measurement => {
                UiInspectionSupportReport::from_scope_rows(scope, &self.measurement)
            }
            UiInspectionScope::Planning => {
                UiInspectionSupportReport::from_scope_rows(scope, &self.planning)
            }
            UiInspectionScope::Mounting => {
                UiInspectionSupportReport::from_scope_rows(scope, &self.mounting)
            }
            UiInspectionScope::Rebind => {
                UiInspectionSupportReport::from_scope_rows(scope, &self.rebind)
            }
        }
    }

    pub fn closure_report(&self) -> UiInspectionClosureReport {
        UiInspectionClosureReport::new(&self.rows)
    }
}

const fn unsupported_scope_rows(scope: UiInspectionScope) -> [UiInspectionScopeSupportRow; 4] {
    [
        UiInspectionScopeSupportRow::unsupported_not_yet_admitted(
            "dsl_package",
            scope,
            UiInspectionMilestoneExpectation::Milestone31,
        ),
        UiInspectionScopeSupportRow::unsupported_not_yet_admitted(
            "inspection",
            scope,
            UiInspectionMilestoneExpectation::Milestone31,
        ),
        UiInspectionScopeSupportRow::unsupported_not_yet_admitted(
            "query_binding",
            scope,
            UiInspectionMilestoneExpectation::Milestone31,
        ),
        UiInspectionScopeSupportRow::unsupported_not_yet_admitted(
            "host_contract",
            scope,
            UiInspectionMilestoneExpectation::Milestone31,
        ),
    ]
}

#[allow(dead_code)]
const fn supported_scope_rows(scope: UiInspectionScope) -> [UiInspectionScopeSupportRow; 4] {
    [
        UiInspectionScopeSupportRow::supported(
            "dsl_package",
            scope,
            UiInspectionSupportWorld::Authoritative,
        ),
        UiInspectionScopeSupportRow::supported(
            "inspection",
            scope,
            UiInspectionSupportWorld::Authoritative,
        ),
        UiInspectionScopeSupportRow::supported(
            "query_binding",
            scope,
            UiInspectionSupportWorld::Authoritative,
        ),
        UiInspectionScopeSupportRow::supported(
            "host_contract",
            scope,
            UiInspectionSupportWorld::Authoritative,
        ),
    ]
}

pub const RUNTIME_INSPECTION_SCOPE_INVENTORY: UiInspectionScopeInventory =
    UiInspectionScopeInventory::new(UiInspectionScopeInventoryFields {
        graph: unsupported_scope_rows(UiInspectionScope::Graph),
        measurement: unsupported_scope_rows(UiInspectionScope::Measurement),
        planning: supported_scope_rows(UiInspectionScope::Planning),
        mounting: unsupported_scope_rows(UiInspectionScope::Mounting),
        rebind: unsupported_scope_rows(UiInspectionScope::Rebind),
    });
