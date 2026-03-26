mod aspect_plans;
mod continuity;

pub(crate) use aspect_plans::{
    canonicalize_entity_registration, canonicalize_relation_registration, lower_aspect_plans,
    lower_relation_integrity_plans,
};
#[cfg(test)]
pub(crate) use continuity::classify_schema_transition;
pub(crate) use continuity::{
    lower_schema_transition, validate_schema_continuity_bundle, validate_schema_transition,
    SchemaContinuityBundleIssue, ValidatedSchemaContinuityBundle,
};
