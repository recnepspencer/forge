use worth_ui::facade::query_binding::{
    UiProjectionConsumptionBudget, UiScalarProjectionBinding,
};

fn readmit_reporting_projection(reporting: &str) {
    UiScalarProjectionBinding::consume_async_result_batch(
        panic!("compile-only binding"),
        panic!("compile-only workspace"),
        reporting,
        None,
        UiProjectionConsumptionBudget::platform_pulse(),
    );
}

fn main() {}
