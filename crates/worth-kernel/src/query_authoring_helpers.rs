use forge_foundational::facade::{AspectKey, CanonicalFieldPath, FieldKey};
use forge_query::facade::{
    AspectFieldKey, ForgeQueryAspectMutationOperation, ForgeQueryAspectTouch,
    ForgeQueryDeclarationAspectContract,
};

pub(crate) fn declaration_aspect_contract_from_slices(
    required: &[&str],
    preserved: &[&str],
    published: &[&str],
    masked: &[&str],
    incompatible: &[&str],
) -> ForgeQueryDeclarationAspectContract {
    ForgeQueryDeclarationAspectContract::new(
        aspect_field_keys(required),
        aspect_field_keys(preserved),
        aspect_field_keys(published),
        aspect_field_keys(masked),
        aspect_field_keys(incompatible),
    )
}

pub(crate) fn aspect_touches<const N: usize>(paths: [&str; N]) -> [ForgeQueryAspectTouch; N] {
    paths.map(aspect_touch)
}

pub(crate) fn aspect_mutation_operations<const N: usize>(
    operations: [&str; N],
) -> [ForgeQueryAspectMutationOperation; N] {
    operations.map(aspect_mutation_operation)
}

fn aspect_field_keys(paths: &[&str]) -> Vec<AspectFieldKey> {
    paths.iter().map(|path| aspect_field_key(path)).collect()
}

fn aspect_field_key(path: &str) -> AspectFieldKey {
    let Some((aspect, field)) = path.rsplit_once('.') else {
        panic!("Worth kernel Query aspect contract path `{path}` must contain a field segment");
    };
    AspectFieldKey::from_authoring_parts(aspect, field)
        .expect("Worth kernel Query aspect contract path must be admitted by Query")
}

fn aspect_touch(path: &str) -> ForgeQueryAspectTouch {
    let Some((aspect, field)) = path.rsplit_once('.') else {
        let aspect_key =
            AspectKey::new(path.to_string()).expect("Worth kernel Query aspect must admit");
        return ForgeQueryAspectTouch::whole_aspect(aspect_key);
    };
    let aspect_key =
        AspectKey::new(aspect.to_string()).expect("Worth kernel Query aspect must admit");
    let field_key = FieldKey::new(field.to_string()).expect("Worth kernel Query field must admit");
    ForgeQueryAspectTouch::aspect_field_path(aspect_key, CanonicalFieldPath::single(field_key))
}

fn aspect_mutation_operation(operation: &str) -> ForgeQueryAspectMutationOperation {
    let Some((operation_kind, path)) = operation.split_once(':') else {
        panic!("Worth kernel Query aspect operation `{operation}` must name operation kind");
    };
    match operation_kind {
        "insert" | "set" => ForgeQueryAspectMutationOperation::set(aspect_touch(path)),
        "delete" | "remove" => ForgeQueryAspectMutationOperation::clear(aspect_touch(path)),
        _ => panic!("Worth kernel Query aspect operation `{operation}` has unknown kind"),
    }
}
