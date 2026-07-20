use crate::authoring::ResultShapeFamily;

pub fn canonical_result_shape_family_digest_part(family: &ResultShapeFamily) -> String {
    format!("shape_family:{family:?}")
}
