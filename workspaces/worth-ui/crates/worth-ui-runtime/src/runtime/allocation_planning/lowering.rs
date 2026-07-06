use crate::runtime::{
    WorthUiExecutionPlanInput, WorthUiPlanLoweringBasis, WorthUiPlanLoweringContext,
    WorthUiPlanNodeInput,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiAllocationPlanningLowering {
    basis: WorthUiPlanLoweringBasis,
    context: WorthUiPlanLoweringContext,
    node_inputs: Vec<WorthUiPlanNodeInput>,
}

impl WorthUiAllocationPlanningLowering {
    pub(crate) fn from_execution_plan_input(plan_input: WorthUiExecutionPlanInput) -> Self {
        Self {
            basis: plan_input.basis().clone(),
            context: plan_input.context().clone(),
            node_inputs: plan_input.node_inputs().to_vec(),
        }
    }

    pub(crate) fn execution_plan_input(&self) -> WorthUiExecutionPlanInput {
        WorthUiExecutionPlanInput::new(
            self.basis.clone(),
            self.context.clone(),
            self.node_inputs.clone(),
            Default::default(),
        )
    }

    pub(crate) fn basis(&self) -> &WorthUiPlanLoweringBasis {
        &self.basis
    }

    pub(crate) fn context(&self) -> &WorthUiPlanLoweringContext {
        &self.context
    }

    pub(crate) fn node_inputs(&self) -> &[WorthUiPlanNodeInput] {
        &self.node_inputs
    }
}
