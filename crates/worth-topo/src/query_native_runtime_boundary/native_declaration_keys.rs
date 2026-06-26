use forge_foundational::facade::{AspectKey, CanonicalFieldPath, FieldKey};
use forge_query::facade::{AspectFieldKey, ForgeQueryAspectTouch};
use schema::facade::{QueryAspectPath, QueryLiveField};

pub(crate) fn query_live_field_key(field: QueryLiveField) -> AspectFieldKey {
    native_aspect_field_key(field.aspect(), field.field())
}

pub(crate) fn query_aspect_field_key(path: QueryAspectPath) -> AspectFieldKey {
    native_aspect_field_key(path.section(), path.field())
}

pub(crate) fn query_aspect_touch(path: QueryAspectPath) -> ForgeQueryAspectTouch {
    let aspect = AspectKey::new(path.section())
        .expect("schema query aspect sections must admit as native aspect keys");
    let field = FieldKey::new(path.field())
        .expect("schema query aspect fields must admit as native field keys");
    ForgeQueryAspectTouch::aspect_field_path(
        aspect,
        CanonicalFieldPath::new([field])
            .expect("schema query aspect fields must build canonical field paths"),
    )
}

fn native_aspect_field_key(aspect: &str, field: &str) -> AspectFieldKey {
    let aspect =
        AspectKey::new(aspect).expect("schema query aspect sections must admit as native keys");
    let field = FieldKey::new(field).expect("schema query fields must admit as native keys");
    AspectFieldKey::from_native_keys(&aspect, &field)
}
