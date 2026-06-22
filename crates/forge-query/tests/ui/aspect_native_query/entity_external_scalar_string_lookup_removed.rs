use std::collections::BTreeMap;

use forge_foundational::facade::{AspectValue, CanonicalFieldPath};
use forge_query::facade::ForgeQueryEntity;

fn main() {
    let entity = entity_fixture();
    let _ = entity.external_scalar_value("title.value");

    let entity = entity_fixture();
    let _ = entity.external_projection_values();

    let _ = ForgeQueryEntity::from_external_projection_values(
        identity_fixture(),
        BTreeMap::<CanonicalFieldPath, AspectValue>::new(),
    );

    let entity = entity_fixture();
    let _ = entity.external_aspect_value("title.value");
}

fn entity_fixture() -> ForgeQueryEntity {
    panic!("fixture only")
}

fn identity_fixture() -> forge_query::facade::ForgeQueryEntityIdentity {
    panic!("fixture only")
}
