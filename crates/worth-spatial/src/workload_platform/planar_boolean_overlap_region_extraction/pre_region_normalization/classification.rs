use std::collections::{BTreeMap, BTreeSet};

use crate::workload_platform::planar_boolean_common_plane::PlanarBooleanCommonPlaneOperandSide;
use crate::workload_platform::planar_boolean_edge_splitting::PlanarBooleanOverlapChainBoundaryRole;

use super::counters::PlanarBooleanPreRegionNormalizationCounters;
use super::denial::PlanarBooleanPreRegionNormalizationDenial;
use super::identity::{normalization_outcome_identity, normalization_set_identity};
use super::input::PlanarBooleanPreRegionNormalizationInput;
use super::product::{
    PlanarBooleanOppositeSenseOverlapNormalizationSet, PlanarBooleanPreRegionNormalizationBundle,
};
use super::rows::PlanarBooleanOppositeSenseOverlapNormalizationRow;
use super::validation::{
    ambiguous_ordering, relevant_lineage_rows, unstable_tie_breaker, validate_input_identities,
};

pub(super) fn build_pre_region_normalization_bundle(
    input: PlanarBooleanPreRegionNormalizationInput<'_>,
) -> Result<PlanarBooleanPreRegionNormalizationBundle, PlanarBooleanPreRegionNormalizationDenial> {
    let mut counters = PlanarBooleanPreRegionNormalizationCounters::default();
    validate_input_identities(input, &mut counters)?;

    let shared_area = input
        .shared_area_admission()
        .shared_area_admission_outcomes();
    let request_identity = input.shared_area_admission().request_identity().to_string();
    let mut rows = Vec::new();

    for shared_area_row in shared_area.rows() {
        let relevant =
            relevant_lineage_rows(shared_area_row, input.chain_lineage_map(), &mut counters)?;
        rows.push(normalize_row(
            &request_identity,
            shared_area_row,
            &relevant,
            &mut counters,
        )?);
        counters.admitted_normalization();
    }

    let set = PlanarBooleanOppositeSenseOverlapNormalizationSet::new(
        normalization_set_identity(&request_identity, rows.len()),
        request_identity.clone(),
        input
            .shared_area_admission()
            .arrangement_graph_identity()
            .to_string(),
        input
            .shared_area_admission()
            .cell_set_identity()
            .to_string(),
        input
            .shared_area_admission()
            .ordering_basis_identity()
            .to_string(),
        rows,
    );

    Ok(PlanarBooleanPreRegionNormalizationBundle::new(
        format!(
            "pre-region-normalization:{}:{}",
            request_identity,
            set.rows().len()
        ),
        set,
        counters,
    ))
}

fn normalize_row(
    request_identity: &str,
    shared_area_row: &crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanSharedAreaAdmissionOutcomeRow,
    relevant: &[&crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanOverlapChainRegionLineageRow],
    counters: &mut PlanarBooleanPreRegionNormalizationCounters,
) -> Result<
    PlanarBooleanOppositeSenseOverlapNormalizationRow,
    PlanarBooleanPreRegionNormalizationDenial,
> {
    let mut saw_left = false;
    let mut saw_right = false;
    let mut saw_positive = false;
    let mut saw_negative = false;
    let mut saw_full_overlap = false;
    let mut saw_start = false;
    let mut saw_interior = false;
    let mut saw_end = false;
    let mut chain_ids = BTreeSet::new();
    let mut fragment_ids = BTreeSet::new();
    let mut lineage_ids = BTreeSet::new();
    let mut source_edges = BTreeSet::new();
    let mut source_loops = BTreeSet::new();
    let mut orientation_by_source_loop =
        BTreeMap::<String, Vec<(PlanarBooleanCommonPlaneOperandSide, i8)>>::new();
    let mut persistent_names = BTreeSet::new();
    let shared_area_source_loops = shared_area_row
        .source_loop_identities()
        .iter()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    let source_only_full_span_normalization = relevant.iter().all(|lineage| {
        let matching_roles = matching_boundary_roles(lineage, &shared_area_source_loops);
        lineage.participating_island_identities().is_empty()
            && !matching_roles.is_empty()
            && matching_roles
                .iter()
                .any(|role| *role == PlanarBooleanOverlapChainBoundaryRole::FullOverlapSpan)
    });

    for lineage in relevant {
        counters.examined_lineage_row();
        chain_ids.insert(lineage.chain_identity().to_string());
        fragment_ids.extend(lineage.fragment_identities().iter().cloned());
        lineage_ids.insert(lineage.lineage_identity().to_string());
        source_edges.extend(lineage.source_edge_identities().iter().cloned());
        source_loops.extend(lineage.source_loop_identities().iter().cloned());
        persistent_names.extend(
            lineage
                .propagated_persistent_name_identities()
                .iter()
                .cloned(),
        );
        for ((source_loop_identity, side), sign) in lineage
            .source_loop_identities()
            .iter()
            .zip(lineage.source_loop_operand_sides())
            .zip(lineage.source_loop_winding_signs())
        {
            let orientations = orientation_by_source_loop
                .entry(source_loop_identity.clone())
                .or_default();
            if !orientations.contains(&(*side, *sign)) {
                orientations.push((*side, *sign));
            }
        }
        for side in lineage.source_loop_operand_sides() {
            match side {
                PlanarBooleanCommonPlaneOperandSide::Left => saw_left = true,
                PlanarBooleanCommonPlaneOperandSide::Right => saw_right = true,
            }
        }
        for sign in lineage.source_loop_winding_signs() {
            if *sign > 0 {
                saw_positive = true;
            } else if *sign < 0 {
                saw_negative = true;
            }
        }
        for role in matching_boundary_roles(lineage, &shared_area_source_loops) {
            match role {
                PlanarBooleanOverlapChainBoundaryRole::FullOverlapSpan => saw_full_overlap = true,
                PlanarBooleanOverlapChainBoundaryRole::OverlapStartBoundary => saw_start = true,
                PlanarBooleanOverlapChainBoundaryRole::OverlapInteriorFragment => {
                    saw_interior = true
                }
                PlanarBooleanOverlapChainBoundaryRole::OverlapEndBoundary => saw_end = true,
            }
        }
    }

    if saw_start && saw_end && !source_only_full_span_normalization {
        return Err(ambiguous_ordering(shared_area_row, counters));
    }
    let Some((canonical_operand_side, canonical_winding_sign)) = stable_canonical_orientation(
        &orientation_by_source_loop,
        source_only_full_span_normalization,
    ) else {
        trace_unstable_orientation(shared_area_row, &orientation_by_source_loop);
        return Err(unstable_tie_breaker(shared_area_row, counters));
    };
    if (!saw_left && !saw_right) || (!saw_positive && !saw_negative) {
        return Err(unstable_tie_breaker(shared_area_row, counters));
    }

    Ok(PlanarBooleanOppositeSenseOverlapNormalizationRow::new(
        normalization_outcome_identity(request_identity, shared_area_row.outcome_identity()),
        shared_area_row.outcome_identity().to_string(),
        shared_area_row.island_identity().to_string(),
        shared_area_row.neighborhood_identity().to_string(),
        shared_area_row
            .area_overlap_component_identity()
            .to_string(),
        canonical_operand_side,
        canonical_winding_sign,
        chain_ids.into_iter().collect(),
        fragment_ids.into_iter().collect(),
        lineage_ids.into_iter().collect(),
        source_edges.into_iter().collect(),
        source_loops.into_iter().collect(),
        collect_boundary_roles(
            source_only_full_span_normalization,
            saw_full_overlap,
            saw_start,
            saw_interior,
            saw_end,
        ),
        persistent_names.into_iter().collect(),
    ))
}

