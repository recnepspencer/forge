use std::collections::BTreeMap;

use super::counters::PlanarBooleanPostAdmissionNormalizationCounters;
use super::denial::{
    PlanarBooleanPostAdmissionNormalizationDenial,
    PlanarBooleanPostAdmissionNormalizationDenialKind,
};
use super::identity::{canonical_row_identity, canonical_set_identity};
use super::input::PlanarBooleanPostAdmissionNormalizationInput;
use super::product::{
    PlanarBooleanOverlapRegionCanonicalWindingSet, PlanarBooleanPostAdmissionNormalizationBundle,
};
use super::rows::{
    PlanarBooleanOverlapRegionCanonicalWindingRow,
    PlanarBooleanOverlapRegionCanonicalWindingSourceKind,
};
use super::validation::{
    admitted_witness_key, boundary_only_witness_key, boundary_witness_mismatch_denial,
    canonicalized_strings, ordered_witness_strings, validate_input_identities,
};

pub(super) fn build_post_admission_normalization_bundle(
    input: PlanarBooleanPostAdmissionNormalizationInput<'_>,
) -> Result<
    PlanarBooleanPostAdmissionNormalizationBundle,
    PlanarBooleanPostAdmissionNormalizationDenial,
> {
    let mut counters = PlanarBooleanPostAdmissionNormalizationCounters::default();
    validate_input_identities(input, &counters)?;

    let boundary = input.region_candidate_boundary();
    let admitted = boundary.admitted_overlap_regions();
    let boundary_only = boundary.boundary_only_overlap_outcomes();
    let request_identity = admitted.request_identity().to_string();
    let mut rows = Vec::new();

    ensure_unique_admitted_witnesses(admitted.rows(), &mut counters)?;
    for row in admitted.rows() {
        counters.examined_admitted_region();
        rows.push(canonicalize_admitted_region_row(
            &request_identity,
            row,
            &mut counters,
        )?);
        counters.admitted_canonical_row();
    }

    ensure_unique_boundary_only_witnesses(boundary_only.rows(), &mut counters)?;
    for row in boundary_only.rows() {
        counters.examined_boundary_only_outcome();
        rows.push(canonicalize_boundary_only_row(
            &request_identity,
            row,
            &mut counters,
        )?);
        counters.admitted_canonical_row();
    }

    Ok(PlanarBooleanPostAdmissionNormalizationBundle::new(
        format!(
            "post-admission-normalization:{}:{}",
            request_identity,
            rows.len()
        ),
        PlanarBooleanOverlapRegionCanonicalWindingSet::new(
            canonical_set_identity(&request_identity, rows.len()),
            request_identity.clone(),
            admitted.arrangement_graph_identity().to_string(),
            admitted.cell_set_identity().to_string(),
            admitted.ordering_basis_identity().to_string(),
            rows,
        ),
        boundary.clone(),
        counters,
    ))
}

fn ensure_unique_admitted_witnesses(
    rows: &[crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanAdmittedOverlapRegionRow],
    counters: &mut PlanarBooleanPostAdmissionNormalizationCounters,
) -> Result<(), PlanarBooleanPostAdmissionNormalizationDenial> {
    let mut grouped = BTreeMap::<String, Vec<&str>>::new();
    for row in rows {
        grouped
            .entry(admitted_witness_key(row))
            .or_default()
            .push(row.admitted_region_identity());
    }
    if let Some(conflicting_rows) = grouped.values().find(|rows| rows.len() != 1) {
        counters.denied_canonical_row();
        return Err(PlanarBooleanPostAdmissionNormalizationDenial::new(
            PlanarBooleanPostAdmissionNormalizationDenialKind::AmbiguousCanonicalWindingDenied,
            conflicting_rows[0],
            *counters,
            "post-admission canonical winding denies admitted-region witnesses that still admit more than one plausible final canonical winding surface",
        ));
    }
    Ok(())
}

fn ensure_unique_boundary_only_witnesses(
    rows: &[crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanBoundaryOnlyOverlapOutcomeRow],
    counters: &mut PlanarBooleanPostAdmissionNormalizationCounters,
) -> Result<(), PlanarBooleanPostAdmissionNormalizationDenial> {
    let mut grouped = BTreeMap::<String, Vec<&str>>::new();
    for row in rows {
        grouped
            .entry(boundary_only_witness_key(row))
            .or_default()
            .push(row.outcome_identity());
    }
    if let Some(conflicting_rows) = grouped.values().find(|rows| rows.len() != 1) {
        counters.denied_canonical_row();
        return Err(PlanarBooleanPostAdmissionNormalizationDenial::new(
            PlanarBooleanPostAdmissionNormalizationDenialKind::AmbiguousCanonicalBoundaryDenied,
            conflicting_rows[0],
            *counters,
            "post-admission boundary normalization denies boundary-only witnesses that still admit more than one plausible canonical boundary order",
        ));
    }
    Ok(())
}

