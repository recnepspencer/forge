mod declarations;
mod keys;
mod native_field_patch;

pub(crate) use declarations::{entity_string_field_aspect, lifecycle_string_aspect};
#[cfg(test)]
pub(crate) use declarations::{
    relation_source_endpoint_aspect, relation_string_field_aspect, relation_target_endpoint_aspect,
};
#[cfg(test)]
pub(crate) use keys::aspect_key;
pub(crate) use native_field_patch::single_native_string_aspect_field_patch;
