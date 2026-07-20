use crate::authoring::{RawAuthoredQuery, RawAuthoredResultShape};
use crate::facade::foundation::{AspectFieldSelector, AuthoredResultShapeField, RootEntityKey};

pub fn runtime_detail_query() -> crate::authoring::DetailAuthoredQuery {
    RawAuthoredQuery::detail_builder(RootEntityKey::new("user").unwrap())
        .project(AspectFieldSelector::new("identity", "id").unwrap())
        .build()
        .unwrap()
}

pub fn runtime_detail_result_shape() -> crate::authoring::DetailAuthoredResultShape {
    RawAuthoredResultShape::detail_builder()
        .field(AuthoredResultShapeField::new("identity", "id", "id").unwrap())
        .build()
        .unwrap()
}
