use crate::authority::commit::preparation::packets::invariant::InvariantWorkPacket;
use crate::authority::commit::preparation::planning::context::PreparationPlanningContext;
use crate::authority::commit::preparation::planning::strategy::PreparationStrategy;

#[derive(Clone)]
pub(crate) struct PreparedInvariantExecution<'runtime> {
    pub(crate) context: PreparationPlanningContext,
    pub(crate) strategy: PreparationStrategy,
    pub(crate) packets: Vec<InvariantWorkPacket<'runtime>>,
}

#[derive(Clone, Default)]
pub(crate) struct PreparationWorkPlan<'runtime> {
    pub(crate) invariant_execution: Option<PreparedInvariantExecution<'runtime>>,
}
