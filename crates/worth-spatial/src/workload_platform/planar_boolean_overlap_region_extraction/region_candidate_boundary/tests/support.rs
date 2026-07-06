use crate::workload_platform::planar_boolean_overlap_region_extraction::arrangement_graph::cell_classification::tests::fixtures::{
    admitted_graph, inside_both_multi_boundary_graph, multi_cell_graph,
};
use crate::workload_platform::planar_boolean_overlap_region_extraction::{
    PlanarBooleanBoundaryOnlyOverlapOutcomeSet, PlanarBooleanBoundaryContactClassificationBundle,
    PlanarBooleanDeniedOverlapRegionCandidateSet, PlanarBooleanMixedBoundaryAreaOutcomeRow,
    PlanarBooleanMixedBoundaryAreaOutcomeSet, PlanarBooleanOppositeSenseOverlapNormalizationRow,
    PlanarBooleanOppositeSenseOverlapNormalizationSet, PlanarBooleanOverlapCellContainmentInput,
    PlanarBooleanOverlapCellContainmentMap, PlanarBooleanOverlapCellWindingField,
    PlanarBooleanOverlapCellWindingFieldInput, PlanarBooleanOverlapChainRegionLineageMap,
    PlanarBooleanOverlapChainRegionLineageRow, PlanarBooleanOverlapIslandCandidateInput,
    PlanarBooleanOverlapIslandComponentBundle, PlanarBooleanOverlapRegionCandidateBoundaryBundle,
    PlanarBooleanOverlapRegionCandidateBoundaryInput, PlanarBooleanPreRegionNormalizationBundle,
    PlanarBooleanSharedAreaAdmissionBundle,
};
use crate::workload_platform::planar_boolean_loop_reconstruction::test_support::LoopFixtureEntryOrder;

use crate::workload_platform::planar_boolean_common_plane::PlanarBooleanCommonPlaneOperandSide::Left;
use crate::workload_platform::planar_boolean_edge_splitting::PlanarBooleanOverlapChainBoundaryRole::FullOverlapSpan;

pub(super) fn region_candidate_graph()
-> crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanCoplanarOverlapArrangementGraph{
    inside_both_multi_boundary_graph()
}

pub(super) fn admitted_shared_area_bundle(
    arrangement: &crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanCoplanarOverlapArrangementGraph,
) -> PlanarBooleanSharedAreaAdmissionBundle {
    let containment = PlanarBooleanOverlapCellContainmentMap::admit(
        PlanarBooleanOverlapCellContainmentInput::from_arrangement(arrangement),
    )
    .expect("fixture arrangement should admit containment");
    let winding = PlanarBooleanOverlapCellWindingField::admit(
        PlanarBooleanOverlapCellWindingFieldInput::from_arrangement(arrangement, &containment),
    )
    .expect("fixture arrangement should admit winding");
    let island_bundle = PlanarBooleanOverlapIslandComponentBundle::admit(
        PlanarBooleanOverlapIslandCandidateInput::from_cell_classification(
            arrangement,
            &containment,
            &winding,
        ),
    )
    .expect("fixture arrangement should admit island component bundle");
    let boundary_bundle = island_bundle
        .classify_boundary_contact_components()
        .expect("fixture bundle should admit boundary classification");
    boundary_bundle
        .admit_shared_area_components(&containment, &winding)
        .expect("fixture bundle should admit shared area classification")
}

pub(super) fn admitted_pre_region_bundle(
    arrangement: &crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanCoplanarOverlapArrangementGraph,
) -> PlanarBooleanPreRegionNormalizationBundle {
    let shared_area_bundle = admitted_shared_area_bundle(arrangement);
    let chain_lineage_map = synthetic_chain_lineage_map(&shared_area_bundle);
    shared_area_bundle
        .normalize_pre_region_coincidence(&chain_lineage_map)
        .expect("fixture shared-area bundle should admit pre-region normalization")
}

pub(super) fn admitted_region_candidate_bundle(
    arrangement: &crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanCoplanarOverlapArrangementGraph,
) -> PlanarBooleanOverlapRegionCandidateBoundaryBundle {
    let shared_area_bundle = admitted_shared_area_bundle(arrangement);
    let pre_region_bundle = admitted_pre_region_bundle(arrangement);
    pre_region_bundle
        .promote_overlap_region_candidates(&shared_area_bundle)
        .expect("fixture bundle should promote overlap-region candidates")
}

pub(super) fn replayed_inputs() -> (
    PlanarBooleanSharedAreaAdmissionBundle,
    PlanarBooleanPreRegionNormalizationBundle,
    PlanarBooleanSharedAreaAdmissionBundle,
    PlanarBooleanPreRegionNormalizationBundle,
) {
    let canonical = admitted_graph(LoopFixtureEntryOrder::Canonical);
    let replayed = admitted_graph(LoopFixtureEntryOrder::Replayed);
    (
        admitted_shared_area_bundle(&canonical),
        admitted_pre_region_bundle(&canonical),
        admitted_shared_area_bundle(&replayed),
        admitted_pre_region_bundle(&replayed),
    )
}

