use std::collections::BTreeSet;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use super::{
    current_worth_touched_graph_conflict_source_firewall_report,
    scan_worth_touched_graph_conflict_source_firewall_region_for_tests,
    WorthTouchedGraphConflictForbiddenSurface, WorthTouchedGraphConflictSourceFirewallReport,
};

#[test]
fn source_firewall_scans_kernel_topo_and_spatial_regions() {
    let report = current_worth_touched_graph_conflict_source_firewall_report()
        .expect("phase 12 source firewall should scan the current workspace");

    assert_eq!(report.scanned_region_count(), 3);
    assert!(report.scanned_source_count() > 0);
    assert!(report.violations().is_empty(), "{:?}", report.violations());
    let regions = report
        .region_reports()
        .iter()
        .map(|row| row.region_label())
        .collect::<BTreeSet<_>>();
    assert_eq!(
        regions,
        BTreeSet::from([
            "kernel_workload_composition",
            "spatial_touched_graph_conflict",
            "topology_touched_graph_conflict",
        ])
    );
    assert_eq!(
        report.covered_forbidden_surfaces(),
        BTreeSet::from([
            WorthTouchedGraphConflictForbiddenSurface::EntityOnlyOverlapHelper,
            WorthTouchedGraphConflictForbiddenSurface::BroadTopologyScan,
            WorthTouchedGraphConflictForbiddenSurface::BroadEvidenceScan,
            WorthTouchedGraphConflictForbiddenSurface::RollbackAdmission,
            WorthTouchedGraphConflictForbiddenSurface::CallerOwnedCompatibility,
            WorthTouchedGraphConflictForbiddenSurface::GenericOverlapSecondAuthorityLane,
        ])
    );
}