fn canonicalize_admitted_region_row(
    request_identity: &str,
    row: &crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanAdmittedOverlapRegionRow,
    counters: &mut PlanarBooleanPostAdmissionNormalizationCounters,
) -> Result<
    PlanarBooleanOverlapRegionCanonicalWindingRow,
    PlanarBooleanPostAdmissionNormalizationDenial,
> {
    let canonical_boundary_segment_identities =
        ordered_witness_strings(row.canonical_boundary_segment_witness());
    let canonical_source_loop_identities =
        ordered_witness_strings(row.canonical_source_loop_witness());
    let canonical_source_edge_identities = canonicalized_strings(row.source_edge_identities());

    let (canonical_boundary_segment_identities, canonical_source_loop_identities) = match (
        canonical_boundary_segment_identities,
        canonical_source_loop_identities,
    ) {
        (Some(boundary_segments), Some(source_loops)) => (boundary_segments, source_loops),
        _ => {
            counters.denied_canonical_row();
            return Err(boundary_witness_mismatch_denial(
                PlanarBooleanOverlapRegionCanonicalWindingSourceKind::AdmittedRegion,
                row.admitted_region_identity(),
                counters,
            ));
        }
    };

    if canonical_source_edge_identities.is_empty() {
        counters.denied_canonical_row();
        return Err(boundary_witness_mismatch_denial(
            PlanarBooleanOverlapRegionCanonicalWindingSourceKind::AdmittedRegion,
            row.admitted_region_identity(),
            counters,
        ));
    }

    Ok(PlanarBooleanOverlapRegionCanonicalWindingRow::new(
        canonical_row_identity(
            request_identity,
            PlanarBooleanOverlapRegionCanonicalWindingSourceKind::AdmittedRegion,
            row.admitted_region_identity(),
        ),
        PlanarBooleanOverlapRegionCanonicalWindingSourceKind::AdmittedRegion,
        row.admitted_region_identity().to_string(),
        row.island_identity().to_string(),
        row.neighborhood_identity().to_string(),
        Some(row.area_overlap_component_identity().to_string()),
        Some(row.canonical_operand_side()),
        Some(row.canonical_winding_sign()),
        canonicalized_strings(row.boundary_component_identities()),
        canonical_boundary_segment_identities,
        canonical_source_loop_identities,
        canonicalized_strings(row.chain_identities()),
        canonicalized_strings(row.fragment_identities()),
        canonicalized_strings(row.lineage_identities()),
        canonical_source_edge_identities,
        row.boundary_roles().to_vec(),
        canonicalized_strings(row.propagated_persistent_name_identities()),
    ))
}

fn canonicalize_boundary_only_row(
    request_identity: &str,
    row: &crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanBoundaryOnlyOverlapOutcomeRow,
    counters: &mut PlanarBooleanPostAdmissionNormalizationCounters,
) -> Result<
    PlanarBooleanOverlapRegionCanonicalWindingRow,
    PlanarBooleanPostAdmissionNormalizationDenial,
> {
    let canonical_boundary_segment_identities =
        ordered_witness_strings(row.canonical_boundary_segment_witness());
    let canonical_source_loop_identities =
        ordered_witness_strings(row.canonical_source_loop_witness());
    let (canonical_boundary_segment_identities, canonical_source_loop_identities) = match (
        canonical_boundary_segment_identities,
        canonical_source_loop_identities,
    ) {
        (Some(boundary_segments), Some(source_loops)) => (boundary_segments, source_loops),
        _ => {
            counters.denied_canonical_row();
            return Err(boundary_witness_mismatch_denial(
                PlanarBooleanOverlapRegionCanonicalWindingSourceKind::BoundaryOnlyOutcome,
                row.outcome_identity(),
                counters,
            ));
        }
    };

    if canonical_boundary_segment_identities.is_empty()
        || canonical_source_loop_identities.is_empty()
    {
        counters.denied_canonical_row();
        return Err(boundary_witness_mismatch_denial(
            PlanarBooleanOverlapRegionCanonicalWindingSourceKind::BoundaryOnlyOutcome,
            row.outcome_identity(),
            counters,
        ));
    }

    Ok(PlanarBooleanOverlapRegionCanonicalWindingRow::new(
        canonical_row_identity(
            request_identity,
            PlanarBooleanOverlapRegionCanonicalWindingSourceKind::BoundaryOnlyOutcome,
            row.outcome_identity(),
        ),
        PlanarBooleanOverlapRegionCanonicalWindingSourceKind::BoundaryOnlyOutcome,
        row.outcome_identity().to_string(),
        row.island_identity().to_string(),
        row.neighborhood_identity().to_string(),
        None,
        None,
        None,
        canonicalized_strings(row.boundary_component_identities()),
        canonical_boundary_segment_identities,
        canonical_source_loop_identities,
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
    ))
}
