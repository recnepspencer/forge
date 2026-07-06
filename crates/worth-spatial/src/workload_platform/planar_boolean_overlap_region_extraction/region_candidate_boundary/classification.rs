use std::collections::BTreeMap;

use super::counters::PlanarBooleanOverlapRegionCandidateBoundaryCounters;
use super::denial::PlanarBooleanDeniedOverlapRegionCandidateKind::{
    ContradictoryPromotionPostureDenied, MissingNormalizationDenied,
    MixedBoundaryAreaRequiresFurtherDecompositionDenied,
};
use super::denial::PlanarBooleanOverlapRegionCandidateBoundaryDenial;
use super::identity::{
    admitted_region_identity, boundary_only_outcome_identity, candidate_identity,
    denied_candidate_identity, set_identity,
};
use super::input::PlanarBooleanOverlapRegionCandidateBoundaryInput;
use super::product::{
    PlanarBooleanAdmittedOverlapRegionSet, PlanarBooleanBoundaryOnlyOverlapOutcomeSet,
    PlanarBooleanDeniedOverlapRegionCandidateSet,
    PlanarBooleanOverlapRegionCandidateBoundaryBundle, PlanarBooleanOverlapRegionCandidateSet,
};
use super::rows::{
    PlanarBooleanAdmittedOverlapRegionRow, PlanarBooleanBoundaryOnlyOverlapOutcomeRow,
    PlanarBooleanDeniedOverlapRegionCandidateRow, PlanarBooleanOverlapRegionCandidateRow,
};
use super::validation::{validate_input_identities, validate_normalization_coverage};

pub(super) fn promote_region_candidate_boundary_bundle(
    input: PlanarBooleanOverlapRegionCandidateBoundaryInput<'_>,
) -> Result<
    PlanarBooleanOverlapRegionCandidateBoundaryBundle,
    PlanarBooleanOverlapRegionCandidateBoundaryDenial,
