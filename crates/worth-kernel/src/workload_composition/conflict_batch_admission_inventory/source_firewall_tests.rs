use super::test_support::{assert_violation_signatures, temp_firewall_root, write_source};
use super::{
    current_conflict_batch_admission_inventory, ConflictBatchAdmissionScanPattern,
    ConflictBatchAdmissionSourceFirewallReport,
};

#[test]
fn unclassified_conflict_surface_fails_closeout() {
    let root = temp_firewall_root("unclassified_conflict_surface");
    write_source(
        &root,
        "unclassified.rs",
        "#[allow(dead_code)]\n\
         pub async fn compute_overlap_conflict(\n\
             input: usize,\n\
         ) -> usize {\n\
             input\n\
         }\n\
         pub(crate) async fn try_lock_then_admit_batch(\n\
             count: usize,\n\
         ) -> usize {\n\
             count\n\
         }\n\
         pub async fn lock_first_batch_admission(\n\
             batch: usize,\n\
         ) -> usize {\n\
             batch\n\
         }\n\
         pub(crate) const fn caller_serialization_hint() -> usize { 0 }\n\
         pub fn speculative_rollback_admission(\n\
             replay_scope: usize,\n\
         ) -> usize {\n\
             replay_scope\n\
         }\n\
         pub(crate) fn require_rebuild_motion_compatibility(\n\
             posture: usize,\n\
         ) -> usize {\n\
             posture\n\
         }\n\
         pub struct PointSplitCompatibilityBasis;\n",
    );
    let inventory =
        current_conflict_batch_admission_inventory().expect("current inventory should build");
    let report =
        ConflictBatchAdmissionSourceFirewallReport::scan_root_against_inventory(&root, &inventory)
            .expect("firewall scan should complete");

    assert_eq!(report.violations().len(), 7);
    assert_violation_signatures(
        &report,
        &[
            (
                "unclassified.rs",
                "compute_overlap_conflict",
                ConflictBatchAdmissionScanPattern::OrdinaryOverlapHelper,
            ),
            (
                "unclassified.rs",
                "try_lock_then_admit_batch",
                ConflictBatchAdmissionScanPattern::LockFirstAdmission,
            ),
            (
                "unclassified.rs",
                "lock_first_batch_admission",
                ConflictBatchAdmissionScanPattern::LockFirstAdmission,
            ),
            (
                "unclassified.rs",
                "caller_serialization_hint",
                ConflictBatchAdmissionScanPattern::CallerOwnedSerializationHint,
            ),
            (
                "unclassified.rs",
                "speculative_rollback_admission",
                ConflictBatchAdmissionScanPattern::SpeculativeRollbackAdmission,
            ),
            (
                "unclassified.rs",
                "require_rebuild_motion_compatibility",
                ConflictBatchAdmissionScanPattern::CallerOwnedCompatibilityList,
            ),
            (
                "unclassified.rs",
                "PointSplitCompatibilityBasis",
                ConflictBatchAdmissionScanPattern::CallerOwnedCompatibilityList,
            ),
        ],
    );
    assert!(report.ensure_clean().is_err());
}

#[test]
fn rollback_admission_paths_require_exact_inventory_rows() {
    let root = temp_firewall_root("rollback_admission_path");
    write_source(
        &root,
        "crates/worth-spatial/src/replay_undo_semantic_graph/undo_family_execution/rollback_admission.rs",
        "pub struct RollbackAdmissionFixture<T>(T);\n\
         impl<T> RollbackAdmissionFixture<T> {\n\
             pub async fn lower_new_undo_scope_product_from_shortcut(\n\
                 &self,\n\
             ) -> usize {\n\
                 0\n\
             }\n\
         }\n",
    );
    let inventory =
        current_conflict_batch_admission_inventory().expect("current inventory should build");
    let report =
        ConflictBatchAdmissionSourceFirewallReport::scan_root_against_inventory(&root, &inventory)
            .expect("firewall scan should complete");

    assert_eq!(report.violations().len(), 2);
    assert_violation_signatures(
        &report,
        &[
            (
                "crates/worth-spatial/src/replay_undo_semantic_graph/undo_family_execution/rollback_admission.rs",
                "RollbackAdmissionFixture",
                ConflictBatchAdmissionScanPattern::RollbackAdmission,
            ),
            (
                "crates/worth-spatial/src/replay_undo_semantic_graph/undo_family_execution/rollback_admission.rs",
                "RollbackAdmissionFixture::lower_new_undo_scope_product_from_shortcut",
                ConflictBatchAdmissionScanPattern::RollbackAdmission,
            ),
        ],
    );
    assert!(report.ensure_clean().is_err());
}

