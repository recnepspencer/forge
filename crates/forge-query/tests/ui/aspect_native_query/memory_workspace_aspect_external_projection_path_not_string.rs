use forge_foundational::facade::{AspectKey, CanonicalFieldPath, FieldKey};
use forge_query::facade::{ForgeQueryAspect, ForgeQueryAspectTouch};

fn main() {
    let aspect = ForgeQueryAspect::new(
        ForgeQueryAspectTouch::aspect_field_path(
            AspectKey::new("title").unwrap(),
            CanonicalFieldPath::single(FieldKey::new("value").unwrap()),
        ),
        CanonicalFieldPath::new([
            FieldKey::new("title").unwrap(),
            FieldKey::new("value").unwrap(),
        ])
        .unwrap(),
    )
    .unwrap();
    let _: &str = aspect.external_projection_path();
    let _: &str = aspect.native_field_path();
}
