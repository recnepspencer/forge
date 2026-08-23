use worth_ui::facade::query_binding::{
    UiProjectionConsumptionBudget, UiQueryObservationReportingProjection,
    UiScalarProjectionBinding,
};

fn readmit_reporting_projection(
    binding: &mut UiScalarProjectionBinding,
    reporting: UiQueryObservationReportingProjection,
) {
    binding.consume_async_result_batch(
        panic!("compile-only workspace"),
        reporting.into(),
        None,
        UiProjectionConsumptionBudget::platform_pulse(),
    );
}

fn main() {}
