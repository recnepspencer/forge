use crate::workload_platform::planar_boolean_common_plane::PlanarBooleanCommonPlaneOperandSide::Left;
use crate::workload_platform::planar_boolean_edge_splitting::PlanarBooleanOverlapChainBoundaryRole::FullOverlapSpan;
use crate::workload_platform::planar_boolean_loop_reconstruction::test_support::LoopFixtureEntryOrder;
use crate::workload_platform::planar_boolean_overlap_region_extraction::arrangement_graph::cell_classification::tests::fixtures::{
    admitted_graph, inside_both_multi_boundary_graph,
};
use crate::workload_platform::planar_boolean_overlap_region_extraction::{
    PlanarBooleanBoundaryContactClassificationBundle, PlanarBooleanOverlapCellContainmentInput,
    PlanarBooleanOverlapCellContainmentMap, PlanarBooleanOverlapCellWindingField,
    PlanarBooleanOverlapCellWindingFieldInput, PlanarBooleanOverlapChainRegionLineageMap,
    PlanarBooleanOverlapChainRegionLineageRow, PlanarBooleanOverlapIslandCandidateInput,
    PlanarBooleanOverlapIslandComponentBundle, PlanarBooleanOverlapRegionDecisionLog,
    PlanarBooleanOverlapRegionIdentityLineageBundle, PlanarBooleanOverlapRegionIdentityMap,
    PlanarBooleanOverlapRegionIdentityRow, PlanarBooleanOverlapRegionLedgerAssemblyBundle,
    PlanarBooleanOverlapRegionLedgerAssemblyInput,
    PlanarBooleanOverlapRegionPersistentNamePropagationMap,
    PlanarBooleanOverlapRegionSubshapeSignatureMap, PlanarBooleanPreRegionNormalizationBundle,
    PlanarBooleanSharedAreaAdmissionBundle,
};

pub(crate) fn canonical_graph()
-> crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanCoplanarOverlapArrangementGraph {
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
                format!("synthetic-phase-fourteen-lineage-row:{index}"),
                format!("synthetic-phase-fourteen-lineage:{index}"),
                format!("synthetic-phase-fourteen-chain:{index}"),
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
        "synthetic-phase-fourteen-lineage-map".to_string(),
        shared_area_bundle.request_identity().to_string(),
        rows,
    )
}

fn canonical_bundle(
    arrangement: &crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanCoplanarOverlapArrangementGraph,
) -> crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanPostAdmissionNormalizationBundle {
    let shared_area_bundle = admitted_shared_area_bundle(arrangement);
    let pre_region_bundle = PlanarBooleanPreRegionNormalizationBundle::from_shared_area_admission(
        &shared_area_bundle,
        &synthetic_chain_lineage_map(&shared_area_bundle),
    )
    .expect("fixture shared-area bundle should admit pre-region normalization");
    pre_region_bundle
        .promote_overlap_region_candidates(&shared_area_bundle)
        .expect("fixture bundle should promote overlap-region candidates")
        .normalize_post_admission_canonical_winding()
        .expect("fixture candidate bundle should canonicalize")
}

pub(crate) fn identity_bundle(
    arrangement: &crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanCoplanarOverlapArrangementGraph,
) -> PlanarBooleanOverlapRegionIdentityLineageBundle {
    canonical_bundle(arrangement)
        .mint_overlap_region_identity_lineage()
        .expect("fixture canonical bundle should admit identity minting")
}

pub(super) fn ledger_bundle(
    arrangement: &crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanCoplanarOverlapArrangementGraph,
) -> PlanarBooleanOverlapRegionLedgerAssemblyBundle {
    identity_bundle(arrangement)
        .mint_overlap_region_ledger()
        .expect("fixture identity bundle should admit overlap ledger assembly")
}

pub(crate) fn replayed_inputs() -> (
    PlanarBooleanOverlapRegionIdentityLineageBundle,
    PlanarBooleanOverlapRegionIdentityLineageBundle,
) {
    let canonical = admitted_graph(LoopFixtureEntryOrder::Canonical);
    let replayed = admitted_graph(LoopFixtureEntryOrder::Replayed);
    (identity_bundle(&canonical), identity_bundle(&replayed))
}

