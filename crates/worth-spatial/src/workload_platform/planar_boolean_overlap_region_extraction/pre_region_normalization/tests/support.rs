use crate::workload_platform::planar_boolean_common_plane::PlanarBooleanCommonPlaneOperandSide::{
    Left, Right,
};
use crate::workload_platform::planar_boolean_edge_splitting::PlanarBooleanOverlapChainBoundaryRole::{
    FullOverlapSpan, OverlapEndBoundary, OverlapStartBoundary,
};
use crate::workload_platform::planar_boolean_loop_reconstruction::test_support::LoopFixtureEntryOrder;
use crate::workload_platform::planar_boolean_overlap_region_extraction::arrangement_graph::cell_classification::tests::fixtures::{
    admitted_graph, inside_both_multi_boundary_graph,
};
use crate::workload_platform::planar_boolean_overlap_region_extraction::{
    PlanarBooleanOverlapCellContainmentInput, PlanarBooleanOverlapCellContainmentMap,
    PlanarBooleanOverlapCellWindingField, PlanarBooleanOverlapCellWindingFieldInput,
    PlanarBooleanOverlapChainRegionLineageMap, PlanarBooleanOverlapChainRegionLineageRow,
    PlanarBooleanOverlapIslandCandidateInput, PlanarBooleanOverlapIslandComponentBundle,
    PlanarBooleanPreRegionNormalizationBundle, PlanarBooleanSharedAreaAdmissionBundle,
};

pub(super) fn admitted_pre_region_bundle(
    arrangement: &crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanCoplanarOverlapArrangementGraph,
) -> PlanarBooleanPreRegionNormalizationBundle {
    let shared_area_bundle = admitted_shared_area_bundle(arrangement);
    let chain_lineage_map = synthetic_chain_lineage_map(&shared_area_bundle, false, false, false);
    shared_area_bundle
        .normalize_pre_region_coincidence(&chain_lineage_map)
        .expect("fixture shared-area bundle should admit pre-region normalization")
}

pub(super) fn replayed_shared_area_bundles() -> (
    PlanarBooleanSharedAreaAdmissionBundle,
    PlanarBooleanSharedAreaAdmissionBundle,
) {
    let canonical = admitted_graph(LoopFixtureEntryOrder::Canonical);
    let replayed = admitted_graph(LoopFixtureEntryOrder::Replayed);
    (
        admitted_shared_area_bundle(&canonical),
        admitted_shared_area_bundle(&replayed),
    )
}

pub(super) fn shared_area_graph()
-> crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanCoplanarOverlapArrangementGraph{
    inside_both_multi_boundary_graph()
}

