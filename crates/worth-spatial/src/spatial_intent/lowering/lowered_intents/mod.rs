mod entrypoints;
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
pub(crate) use entrypoints::{
    lower_admitted_anchor_match_constraint_semantic_intent_with_catalog,
    lower_admitted_lies_on_constraint_semantic_intent_with_catalog,
    lower_admitted_move_semantic_intent_with_catalog,
    lower_admitted_offset_semantic_intent_with_catalog,
    lower_admitted_points_toward_constraint_semantic_intent_with_catalog,
    lower_admitted_reorient_semantic_intent_with_catalog,
    lower_admitted_rotate_semantic_intent_with_catalog,
};
#[cfg(test)]
pub(crate) use entrypoints::{
    lower_admitted_move_semantic_intent, lower_admitted_reorient_semantic_intent,
};
pub use runtime_declaration::SpatialLoweringDenial;
pub(crate) use runtime_declaration::{LoweredSpatialIntent, LoweredSpatialOperation};
#[cfg(test)]
pub(crate) use runtime_declaration::{LoweredSpatialIntentFamily, LoweredSpatialNumericPosture};

#[cfg(test)]
#[path = "../lowered_intents_tests.rs"]
mod lowered_intents_tests;