pub(super) fn missing_signature_bundle(
    arrangement: &crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanCoplanarOverlapArrangementGraph,
) -> PlanarBooleanOverlapRegionIdentityLineageBundle {
    let bundle = identity_bundle(arrangement);
    PlanarBooleanOverlapRegionIdentityLineageBundle::new(
        "synthetic-missing-signature-bundle".to_string(),
        bundle.overlap_region_identity_map().clone(),
        bundle.persistent_name_propagation_map().clone(),
        PlanarBooleanOverlapRegionSubshapeSignatureMap::new(
            "synthetic-empty-signature-map".to_string(),
            bundle.subshape_signature_map().request_identity().to_string(),
            Vec::new(),
        ),
        bundle.source_post_admission_normalization().clone(),
        bundle.counters(),
    )
}

pub(super) fn foreign_lineage_bundle(
    arrangement: &crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanCoplanarOverlapArrangementGraph,
) -> PlanarBooleanOverlapRegionIdentityLineageBundle {
    let bundle = identity_bundle(arrangement);
    PlanarBooleanOverlapRegionIdentityLineageBundle::new(
        "synthetic-foreign-lineage-bundle".to_string(),
        PlanarBooleanOverlapRegionIdentityMap::new(
            bundle.overlap_region_identity_map().map_identity().to_string(),
            bundle.overlap_region_identity_map().request_identity().to_string(),
            "synthetic-foreign-arrangement".to_string(),
            bundle.overlap_region_identity_map().cell_set_identity().to_string(),
            bundle
                .overlap_region_identity_map()
                .ordering_basis_identity()
                .to_string(),
            bundle.overlap_region_identity_map().rows().to_vec(),
        ),
        bundle.persistent_name_propagation_map().clone(),
        bundle.subshape_signature_map().clone(),
        bundle.source_post_admission_normalization().clone(),
        bundle.counters(),
    )
}

pub(super) fn synthetic_identity_row_bundle(
    arrangement: &crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanCoplanarOverlapArrangementGraph,
) -> PlanarBooleanOverlapRegionIdentityLineageBundle {
    let bundle = identity_bundle(arrangement);
    let row = &bundle.overlap_region_identity_map().rows()[0];
    let synthetic_row = PlanarBooleanOverlapRegionIdentityRow::new(
        row.region_identity().to_string(),
        row.canonical_winding_identity().to_string(),
        row.source_kind(),
        "synthetic-missing-source".to_string(),
        row.island_identity().to_string(),
        row.neighborhood_identity().to_string(),
        row.area_overlap_component_identity().map(str::to_string),
        row.canonical_operand_side(),
        row.canonical_winding_sign(),
        row.canonical_boundary_segment_identities().to_vec(),
        row.canonical_source_loop_identities().to_vec(),
        row.lineage_identities().to_vec(),
        row.source_edge_identities().to_vec(),
        row.boundary_roles().to_vec(),
    );
    PlanarBooleanOverlapRegionIdentityLineageBundle::new(
        "synthetic-overlap-row-bundle".to_string(),
        PlanarBooleanOverlapRegionIdentityMap::new(
            bundle.overlap_region_identity_map().map_identity().to_string(),
            bundle.overlap_region_identity_map().request_identity().to_string(),
            bundle
                .overlap_region_identity_map()
                .arrangement_graph_identity()
                .to_string(),
            bundle.overlap_region_identity_map().cell_set_identity().to_string(),
            bundle
                .overlap_region_identity_map()
                .ordering_basis_identity()
                .to_string(),
            vec![synthetic_row],
        ),
        PlanarBooleanOverlapRegionPersistentNamePropagationMap::new(
            bundle
                .persistent_name_propagation_map()
                .map_identity()
                .to_string(),
            bundle
                .persistent_name_propagation_map()
                .request_identity()
                .to_string(),
            bundle.persistent_name_propagation_map().rows().to_vec(),
        ),
        bundle.subshape_signature_map().clone(),
        bundle.source_post_admission_normalization().clone(),
        bundle.counters(),
    )
}

pub(super) fn direct_bundle(
    identity_lineage: &PlanarBooleanOverlapRegionIdentityLineageBundle,
) -> PlanarBooleanOverlapRegionLedgerAssemblyBundle {
    PlanarBooleanOverlapRegionLedgerAssemblyBundle::admit(
        PlanarBooleanOverlapRegionLedgerAssemblyInput::new(identity_lineage),
    )
    .expect("direct phase-fourteen admission should succeed")
}

pub(super) fn decision_log(
    bundle: &PlanarBooleanOverlapRegionLedgerAssemblyBundle,
) -> &PlanarBooleanOverlapRegionDecisionLog {
    bundle.decision_log()
}
