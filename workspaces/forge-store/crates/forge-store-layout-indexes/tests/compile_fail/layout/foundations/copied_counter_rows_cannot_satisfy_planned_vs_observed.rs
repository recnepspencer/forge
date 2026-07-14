use forge_store_layout_indexes::{
    AccessPathCounterSnapshot, AccessPlanIdentity, BaselineBTreeLookupCounterReceipt,
    PlannedCounterObservation,
};

fn forge(
    plan_binding: AccessPlanIdentity,
    planned: AccessPathCounterSnapshot,
    observed: AccessPathCounterSnapshot,
    observation: PlannedCounterObservation,
) -> BaselineBTreeLookupCounterReceipt {
    BaselineBTreeLookupCounterReceipt {
        plan_binding,
        planned,
        observed,
        observation,
    }
}

fn main() {}
