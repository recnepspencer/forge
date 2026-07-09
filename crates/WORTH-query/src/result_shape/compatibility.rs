use crate::authoring::{QueryFamily, ResultShapeFamily};

pub(crate) fn family_matches_query(
    query_family: &QueryFamily,
    result_shape_family: &ResultShapeFamily,
) -> bool {
    matches!(
        (query_family, result_shape_family),
        (QueryFamily::Detail, ResultShapeFamily::Detail)
            | (QueryFamily::Collection, ResultShapeFamily::Collection)
    )
}
