use std::collections::{BTreeMap, BTreeSet};

use super::counters::PlanarBooleanOverlapRegionLedgerAssemblyCounters;
use super::denial::{
    PlanarBooleanOverlapRegionLedgerAssemblyDenial,
    PlanarBooleanOverlapRegionLedgerAssemblyDenialKind,
};
use super::rows::PlanarBooleanOverlapRegionLedgerRow;
use crate::workload_platform::planar_boolean_overlap_region_extraction::{
    PlanarBooleanAdmittedOverlapRegionSet, PlanarBooleanBoundaryOnlyOverlapOutcomeSet,
    PlanarBooleanOverlapRegionCanonicalWindingRow,
    PlanarBooleanOverlapRegionIdentityRow, PlanarBooleanOverlapRegionPersistentNamePropagationRow,
    PlanarBooleanOverlapRegionSubshapeSignatureRow,
};

pub(super) fn validate_input_identities(
    input: PlanarBooleanOverlapRegionLedgerAssemblyInput<'_>,
    counters: &PlanarBooleanOverlapRegionLedgerAssemblyCounters,
) -> Result<(), PlanarBooleanOverlapRegionLedgerAssemblyDenial> {
    let bundle = input.identity_lineage();
    let identity_map = bundle.overlap_region_identity_map();
    let canonical_bundle = bundle.source_post_admission_normalization();
    let canonical = canonical_bundle.overlap_region_canonical_winding();
    let boundary = canonical_bundle.source_region_candidate_boundary();
    let admitted = boundary.admitted_overlap_regions();

    if identity_map.request_identity().is_empty()
        || identity_map.arrangement_graph_identity().is_empty()
        || identity_map.cell_set_identity().is_empty()
        || identity_map.ordering_basis_identity().is_empty()
    {
        return Err(PlanarBooleanOverlapRegionLedgerAssemblyDenial::new(
            PlanarBooleanOverlapRegionLedgerAssemblyDenialKind::InputIdentityMismatchDenied,
            identity_map.request_identity(),
            *counters,
            "overlap ledger assembly requires a complete phase-thirteen identity basis",
        ));
    }

    if identity_map.request_identity() != canonical.request_identity()
        || identity_map.arrangement_graph_identity() != canonical.arrangement_graph_identity()
        || identity_map.cell_set_identity() != canonical.cell_set_identity()
        || identity_map.ordering_basis_identity() != canonical.ordering_basis_identity()
    {
        return Err(PlanarBooleanOverlapRegionLedgerAssemblyDenial::new(
            PlanarBooleanOverlapRegionLedgerAssemblyDenialKind::InputIdentityMismatchDenied,
            identity_map.request_identity(),
            *counters,
            "overlap ledger assembly denies phase-thirteen bundles whose identity basis does not match the carried phase-twelve canonical winding proof",
        ));
    }

    if admitted.request_identity() != canonical.request_identity()
        || admitted.arrangement_graph_identity() != canonical.arrangement_graph_identity()
        || admitted.cell_set_identity() != canonical.cell_set_identity()
        || admitted.ordering_basis_identity() != canonical.ordering_basis_identity()
    {
        return Err(PlanarBooleanOverlapRegionLedgerAssemblyDenial::new(
            PlanarBooleanOverlapRegionLedgerAssemblyDenialKind::ForeignPriorProofLineageDenied,
            admitted.request_identity(),
            *counters,
            "overlap ledger assembly denies foreign or mismatched prior proof products",
        ));
    }

    Ok(())
}

