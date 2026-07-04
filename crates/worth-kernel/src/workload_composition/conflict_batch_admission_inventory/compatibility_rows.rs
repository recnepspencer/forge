use super::error::ConflictBatchAdmissionInventoryError;
use super::row::ConflictBatchAdmissionInventoryRow;

pub(crate) fn compatibility_inventory_rows(
) -> Result<Vec<ConflictBatchAdmissionInventoryRow>, ConflictBatchAdmissionInventoryError> {
    let mut rows = Vec::new();
    rows.extend(super::topology_compatibility_rows::topology_compatibility_inventory_rows()?);
    rows.extend(super::spatial_compatibility_rows::spatial_compatibility_inventory_rows()?);
    Ok(rows)
}
