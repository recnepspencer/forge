use forge_foundational::facade::{AspectValue, InternedString};
use forge_query::facade::{
    ForgeQueryDerivedPatchPayload, ForgeQueryDerivedView, ForgeQueryDerivedViewMaterialization,
    ForgeQueryMutationDelta, ForgeQueryRetainedFieldPath, ForgeQueryRetainedMaterializedRow,
};
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::query_native_runtime_boundary::{native_retained_field_path, native_string};

use super::TopologyQuerySurfaceError;

const PAYLOAD_FIELD: [&str; 2] = ["retained_payload", "json"];

pub(crate) fn retained_payload_scalars<T>(
    payload: &T,
) -> Result<Vec<(ForgeQueryRetainedFieldPath, AspectValue)>, TopologyQuerySurfaceError>
where
    T: Serialize,
{
    let field_path = retained_payload_field_path()?;
    let encoded = serde_json::to_string(payload).map_err(|error| {
        TopologyQuerySurfaceError::new(format!("failed to encode retained payload: {error}"))
    })?;
    Ok(vec![(field_path, native_string(encoded))])
}

pub(crate) fn publish_retained_payload<T>(
    view_name: &str,
    materialization: &mut ForgeQueryDerivedViewMaterialization,
    payload: &T,
) -> ForgeQueryDerivedPatchPayload
where
    T: Serialize,
{
    let scalars = retained_payload_scalars(payload)
        .unwrap_or_else(|error| panic!("retained payload for `{view_name}` must encode: {error}"));
    materialization
        .replace_retained_scalar_row(scalars.clone())
        .unwrap_or_else(|error| {
            panic!("retained payload for `{view_name}` must materialize: {error}")
        });
    ForgeQueryDerivedPatchPayload::from_retained_scalar_values(scalars)
        .unwrap_or_else(|error| panic!("retained payload for `{view_name}` must patch: {error}"))
}

pub(crate) fn decode_retained_payload_row<T>(
    row: &ForgeQueryRetainedMaterializedRow,
    view_name: &str,
) -> Result<T, TopologyQuerySurfaceError>
where
    T: DeserializeOwned,
{
    let field_path = retained_payload_field_path()?;
    let Some(value) = row.field_value_at(&field_path) else {
        return Err(TopologyQuerySurfaceError::new(format!(
            "retained surface `{view_name}` is missing retained payload"
        )));
    };
    let json = match value {
        AspectValue::String(InternedString::Raw(value)) => value.as_str(),
        AspectValue::String(InternedString::Symbol(_)) => {
            return Err(TopologyQuerySurfaceError::new(format!(
                "retained surface `{view_name}` carried symbol-backed payload without an interner"
            )));
        }
        _ => {
            return Err(TopologyQuerySurfaceError::new(format!(
                "retained surface `{view_name}` carried non-string payload"
            )));
        }
    };
    serde_json::from_str(json).map_err(|error| {
        TopologyQuerySurfaceError::new(format!(
            "retained surface `{view_name}` payload failed to decode: {error}"
        ))
    })
}

pub(crate) fn decode_single_retained_payload_row<T>(
    rows: &[ForgeQueryRetainedMaterializedRow],
    view_name: &str,
) -> Result<T, TopologyQuerySurfaceError>
where
    T: DeserializeOwned,
{
    match rows {
        [] => Err(TopologyQuerySurfaceError::new(format!(
            "retained surface `{view_name}` expected one row, found none"
        ))),
        [row] => decode_retained_payload_row(row, view_name),
        rows => Err(TopologyQuerySurfaceError::new(format!(
            "retained surface `{view_name}` expected one row, found {}",
            rows.len()
        ))),
    }
}

pub(crate) fn incremental_patch_touches(
    view: &ForgeQueryDerivedView,
    delta: &ForgeQueryMutationDelta,
) -> Vec<forge_query::facade::ForgeQueryAspectTouch> {
    if view.produced_aspect_touches().is_empty() {
        delta.admitted_touched_aspects().to_vec()
    } else {
        view.produced_aspect_touches().to_vec()
    }
}

pub(crate) fn refresh_patch_touches(
    view: &ForgeQueryDerivedView,
) -> Vec<forge_query::facade::ForgeQueryAspectTouch> {
    if view.produced_aspect_touches().is_empty() {
        view.dependency_aspect_touches().to_vec()
    } else {
        view.produced_aspect_touches().to_vec()
    }
}

pub(crate) fn retained_payload_field_path(
) -> Result<ForgeQueryRetainedFieldPath, TopologyQuerySurfaceError> {
    native_retained_field_path(PAYLOAD_FIELD).map_err(|error| {
        TopologyQuerySurfaceError::new(format!(
            "failed to build retained payload field path: {error}"
        ))
    })
}
