mod entrypoints;
mod phases;
mod progression;
mod progression_support;
mod runtime_declaration;
mod runtime_payload;
mod runtime_targets;

pub use entrypoints::{
    lower_admitted_anchor_match_constraint_intent,
    lower_admitted_anchor_match_constraint_intent_with_catalog,
    lower_admitted_lies_on_constraint_intent,
    lower_admitted_lies_on_constraint_intent_with_catalog, lower_admitted_move_intent,
    lower_admitted_move_intent_with_catalog, lower_admitted_offset_intent,
    lower_admitted_offset_intent_with_catalog, lower_admitted_points_toward_constraint_intent,
    lower_admitted_points_toward_constraint_intent_with_catalog, lower_admitted_reorient_intent,
    lower_admitted_reorient_intent_with_catalog, lower_admitted_rotate_intent,
    lower_admitted_rotate_intent_with_catalog,
};
pub use phases::LoweredSpatialIntentPhase;
pub(crate) use runtime_declaration::LoweredSpatialOperation;
pub use runtime_declaration::{
    admit_lowered_spatial_runtime_intent, LoweredSpatialIntent, LoweredSpatialIntentArtifact,
    LoweredSpatialIntentFamily, LoweredSpatialNumericPosture, LoweredSpatialRuntimeDeclaration,
    LoweredSpatialTargetBindingPosture, RuntimeAnchorSemantic, SpatialLoweringDenial,
};
#[cfg(test)]
pub(crate) use runtime_payload::LoweredSpatialRuntimePayload;

#[cfg(test)]
#[path = "../lowered_intents_tests.rs"]
mod lowered_intents_tests;