#[test]
fn duplicate_split_compatibility_decisions_require_exact_inventory_rows() {
    let root = temp_firewall_root("duplicate_split_compatibility");
    write_source(
        &root,
        "crates/worth-spatial/src/workload_platform/planar_boolean_edge_splitting/duplicate_split_normalization/contradiction_basis.rs",
        "#[allow(dead_code)]\n\
         pub fn reject_contradictory_new_basis(\n\
             basis: usize,\n\
         ) -> usize {\n\
             basis\n\
         }\n",
    );
    let inventory =
        current_conflict_batch_admission_inventory().expect("current inventory should build");
    let report =
        ConflictBatchAdmissionSourceFirewallReport::scan_root_against_inventory(&root, &inventory)
            .expect("firewall scan should complete");

    assert_eq!(report.violations().len(), 1);
    assert_violation_signatures(
        &report,
        &[(
            "crates/worth-spatial/src/workload_platform/planar_boolean_edge_splitting/duplicate_split_normalization/contradiction_basis.rs",
            "reject_contradictory_new_basis",
            ConflictBatchAdmissionScanPattern::CallerOwnedCompatibilityList,
        )],
    );
    assert!(report.ensure_clean().is_err());
}

#[test]
fn broad_scan_directory_rows_do_not_mask_new_surfaces() {
    let root = temp_firewall_root("broad_scan_directory_mask");
    write_source(
        &root,
        "crates/worth-spatial/src/workload_platform/evidence_lookup/new_broad.rs",
        "pub struct BroadOverlapScanExecution;\n\
         impl BroadOverlapScanExecution {\n\
             pub fn broad_overlap_scan_execution(\n\
                 &self,\n\
             ) -> usize {\n\
                 0\n\
             }\n\
         }\n",
    );
    let inventory =
        current_conflict_batch_admission_inventory().expect("current inventory should build");
    let report =
        ConflictBatchAdmissionSourceFirewallReport::scan_root_against_inventory(&root, &inventory)
            .expect("firewall scan should complete");

    assert_eq!(report.violations().len(), 2);
    assert_violation_signatures(
        &report,
        &[
            (
                "crates/worth-spatial/src/workload_platform/evidence_lookup/new_broad.rs",
                "BroadOverlapScanExecution",
                ConflictBatchAdmissionScanPattern::BroadOverlapScan,
            ),
            (
                "crates/worth-spatial/src/workload_platform/evidence_lookup/new_broad.rs",
                "BroadOverlapScanExecution::broad_overlap_scan_execution",
                ConflictBatchAdmissionScanPattern::BroadOverlapScan,
            ),
        ],
    );
    assert!(report.ensure_clean().is_err());
}

#[test]
fn certification_coordination_surfaces_are_not_invisible() {
    let root = temp_firewall_root("certification_coordination_surface");
    write_source(
        &root,
        "crates/worth-spatial/src/certification/public_facade_contracts/contracts/overlap.rs",
        "pub struct CertificationOverlapHelper;\n\
         impl CertificationOverlapHelper {\n\
             pub fn certification_overlap_compatibility_helper(\n\
                 &self,\n\
             ) -> usize {\n\
                 0\n\
             }\n\
         }\n",
    );
    let inventory =
        current_conflict_batch_admission_inventory().expect("current inventory should build");
    let report =
        ConflictBatchAdmissionSourceFirewallReport::scan_root_against_inventory(&root, &inventory)
            .expect("firewall scan should complete");

    assert_eq!(report.violations().len(), 2);
    assert_violation_signatures(
        &report,
        &[
            (
                "crates/worth-spatial/src/certification/public_facade_contracts/contracts/overlap.rs",
                "CertificationOverlapHelper",
                ConflictBatchAdmissionScanPattern::OrdinaryOverlapHelper,
            ),
            (
                "crates/worth-spatial/src/certification/public_facade_contracts/contracts/overlap.rs",
                "CertificationOverlapHelper::certification_overlap_compatibility_helper",
                ConflictBatchAdmissionScanPattern::OrdinaryOverlapHelper,
            ),
        ],
    );
    assert!(report.ensure_clean().is_err());
}
