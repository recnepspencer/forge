use crate::{
    PlannedScrubWindowStatus, ScrubCounterSnapshot, ScrubExecutionDenial, ScrubExecutionDenialKind,
    ScrubLocalitySummary, ScrubMode, ScrubPlan, ScrubResumeToken, ScrubWindow,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrubIntegrityFinding {
    Intact,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScrubProgressReport {
    counters: ScrubCounterSnapshot,
    locality: Option<ScrubLocalitySummary>,
    interrupted: bool,
}

impl ScrubProgressReport {
    pub const fn counters(&self) -> ScrubCounterSnapshot {
        self.counters
    }

    pub const fn locality(&self) -> Option<ScrubLocalitySummary> {
        self.locality
    }

    pub const fn interrupted(&self) -> bool {
        self.interrupted
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScrubExecutionReceipt {
    mode: ScrubMode,
    finding: ScrubIntegrityFinding,
    progress: ScrubProgressReport,
    resume_token: Option<ScrubResumeToken>,
}

impl ScrubExecutionReceipt {
    pub const fn mode(&self) -> ScrubMode {
        self.mode
    }

    pub const fn finding(&self) -> ScrubIntegrityFinding {
        self.finding
    }

    pub const fn counters(&self) -> ScrubCounterSnapshot {
        self.progress.counters()
    }

    pub const fn progress(&self) -> &ScrubProgressReport {
        &self.progress
    }

    pub const fn locality(&self) -> Option<ScrubLocalitySummary> {
        self.progress.locality()
    }

    pub const fn resume_token(&self) -> Option<ScrubResumeToken> {
        self.resume_token
    }

    pub const fn proves_recovery_behavior(&self) -> bool {
        false
    }

    pub const fn proves_repair_behavior(&self) -> bool {
        false
    }

    pub const fn proves_blob_lifecycle(&self) -> bool {
        false
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScrubExecution;

impl ScrubExecution {
    pub fn run(plan: ScrubPlan<'_>) -> Result<ScrubExecutionReceipt, ScrubExecutionDenial> {
        execute_from(plan, 0, ScrubCounterSnapshot::planned(0), None)
    }

    pub fn resume(
        plan: ScrubPlan<'_>,
        token: ScrubResumeToken,
    ) -> Result<ScrubExecutionReceipt, ScrubExecutionDenial> {
        if token.plan_identity() != plan.plan_identity() {
            return Err(ScrubExecutionDenial::new(
                ScrubExecutionDenialKind::ResumeTokenForDifferentPlan,
            ));
        }
        if token.next_window_index() > plan.windows().len() {
            return Err(ScrubExecutionDenial::new(
                ScrubExecutionDenialKind::ResumeTokenPastEnd,
            ));
        }
        let revalidated = token
            .next_window_index()
            .checked_sub(1)
            .and_then(|index| plan.windows().get(index).map(|planned| planned.window()));
        execute_from(
            plan,
            token.next_window_index(),
            token.counters(),
            revalidated,
        )
    }
}

fn execute_from(
    plan: ScrubPlan<'_>,
    start_index: usize,
    mut counters: ScrubCounterSnapshot,
    revalidated: Option<ScrubWindow<'_>>,
) -> Result<ScrubExecutionReceipt, ScrubExecutionDenial> {
    counters = ScrubCounterSnapshot::planned(plan.windows().len() as u64).merge_runtime(counters);
    let completed_at_slice_start = counters.completed_window_count();
    let mut locality = None;

    if let Some(window) = revalidated {
        counters = counters.with_revalidated(window.len_bytes(), window.checksum());
        locality = merge_locality(locality, window);
    }

    for (index, planned) in plan.windows().iter().copied().enumerate().skip(start_index) {
        if should_yield(
            plan.yield_after_windows(),
            counters.completed_window_count() - completed_at_slice_start,
        ) {
            counters = counters.with_interruption();
            return Ok(receipt(
                plan.mode(),
                counters,
                locality,
                Some(ScrubResumeToken::new(plan.plan_identity(), index, counters)),
            ));
        }

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
    }

    Ok(receipt(plan.mode(), counters, locality, None))
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

fn receipt(
    mode: ScrubMode,
    counters: ScrubCounterSnapshot,
    locality: Option<ScrubLocalitySummary>,
    resume_token: Option<ScrubResumeToken>,
) -> ScrubExecutionReceipt {
    ScrubExecutionReceipt {
        mode,
        finding: ScrubIntegrityFinding::Intact,
        progress: ScrubProgressReport {
            counters,
            locality,
            interrupted: resume_token.is_some(),
        },
        resume_token,
    }
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
