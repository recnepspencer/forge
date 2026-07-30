use worth_store_physical_integrity::{
    PausedScrubExecution, ScrubExecution, ScrubExecutionOutcome, ScrubPlan,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalIntegrityScrubWorkflow;

impl PhysicalIntegrityScrubWorkflow {
    pub fn run<'runtime, 'lease>(
        plan: ScrubPlan<'runtime, 'lease>,
    ) -> ScrubExecutionOutcome<'runtime, 'lease> {
        ScrubExecution::run(plan)
    }

    pub fn resume<'runtime, 'lease>(
        paused: PausedScrubExecution<'runtime, 'lease>,
    ) -> ScrubExecutionOutcome<'runtime, 'lease> {
        paused.resume()
    }
}
