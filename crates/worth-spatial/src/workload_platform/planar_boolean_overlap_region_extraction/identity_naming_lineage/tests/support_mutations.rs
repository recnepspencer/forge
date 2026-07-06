use crate::workload_platform::planar_boolean_overlap_region_extraction::{
    PlanarBooleanOverlapRegionCanonicalWindingRow, PlanarBooleanOverlapRegionCanonicalWindingSet,
    PlanarBooleanPostAdmissionNormalizationBundle,
};

use super::support::canonical_bundle;

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
    let canonical = canonical_set_from_bundle(
        &bundle,
        "synthetic-conflicting-name-set",
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
    let canonical = canonical_set_from_bundle(
        &bundle,
        "synthetic-duplicate-identity-set",
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
    let canonical = canonical_set_from_bundle(
        &bundle,
        "synthetic-payload-permuted-canonical-set",
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
    let canonical = canonical_set_from_bundle(
        &bundle,
        "synthetic-same-row-count-distinct-set",
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
    let forward = canonical_set_from_bundle(
        &bundle,
        "synthetic-forward-ordered-canonical-set",
        vec![row.clone(), distinct.clone()],
    );
    let reversed = canonical_set_from_bundle(
        &bundle,
        "synthetic-reversed-ordered-canonical-set",
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

fn canonical_set_from_bundle(
    bundle: &PlanarBooleanPostAdmissionNormalizationBundle,
    identity: &str,
    rows: Vec<PlanarBooleanOverlapRegionCanonicalWindingRow>,
) -> PlanarBooleanOverlapRegionCanonicalWindingSet {
    PlanarBooleanOverlapRegionCanonicalWindingSet::new(
        identity.to_string(),
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
        rows,
    )
}
