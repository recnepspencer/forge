use crate::runtime::{
    WorthUiPlanLoweringBasis, WorthUiPlanLoweringContext, WorthUiPlanLoweringCounters,
    WorthUiPlanNodeInput,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiExecutionPlanInput {
    basis: WorthUiPlanLoweringBasis,
    context: WorthUiPlanLoweringContext,
    node_inputs: Vec<WorthUiPlanNodeInput>,
    counters: WorthUiPlanLoweringCounters,
}

impl WorthUiExecutionPlanInput {
    pub(crate) fn new(
        basis: WorthUiPlanLoweringBasis,
        context: WorthUiPlanLoweringContext,
        mut node_inputs: Vec<WorthUiPlanNodeInput>,
        counters: WorthUiPlanLoweringCounters,
    ) -> Self {
        node_inputs.sort_by(|left, right| {
            left.family()
                .cmp(&right.family())
                .then_with(|| left.identity_basis().cmp(right.identity_basis()))
        });
        Self {
            basis,
            context,
            node_inputs,
            counters,
        }
    }

    pub fn basis(&self) -> &WorthUiPlanLoweringBasis {
        &self.basis
    }

    #[cfg(test)]
    pub fn context(&self) -> &WorthUiPlanLoweringContext {
        &self.context
    }

    pub fn node_inputs(&self) -> &[WorthUiPlanNodeInput] {
        &self.node_inputs
    }

    pub fn counters(&self) -> WorthUiPlanLoweringCounters {
        self.counters
    }
}
