use std::collections::BTreeSet;

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
    let mut persistent_names = BTreeSet::new();

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
        for role in lineage.boundary_roles() {
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

    if saw_start && saw_end {
        return Err(ambiguous_ordering(shared_area_row, counters));
    }
    if (saw_left && saw_right)
        || (saw_positive && saw_negative)
        || (!saw_left && !saw_right)
        || (!saw_positive && !saw_negative)
    {
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
        if saw_left {
            PlanarBooleanCommonPlaneOperandSide::Left
        } else {
            PlanarBooleanCommonPlaneOperandSide::Right
        },
        if saw_positive { 1 } else { -1 },
        chain_ids.into_iter().collect(),
        fragment_ids.into_iter().collect(),
        lineage_ids.into_iter().collect(),
        source_edges.into_iter().collect(),
        source_loops.into_iter().collect(),
        collect_boundary_roles(saw_full_overlap, saw_start, saw_interior, saw_end),
        persistent_names.into_iter().collect(),
    ))
}

fn collect_boundary_roles(
    saw_full_overlap: bool,
    saw_start: bool,
    saw_interior: bool,
    saw_end: bool,
) -> Vec<PlanarBooleanOverlapChainBoundaryRole> {
    let mut roles = Vec::new();
    if saw_full_overlap {
        roles.push(PlanarBooleanOverlapChainBoundaryRole::FullOverlapSpan);
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