pub(super) fn boundary_only_region_candidate_bundle(
) -> PlanarBooleanOverlapRegionCandidateBoundaryBundle {
    let shared_area_bundle = admitted_shared_area_bundle(&multi_cell_graph());
    let pre_region_bundle = missing_normalization_bundle(&shared_area_bundle);
    pre_region_bundle
        .promote_overlap_region_candidates(&shared_area_bundle)
        .expect("boundary-only fixture should still produce a phase-eleven bundle")
}

fn synthetic_chain_lineage_map(
    shared_area_bundle: &PlanarBooleanSharedAreaAdmissionBundle,
) -> PlanarBooleanOverlapChainRegionLineageMap {
    let rows = shared_area_bundle
        .shared_area_admission_outcomes()
        .rows()
        .iter()
        .enumerate()
        .map(|(index, row)| {
            PlanarBooleanOverlapChainRegionLineageRow::new(
                format!("synthetic-lineage-row:{index}"),
                format!("synthetic-lineage:{index}"),
                format!("synthetic-chain:{index}"),
                row.boundary_segment_identities()
                    .iter()
                    .map(|edge| format!("{edge}:fragment"))
                    .collect(),
                row.source_loop_identities().to_vec(),
                vec![Left; row.source_loop_identities().len().max(1)],
                vec![1; row.source_loop_identities().len().max(1)],
                row.boundary_segment_identities().to_vec(),
                vec![FullOverlapSpan; row.source_loop_identities().len().max(1)],
                row.source_loop_identities().to_vec(),
                vec![row.island_identity().to_string()],
                row.source_loop_identities()
                    .iter()
                    .map(|identity| format!("{identity}:name"))
                    .collect(),
            )
        })
        .collect();
    PlanarBooleanOverlapChainRegionLineageMap::new(
        "synthetic-pre-region-lineage-map".to_string(),
        shared_area_bundle.request_identity().to_string(),
        rows,
    )
}

pub(super) fn missing_normalization_bundle(
    shared_area_bundle: &PlanarBooleanSharedAreaAdmissionBundle,
) -> PlanarBooleanPreRegionNormalizationBundle {
    let set = PlanarBooleanOppositeSenseOverlapNormalizationSet::new(
        "synthetic-empty-normalization-set".to_string(),
        shared_area_bundle.request_identity().to_string(),
        shared_area_bundle.arrangement_graph_identity().to_string(),
        shared_area_bundle.cell_set_identity().to_string(),
        shared_area_bundle.ordering_basis_identity().to_string(),
        Vec::new(),
    );
    PlanarBooleanPreRegionNormalizationBundle::new(
        "synthetic-empty-normalization-bundle".to_string(),
        set,
        Default::default(),
    )
}

pub(super) fn orphan_normalization_bundle(
    shared_area_bundle: &PlanarBooleanSharedAreaAdmissionBundle,
) -> PlanarBooleanPreRegionNormalizationBundle {
    let row = &shared_area_bundle.shared_area_admission_outcomes().rows()[0];
    let synthetic_row = PlanarBooleanOppositeSenseOverlapNormalizationRow::new(
        "synthetic-orphan-normalization".to_string(),
        format!("{}:orphan", row.outcome_identity()),
        row.island_identity().to_string(),
        row.neighborhood_identity().to_string(),
        row.area_overlap_component_identity().to_string(),
        Left,
        1,
        vec!["synthetic-chain".to_string()],
        vec!["synthetic-fragment".to_string()],
        vec!["synthetic-lineage".to_string()],
        row.boundary_segment_identities().to_vec(),
        row.source_loop_identities().to_vec(),
        vec![FullOverlapSpan],
        vec!["synthetic-name".to_string()],
    );
    let set = PlanarBooleanOppositeSenseOverlapNormalizationSet::new(
        "synthetic-orphan-normalization-set".to_string(),
        shared_area_bundle.request_identity().to_string(),
        shared_area_bundle.arrangement_graph_identity().to_string(),
        shared_area_bundle.cell_set_identity().to_string(),
        shared_area_bundle.ordering_basis_identity().to_string(),
        vec![synthetic_row],
    );
    PlanarBooleanPreRegionNormalizationBundle::new(
        "synthetic-orphan-normalization-bundle".to_string(),
        set,
        Default::default(),
    )
}

