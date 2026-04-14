use crate::authoring::{
    AspectFieldKey, QueryFamily, RawAuthoredQuery, RawAuthoredResultShape, ResultShapeFamily,
};

use super::error::AuthoredBundleError;

pub(super) fn enforce_family_match(
    query_family: QueryFamily,
    result_shape_family: ResultShapeFamily,
) -> Result<(), AuthoredBundleError> {
    let matches = matches!(
        (&query_family, &result_shape_family),
        (QueryFamily::Detail, ResultShapeFamily::Detail)
            | (QueryFamily::Collection, ResultShapeFamily::Collection)
    );
    if matches {
        Ok(())
    } else {
        Err(AuthoredBundleError::QueryShapeFamilyMismatch {
            query_family,
            result_shape_family,
        })
    }
}

pub(super) fn enforce_shape_projection_compatibility(
    query: &RawAuthoredQuery,
    result_shape: &RawAuthoredResultShape,
) -> Result<(), AuthoredBundleError> {
    let projection_field_set = query.projection_field_set();
    for field in result_shape.fields() {
        let key = AspectFieldKey::from_parts(
            field.source_aspect_name().clone(),
            field.source_field_name().clone(),
        );
        if !projection_field_set.contains(&key) {
            return Err(AuthoredBundleError::UnprojectedShapeField {
                source_aspect: field.source_aspect().to_string(),
                source_field: field.source_field().to_string(),
                delivered_name: field.delivered_name().to_string(),
            });
        }
    }
    Ok(())
}
