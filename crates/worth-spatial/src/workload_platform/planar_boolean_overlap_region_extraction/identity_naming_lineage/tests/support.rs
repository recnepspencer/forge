use crate::workload_platform::planar_boolean_common_plane::PlanarBooleanCommonPlaneOperandSide::Left;
use crate::workload_platform::planar_boolean_edge_splitting::PlanarBooleanOverlapChainBoundaryRole::FullOverlapSpan;
use crate::workload_platform::planar_boolean_loop_reconstruction::test_support::LoopFixtureEntryOrder;
use crate::workload_platform::planar_boolean_overlap_region_extraction::arrangement_graph::cell_classification::tests::fixtures::{
    admitted_graph, inside_both_multi_boundary_graph, multi_cell_graph,
};
use crate::workload_platform::planar_boolean_overlap_region_extraction::{
    PlanarBooleanBoundaryContactClassificationBundle, PlanarBooleanBoundaryOnlyOverlapOutcomeSet,
    PlanarBooleanBoundaryOnlyOverlapOutcomeRow, PlanarBooleanOverlapCellContainmentInput,
    PlanarBooleanOverlapCellContainmentMap, PlanarBooleanOverlapCellWindingField,
    PlanarBooleanOverlapCellWindingFieldInput, PlanarBooleanOverlapChainRegionLineageMap,
    PlanarBooleanOverlapChainRegionLineageRow, PlanarBooleanOverlapIslandCandidateInput,
    PlanarBooleanOverlapIslandComponentBundle, PlanarBooleanOverlapRegionCandidateBoundaryBundle,
    PlanarBooleanOverlapRegionIdentityLineageBundle, PlanarBooleanOverlapRegionIdentityLineageInput,
    PlanarBooleanOppositeSenseOverlapNormalizationSet, PlanarBooleanPostAdmissionNormalizationBundle,
    PlanarBooleanPreRegionNormalizationBundle, PlanarBooleanSharedAreaAdmissionBundle,
};

pub(super) fn canonical_graph()
-> crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanCoplanarOverlapArrangementGraph{
    inside_both_multi_boundary_graph()
}

fn admitted_shared_area_bundle(
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
    let boundary_bundle: PlanarBooleanBoundaryContactClassificationBundle = island_bundle
        .classify_boundary_contact_components()
        .expect("fixture bundle should admit boundary classification");
    boundary_bundle
        .admit_shared_area_components(&containment, &winding)
        .expect("fixture bundle should admit shared area classification")
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
        "synthetic-phase-thirteen-lineage-map".to_string(),
        shared_area_bundle.request_identity().to_string(),
        rows,
    )
}

fn admitted_region_candidate_bundle(
    arrangement: &crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanCoplanarOverlapArrangementGraph,
) -> PlanarBooleanOverlapRegionCandidateBoundaryBundle {
    let shared_area_bundle = admitted_shared_area_bundle(arrangement);
    let pre_region_bundle = PlanarBooleanPreRegionNormalizationBundle::from_shared_area_admission(
        &shared_area_bundle,
        &synthetic_chain_lineage_map(&shared_area_bundle),
    )
    .expect("fixture shared-area bundle should admit pre-region normalization");
    pre_region_bundle
        .promote_overlap_region_candidates(&shared_area_bundle)
        .expect("fixture bundle should promote overlap-region candidates")
}

pub(super) fn canonical_bundle(
    arrangement: &crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanCoplanarOverlapArrangementGraph,
) -> PlanarBooleanPostAdmissionNormalizationBundle {
    admitted_region_candidate_bundle(arrangement)
        .normalize_post_admission_canonical_winding()
        .expect("fixture canonical bundle should admit post-admission normalization")
}

pub(super) fn boundary_only_bundle() -> PlanarBooleanPostAdmissionNormalizationBundle {
    let shared_area_bundle = admitted_shared_area_bundle(&multi_cell_graph());
    let empty_set = PlanarBooleanOppositeSenseOverlapNormalizationSet::new(
        "synthetic-empty-normalization-set".to_string(),
        shared_area_bundle.request_identity().to_string(),
        shared_area_bundle.arrangement_graph_identity().to_string(),
        shared_area_bundle.cell_set_identity().to_string(),
        shared_area_bundle.ordering_basis_identity().to_string(),
        Vec::new(),
    );
    let pre_region_bundle = PlanarBooleanPreRegionNormalizationBundle::new(
        "synthetic-empty-normalization-bundle".to_string(),
        empty_set,
        Default::default(),
    );
    pre_region_bundle
        .promote_overlap_region_candidates(&shared_area_bundle)
        .expect("boundary-only fixture should still produce candidate bundle")
        .normalize_post_admission_canonical_winding()
        .expect("boundary-only fixture should still canonicalize")
}

pub(super) fn replayed_inputs() -> (
    PlanarBooleanPostAdmissionNormalizationBundle,
    PlanarBooleanPostAdmissionNormalizationBundle,
) {
    let canonical = admitted_graph(LoopFixtureEntryOrder::Canonical);
    let replayed = admitted_graph(LoopFixtureEntryOrder::Replayed);
    (canonical_bundle(&canonical), canonical_bundle(&replayed))
}

pub(super) fn identity_bundle(
    arrangement: &crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanCoplanarOverlapArrangementGraph,
) -> PlanarBooleanOverlapRegionIdentityLineageBundle {
    canonical_bundle(arrangement)
        .mint_overlap_region_identity_lineage()
        .expect("fixture canonical bundle should admit phase-thirteen minting")
}

pub(super) fn canonical_identity_map(
    bundle: &PlanarBooleanOverlapRegionIdentityLineageBundle,
) -> &crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanOverlapRegionIdentityMap{
    bundle.overlap_region_identity_map()
}
