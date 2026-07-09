use worth_store_physical_integrity::{
    ScrubExecution, ScrubExecutionDenial, ScrubExecutionReceipt, ScrubPlan, ScrubResumeToken,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalIntegrityScrubWorkflow;

impl PhysicalIntegrityScrubWorkflow {
    pub fn run(plan: ScrubPlan<'_>) -> Result<ScrubExecutionReceipt, ScrubExecutionDenial> {
        ScrubExecution::run(plan)
    }

    pub fn resume(
        plan: ScrubPlan<'_>,
        token: ScrubResumeToken,
    ) -> Result<ScrubExecutionReceipt, ScrubExecutionDenial> {
        ScrubExecution::resume(plan, token)
    }
}