pub(super) fn validate_supporting_rows(
    persistent_name_rows: &[PlanarBooleanOverlapRegionPersistentNamePropagationRow],
    signature_rows: &[PlanarBooleanOverlapRegionSubshapeSignatureRow],
    identity_rows: &[PlanarBooleanOverlapRegionIdentityRow],
    counters: &mut PlanarBooleanOverlapRegionLedgerAssemblyCounters,
) -> Result<(), PlanarBooleanOverlapRegionLedgerAssemblyDenial> {
    let valid_regions = identity_rows
        .iter()
        .map(|row| row.region_identity())
        .collect::<BTreeSet<_>>();

    for row in persistent_name_rows {
        if !valid_regions.contains(row.region_identity()) {
            counters.denied_row();
            return Err(PlanarBooleanOverlapRegionLedgerAssemblyDenial::new(
                PlanarBooleanOverlapRegionLedgerAssemblyDenialKind::MissingPriorProofProductDenied,
                row.region_identity(),
                *counters,
                "overlap ledger assembly denies propagated-name rows that are not justified by a minted overlap-region identity",
            ));
        }
    }

    for row in signature_rows {
        if !valid_regions.contains(row.region_identity()) {
            counters.denied_row();
            return Err(PlanarBooleanOverlapRegionLedgerAssemblyDenial::new(
                PlanarBooleanOverlapRegionLedgerAssemblyDenialKind::MissingPriorProofProductDenied,
                row.region_identity(),
                *counters,
                "overlap ledger assembly denies subshape-signature rows that are not justified by a minted overlap-region identity",
            ));
        }
    }

    Ok(())
}

pub(super) fn signature_rows_by_region<'a>(
    signature_rows: &'a [PlanarBooleanOverlapRegionSubshapeSignatureRow],
) -> BTreeMap<&'a str, &'a PlanarBooleanOverlapRegionSubshapeSignatureRow> {
    signature_rows
        .iter()
        .map(|row| (row.region_identity(), row))
        .collect()
}

pub(super) fn persistent_names_by_region<'a>(
    rows: &'a [PlanarBooleanOverlapRegionPersistentNamePropagationRow],
) -> BTreeMap<&'a str, Vec<&'a PlanarBooleanOverlapRegionPersistentNamePropagationRow>> {
    let mut grouped = BTreeMap::<&str, Vec<&PlanarBooleanOverlapRegionPersistentNamePropagationRow>>::new();
    for row in rows {
        grouped.entry(row.region_identity()).or_default().push(row);
    }
    grouped
}

pub(super) fn canonical_rows_by_identity<'a>(
    rows: &'a [PlanarBooleanOverlapRegionCanonicalWindingRow],
) -> BTreeMap<&'a str, &'a PlanarBooleanOverlapRegionCanonicalWindingRow> {
    rows.iter()
        .map(|row| (row.canonical_winding_identity(), row))
        .collect()
}

pub(super) fn require_canonical_row<'a>(
    identity_row: &PlanarBooleanOverlapRegionIdentityRow,
    canonical_rows: &'a BTreeMap<&str, &'a PlanarBooleanOverlapRegionCanonicalWindingRow>,
    counters: &mut PlanarBooleanOverlapRegionLedgerAssemblyCounters,
) -> Result<&'a PlanarBooleanOverlapRegionCanonicalWindingRow, PlanarBooleanOverlapRegionLedgerAssemblyDenial> {
    canonical_rows
        .get(identity_row.canonical_winding_identity())
        .copied()
        .ok_or_else(|| {
            counters.denied_row();
            PlanarBooleanOverlapRegionLedgerAssemblyDenial::new(
                PlanarBooleanOverlapRegionLedgerAssemblyDenialKind::MissingCanonicalWindingProofDenied,
                identity_row.region_identity(),
                *counters,
                "overlap ledger assembly denies identity rows that are missing their carried phase-twelve canonical winding proof",
            )
        })
}

