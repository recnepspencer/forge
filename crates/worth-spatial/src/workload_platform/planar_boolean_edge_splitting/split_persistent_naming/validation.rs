use std::collections::BTreeSet;

use super::counters::PlanarBooleanSplitPersistentNamingCounters;
use super::denial::{
    PlanarBooleanSplitPersistentNamingDenial, PlanarBooleanSplitPersistentNamingDenialKind,
};
use super::input::contains_geometry_or_display_authority;
use super::naming_row::PlanarBooleanSplitPersistentNameRow;

pub(crate) fn validate_persistent_name_rows(
    rows: &[PlanarBooleanSplitPersistentNameRow],
    counters: &mut PlanarBooleanSplitPersistentNamingCounters,
) -> Result<(), PlanarBooleanSplitPersistentNamingDenial> {
    if rows.is_empty() {
        counters.rejected_dangling_reference();
        return Err(PlanarBooleanSplitPersistentNamingDenial::new(
            PlanarBooleanSplitPersistentNamingDenialKind::MissingSplitArtifact,
            "split-persistent-naming",
            "persistent naming requires at least one split artifact row",
        ));
    }
    let mut row_identities = BTreeSet::new();
    let mut persistent_names = BTreeSet::new();
    for row in rows {
        reject_geometry_or_display_authority(row, counters)?;
        if !row_identities.insert(row.row_identity().to_string()) {
            counters.rejected_duplicate_name();
            return Err(PlanarBooleanSplitPersistentNamingDenial::new(
                PlanarBooleanSplitPersistentNamingDenialKind::DuplicatePersistentName,
                row.row_identity(),
                "split persistent-name row identities must be unique",
            ));
        }
        if !persistent_names.insert(row.persistent_name_identity().to_string()) {
            counters.rejected_duplicate_name();
            return Err(PlanarBooleanSplitPersistentNamingDenial::new(
                PlanarBooleanSplitPersistentNamingDenialKind::DuplicatePersistentName,
                row.persistent_name_identity(),
                "split persistent-name identities must be unique",
            ));
        }
        if row.source_edge_identity().is_empty()
            || row.artifact_identity().is_empty()
            || row.identity_evolution_result_digest().is_empty()
        {
            counters.rejected_dangling_reference();
            return Err(PlanarBooleanSplitPersistentNamingDenial::new(
                PlanarBooleanSplitPersistentNamingDenialKind::DanglingPersistentNameReference,
                row.row_identity(),
                "split persistent-name rows must bind source, artifact, and Query evolution identities",
            ));
        }
    }
    Ok(())
}

fn reject_geometry_or_display_authority(
    row: &PlanarBooleanSplitPersistentNameRow,
    counters: &mut PlanarBooleanSplitPersistentNamingCounters,
) -> Result<(), PlanarBooleanSplitPersistentNamingDenial> {
    let has_authority_poison = contains_geometry_or_display_authority(row.source_edge_identity())
        || contains_geometry_or_display_authority(row.artifact_identity())
        || row
            .event_cause_identities()
            .iter()
            .any(|identity| contains_geometry_or_display_authority(identity));
    if has_authority_poison {
        counters.rejected_geometry_authority_attempt();
        return Err(PlanarBooleanSplitPersistentNamingDenial::new(
            PlanarBooleanSplitPersistentNamingDenialKind::GeometryOrDisplayAuthorityRejected,
            row.row_identity(),
            "persistent names must not be minted from geometry, display, or debug authority",
        ));
    }
    Ok(())
}
