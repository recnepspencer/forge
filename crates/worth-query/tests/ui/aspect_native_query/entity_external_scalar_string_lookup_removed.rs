use std::collections::BTreeMap;

use worth_foundational::facade::{AspectValue, CanonicalFieldPath};
use worth_query::facade::foundation::WorthQueryEntity;

fn main() {
    let entity = entity_fixture();
    let _ = entity.external_scalar_value("title.value");

    let entity = entity_fixture();
    let _ = entity.external_projection_values();

    let _ = WorthQueryEntity::from_external_projection_values(
        identity_fixture(),
        BTreeMap::<CanonicalFieldPath, AspectValue>::new(),
    );

    let entity = entity_fixture();
    let _ = entity.external_aspect_value("title.value");
}

fn entity_fixture() -> WorthQueryEntity {
    panic!("fixture only")
}

fn identity_fixture() -> worth_query::facade::foundation::WorthQueryEntityIdentity {
    panic!("fixture only")
}
