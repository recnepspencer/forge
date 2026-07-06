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
    PlanarBooleanOverlapIslandComponentBundle, PlanarBooleanOverlapRegionCanonicalWindingRow,
    PlanarBooleanOverlapRegionCanonicalWindingSet, PlanarBooleanOverlapRegionCandidateBoundaryBundle,
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

pub(super) fn conflicting_persistent_name_bundle(
    arrangement: &crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanCoplanarOverlapArrangementGraph,
) -> PlanarBooleanPostAdmissionNormalizationBundle {
    let bundle = canonical_bundle(arrangement);
    let row = bundle.overlap_region_canonical_winding().rows()[0].clone();
    let duplicate = PlanarBooleanOverlapRegionCanonicalWindingRow::new(
        "synthetic-conflicting-canonical-winding".to_string(),
        row.source_kind(),
        "synthetic-conflicting-source".to_string(),
        row.island_identity().to_string(),
        row.neighborhood_identity().to_string(),
        Some("synthetic-conflicting-area".to_string()),
        row.canonical_operand_side(),
        row.canonical_winding_sign(),
        row.boundary_component_identities().to_vec(),
        row.canonical_boundary_segment_identities().to_vec(),
        row.canonical_source_loop_identities().to_vec(),
        row.chain_identities().to_vec(),
        row.fragment_identities().to_vec(),
        row.lineage_identities().to_vec(),
        row.source_edge_identities().to_vec(),
        row.boundary_roles().to_vec(),
        row.propagated_persistent_name_identities().to_vec(),
    );
    let canonical = PlanarBooleanOverlapRegionCanonicalWindingSet::new(
        "synthetic-conflicting-name-set".to_string(),
        bundle
            .overlap_region_canonical_winding()
            .request_identity()
            .to_string(),
        bundle
            .overlap_region_canonical_winding()
            .arrangement_graph_identity()
            .to_string(),
        bundle
            .overlap_region_canonical_winding()
            .cell_set_identity()
            .to_string(),
        bundle
            .overlap_region_canonical_winding()
            .ordering_basis_identity()
            .to_string(),
        vec![row, duplicate],
    );
    PlanarBooleanPostAdmissionNormalizationBundle::new(
        "synthetic-conflicting-name-bundle".to_string(),
        canonical,
        bundle.source_region_candidate_boundary().clone(),
        bundle.counters(),
    )
}

pub(super) fn duplicate_identity_bundle(
    arrangement: &crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanCoplanarOverlapArrangementGraph,
) -> PlanarBooleanPostAdmissionNormalizationBundle {
    let bundle = canonical_bundle(arrangement);
    let row = bundle.overlap_region_canonical_winding().rows()[0].clone();
    let canonical = PlanarBooleanOverlapRegionCanonicalWindingSet::new(
        "synthetic-duplicate-identity-set".to_string(),
        bundle
            .overlap_region_canonical_winding()
            .request_identity()
            .to_string(),
        bundle
            .overlap_region_canonical_winding()
            .arrangement_graph_identity()
            .to_string(),
        bundle
            .overlap_region_canonical_winding()
            .cell_set_identity()
            .to_string(),
        bundle
            .overlap_region_canonical_winding()
            .ordering_basis_identity()
            .to_string(),
        vec![row.clone(), row],
    );
    PlanarBooleanPostAdmissionNormalizationBundle::new(
        "synthetic-duplicate-identity-bundle".to_string(),
        canonical,
        bundle.source_region_candidate_boundary().clone(),
        bundle.counters(),
    )
}

pub(super) fn payload_permuted_canonical_bundle(
    arrangement: &crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanCoplanarOverlapArrangementGraph,
) -> PlanarBooleanPostAdmissionNormalizationBundle {
    let bundle = canonical_bundle(arrangement);
    let row = &bundle.overlap_region_canonical_winding().rows()[0];
    let mut reversed_boundary_components = row.boundary_component_identities().to_vec();
    reversed_boundary_components.reverse();
    let mut reversed_chain_identities = row.chain_identities().to_vec();
    reversed_chain_identities.reverse();
    let permuted = PlanarBooleanOverlapRegionCanonicalWindingRow::new(
        row.canonical_winding_identity().to_string(),
        row.source_kind(),
        row.source_identity().to_string(),
        row.island_identity().to_string(),
        row.neighborhood_identity().to_string(),
        row.area_overlap_component_identity().map(str::to_string),
        row.canonical_operand_side(),
        row.canonical_winding_sign(),
        reversed_boundary_components,
        row.canonical_boundary_segment_identities().to_vec(),
        row.canonical_source_loop_identities().to_vec(),
        reversed_chain_identities,
        row.fragment_identities().to_vec(),
        row.lineage_identities().to_vec(),
        row.source_edge_identities().to_vec(),
        row.boundary_roles().to_vec(),
        row.propagated_persistent_name_identities().to_vec(),
    );
    let canonical = PlanarBooleanOverlapRegionCanonicalWindingSet::new(
        "synthetic-payload-permuted-canonical-set".to_string(),
        bundle
            .overlap_region_canonical_winding()
            .request_identity()
            .to_string(),
        bundle
            .overlap_region_canonical_winding()
            .arrangement_graph_identity()
            .to_string(),
        bundle
            .overlap_region_canonical_winding()
            .cell_set_identity()
            .to_string(),
        bundle
            .overlap_region_canonical_winding()
            .ordering_basis_identity()
            .to_string(),
        vec![permuted],
    );
    PlanarBooleanPostAdmissionNormalizationBundle::new(
        "synthetic-payload-permuted-canonical-bundle".to_string(),
        canonical,
        bundle.source_region_candidate_boundary().clone(),
        bundle.counters(),
    )
}

