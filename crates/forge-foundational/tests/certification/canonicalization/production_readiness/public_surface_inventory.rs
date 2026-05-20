use forge_foundational::{
    canonical_milestone2_production_readiness_report,
    canonicalization_api::{
        canonical_public_surface_inventory, CanonicalPublicLane, CanonicalPublicSurfaceEntry,
    },
    CanonicalHarnessExpansionPoint,
};
use std::collections::BTreeSet;
use std::path::Path;

#[test]
fn canonical_public_surface_inventory_is_exact_and_grouped_by_lane() {
    let inventory = canonical_public_surface_inventory();

    assert_exact_paths(
        inventory,
        &[
            "forge_foundational::canonicalization_api::common_path",
            "forge_foundational::canonicalization_api::lower_lane::basis",
            "forge_foundational::canonicalization_api::lower_lane::comparison",
            "forge_foundational::canonicalization_api::lower_lane::export",
            "forge_foundational::canonicalization_api::lower_lane::digest",
            "forge_foundational::canonicalization_api::stronger_lane",
            "forge_foundational::canonicalization_api::stronger_lane::readiness",
        ],
    );
    assert_eq!(
        inventory
            .iter()
            .filter(|entry| entry.lane() == CanonicalPublicLane::CommonPath)
            .count(),
        1
    );
    assert_eq!(
        inventory
            .iter()
            .filter(|entry| entry.lane() == CanonicalPublicLane::LowerLane)
            .count(),
        4
    );
    assert_eq!(
        inventory
            .iter()
            .filter(|entry| entry.lane() == CanonicalPublicLane::StrongerLane)
            .count(),
        2
    );
    assert!(inventory
        .iter()
        .all(|entry| !entry.teaches().trim().is_empty()));
    assert!(inventory
        .iter()
        .all(|entry| !entry.does_not_hide().trim().is_empty()));
}

#[test]
fn readiness_artifact_freezes_public_surface_inventory_and_grouped_lane_evidence() {
    let report = canonical_milestone2_production_readiness_report();

    assert_eq!(
        report.public_surface_inventory(),
        canonical_public_surface_inventory()
    );
    assert!(report
        .harness_expansion_points()
        .contains(&CanonicalHarnessExpansionPoint::GroupedPublicSurfaceLane));
    assert!(
        crate_root_path(report.public_surface_evidence_path()).is_file(),
        "grouped public-surface evidence path must point at a real certification file"
    );
    assert!(
        crate_root_path(report.public_surface_compile_fail_path()).is_file(),
        "grouped public-surface compile-fail path must point at a real fixture"
    );
}

fn crate_root_path(relative: &str) -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(relative)
}

fn assert_exact_paths(actual: &[CanonicalPublicSurfaceEntry], expected: &[&str]) {
    let actual_paths: BTreeSet<_> = actual.iter().map(|entry| entry.path()).collect();
    let expected_paths: BTreeSet<_> = expected.iter().copied().collect();

    assert_eq!(
        actual.len(),
        expected.len(),
        "public-surface inventory contains duplicate rows"
    );
    assert_eq!(
        actual_paths, expected_paths,
        "public-surface inventory changed without updating certification"
    );
}
