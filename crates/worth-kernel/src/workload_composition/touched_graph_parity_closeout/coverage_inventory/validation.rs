use std::collections::BTreeSet;

use super::current::CrossFamilyCoverageInventoryError;
use super::row::{CrossFamilyCoverageFamilyKind as FamilyKind, CrossFamilyCoverageRow};

pub(crate) fn validate_rows(
    rows: &[CrossFamilyCoverageRow],
) -> Result<(), CrossFamilyCoverageInventoryError> {
    let mut covered_families = BTreeSet::new();
    for row in rows {
        let row: &CrossFamilyCoverageRow = row;
        if row.source_path().is_empty()
            || row.current_owner_crate().is_empty()
            || row.replacement_lane().is_empty()
            || row.selected_identity_fields_consumed().is_empty()
            || row.current_surface().is_empty()
            || row.upstream_authority_source().is_empty()
            || row.selected_route_or_equivalence_source().is_empty()
            || row.public_or_internal_consumer_kind().is_empty()
            || row.residue_posture().is_empty()
        {
            return Err(CrossFamilyCoverageInventoryError::InvalidCoverageRow);
        }
        if row.ordinary_path_live_caller_surface().is_empty()
            || row.ordinary_path_live_caller_path().is_empty()
        {
            return Err(CrossFamilyCoverageInventoryError::InvalidCoverageRow);
        }
        if !source_path_exists(row.source_path())
            || !source_path_exists(row.replacement_lane())
            || !source_path_exists(row.ordinary_path_live_caller_path())
        {
            return Err(CrossFamilyCoverageInventoryError::InvalidCoverageRow);
        }
        if row.ordinary_path_reachable() {
            covered_families.insert(row.family_kind());
        } else {
            return Err(CrossFamilyCoverageInventoryError::MissingLiveCallerProof);
        }
    }

    if FamilyKind::ALL
        .iter()
        .any(|family_kind| !covered_families.contains(family_kind))
    {
        return Err(CrossFamilyCoverageInventoryError::InvalidCoverageRow);
    }

    Ok(())
}

fn source_path_exists(source_path: &str) -> bool {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join(source_path)
        .exists()
}