pub(super) fn synthetic_chain_lineage_map(
    shared_area_bundle: &PlanarBooleanSharedAreaAdmissionBundle,
    reverse_source_edge_sense: bool,
    ambiguous_ordering: bool,
    unstable_orientation: bool,
) -> PlanarBooleanOverlapChainRegionLineageMap {
    let rows = shared_area_bundle
        .shared_area_admission_outcomes()
        .rows()
        .iter()
        .enumerate()
        .map(|(index, row)| {
            let mut source_edges = row.boundary_segment_identities().to_vec();
            if reverse_source_edge_sense {
                source_edges.reverse();
            }
            let source_loops = if unstable_orientation {
                vec![
                    row.source_loop_identities()[0].clone(),
                    row.source_loop_identities()[0].clone(),
                ]
            } else {
                row.source_loop_identities().to_vec()
            };
            let operand_sides = if unstable_orientation {
                vec![Left, Right]
            } else {
                vec![Left; source_loops.len().max(1)]
            };
            let winding_signs = if unstable_orientation {
                vec![1, -1]
            } else {
                vec![1; operand_sides.len()]
            };
            let boundary_roles = if ambiguous_ordering {
                vec![OverlapStartBoundary, OverlapEndBoundary]
            } else {
                vec![FullOverlapSpan; operand_sides.len()]
            };
            PlanarBooleanOverlapChainRegionLineageRow::new(
                format!("synthetic-lineage-row:{index}"),
                format!("synthetic-lineage:{index}"),
                format!("synthetic-chain:{index}"),
                source_edges
                    .iter()
                    .map(|edge| format!("{edge}:fragment"))
                    .collect(),
                source_loops,
                operand_sides,
                winding_signs,
                source_edges,
                boundary_roles,
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

pub(super) fn synthetic_missing_lineage_map(
    shared_area_bundle: &PlanarBooleanSharedAreaAdmissionBundle,
) -> PlanarBooleanOverlapChainRegionLineageMap {
    let row = &shared_area_bundle.shared_area_admission_outcomes().rows()[0];
    PlanarBooleanOverlapChainRegionLineageMap::new(
        "synthetic-missing-lineage-map".to_string(),
        shared_area_bundle.request_identity().to_string(),
        vec![PlanarBooleanOverlapChainRegionLineageRow::new(
            "synthetic-missing-lineage-row".to_string(),
            "synthetic-missing-lineage".to_string(),
            "synthetic-missing-chain".to_string(),
            vec!["foreign-fragment".to_string()],
            vec!["foreign-loop".to_string()],
            vec![Left],
            vec![1],
            vec!["foreign-edge".to_string()],
            vec![OverlapStartBoundary],
            vec!["foreign-loop".to_string()],
            vec![format!("{}:foreign", row.island_identity())],
            vec!["foreign-name".to_string()],
        )],
    )
}

pub(super) fn operand_permuted_chain_lineage_map(
    shared_area_bundle: &PlanarBooleanSharedAreaAdmissionBundle,
) -> PlanarBooleanOverlapChainRegionLineageMap {
    let row = &shared_area_bundle.shared_area_admission_outcomes().rows()[0];
    let mut source_loops = row.source_loop_identities().to_vec();
    source_loops.reverse();
    PlanarBooleanOverlapChainRegionLineageMap::new(
        "synthetic-pre-region-lineage-map".to_string(),
        shared_area_bundle.request_identity().to_string(),
        vec![PlanarBooleanOverlapChainRegionLineageRow::new(
            "synthetic-lineage-row:0".to_string(),
            "synthetic-lineage:0".to_string(),
            "synthetic-chain:0".to_string(),
            row.boundary_segment_identities()
                .iter()
                .map(|edge| format!("{edge}:fragment"))
                .collect(),
            source_loops,
            vec![Left; row.source_loop_identities().len()],
            vec![1; row.source_loop_identities().len()],
            row.boundary_segment_identities().to_vec(),
            vec![FullOverlapSpan; row.source_loop_identities().len()],
            row.source_loop_identities().to_vec(),
            vec![row.island_identity().to_string()],
            row.source_loop_identities()
                .iter()
                .map(|identity| format!("{identity}:name"))
                .collect(),
        )],
    )
}

pub(super) fn chain_lineage_map_with_unrelated_conflict(
    shared_area_bundle: &PlanarBooleanSharedAreaAdmissionBundle,
) -> PlanarBooleanOverlapChainRegionLineageMap {
    let row = &shared_area_bundle.shared_area_admission_outcomes().rows()[0];
    let mut rows = synthetic_chain_lineage_map(shared_area_bundle, false, false, false)
        .rows()
        .to_vec();
    rows.push(PlanarBooleanOverlapChainRegionLineageRow::new(
        "synthetic-unrelated-conflict-row".to_string(),
        "synthetic-unrelated-conflict-lineage".to_string(),
        "synthetic-unrelated-conflict-chain".to_string(),
        vec!["foreign-fragment".to_string()],
        row.source_loop_identities().to_vec(),
        vec![Right],
        vec![-1],
        vec!["foreign-edge".to_string()],
        vec![FullOverlapSpan],
        row.source_loop_identities().to_vec(),
        vec![row.island_identity().to_string()],
        vec!["foreign-name".to_string()],
    ));
    PlanarBooleanOverlapChainRegionLineageMap::new(
        "synthetic-pre-region-lineage-map-with-unrelated-conflict".to_string(),
        shared_area_bundle.request_identity().to_string(),
        rows,
    )
}

pub(super) fn contradictory_localized_chain_lineage_map(
    shared_area_bundle: &PlanarBooleanSharedAreaAdmissionBundle,
) -> PlanarBooleanOverlapChainRegionLineageMap {
    let row = &shared_area_bundle.shared_area_admission_outcomes().rows()[0];
    let source_edges = row.boundary_segment_identities().to_vec();
    let fragments = source_edges
        .iter()
        .map(|edge| format!("{edge}:fragment"))
        .collect::<Vec<_>>();
    let source_loops = row.source_loop_identities().to_vec();
    PlanarBooleanOverlapChainRegionLineageMap::new(
        "synthetic-contradictory-localized-lineage-map".to_string(),
        shared_area_bundle.request_identity().to_string(),
        vec![
            PlanarBooleanOverlapChainRegionLineageRow::new(
                "synthetic-contradictory-localized-row:left".to_string(),
                "synthetic-contradictory-localized-lineage:left".to_string(),
                "synthetic-contradictory-localized-chain:left".to_string(),
                fragments.clone(),
                source_loops.clone(),
                vec![Left; source_loops.len()],
                vec![1; source_loops.len()],
                source_edges.clone(),
                vec![FullOverlapSpan; source_loops.len()],
                row.source_loop_identities().to_vec(),
                vec![row.island_identity().to_string()],
                row.source_loop_identities()
                    .iter()
                    .map(|identity| format!("{identity}:left"))
                    .collect(),
            ),
            PlanarBooleanOverlapChainRegionLineageRow::new(
                "synthetic-contradictory-localized-row:right".to_string(),
                "synthetic-contradictory-localized-lineage:right".to_string(),
                "synthetic-contradictory-localized-chain:right".to_string(),
                fragments,
                source_loops.clone(),
                vec![Right; source_loops.len()],
                vec![-1; source_loops.len()],
                source_edges,
                vec![FullOverlapSpan; source_loops.len()],
                row.source_loop_identities().to_vec(),
                vec![row.island_identity().to_string()],
                row.source_loop_identities()
                    .iter()
                    .map(|identity| format!("{identity}:right"))
                    .collect(),
            ),
        ],
    )
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