pub(super) fn require_signature_row<'a>(
    region_identity: &str,
    signature_rows: &'a BTreeMap<&str, &'a PlanarBooleanOverlapRegionSubshapeSignatureRow>,
    counters: &mut PlanarBooleanOverlapRegionLedgerAssemblyCounters,
) -> Result<&'a PlanarBooleanOverlapRegionSubshapeSignatureRow, PlanarBooleanOverlapRegionLedgerAssemblyDenial> {
    signature_rows.get(region_identity).copied().ok_or_else(|| {
        counters.denied_row();
        PlanarBooleanOverlapRegionLedgerAssemblyDenial::new(
            PlanarBooleanOverlapRegionLedgerAssemblyDenialKind::MissingPriorProofProductDenied,
            region_identity,
            *counters,
            "overlap ledger assembly denies overlap-region identities that are missing their phase-thirteen subshape-signature proof",
        )
    })
}

pub(super) fn validate_identity_matches_canonical(
    identity_row: &PlanarBooleanOverlapRegionIdentityRow,
    canonical_row: &PlanarBooleanOverlapRegionCanonicalWindingRow,
    counters: &mut PlanarBooleanOverlapRegionLedgerAssemblyCounters,
) -> Result<(), PlanarBooleanOverlapRegionLedgerAssemblyDenial> {
    if identity_row.source_kind() != canonical_row.source_kind()
        || identity_row.source_identity() != canonical_row.source_identity()
    {
        counters.denied_row();
        return Err(PlanarBooleanOverlapRegionLedgerAssemblyDenial::new(
            PlanarBooleanOverlapRegionLedgerAssemblyDenialKind::SyntheticOverlapRowDenied,
            identity_row.region_identity(),
            *counters,
            "overlap ledger assembly denies identity rows that no longer match their carried canonical winding source witness",
        ));
    }
    Ok(())
}

pub(super) fn validate_source_truth(
    identity_row: &PlanarBooleanOverlapRegionIdentityRow,
    admitted: &PlanarBooleanAdmittedOverlapRegionSet,
    boundary_only: &PlanarBooleanBoundaryOnlyOverlapOutcomeSet,
    counters: &mut PlanarBooleanOverlapRegionLedgerAssemblyCounters,
) -> Result<(), PlanarBooleanOverlapRegionLedgerAssemblyDenial> {
    let admitted_identities = admitted
        .rows()
        .iter()
        .map(|row| row.admitted_region_identity())
        .collect::<BTreeSet<_>>();
    let boundary_only_identities = boundary_only
        .rows()
        .iter()
        .map(|row| row.outcome_identity())
        .collect::<BTreeSet<_>>();

    let found = match identity_row.source_kind() {
        crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanOverlapRegionCanonicalWindingSourceKind::AdmittedRegion => {
            admitted_identities.contains(identity_row.source_identity())
        }
        crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanOverlapRegionCanonicalWindingSourceKind::BoundaryOnlyOutcome => {
            boundary_only_identities.contains(identity_row.source_identity())
        }
    };

    if !found {
        counters.denied_row();
        return Err(PlanarBooleanOverlapRegionLedgerAssemblyDenial::new(
            PlanarBooleanOverlapRegionLedgerAssemblyDenialKind::SyntheticOverlapRowDenied,
            identity_row.region_identity(),
            *counters,
            "overlap ledger assembly denies overlap rows whose source truth cannot be justified from the carried admitted-region or boundary-only proof products",
        ));
    }

    Ok(())
}

pub(super) fn validate_ledger_rows(
    rows: &[PlanarBooleanOverlapRegionLedgerRow],
    counters: &mut PlanarBooleanOverlapRegionLedgerAssemblyCounters,
) -> Result<(), PlanarBooleanOverlapRegionLedgerAssemblyDenial> {
    let mut seen = BTreeSet::new();
    for row in rows {
        if !seen.insert(row.ledger_row_identity()) {
            counters.denied_row();
            return Err(PlanarBooleanOverlapRegionLedgerAssemblyDenial::new(
                PlanarBooleanOverlapRegionLedgerAssemblyDenialKind::SyntheticOverlapRowDenied,
                row.ledger_row_identity(),
                *counters,
                "overlap ledger assembly denies duplicate or synthetic ledger rows",
            ));
        }
    }
    Ok(())
}

use crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanOverlapRegionLedgerAssemblyInput;
