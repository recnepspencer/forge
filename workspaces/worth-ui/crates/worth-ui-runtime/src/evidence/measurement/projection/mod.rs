mod fact_receipt;
#[cfg(test)]
mod fact_receipt_tests;
#[cfg(test)]
pub(crate) mod fact_test_support;
mod inspection_receipt;
#[cfg(test)]
mod inspection_receipt_tests;
#[cfg(test)]
pub(crate) mod query_context_test_support;
mod settled_fact_receipt;
#[cfg(test)]
pub(crate) mod variant_test_support;

pub(crate) use fact_receipt::admit_declared_measurement_projection_fact_receipt;
pub use fact_receipt::{
    consume_declared_measurement_projection_facts, UiProjectionFactReceipt,
    UiProjectionFactReceiptDenial,
};
pub(crate) use inspection_receipt::{
    project_measurement_inspection_compatibility_view, project_measurement_inspection_denial_view,
    project_measurement_inspection_view,
};
pub use settled_fact_receipt::consume_settled_query_measurement_fact;
pub(crate) use settled_fact_receipt::UiSettledQueryFactKey;
pub use settled_fact_receipt::UiSettledQueryFactReceipt;
