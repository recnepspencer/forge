use forge_foundational::facade::{
    AspectFieldLocator, AspectKey, CanonicalFieldPath, FieldKey, LocatorAuthority,
};

pub(crate) fn aspect_key(label: &str) -> Result<AspectKey, String> {
    AspectKey::new(label).ok_or_else(|| format!("`{label}` is not a foundational aspect key"))
}

pub(crate) fn field_key(label: &str) -> Result<FieldKey, String> {
    FieldKey::new(label).ok_or_else(|| format!("`{label}` is not a foundational field key"))
}

pub(crate) fn planned_single_field_locator(
    aspect_key: AspectKey,
    field_key: FieldKey,
) -> AspectFieldLocator {
    planned_field_path_locator(aspect_key, CanonicalFieldPath::single(field_key))
}

pub(crate) fn planned_field_path_locator(
    aspect_key: AspectKey,
    field_path: CanonicalFieldPath,
) -> AspectFieldLocator {
    AspectFieldLocator::new(LocatorAuthority::Planned, aspect_key, field_path)
}