fn matching_boundary_roles(
    lineage: &crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanOverlapChainRegionLineageRow,
    shared_area_source_loops: &BTreeSet<&str>,
) -> Vec<PlanarBooleanOverlapChainBoundaryRole> {
    lineage
        .source_loop_identities()
        .iter()
        .zip(lineage.boundary_roles())
        .filter_map(|(source_loop_identity, role)| {
            shared_area_source_loops
                .contains(source_loop_identity.as_str())
                .then_some(*role)
        })
        .collect()
}

fn trace_unstable_orientation(
    shared_area_row: &crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanSharedAreaAdmissionOutcomeRow,
    orientation_by_source_loop: &BTreeMap<String, Vec<(PlanarBooleanCommonPlaneOperandSide, i8)>>,
) {
    if std::env::var_os("WORTH_TRACE_PRE_REGION_ORIENTATION").is_none() {
        return;
    }
    let summary = orientation_by_source_loop
        .iter()
        .map(|(source_loop, orientations)| {
            format!(
                "{}:{:?}",
                &source_loop[..source_loop.len().min(16)],
                orientations
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    eprintln!(
        "pre-region unstable orientation: source_loop_count={} loops={}",
        shared_area_row.source_loop_identities().len(),
        summary
    );
}

fn stable_canonical_orientation(
    orientation_by_source_loop: &BTreeMap<String, Vec<(PlanarBooleanCommonPlaneOperandSide, i8)>>,
    source_only_full_span_normalization: bool,
) -> Option<(PlanarBooleanCommonPlaneOperandSide, i8)> {
    let mut canonical = Vec::new();
    for (source_loop_identity, orientations) in orientation_by_source_loop {
        if orientations.len() != 1 && !source_only_full_span_normalization {
            return None;
        }
        let (side, sign) = canonical_orientation_for_loop(orientations)?;
        if sign == 0 {
            return None;
        }
        canonical.push((source_loop_identity.as_str(), side, sign));
    }
    canonical.sort_by(|left, right| left.0.cmp(right.0));
    canonical.first().map(|(_, side, sign)| (*side, *sign))
}

fn canonical_orientation_for_loop(
    orientations: &[(PlanarBooleanCommonPlaneOperandSide, i8)],
) -> Option<(PlanarBooleanCommonPlaneOperandSide, i8)> {
    let mut nonzero = orientations
        .iter()
        .copied()
        .filter(|(_, sign)| *sign != 0)
        .collect::<Vec<_>>();
    nonzero.sort_by(|left, right| orientation_sort_key(*left).cmp(&orientation_sort_key(*right)));
    nonzero.first().copied()
}

fn orientation_sort_key(orientation: (PlanarBooleanCommonPlaneOperandSide, i8)) -> String {
    let (side, sign) = orientation;
    format!("{}:{sign}", side.query_key())
}

fn collect_boundary_roles(
    source_only_full_span_normalization: bool,
    saw_full_overlap: bool,
    saw_start: bool,
    saw_interior: bool,
    saw_end: bool,
) -> Vec<PlanarBooleanOverlapChainBoundaryRole> {
    let mut roles = Vec::new();
    if saw_full_overlap {
        roles.push(PlanarBooleanOverlapChainBoundaryRole::FullOverlapSpan);
        if source_only_full_span_normalization {
            return roles;
        }
    }
    if saw_start {
        roles.push(PlanarBooleanOverlapChainBoundaryRole::OverlapStartBoundary);
    }
    if saw_interior {
        roles.push(PlanarBooleanOverlapChainBoundaryRole::OverlapInteriorFragment);
    }
    if saw_end {
        roles.push(PlanarBooleanOverlapChainBoundaryRole::OverlapEndBoundary);
    }
    roles
}
