use forge_foundational::facade::{AspectValue, CanonicalFieldPath};

use crate::memory_workspace::ForgeQueryEntity;

pub(super) fn consumed_scalar_value_from_entity_path(
    row: &ForgeQueryEntity,
    field_path: &CanonicalFieldPath,
) -> Result<Option<AspectValue>, String> {
    Ok(row.scalar_value_at(field_path).cloned())
}
