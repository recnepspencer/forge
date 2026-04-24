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

mod active_closeout;
mod active_continuation;
mod active_delivery;
mod active_lifecycle;
mod active_preview_isolation;
mod active_sharing;
mod admission;
mod bridge_lowering;
mod bridge_parity;
mod budget_denial;
mod certification;
mod declaration_budget;
mod declaration_parity;
mod delivery_intent;
mod diagnostic_bundles;
mod diagnostics;
mod dimension_denial;
mod equivalence;
mod family_selection;
mod runtime_certification;
mod slice_intent;
mod support;
