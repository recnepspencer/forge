use std::collections::{BTreeMap, BTreeSet};

use super::counters::PlanarBooleanOverlapRegionIdentityLineageCounters;
use super::denial::{
    PlanarBooleanOverlapRegionIdentityLineageDenial,
    PlanarBooleanOverlapRegionIdentityLineageDenialKind,
};
use super::rows::{
    PlanarBooleanOverlapRegionIdentityRow, PlanarBooleanOverlapRegionPersistentNamePropagationRow,
};
use crate::workload_platform::planar_boolean_overlap_region_extraction::{
    PlanarBooleanOverlapRegionCanonicalWindingRow, PlanarBooleanOverlapRegionIdentityLineageInput,
};

pub(super) fn validate_input_identities(
    input: PlanarBooleanOverlapRegionIdentityLineageInput<'_>,
    counters: &PlanarBooleanOverlapRegionIdentityLineageCounters,
) -> Result<(), PlanarBooleanOverlapRegionIdentityLineageDenial> {
    let canonical = input
        .post_admission_normalization()
        .overlap_region_canonical_winding();
    if canonical.request_identity().is_empty()
        || canonical.arrangement_graph_identity().is_empty()
        || canonical.cell_set_identity().is_empty()
        || canonical.ordering_basis_identity().is_empty()
    {
        return Err(PlanarBooleanOverlapRegionIdentityLineageDenial::new(
            PlanarBooleanOverlapRegionIdentityLineageDenialKind::InputIdentityMismatchDenied,
            canonical.request_identity(),
            *counters,
            "overlap-region identity and lineage minting requires a complete phase-twelve canonical proof basis",
        ));
    }
    Ok(())
}

pub(super) fn canonical_signature_basis(row: &PlanarBooleanOverlapRegionCanonicalWindingRow) -> String {
    let mut parts = vec![
        row.canonical_winding_identity().to_string(),
        row.island_identity().to_string(),
        row.neighborhood_identity().to_string(),
        row.area_overlap_component_identity().unwrap_or("boundary-only").to_string(),
    ];
    parts.extend(row.canonical_boundary_segment_identities().iter().cloned());
    parts.extend(row.canonical_source_loop_identities().iter().cloned());
    parts.extend(row.lineage_identities().iter().cloned());
    parts.join("|")
}

pub(super) fn validate_unique_region_identities(
    rows: &[PlanarBooleanOverlapRegionIdentityRow],
    counters: &mut PlanarBooleanOverlapRegionIdentityLineageCounters,
) -> Result<(), PlanarBooleanOverlapRegionIdentityLineageDenial> {
    let mut grouped = BTreeMap::<&str, Vec<&str>>::new();
    for row in rows {
        grouped
            .entry(row.region_identity())
            .or_default()
            .push(row.source_identity());
    }
    if let Some((identity, _)) = grouped.iter().find(|(_, sources)| sources.len() != 1) {
        counters.denied_row();
        return Err(PlanarBooleanOverlapRegionIdentityLineageDenial::new(
            PlanarBooleanOverlapRegionIdentityLineageDenialKind::DuplicateRegionIdentityDenied,
            *identity,
            *counters,
            "overlap-region identity minting denies canonical rows that still collapse to the same minted region identity",
        ));
    }
    Ok(())
}

pub(super) fn validate_persistent_name_rows(
    rows: &[PlanarBooleanOverlapRegionPersistentNamePropagationRow],
    identity_rows: &[PlanarBooleanOverlapRegionIdentityRow],
    counters: &mut PlanarBooleanOverlapRegionIdentityLineageCounters,
) -> Result<(), PlanarBooleanOverlapRegionIdentityLineageDenial> {
    let valid_region_identities = identity_rows
        .iter()
        .map(|row| row.region_identity())
        .collect::<BTreeSet<_>>();
    let mut names_to_regions = BTreeMap::<&str, Vec<&str>>::new();

    for row in rows {
        if row.persistent_name_identity().is_empty() || !valid_region_identities.contains(row.region_identity()) {
            counters.denied_row();
            return Err(PlanarBooleanOverlapRegionIdentityLineageDenial::new(
                PlanarBooleanOverlapRegionIdentityLineageDenialKind::DanglingPersistentNameReferenceDenied,
                row.region_identity(),
                *counters,
                "overlap-region persistent-name propagation denies empty or dangling propagated-name references",
            ));
        }
        names_to_regions
            .entry(row.persistent_name_identity())
            .or_default()
            .push(row.region_identity());
    }

    if let Some((persistent_name, regions)) = names_to_regions
        .iter()
        .find(|(_, regions)| regions.iter().collect::<BTreeSet<_>>().len() != 1)
    {
        counters.denied_row();
        return Err(PlanarBooleanOverlapRegionIdentityLineageDenial::new(
            PlanarBooleanOverlapRegionIdentityLineageDenialKind::ConflictingPersistentNamePropagationDenied,
            *persistent_name,
            *counters,
            if regions.len() > 1 {
                "overlap-region persistent-name propagation denies the same persistent name surviving into more than one overlap-region identity"
            } else {
                "overlap-region persistent-name propagation denies contradictory name survival"
            },
        ));
    }

    Ok(())
}
