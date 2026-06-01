mod declarations;
mod external_json_ingress;
mod external_json_projection;
mod keys;

pub(crate) use declarations::{entity_string_field_aspect, lifecycle_string_aspect};
#[cfg(test)]
pub(crate) use declarations::{
    relation_source_endpoint_aspect, relation_string_field_aspect, relation_target_endpoint_aspect,
};
pub(crate) use external_json_ingress::{
    aspect_field_patch_from_external_json_values,
    lower_external_json_through_scalar_string_contract,
    single_aspect_field_patch_from_external_json,
};
pub(crate) use external_json_projection::project_aspect_value_to_workspace_json;
pub(crate) use keys::{aspect_key, field_key, planned_single_field_locator, terminal_field_label};
