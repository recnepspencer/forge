use crate::facade::RuntimeComplexityCounters;

#[derive(Debug, Clone, Copy)]
pub(crate) struct ComplexityBudget {
    pub(crate) label: &'static str,
    pub(crate) max: usize,
    pub(crate) selector: fn(&RuntimeComplexityCounters) -> usize,
}

pub(crate) fn assert_counter_at_most(
    counters: &RuntimeComplexityCounters,
    selector: impl Fn(&RuntimeComplexityCounters) -> usize,
    max: usize,
    label: &str,
) {
    let actual = selector(counters);
    assert!(actual <= max, "{label} exceeded budget: {actual} > {max}");
}

pub(crate) fn workflow_budgets() -> Vec<ComplexityBudget> {
    vec![
        ComplexityBudget {
            label: "full_state_clones",
            max: 0,
            selector: |counters| counters.full_state_clones,
        },
        ComplexityBudget {
            label: "snapshot_pin_full_rebuilds",
            max: 0,
            selector: |counters| counters.snapshot_pin_full_rebuilds,
        },
    ]
}