pub(super) fn duplicate_normalization_bundle(
    shared_area_bundle: &PlanarBooleanSharedAreaAdmissionBundle,
) -> PlanarBooleanPreRegionNormalizationBundle {
    let row = &shared_area_bundle.shared_area_admission_outcomes().rows()[0];
    let duplicate_rows = vec![
        PlanarBooleanOppositeSenseOverlapNormalizationRow::new(
            "synthetic-duplicate-normalization-a".to_string(),
            row.outcome_identity().to_string(),
            row.island_identity().to_string(),
            row.neighborhood_identity().to_string(),
            row.area_overlap_component_identity().to_string(),
            Left,
            1,
            vec!["synthetic-chain-a".to_string()],
            vec!["synthetic-fragment-a".to_string()],
            vec!["synthetic-lineage-a".to_string()],
            row.boundary_segment_identities().to_vec(),
            row.source_loop_identities().to_vec(),
            vec![FullOverlapSpan],
            vec!["synthetic-name-a".to_string()],
        ),
        PlanarBooleanOppositeSenseOverlapNormalizationRow::new(
            "synthetic-duplicate-normalization-b".to_string(),
            row.outcome_identity().to_string(),
            row.island_identity().to_string(),
            row.neighborhood_identity().to_string(),
            row.area_overlap_component_identity().to_string(),
            Left,
            -1,
            vec!["synthetic-chain-b".to_string()],
            vec!["synthetic-fragment-b".to_string()],
            vec!["synthetic-lineage-b".to_string()],
            row.boundary_segment_identities().to_vec(),
            row.source_loop_identities().to_vec(),
            vec![FullOverlapSpan],
            vec!["synthetic-name-b".to_string()],
        ),
    ];
    let set = PlanarBooleanOppositeSenseOverlapNormalizationSet::new(
        "synthetic-duplicate-normalization-set".to_string(),
        shared_area_bundle.request_identity().to_string(),
        shared_area_bundle.arrangement_graph_identity().to_string(),
        shared_area_bundle.cell_set_identity().to_string(),
        shared_area_bundle.ordering_basis_identity().to_string(),
        duplicate_rows,
    );
    PlanarBooleanPreRegionNormalizationBundle::new(
        "synthetic-duplicate-normalization-bundle".to_string(),
        set,
        Default::default(),
    )
}

pub(super) fn mixed_boundary_shared_area_bundle(
    arrangement: &crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanCoplanarOverlapArrangementGraph,
) -> PlanarBooleanSharedAreaAdmissionBundle {
    let bundle = admitted_shared_area_bundle(arrangement);
    let shared_area_row = &bundle.shared_area_admission_outcomes().rows()[0];
    let mixed_row = PlanarBooleanMixedBoundaryAreaOutcomeRow::new(
        "synthetic-mixed-boundary-area-outcome".to_string(),
        shared_area_row.island_identity().to_string(),
        shared_area_row.neighborhood_identity().to_string(),
        vec![shared_area_row
            .area_overlap_component_identity()
            .to_string()],
        vec!["synthetic-boundary-contact-component".to_string()],
        shared_area_row.cell_identities().to_vec(),
    );
    let mixed_set = PlanarBooleanMixedBoundaryAreaOutcomeSet::new(
        "synthetic-mixed-boundary-area-set".to_string(),
        bundle.request_identity().to_string(),
        bundle.arrangement_graph_identity().to_string(),
        bundle.cell_set_identity().to_string(),
        bundle.ordering_basis_identity().to_string(),
        vec![mixed_row],
    );
    PlanarBooleanSharedAreaAdmissionBundle::new(
        "synthetic-shared-area-with-mixed".to_string(),
        bundle.shared_area_admission_outcomes().clone(),
        mixed_set,
        bundle.pure_boundary_only_outcomes().clone(),
        bundle.counters(),
    )
}

pub(super) fn mixed_boundary_disjoint_shared_area_bundle(
    arrangement: &crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanCoplanarOverlapArrangementGraph,
) -> PlanarBooleanSharedAreaAdmissionBundle {
    let bundle = admitted_shared_area_bundle(arrangement);
    let shared_area_row = &bundle.shared_area_admission_outcomes().rows()[0];
    let mixed_row = PlanarBooleanMixedBoundaryAreaOutcomeRow::new(
        "synthetic-disjoint-mixed-boundary-area-outcome".to_string(),
        shared_area_row.island_identity().to_string(),
        shared_area_row.neighborhood_identity().to_string(),
        vec!["synthetic-foreign-area-component".to_string()],
        vec!["synthetic-boundary-contact-component".to_string()],
        vec!["synthetic-foreign-cell".to_string()],
    );
    let mixed_set = PlanarBooleanMixedBoundaryAreaOutcomeSet::new(
        "synthetic-disjoint-mixed-boundary-area-set".to_string(),
        bundle.request_identity().to_string(),
        bundle.arrangement_graph_identity().to_string(),
        bundle.cell_set_identity().to_string(),
        bundle.ordering_basis_identity().to_string(),
        vec![mixed_row],
    );
    PlanarBooleanSharedAreaAdmissionBundle::new(
        "synthetic-shared-area-with-disjoint-mixed".to_string(),
        bundle.shared_area_admission_outcomes().clone(),
        mixed_set,
        bundle.pure_boundary_only_outcomes().clone(),
        bundle.counters(),
    )
}

pub(super) fn boundary_only_outcome_set(
    bundle: &PlanarBooleanOverlapRegionCandidateBoundaryBundle,
) -> &PlanarBooleanBoundaryOnlyOverlapOutcomeSet {
    bundle.boundary_only_overlap_outcomes()
}

pub(super) fn denied_candidate_set(
    bundle: &PlanarBooleanOverlapRegionCandidateBoundaryBundle,
) -> &PlanarBooleanDeniedOverlapRegionCandidateSet {
    bundle.denied_overlap_region_candidates()
}
