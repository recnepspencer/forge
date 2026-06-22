use forge_foundational::facade::AspectValue;
use forge_query::facade::ForgeQueryEntity;

fn main() {
    let entity = entity_fixture();
    let _ = entity.aspect_value("title");

    let entity = entity_fixture();
    for (aspect_path, value) in entity.aspect_values() {
        let _: (&str, &AspectValue) = (aspect_path, value);
    }
}

fn entity_fixture() -> ForgeQueryEntity {
    panic!("fixture only")
}
