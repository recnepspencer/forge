fn route_raw_host_observation(
    observation: worth_ui::facade::observation_report::UiHostObservationPayload,
) -> worth_ui::facade::interaction::UiSemanticInteraction {
    observation
}

fn route_allocation_reporting(
    allocation: worth_ui::facade::inspection::UiAllocationReceiptInspectionReceipt,
) -> worth_ui::facade::interaction::UiSemanticInteraction {
    allocation
}
