use crate::authoring::RawAuthoredResultShape;
use crate::facade::AuthoredResultShapeField;

pub fn direct_detail_shape() -> crate::authoring::DetailAuthoredResultShape {
    RawAuthoredResultShape::detail_builder()
        .field(AuthoredResultShapeField::new("title", "text", "title").unwrap())
        .field(AuthoredResultShapeField::new("status", "kind", "status").unwrap())
        .build()
        .unwrap()
}

pub fn reordered_detail_shape() -> crate::authoring::DetailAuthoredResultShape {
    RawAuthoredResultShape::detail_builder()
        .field(AuthoredResultShapeField::new("status", "kind", "status").unwrap())
        .field(AuthoredResultShapeField::new("title", "text", "title").unwrap())
        .build()
        .unwrap()
}

pub fn collection_shape() -> crate::authoring::CollectionAuthoredResultShape {
    RawAuthoredResultShape::collection_builder()
        .field(AuthoredResultShapeField::new("title", "text", "title").unwrap())
        .build()
        .unwrap()
}

pub fn reordered_collection_shape() -> crate::authoring::CollectionAuthoredResultShape {
    RawAuthoredResultShape::collection_builder()
        .field(AuthoredResultShapeField::new("status", "kind", "status").unwrap())
        .field(AuthoredResultShapeField::new("title", "text", "title").unwrap())
        .build()
        .unwrap()
}

pub fn duplicate_projection_detail_shape() -> crate::authoring::DetailAuthoredResultShape {
    RawAuthoredResultShape::detail_builder()
        .field(AuthoredResultShapeField::new("title", "text", "title").unwrap())
        .build()
        .unwrap()
}

pub fn single_projection_detail_shape() -> crate::authoring::DetailAuthoredResultShape {
    RawAuthoredResultShape::detail_builder()
        .field(AuthoredResultShapeField::new("title", "text", "title").unwrap())
        .build()
        .unwrap()
}

pub fn unsupported_detail_shape_for_test() -> crate::authoring::RawAuthoredResultShape {
    RawAuthoredResultShape::unsupported_for_test("inspector")
        .with_field(AuthoredResultShapeField::new("title", "text", "title").unwrap())
}
