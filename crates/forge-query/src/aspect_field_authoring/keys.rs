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
    AspectFieldLocator::new(
        LocatorAuthority::Planned,
        aspect_key,
        CanonicalFieldPath::single(field_key),
    )
}

pub(crate) fn terminal_field_label(path: &str) -> Result<&str, String> {
    path.split('.')
        .next_back()
        .filter(|segment| !segment.trim().is_empty())
        .ok_or_else(|| format!("`{path}` does not contain a field segment"))
}