#[test]
fn source_firewall_rejects_registered_overlap_and_scan_relapse() {
    let workspace = temp_dir("tgc-source-firewall");
    let hostile_sources = [
        (
            "crates/worth-spatial/src/workload_platform/projected_overlap_faces/shadow_pair_gate.rs",
            "fn neutral_overlap_helper() {}\n",
            WorthTouchedGraphConflictForbiddenSurface::EntityOnlyOverlapHelper,
        ),
        (
            "crates/worth-spatial/src/workload_platform/projected_overlap_faces/shadow_authority.rs",
            "fn neutral_second_lane() {}\n",
            WorthTouchedGraphConflictForbiddenSurface::GenericOverlapSecondAuthorityLane,
        ),
        (
            "crates/worth-topo/src/derived_topology/invalidation_plan/migrated_products/traversal_views/shadow_scan.rs",
            "fn neutral_topology_scan() {}\n",
            WorthTouchedGraphConflictForbiddenSurface::BroadTopologyScan,
        ),
        (
            "crates/worth-spatial/src/workload_platform/evidence_lookup_source_firewall/shadow_scan.rs",
            "fn neutral_evidence_scan() {}\n",
            WorthTouchedGraphConflictForbiddenSurface::BroadEvidenceScan,
        ),
        (
            "crates/worth-spatial/src/replay_undo_semantic_graph/undo_family_execution/shadow_lock.rs",
            "fn neutral_lock_gate() {}\n",
            WorthTouchedGraphConflictForbiddenSurface::LockFirstAdmission,
        ),
        (
            "crates/worth-topo/src/replay_undo_semantic_graph/undo_family_execution/shadow_retry.rs",
            "fn neutral_retry_gate() {}\n",
            WorthTouchedGraphConflictForbiddenSurface::SpeculativeRollbackAdmission,
        ),
        (
            "crates/worth-spatial/src/workload_platform/high_valence_singularity/shadow_posture.rs",
            "fn neutral_posture_gate() {}\n",
            WorthTouchedGraphConflictForbiddenSurface::CallerOwnedCompatibility,
        ),
        (
            "crates/worth-kernel/src/workload_composition/conflict_input/shadow_delivery.rs",
            "fn neutral_serialization_gate() {}\n",
            WorthTouchedGraphConflictForbiddenSurface::CallerOwnedSerialization,
        ),
        (
            "crates/worth-kernel/src/workload_composition/conflict_input/shadow_borrowed_surface.rs",
            "pub struct SpatialConflictInputRequest;\nfn neutral_borrowed_surface() {}\n",
            WorthTouchedGraphConflictForbiddenSurface::CallerOwnedSerialization,
        ),
        (
            "crates/worth-kernel/src/workload_composition/worth_workload/lookup_consumed_workload/shadow_delivery.rs",
            "fn neutral_lookup_delivery() {}\n",
            WorthTouchedGraphConflictForbiddenSurface::CallerOwnedSerialization,
        ),
        (
            "crates/worth-spatial/src/workload_platform/projected_overlap_faces/face_set.rs",
            "pub struct ProjectedOverlapFaceSet;\nfn neutral_owned_overlap_helper() {}\n",
            WorthTouchedGraphConflictForbiddenSurface::EntityOnlyOverlapHelper,
        ),
        (
            "crates/worth-topo/src/derived_topology/invalidation_plan/migrated_products/traversal_views/closeout.rs",
            "pub struct TraversalViewsMigrationCloseout;\nfn neutral_owned_scan() {}\n",
            WorthTouchedGraphConflictForbiddenSurface::BroadTopologyScan,
        ),
        (
            "crates/worth-spatial/src/replay_undo_semantic_graph/undo_family_execution/rollback_admission.rs",
            "pub enum SpatialUndoFamilyExecutionError { Foreign }\nimpl From<String> for SpatialUndoFamilyExecutionError {\n    fn from(value: String) -> Self {\n        let _ = value;\n        Self::Foreign\n    }\n}\n",
            WorthTouchedGraphConflictForbiddenSurface::LockFirstAdmission,
        ),
        (
            "crates/worth-spatial/src/workload_platform/high_valence_singularity/singularity_workload.rs",
            "pub struct HighValenceSingularityWorkload;\nfn neutral_owned_posture() {}\n",
            WorthTouchedGraphConflictForbiddenSurface::CallerOwnedCompatibility,
        ),
        (
            "crates/worth-kernel/src/workload_composition/conflict_input/spatial.rs",
            "pub struct SpatialConflictInputRequest;\nfn neutral_owned_serialization() {}\n",
            WorthTouchedGraphConflictForbiddenSurface::CallerOwnedSerialization,
        ),
        (
            "crates/worth-kernel/src/workload_composition/worth_workload/lookup_consumed_workload/mod.rs",
            "pub struct LookupConsumedWorkloadComposition;\nfn neutral_owned_lookup_helper() {}\n",
            WorthTouchedGraphConflictForbiddenSurface::CallerOwnedSerialization,
        ),
        (
            "crates/worth-spatial/src/workload_platform/projected_overlap_faces/shadow_borrowed_private.rs",
            "fn bridge_authority_digest() {}\n",
            WorthTouchedGraphConflictForbiddenSurface::EntityOnlyOverlapHelper,
        ),
    ];
    for (relative_path, source, _) in hostile_sources {
        let path = workspace.join(relative_path);
        fs::create_dir_all(path.parent().expect("hostile parent")).expect("create hostile parent");
        fs::write(path, source).expect("write hostile source");
    }

    let report = scan_worth_touched_graph_conflict_source_firewall_region_for_tests(
        "synthetic_region",
        "synthetic:root",
        &workspace,
    )
    .expect("synthetic root should scan");

    for (relative_path, _, forbidden_surface) in hostile_sources {
        assert_violation(
            &report,
            relative_path,
            forbidden_surface,
            "synthetic_region",
        );
    }

    let _ = fs::remove_dir_all(workspace);
}

fn assert_violation(
    report: &WorthTouchedGraphConflictSourceFirewallReport,
    expected_path: &str,
    expected_surface: WorthTouchedGraphConflictForbiddenSurface,
    expected_region: &str,
) {
    assert!(
        report.violations().iter().any(|row| {
            row.region_label() == expected_region
                && row.source_path().ends_with(expected_path)
                && row.forbidden_surface() == expected_surface
        }),
        "missing violation for `{expected_path}` as `{expected_surface:?}`; actual violations: {:?}",
        report
            .violations()
            .iter()
            .map(|row| (
                row.region_label().to_string(),
                row.source_path().to_string(),
                row.surface_name().to_string(),
                row.forbidden_surface(),
            ))
            .collect::<Vec<_>>()
    );
}

fn temp_dir(prefix: &str) -> PathBuf {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time before unix epoch")
        .as_nanos();
    let path = std::env::temp_dir().join(format!("{prefix}-{stamp}"));
    fs::create_dir_all(&path).expect("create temp dir");
    path
}
