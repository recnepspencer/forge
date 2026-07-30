use crate::{
    PlannedScrubWindowStatus, ScrubCounterSnapshot, ScrubLocalitySummary, ScrubPlan, ScrubWindow,
};

use super::{PausedScrubExecution, ScrubExecutionOutcome, ScrubExecutionReceipt};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScrubExecution;

impl ScrubExecution {
    pub fn run<'runtime, 'lease>(
        plan: ScrubPlan<'runtime, 'lease>,
    ) -> ScrubExecutionOutcome<'runtime, 'lease> {
        execute_from(plan, 0, ScrubCounterSnapshot::planned(0), None)
    }
}

pub(super) fn resume_execution<'runtime, 'lease>(
    paused: PausedScrubExecution<'runtime, 'lease>,
) -> ScrubExecutionOutcome<'runtime, 'lease> {
    let revalidated = paused.plan.revalidation_window(paused.next_window_index);
    execute_from(
        paused.plan,
        paused.next_window_index,
        paused.counters,
        revalidated,
    )
}

fn execute_from<'runtime, 'lease>(
    plan: ScrubPlan<'runtime, 'lease>,
    start_index: usize,
    mut counters: ScrubCounterSnapshot,
    revalidated: Option<ScrubWindow<'lease>>,
) -> ScrubExecutionOutcome<'runtime, 'lease> {
    counters = ScrubCounterSnapshot::planned(plan.windows().len() as u64).merge_runtime(counters);
    let completed_at_slice_start = counters.completed_window_count();
    let mut locality = None;

    if let Some(window) = revalidated {
        counters = counters.with_revalidated(window.len_bytes(), window.checksum());
        locality = merge_locality(locality, window);
    }

    let mut index = start_index;
    while index < plan.windows().len() {
        if should_yield(
            plan.yield_after_windows(),
            counters.completed_window_count() - completed_at_slice_start,
        ) {
            counters = counters.with_interruption();
            return ScrubExecutionOutcome::Yielded(PausedScrubExecution {
                plan,
                next_window_index: index,
                counters,
                locality,
            });
        }

        let planned = plan.windows()[index];
        match planned.status() {
            PlannedScrubWindowStatus::Inspect => {
                let window = planned.window();
                counters =
                    counters.with_completed_inspection(window.len_bytes(), window.checksum());
                locality = merge_locality(locality, window);
            }
            PlannedScrubWindowStatus::Skip => {
                counters = counters.with_skipped();
            }
            PlannedScrubWindowStatus::DeferOverBudget(_) => {
                counters = counters.with_deferred_over_budget();
            }
        }
        index += 1;
    }

    ScrubExecutionOutcome::Completed(ScrubExecutionReceipt::completed(
        plan.mode(),
        counters,
        locality,
    ))
}

fn should_yield(yield_after: Option<u64>, completed: u64) -> bool {
    yield_after.is_some_and(|limit| completed >= limit)
}

fn merge_locality(
    locality: Option<ScrubLocalitySummary>,
    window: ScrubWindow<'_>,
) -> Option<ScrubLocalitySummary> {
    Some(match locality {
        Some(summary) => summary.merge(&window),
        None => ScrubLocalitySummary::single(&window),
    })
}

trait ScrubCounterMerge {
    fn merge_runtime(self, runtime: Self) -> Self;
}

impl ScrubCounterMerge for ScrubCounterSnapshot {
    fn merge_runtime(self, runtime: Self) -> Self {
        if runtime.planned_window_count() == 0 {
            return self;
        }
        runtime
    }
}
