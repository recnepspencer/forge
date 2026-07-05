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
    PlanarBooleanOverlapRegionCanonicalWindingSet, PlanarBooleanOverlapRegionCandidateBoundaryInput,
    PlanarBooleanOverlapRegionCandidateSet, PlanarBooleanOppositeSenseOverlapNormalizationSet,
    PlanarBooleanPostAdmissionNormalizationBundle, PlanarBooleanPostAdmissionNormalizationInput,
    PlanarBooleanPreRegionNormalizationBundle, PlanarBooleanSharedAreaAdmissionBundle,
};

pub(super) fn canonical_winding_graph()
-> crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanCoplanarOverlapArrangementGraph {
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
        "synthetic-pre-region-lineage-map".to_string(),
        shared_area_bundle.request_identity().to_string(),
        rows,
    )
}

pub(super) fn admitted_region_candidate_bundle(
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

pub(super) fn boundary_only_region_candidate_bundle() -> PlanarBooleanOverlapRegionCandidateBoundaryBundle {
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
        .expect("boundary-only fixture should still produce a phase-twelve input bundle")
}

pub(super) fn replayed_inputs() -> (
    PlanarBooleanOverlapRegionCandidateBoundaryBundle,
    PlanarBooleanOverlapRegionCandidateBoundaryBundle,
) {
    let canonical = admitted_graph(LoopFixtureEntryOrder::Canonical);
    let replayed = admitted_graph(LoopFixtureEntryOrder::Replayed);
    (
        admitted_region_candidate_bundle(&canonical),
        admitted_region_candidate_bundle(&replayed),
    )
}

pub(super) fn admitted_post_admission_bundle(
    arrangement: &crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanCoplanarOverlapArrangementGraph,
) -> PlanarBooleanPostAdmissionNormalizationBundle {
    let bundle = admitted_region_candidate_bundle(arrangement);
    bundle
        .normalize_post_admission_canonical_winding()
        .expect("fixture admitted region bundle should admit post-admission normalization")
}

pub(super) fn ambiguous_admitted_region_bundle(
    arrangement: &crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanCoplanarOverlapArrangementGraph,
) -> PlanarBooleanOverlapRegionCandidateBoundaryBundle {
    let bundle = admitted_region_candidate_bundle(arrangement);
    let row = bundle.admitted_overlap_regions().rows()[0].clone();
    let duplicate_row =
        crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanAdmittedOverlapRegionRow::new(
            "synthetic-duplicate-admitted-region".to_string(),
            "synthetic-duplicate-candidate".to_string(),
            row.shared_area_admission_outcome_identity().to_string(),
            row.normalization_identity().to_string(),
            row.island_identity().to_string(),
            row.neighborhood_identity().to_string(),
            row.area_overlap_component_identity().to_string(),
            row.cell_identities().to_vec(),
            row.boundary_component_identities().to_vec(),
            row.boundary_segment_identities().to_vec(),
            row.source_loop_identities().to_vec(),
            row.canonical_boundary_segment_witness().to_vec(),
            row.canonical_source_loop_witness().to_vec(),
            row.canonical_operand_side(),
            -row.canonical_winding_sign(),
            row.chain_identities().to_vec(),
            row.fragment_identities().to_vec(),
            row.lineage_identities().to_vec(),
            row.source_edge_identities().to_vec(),
            row.boundary_roles().to_vec(),
            row.propagated_persistent_name_identities().to_vec(),
        );
    let mut rows = bundle.admitted_overlap_regions().rows().to_vec();
    rows.push(duplicate_row);
    let admitted = crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanAdmittedOverlapRegionSet::new(
        "synthetic-ambiguous-admitted-set".to_string(),
        bundle.admitted_overlap_regions().request_identity().to_string(),
        bundle.admitted_overlap_regions().arrangement_graph_identity().to_string(),
        bundle.admitted_overlap_regions().cell_set_identity().to_string(),
        bundle.admitted_overlap_regions().ordering_basis_identity().to_string(),
        rows,
    );
    crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanOverlapRegionCandidateBoundaryBundle::new(
        "synthetic-ambiguous-admitted-bundle".to_string(),
        bundle.overlap_region_candidates().clone(),
        bundle.denied_overlap_region_candidates().clone(),
        admitted,
        bundle.boundary_only_overlap_outcomes().clone(),
        bundle.counters(),
    )
}

pub(super) fn ambiguous_boundary_only_bundle() -> PlanarBooleanOverlapRegionCandidateBoundaryBundle {
    let bundle = boundary_only_region_candidate_bundle();
    let row = bundle.boundary_only_overlap_outcomes().rows()[0].clone();
    let duplicate_row = PlanarBooleanBoundaryOnlyOverlapOutcomeRow::new(
        "synthetic-duplicate-boundary-only".to_string(),
        row.pure_boundary_only_outcome_identity().to_string(),
        row.island_identity().to_string(),
        row.neighborhood_identity().to_string(),
        row.boundary_contact_component_identities().to_vec(),
        row.cell_identities().to_vec(),
        row.boundary_component_identities().to_vec(),
        row.boundary_segment_identities().to_vec(),
        row.source_loop_identities().to_vec(),
        row.canonical_boundary_segment_witness().to_vec(),
        row.canonical_source_loop_witness().to_vec(),
    );
    let mut rows = bundle.boundary_only_overlap_outcomes().rows().to_vec();
    rows.push(duplicate_row);
    let boundary_only = PlanarBooleanBoundaryOnlyOverlapOutcomeSet::new(
        "synthetic-ambiguous-boundary-only-set".to_string(),
        bundle.boundary_only_overlap_outcomes().request_identity().to_string(),
        bundle.boundary_only_overlap_outcomes().arrangement_graph_identity().to_string(),
        bundle.boundary_only_overlap_outcomes().cell_set_identity().to_string(),
        bundle.boundary_only_overlap_outcomes().ordering_basis_identity().to_string(),
        rows,
    );
    PlanarBooleanOverlapRegionCandidateBoundaryBundle::new(
        "synthetic-ambiguous-boundary-only-bundle".to_string(),
        bundle.overlap_region_candidates().clone(),
        bundle.denied_overlap_region_candidates().clone(),
        bundle.admitted_overlap_regions().clone(),
        boundary_only,
        bundle.counters(),
    )
}

pub(super) fn canonical_winding_set(
    bundle: &PlanarBooleanPostAdmissionNormalizationBundle,
) -> &PlanarBooleanOverlapRegionCanonicalWindingSet {
    bundle.overlap_region_canonical_winding()
}

pub(super) fn payload_permuted_region_candidate_bundle(
    arrangement: &crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanCoplanarOverlapArrangementGraph,
) -> PlanarBooleanOverlapRegionCandidateBoundaryBundle {
    let bundle = admitted_region_candidate_bundle(arrangement);
    let row = &bundle.admitted_overlap_regions().rows()[0];
    let mut reversed_boundary_segments = row.boundary_segment_identities().to_vec();
    reversed_boundary_segments.reverse();
    let mut reversed_source_loops = row.source_loop_identities().to_vec();
    reversed_source_loops.reverse();
    let mut reversed_source_edges = row.source_edge_identities().to_vec();
    reversed_source_edges.reverse();
    let permuted_row =
        crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanAdmittedOverlapRegionRow::new(
            row.admitted_region_identity().to_string(),
            row.candidate_identity().to_string(),
            row.shared_area_admission_outcome_identity().to_string(),
            row.normalization_identity().to_string(),
            row.island_identity().to_string(),
            row.neighborhood_identity().to_string(),
            row.area_overlap_component_identity().to_string(),
            row.cell_identities().to_vec(),
            row.boundary_component_identities().to_vec(),
            reversed_boundary_segments,
            reversed_source_loops,
            row.canonical_boundary_segment_witness().to_vec(),
            row.canonical_source_loop_witness().to_vec(),
            row.canonical_operand_side(),
            row.canonical_winding_sign(),
            row.chain_identities().to_vec(),
            row.fragment_identities().to_vec(),
            row.lineage_identities().to_vec(),
            reversed_source_edges,
            row.boundary_roles().to_vec(),
            row.propagated_persistent_name_identities().to_vec(),
        );
    let admitted = crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanAdmittedOverlapRegionSet::new(
        "synthetic-payload-permuted-admitted-set".to_string(),
        bundle.admitted_overlap_regions().request_identity().to_string(),
        bundle.admitted_overlap_regions().arrangement_graph_identity().to_string(),
        bundle.admitted_overlap_regions().cell_set_identity().to_string(),
        bundle.admitted_overlap_regions().ordering_basis_identity().to_string(),
        vec![permuted_row],
    );
    crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanOverlapRegionCandidateBoundaryBundle::new(
        "synthetic-payload-permuted-bundle".to_string(),
        bundle.overlap_region_candidates().clone(),
        bundle.denied_overlap_region_candidates().clone(),
        admitted,
        bundle.boundary_only_overlap_outcomes().clone(),
        bundle.counters(),
    )
}
