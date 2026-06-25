use forge_foundational::facade::{AspectKey, CanonicalFieldPath, FieldKey};
use forge_query::facade::{AspectFieldKey, ForgeQueryAspectTouch};

pub(crate) fn aspect_field_key(path: &str) -> AspectFieldKey {
    let Some((aspect, field)) = path.rsplit_once('.') else {
        panic!("spatial public contract aspect field `{path}` must include a field segment");
    };
    AspectFieldKey::from_authoring_parts(aspect, field)
        .expect("spatial public contract aspect field should admit")
}

pub(crate) fn aspect_touch(path: &str) -> ForgeQueryAspectTouch {
    let Some((aspect, field)) = path.rsplit_once('.') else {
        let aspect_key = AspectKey::new(path.to_string()).expect("aspect should admit");
        return ForgeQueryAspectTouch::whole_aspect(aspect_key);
    };
    let aspect_key = AspectKey::new(aspect.to_string()).expect("aspect should admit");
    let field_key = FieldKey::new(field.to_string()).expect("field should admit");
    ForgeQueryAspectTouch::aspect_field_path(aspect_key, CanonicalFieldPath::single(field_key))
}
