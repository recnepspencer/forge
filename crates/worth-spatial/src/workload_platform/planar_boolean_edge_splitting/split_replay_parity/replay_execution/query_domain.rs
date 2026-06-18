use super::super::parity_receipt::PlanarBooleanEdgeSplitReplayParityDenial;
use super::closeout::PlanarBooleanEdgeSplitCloseout;
use super::replay_product::{
    PlanarBooleanEdgeSplitReplayExecutionMode, PlanarBooleanEdgeSplitReplayProduct,
};
use crate::workload_platform::retained_replay_workload::ReplayReceiptSet;

pub struct PlanarBooleanEdgeSplitReplayQueryDomain;

pub struct PlanarBooleanEdgeSplitReplayQueryInput<'a> {
    original: PlanarBooleanEdgeSplitCloseout<'a>,
    replayed: PlanarBooleanEdgeSplitCloseout<'a>,
    replay_receipts: &'a ReplayReceiptSet,
    execution_mode: PlanarBooleanEdgeSplitReplayExecutionMode,
}

pub struct PlanarBooleanEdgeSplitReplayLoweredPlan<'a> {
    lowered_plan_identity: String,
    input: PlanarBooleanEdgeSplitReplayQueryInput<'a>,
}

impl<'a> PlanarBooleanEdgeSplitReplayQueryInput<'a> {
    pub fn from_closeouts(
        original: PlanarBooleanEdgeSplitCloseout<'a>,
        replayed: PlanarBooleanEdgeSplitCloseout<'a>,
        replay_receipts: &'a ReplayReceiptSet,
        execution_mode: PlanarBooleanEdgeSplitReplayExecutionMode,
    ) -> Self {
        Self {
            original,
            replayed,
            replay_receipts,
            execution_mode,
        }
    }
}

impl PlanarBooleanEdgeSplitReplayQueryDomain {
    pub fn declare<'a>(
        input: PlanarBooleanEdgeSplitReplayQueryInput<'a>,
    ) -> Result<PlanarBooleanEdgeSplitReplayLoweredPlan<'a>, PlanarBooleanEdgeSplitReplayParityDenial>
    {
        let lowered_plan_identity = format!(
            "edge-split-replay-lowered-plan:{:?}:{}:{}:{}",
            input.execution_mode,
            input.original.closeout_identity(),
            input.replayed.closeout_identity(),
            input.replay_receipts.stage_identity().receipt_identity()
        );
        Ok(PlanarBooleanEdgeSplitReplayLoweredPlan {
            lowered_plan_identity,
            input,
        })
    }
}

impl<'a> PlanarBooleanEdgeSplitReplayLoweredPlan<'a> {
    pub fn lowered_plan_identity(&self) -> &str {
        &self.lowered_plan_identity
    }

    pub fn execute(
        self,
    ) -> Result<PlanarBooleanEdgeSplitReplayProduct<'a>, PlanarBooleanEdgeSplitReplayParityDenial>
    {
        PlanarBooleanEdgeSplitReplayProduct::from_retained_closeouts(
            self.input.original,
            self.input.replayed,
            self.input.replay_receipts,
            self.input.execution_mode,
        )
    }
}