pub(super) fn same_row_count_distinct_identity_bundle(
    arrangement: &crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanCoplanarOverlapArrangementGraph,
) -> PlanarBooleanPostAdmissionNormalizationBundle {
    let bundle = canonical_bundle(arrangement);
    let row = &bundle.overlap_region_canonical_winding().rows()[0];
    let distinct = PlanarBooleanOverlapRegionCanonicalWindingRow::new(
        "synthetic-distinct-canonical-winding".to_string(),
        row.source_kind(),
        "synthetic-distinct-source".to_string(),
        row.island_identity().to_string(),
        row.neighborhood_identity().to_string(),
        Some("synthetic-distinct-area".to_string()),
        row.canonical_operand_side(),
        row.canonical_winding_sign(),
        row.boundary_component_identities().to_vec(),
        row.canonical_boundary_segment_identities().to_vec(),
        row.canonical_source_loop_identities().to_vec(),
        row.chain_identities().to_vec(),
        row.fragment_identities().to_vec(),
        vec!["synthetic-distinct-lineage".to_string()],
        row.source_edge_identities().to_vec(),
        row.boundary_roles().to_vec(),
        vec!["synthetic-distinct-name".to_string()],
    );
    let canonical = PlanarBooleanOverlapRegionCanonicalWindingSet::new(
        "synthetic-same-row-count-distinct-set".to_string(),
        bundle
            .overlap_region_canonical_winding()
            .request_identity()
            .to_string(),
        bundle
            .overlap_region_canonical_winding()
            .arrangement_graph_identity()
            .to_string(),
        bundle
            .overlap_region_canonical_winding()
            .cell_set_identity()
            .to_string(),
        bundle
            .overlap_region_canonical_winding()
            .ordering_basis_identity()
            .to_string(),
        vec![distinct],
    );
    PlanarBooleanPostAdmissionNormalizationBundle::new(
        "synthetic-same-row-count-distinct-bundle".to_string(),
        canonical,
        bundle.source_region_candidate_boundary().clone(),
        bundle.counters(),
    )
}

pub(super) fn equivalent_multi_row_canonical_bundles_with_reversed_order(
    arrangement: &crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanCoplanarOverlapArrangementGraph,
) -> (
    PlanarBooleanPostAdmissionNormalizationBundle,
    PlanarBooleanPostAdmissionNormalizationBundle,
) {
    let bundle = canonical_bundle(arrangement);
    let row = bundle.overlap_region_canonical_winding().rows()[0].clone();
    let distinct = PlanarBooleanOverlapRegionCanonicalWindingRow::new(
        "synthetic-second-canonical-winding".to_string(),
        row.source_kind(),
        "synthetic-second-source".to_string(),
        row.island_identity().to_string(),
        row.neighborhood_identity().to_string(),
        Some("synthetic-second-area".to_string()),
        row.canonical_operand_side(),
        row.canonical_winding_sign(),
        row.boundary_component_identities().to_vec(),
        row.canonical_boundary_segment_identities().to_vec(),
        row.canonical_source_loop_identities().to_vec(),
        row.chain_identities().to_vec(),
        row.fragment_identities().to_vec(),
        vec!["synthetic-second-lineage".to_string()],
        row.source_edge_identities().to_vec(),
        row.boundary_roles().to_vec(),
        vec!["synthetic-second-name".to_string()],
    );
    let request_identity = bundle
        .overlap_region_canonical_winding()
        .request_identity()
        .to_string();
    let arrangement_graph_identity = bundle
        .overlap_region_canonical_winding()
        .arrangement_graph_identity()
        .to_string();
    let cell_set_identity = bundle
        .overlap_region_canonical_winding()
        .cell_set_identity()
        .to_string();
    let ordering_basis_identity = bundle
        .overlap_region_canonical_winding()
        .ordering_basis_identity()
        .to_string();

    let forward = PlanarBooleanOverlapRegionCanonicalWindingSet::new(
        "synthetic-forward-ordered-canonical-set".to_string(),
        request_identity.clone(),
        arrangement_graph_identity.clone(),
        cell_set_identity.clone(),
        ordering_basis_identity.clone(),
        vec![row.clone(), distinct.clone()],
    );
    let reversed = PlanarBooleanOverlapRegionCanonicalWindingSet::new(
        "synthetic-reversed-ordered-canonical-set".to_string(),
        request_identity,
        arrangement_graph_identity,
        cell_set_identity,
        ordering_basis_identity,
        vec![distinct, row],
    );

    (
        PlanarBooleanPostAdmissionNormalizationBundle::new(
            "synthetic-forward-ordered-canonical-bundle".to_string(),
            forward,
            bundle.source_region_candidate_boundary().clone(),
            bundle.counters(),
        ),
        PlanarBooleanPostAdmissionNormalizationBundle::new(
            "synthetic-reversed-ordered-canonical-bundle".to_string(),
            reversed,
            bundle.source_region_candidate_boundary().clone(),
            bundle.counters(),
        ),
    )
}
