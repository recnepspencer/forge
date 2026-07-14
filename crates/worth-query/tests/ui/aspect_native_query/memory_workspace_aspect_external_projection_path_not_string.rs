use worth_foundational::facade::{AspectKey, CanonicalFieldPath, FieldKey};
use worth_query::facade::foundation::WorthQueryAspect;
use worth_query::facade::runtime::WorthQueryAspectTouch;

fn main() {
    let aspect = WorthQueryAspect::new(
        WorthQueryAspectTouch::aspect_field_path(
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
