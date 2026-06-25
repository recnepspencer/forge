use std::marker::PhantomData;

use super::WorthUiQueryGraphExecutionReceipt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiPrimitiveConstructionGraphOperation;
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiMountedInteractionGraphOperation;
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiPrimitiveEventDispatchGraphOperation;
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiPrimitiveContentAnatomyGraphOperation;
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiUserIntentTargetBindingGraphOperation;
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiLiveViewStateBindingGraphOperation;
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiLiveViewControlProjectionGraphOperation;
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiLiveViewConditionalProjectionGraphOperation;
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiLiveViewExpressionProjectionGraphOperation;
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiLiveViewReadinessProjectionGraphOperation;
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiLiveViewInteractionIntentGraphOperation;
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiLiveViewPayloadProjectionGraphOperation;
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiCompositionTopologyGraphOperation;
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiCompositionGraphAccessOperation;
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiCompositionContextGraphOperation;
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiCompositionParticipationGraphOperation;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiQueryGraphOperationPlan<Family> {
    execution_receipt: WorthUiQueryGraphExecutionReceipt,
    _family: PhantomData<Family>,
}

pub type WorthUiPrimitiveConstructionGraphPlan =
    WorthUiQueryGraphOperationPlan<WorthUiPrimitiveConstructionGraphOperation>;
pub type WorthUiMountedInteractionGraphPlan =
    WorthUiQueryGraphOperationPlan<WorthUiMountedInteractionGraphOperation>;
pub type WorthUiPrimitiveEventDispatchGraphPlan =
    WorthUiQueryGraphOperationPlan<WorthUiPrimitiveEventDispatchGraphOperation>;
pub type WorthUiPrimitiveContentAnatomyGraphPlan =
    WorthUiQueryGraphOperationPlan<WorthUiPrimitiveContentAnatomyGraphOperation>;
pub type WorthUiUserIntentTargetBindingGraphPlan =
    WorthUiQueryGraphOperationPlan<WorthUiUserIntentTargetBindingGraphOperation>;
pub type WorthUiLiveViewStateBindingGraphPlan =
    WorthUiQueryGraphOperationPlan<WorthUiLiveViewStateBindingGraphOperation>;
pub type WorthUiLiveViewControlProjectionGraphPlan =
    WorthUiQueryGraphOperationPlan<WorthUiLiveViewControlProjectionGraphOperation>;
pub type WorthUiLiveViewConditionalProjectionGraphPlan =
    WorthUiQueryGraphOperationPlan<WorthUiLiveViewConditionalProjectionGraphOperation>;
pub type WorthUiLiveViewExpressionProjectionGraphPlan =
    WorthUiQueryGraphOperationPlan<WorthUiLiveViewExpressionProjectionGraphOperation>;
pub type WorthUiLiveViewReadinessProjectionGraphPlan =
    WorthUiQueryGraphOperationPlan<WorthUiLiveViewReadinessProjectionGraphOperation>;
pub type WorthUiLiveViewInteractionIntentGraphPlan =
    WorthUiQueryGraphOperationPlan<WorthUiLiveViewInteractionIntentGraphOperation>;
pub type WorthUiLiveViewPayloadProjectionGraphPlan =
    WorthUiQueryGraphOperationPlan<WorthUiLiveViewPayloadProjectionGraphOperation>;
pub type WorthUiCompositionTopologyGraphPlan =
    WorthUiQueryGraphOperationPlan<WorthUiCompositionTopologyGraphOperation>;
pub type WorthUiCompositionGraphAccessPlan =
    WorthUiQueryGraphOperationPlan<WorthUiCompositionGraphAccessOperation>;
pub type WorthUiCompositionContextGraphPlan =
    WorthUiQueryGraphOperationPlan<WorthUiCompositionContextGraphOperation>;
pub type WorthUiCompositionParticipationGraphPlan =
    WorthUiQueryGraphOperationPlan<WorthUiCompositionParticipationGraphOperation>;

impl<Family> WorthUiQueryGraphOperationPlan<Family> {
    pub(crate) fn new(execution_receipt: WorthUiQueryGraphExecutionReceipt) -> Self {
        Self {
            execution_receipt,
            _family: PhantomData,
        }
    }

    pub fn execution_receipt(&self) -> &WorthUiQueryGraphExecutionReceipt {
        &self.execution_receipt
    }

    pub fn into_execution_receipt(self) -> WorthUiQueryGraphExecutionReceipt {
        self.execution_receipt
    }
}
