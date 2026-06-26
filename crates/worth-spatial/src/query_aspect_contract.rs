use forge_foundational::facade::{AspectKey, CanonicalFieldPath, FieldKey};
#[cfg(test)]
use forge_query::facade::ForgeQueryAspectMutationOperation;
use forge_query::facade::{
    AspectFieldKey, ForgeQueryAspectTouch, ForgeQueryDeclarationAspectContract,
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

fn aspect_field_keys(paths: &[&str]) -> Vec<AspectFieldKey> {
    paths.iter().map(|path| aspect_field_key(path)).collect()
}

fn aspect_field_key(path: &str) -> AspectFieldKey {
    let Some((aspect, field)) = path.rsplit_once('.') else {
        panic!("Worth spatial Query aspect contract path `{path}` must contain a field segment");
    };
    AspectFieldKey::from_authoring_parts(aspect, field)
        .expect("Worth spatial Query aspect contract path must be admitted by Query")
}

pub(crate) fn aspect_touches_from_paths(paths: &[String]) -> Vec<ForgeQueryAspectTouch> {
    paths
        .iter()
        .map(|path| aspect_touch_from_path(path))
        .collect()
}

#[cfg(test)]
pub(crate) fn aspect_mutation_operations<const N: usize>(
    operations: [&str; N],
) -> [ForgeQueryAspectMutationOperation; N] {
    operations.map(aspect_mutation_operation)
}

pub(crate) fn aspect_touch_from_path(path: &str) -> ForgeQueryAspectTouch {
    let Some((aspect, field)) = path.rsplit_once('.') else {
        let aspect_key =
            AspectKey::new(path.to_string()).expect("Worth spatial Query aspect must admit");
        return ForgeQueryAspectTouch::whole_aspect(aspect_key);
    };
    let aspect_key =
        AspectKey::new(aspect.to_string()).expect("Worth spatial Query aspect must admit");
    let field_key = FieldKey::new(field.to_string()).expect("Worth spatial Query field must admit");
    ForgeQueryAspectTouch::aspect_field_path(aspect_key, CanonicalFieldPath::single(field_key))
}

#[cfg(test)]
fn aspect_mutation_operation(operation: &str) -> ForgeQueryAspectMutationOperation {
    let Some((operation_kind, path)) = operation.split_once(':') else {
        panic!("Worth spatial Query aspect operation `{operation}` must name operation kind");
    };
    match operation_kind {
        "insert" | "set" => ForgeQueryAspectMutationOperation::set(aspect_touch_from_path(path)),
        "delete" | "remove" => {
            ForgeQueryAspectMutationOperation::clear(aspect_touch_from_path(path))
        }
        _ => panic!("Worth spatial Query aspect operation `{operation}` has unknown kind"),
    }
}
