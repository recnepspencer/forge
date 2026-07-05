use super::rows::PlanarBooleanOverlapRegionExtractionDirectoryCutoverMap;
use super::scan::PlanarBooleanOverlapRegionExtractionPathDenial as Denial;

#[test]
fn every_concrete_artifact_has_one_owner_and_consuming_phase() {
    let map = PlanarBooleanOverlapRegionExtractionDirectoryCutoverMap::phase_2();
    assert!(map.certifies_one_owner_per_artifact());
    assert!(map.certifies_one_consuming_phase_per_artifact());
    map.certifies_live_artifact_contracts()
        .expect("phase 2 artifact rows should match live owner and consumer sources");
    assert!(map
        .artifact_rows()
        .iter()
        .any(|row| row.artifact_name() == "PlanarBooleanOverlapOperatorClassificationMatrix"));
    assert!(map
        .artifact_rows()
        .iter()
        .any(|row| row.artifact_name() == "PlanarBooleanOverlapRegistrationContract"));
    map.certifies_legacy_surface_contracts()
        .expect("deleted wrapper residue paths should stay absent");
}

#[test]
fn live_overlap_lane_contains_no_local_clone_or_helper_paths() {
    let map = PlanarBooleanOverlapRegionExtractionDirectoryCutoverMap::phase_2();
    map.certify_live_overlap_lane()
        .expect("live overlap lane should match the declared cutover map");
}

#[test]
fn local_clone_paths_are_rejected() {
    let map = PlanarBooleanOverlapRegionExtractionDirectoryCutoverMap::phase_2();
    assert_eq!(
        map.certify_overlap_lane_path(
            "crates/worth-spatial/src/workload_platform/planar_boolean_overlap_region_extraction/touched_graph/mod.rs",
        ),
        Err(Denial::LocalTouchedGraphClone)
    );
    assert_eq!(
        map.certify_overlap_lane_path(
            "crates/worth-spatial/src/workload_platform/planar_boolean_overlap_region_extraction/selected_route/mod.rs",
        ),
        Err(Denial::LocalSelectedRouteClone)
    );
    assert_eq!(
        map.certify_overlap_lane_path(
            "crates/worth-spatial/src/workload_platform/planar_boolean_overlap_region_extraction/query_posture/mod.rs",
        ),
        Err(Denial::LocalQueryPostureClone)
    );
    assert_eq!(
        map.certify_overlap_lane_path(
            "crates/worth-spatial/src/workload_platform/planar_boolean_overlap_region_extraction/representative_family_coverage/mod.rs",
        ),
        Err(Denial::LocalRepresentativeCoverageClone)
    );
}
