use crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanPreRegionNormalizationDenialKind;

use super::support::{
    admitted_shared_area_bundle, contradictory_localized_chain_lineage_map, shared_area_graph,
    synthetic_chain_lineage_map, synthetic_missing_lineage_map,
};

#[test]
fn pre_region_normalization_rejects_ambiguous_coincident_ordering() {
    let shared_area_bundle = admitted_shared_area_bundle(&shared_area_graph());
    let denial = shared_area_bundle
        .normalize_pre_region_coincidence(&synthetic_chain_lineage_map(
            &shared_area_bundle,
            false,
            true,
            false,
        ))
        .expect_err("ambiguous coincident ordering must deny before region promotion");

    assert_eq!(
        denial.kind(),
        PlanarBooleanPreRegionNormalizationDenialKind::AmbiguousOppositeSenseOverlapOrderingDenied
    );
}

#[test]
fn pre_region_normalization_rejects_unstable_orientation_tie_breakers() {
    let shared_area_bundle = admitted_shared_area_bundle(&shared_area_graph());
    let denial = shared_area_bundle
        .normalize_pre_region_coincidence(&synthetic_chain_lineage_map(
            &shared_area_bundle,
            false,
            false,
            true,
        ))
        .expect_err("unstable opposite-sense tie breakers must deny before region promotion");

    assert_eq!(
        denial.kind(),
        PlanarBooleanPreRegionNormalizationDenialKind::UnstableOrientationTieBreakerDenied
    );
}

#[test]
fn pre_region_normalization_rejects_missing_matching_chain_lineage() {
    let shared_area_bundle = admitted_shared_area_bundle(&shared_area_graph());
    let denial = shared_area_bundle
        .normalize_pre_region_coincidence(&synthetic_missing_lineage_map(&shared_area_bundle))
        .expect_err("shared-area outcomes without matching chain lineage must deny");

    assert_eq!(
        denial.kind(),
        PlanarBooleanPreRegionNormalizationDenialKind::MissingChainLineageForSharedAreaOutcomeDenied
    );
}

#[test]
fn pre_region_normalization_rejects_contradictory_component_local_witness() {
    let shared_area_bundle = admitted_shared_area_bundle(&shared_area_graph());
    let denial = shared_area_bundle
        .normalize_pre_region_coincidence(&contradictory_localized_chain_lineage_map(
            &shared_area_bundle,
        ))
        .expect_err("contradictory component-local operand and winding proof must deny");

    assert_eq!(
        denial.kind(),
        PlanarBooleanPreRegionNormalizationDenialKind::UnstableOrientationTieBreakerDenied
    );
}
