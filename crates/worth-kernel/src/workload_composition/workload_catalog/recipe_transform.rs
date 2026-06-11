use worth_spatial::facade::transform_workload::{
    RotationTurn, TransformReorientation, TransformSequence, VectorDelta,
};

use super::recipe_kind::{RetainedReplayRecipe, TransformRecipe, WorkloadCatalogRecipeKind};

impl TransformRecipe {
    pub(crate) fn sequence(self) -> TransformSequence {
        match self {
            Self::MovementRotationStack => TransformSequence::new()
                .translate(VectorDelta::xy(10, 0))
                .rotate(RotationTurn::quarter_turn_clockwise())
                .reorient(TransformReorientation::preserves_handedness())
                .cancel_with_exact_replay(16),
            Self::HostileCancellation => TransformSequence::new()
                .translate(VectorDelta::xy(10, 0))
                .rotate(RotationTurn::quarter_turn_clockwise())
                .reorient(TransformReorientation::preserves_handedness())
                .cancel_with_exact_replay(64),
        }
    }
}

impl WorkloadCatalogRecipeKind {
    pub(crate) fn default_transform_recipe(self) -> TransformRecipe {
        match self {
            Self::CoplanarOverlapStorm | Self::TransformCycle | Self::RetainedCancellationChain => {
                TransformRecipe::HostileCancellation
            }
            _ => TransformRecipe::MovementRotationStack,
        }
    }

    pub(crate) fn default_retained_replay_recipe(self) -> RetainedReplayRecipe {
        match self {
            Self::CoplanarOverlapStorm
            | Self::HighValenceVertex
            | Self::RetainedCancellationChain => RetainedReplayRecipe::RetainedCancellationChain,
            _ => RetainedReplayRecipe::StageReceiptOnly,
        }
    }
}
