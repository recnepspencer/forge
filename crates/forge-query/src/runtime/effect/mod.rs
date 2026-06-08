mod declaration;
mod delivery;
mod follow_on;
mod inspection;
mod phase;
mod registry;
mod routing;

pub use declaration::{
    ForgeQueryEffectCondition, ForgeQueryEffectDeclaration, ForgeQueryEffectExpression,
    ForgeQueryEffectExpressionFailurePosture, ForgeQueryEffectSuppressionPolicy,
    ForgeQueryEffectTrigger, ForgeQueryEffectTriggerSourceKind,
};
pub use delivery::{
    ForgeQueryEffectCounters, ForgeQueryEffectDelivery, ForgeQueryEffectDeliveryFamily,
    ForgeQueryEffectHandle,
};
pub use follow_on::{
    ForgeQueryEffectWriteAdjacentTrigger, ForgeQueryEffectWriteAdjacentTriggerClass,
};
pub use inspection::ForgeQueryEffectInspectionEvidence;
pub use phase::{
    ForgeQueryEffectIdempotence, ForgeQueryEffectLoopPrevention, ForgeQueryEffectPhase,
    ForgeQueryEffectPhaseEvidence,
};

pub(super) use registry::{insert_effect_runtime, ForgeQueryEffectIndex, ForgeQueryEffectRuntime};
pub(super) use routing::{admit_effect_declaration, route_effect_deliveries};
