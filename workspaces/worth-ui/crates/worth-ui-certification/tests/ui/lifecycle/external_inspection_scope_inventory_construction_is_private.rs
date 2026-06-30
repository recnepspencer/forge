use worth_ui_inspection::{
    UiInspectionMilestoneExpectation, UiInspectionScopeInventory,
    UiInspectionScopeInventoryFields, UiInspectionScopeSupportRow,
};

fn main() {
    let fields = UiInspectionScopeInventoryFields {
        dsl_package: UiInspectionScopeSupportRow::unsupported_not_yet_admitted(
            "dsl_package",
            worth_ui_inspection::UiInspectionScope::Graph,
            UiInspectionMilestoneExpectation::Milestone31,
        ),
        inspection: UiInspectionScopeSupportRow::unsupported_not_yet_admitted(
            "inspection",
            worth_ui_inspection::UiInspectionScope::Graph,
            UiInspectionMilestoneExpectation::Milestone31,
        ),
        query_binding: UiInspectionScopeSupportRow::unsupported_not_yet_admitted(
            "query_binding",
            worth_ui_inspection::UiInspectionScope::Graph,
            UiInspectionMilestoneExpectation::Milestone31,
        ),
        host_contract: UiInspectionScopeSupportRow::unsupported_not_yet_admitted(
            "host_contract",
            worth_ui_inspection::UiInspectionScope::Graph,
            UiInspectionMilestoneExpectation::Milestone31,
        ),
    };

    let _ = UiInspectionScopeInventory::new(fields);
}