> {
    validate_input_identities(input)?;
    validate_normalization_coverage(input)?;

    let shared_area_bundle = input.shared_area_admission();
    let normalization_set = input
        .pre_region_normalization()
        .opposite_sense_overlap_normalizations();
    let request_identity = shared_area_bundle.request_identity().to_string();
    let arrangement_graph_identity = shared_area_bundle.arrangement_graph_identity().to_string();
    let cell_set_identity = shared_area_bundle.cell_set_identity().to_string();
    let ordering_basis_identity = shared_area_bundle.ordering_basis_identity().to_string();
    let normalizations = normalization_set.rows().iter().fold(
        BTreeMap::<&str, Vec<_>>::new(),
        |mut grouped, row| {
            grouped
                .entry(row.shared_area_admission_outcome_identity())
                .or_default()
                .push(row);
            grouped
        },
    );
    let mut counters = PlanarBooleanOverlapRegionCandidateBoundaryCounters::default();
    let mut candidates = Vec::new();
    let mut denied = Vec::new();
    let mut admitted = Vec::new();
    let mut boundary_only = Vec::new();

    for row in shared_area_bundle.shared_area_admission_outcomes().rows() {
        counters.examined_shared_area_outcome();
        let normalization_rows = normalizations.get(row.outcome_identity());
        let contradictory_normalization = normalization_rows.is_some_and(|rows| rows.len() != 1);
        let localization_mismatch = normalization_rows
            .and_then(|rows| rows.first())
            .is_some_and(|normalization| {
                normalization.island_identity() != row.island_identity()
                    || normalization.neighborhood_identity() != row.neighborhood_identity()
                    || normalization.area_overlap_component_identity()
                        != row.area_overlap_component_identity()
            });
        if contradictory_normalization || localization_mismatch {
            counters.denied_candidate();
            denied.push(PlanarBooleanDeniedOverlapRegionCandidateRow::new(
                denied_candidate_identity(&request_identity, row.outcome_identity()),
                row.island_identity().to_string(),
                row.neighborhood_identity().to_string(),
                vec![row.area_overlap_component_identity().to_string()],
                row.boundary_component_identities().to_vec(),
                row.cell_identities().to_vec(),
                ContradictoryPromotionPostureDenied,
            ));
        } else if let Some(normalization) =
            normalization_rows.and_then(|rows| rows.first()).copied()
        {
            let candidate_identity = candidate_identity(&request_identity, row.outcome_identity());
            counters.promoted_candidate();
            candidates.push(PlanarBooleanOverlapRegionCandidateRow::new(
                candidate_identity.clone(),
                row.outcome_identity().to_string(),
                normalization.normalization_identity().to_string(),
                row.island_identity().to_string(),
                row.neighborhood_identity().to_string(),
                row.area_overlap_component_identity().to_string(),
                row.cell_identities().to_vec(),
                row.boundary_component_identities().to_vec(),
                row.boundary_segment_identities().to_vec(),
                row.source_loop_identities().to_vec(),
                normalization.canonical_operand_side(),
                normalization.canonical_winding_sign(),
                normalization.chain_identities().to_vec(),
                normalization.fragment_identities().to_vec(),
                normalization.lineage_identities().to_vec(),
                normalization.source_edge_identities().to_vec(),
                normalization.boundary_roles().to_vec(),
                normalization
                    .propagated_persistent_name_identities()
                    .to_vec(),
            ));
            counters.admitted_overlap_region();
            admitted.push(PlanarBooleanAdmittedOverlapRegionRow::new(
                admitted_region_identity(&request_identity, &candidate_identity),
                candidate_identity,
                row.outcome_identity().to_string(),
                normalization.normalization_identity().to_string(),
                row.island_identity().to_string(),
                row.neighborhood_identity().to_string(),
                row.area_overlap_component_identity().to_string(),
                row.cell_identities().to_vec(),
                row.boundary_component_identities().to_vec(),
                row.boundary_segment_identities().to_vec(),
                row.source_loop_identities().to_vec(),
                row.boundary_segment_identities().to_vec(),
                row.source_loop_identities().to_vec(),
                normalization.canonical_operand_side(),
                normalization.canonical_winding_sign(),
                normalization.chain_identities().to_vec(),
                normalization.fragment_identities().to_vec(),
                normalization.lineage_identities().to_vec(),
                normalization.source_edge_identities().to_vec(),
                normalization.boundary_roles().to_vec(),
                normalization
                    .propagated_persistent_name_identities()
                    .to_vec(),
            ));
        } else {
            counters.denied_candidate();
            denied.push(PlanarBooleanDeniedOverlapRegionCandidateRow::new(
                denied_candidate_identity(&request_identity, row.outcome_identity()),
                row.island_identity().to_string(),
                row.neighborhood_identity().to_string(),
                vec![row.area_overlap_component_identity().to_string()],
                row.boundary_component_identities().to_vec(),
                row.cell_identities().to_vec(),
                MissingNormalizationDenied,
            ));
        }
    }

    for row in shared_area_bundle.mixed_boundary_area_outcomes().rows() {
        counters.denied_candidate();
        denied.push(PlanarBooleanDeniedOverlapRegionCandidateRow::new(
            denied_candidate_identity(&request_identity, row.outcome_identity()),
            row.island_identity().to_string(),
            row.neighborhood_identity().to_string(),
            row.area_overlap_component_identities().to_vec(),
            row.boundary_contact_component_identities().to_vec(),
            row.cell_identities().to_vec(),
            MixedBoundaryAreaRequiresFurtherDecompositionDenied,
        ));
    }

    for row in shared_area_bundle.pure_boundary_only_outcomes().rows() {
        counters.boundary_only_outcome();
        boundary_only.push(PlanarBooleanBoundaryOnlyOverlapOutcomeRow::new(
            boundary_only_outcome_identity(&request_identity, row.outcome_identity()),
            row.outcome_identity().to_string(),
            row.island_identity().to_string(),
            row.neighborhood_identity().to_string(),
            row.boundary_contact_component_identities().to_vec(),
            row.cell_identities().to_vec(),
            row.boundary_component_identities().to_vec(),
            row.boundary_segment_identities().to_vec(),
            row.source_loop_identities().to_vec(),
            row.boundary_segment_identities().to_vec(),
            row.source_loop_identities().to_vec(),
        ));
    }

    Ok(PlanarBooleanOverlapRegionCandidateBoundaryBundle::new(
        format!(
            "overlap-region-candidate-boundary:{}:{}:{}",
            request_identity,
            candidates.len(),
            denied.len()
        ),
        PlanarBooleanOverlapRegionCandidateSet::new(
            set_identity(&request_identity, "candidate", candidates.len()),
            request_identity.clone(),
            arrangement_graph_identity.clone(),
            cell_set_identity.clone(),
            ordering_basis_identity.clone(),
            candidates,
        ),
        PlanarBooleanDeniedOverlapRegionCandidateSet::new(
            set_identity(&request_identity, "denied-candidate", denied.len()),
            request_identity.clone(),
            arrangement_graph_identity.clone(),
            cell_set_identity.clone(),
            ordering_basis_identity.clone(),
            denied,
        ),
        PlanarBooleanAdmittedOverlapRegionSet::new(
            set_identity(&request_identity, "admitted-region", admitted.len()),
            request_identity.clone(),
            arrangement_graph_identity.clone(),
            cell_set_identity.clone(),
            ordering_basis_identity.clone(),
            admitted,
        ),
        PlanarBooleanBoundaryOnlyOverlapOutcomeSet::new(
            set_identity(&request_identity, "boundary-only", boundary_only.len()),
            request_identity,
            arrangement_graph_identity,
            cell_set_identity,
            ordering_basis_identity,
            boundary_only,
        ),
        counters,
    ))
}
