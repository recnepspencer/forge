use crate::runtime::{WorthUiQueryLiveRebindOutcome, WorthUiQueryLiveRebindPlan};

pub(in crate::runtime) fn denied_query_rebind_count(plan: &WorthUiQueryLiveRebindPlan) -> usize {
    plan.entries()
        .iter()
        .filter(|entry| matches!(entry.outcome(), WorthUiQueryLiveRebindOutcome::Deny(_)))
        .count()
}
