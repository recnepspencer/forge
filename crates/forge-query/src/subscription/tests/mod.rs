use super::*;

fn roomy_budget() -> QuerySubscriptionWorkBudget {
    QuerySubscriptionWorkBudget::scratch_buffer_only(8, 8, 8, 32, 1)
}

fn roomy_slice_budget() -> QuerySubscriptionSliceBudget {
    QuerySubscriptionSliceBudget::scratch_buffer_only(8, 8, 8, 8, 8, 8, 8, 8)
}

fn roomy_lowering_budget() -> QuerySubscriptionBridgeLoweringBudget {
    QuerySubscriptionBridgeLoweringBudget::admitted(1, 8, 8, 1, 1)
}

mod active;
mod active_delivery;
mod admission;
mod certification;
mod declaration;
mod diagnostic_bundles;
mod diagnostics;
mod runtime_certification;
mod runtime_certification_closure_support;
mod selection;
mod support;

pub(crate) use runtime_certification_closure_support::runtime_backed_subscription_certification_summary;
