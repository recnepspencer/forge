use std::path::{Path, PathBuf};
use std::process::Command;

#[test]
fn descriptive_shared_artifacts_cannot_reenter_store_authority() {
    for (binary, source, authority) in [
        (
            "shared_audit_record_cannot_construct_control_record",
            "OperationalAuditRecord",
            "OperationalControlRecord",
        ),
        (
            "terminal_export_cannot_construct_authorization",
            "OperationalEvidenceExport",
            "AuthorizedBackupRestorePlan",
        ),
        (
            "support_bundle_cannot_construct_operational_authority",
            "OperationalAuditSupportPayload",
            "ExecutionReadyRepair",
        ),
        (
            "forensic_bundle_cannot_construct_restore_source",
            "ForensicCustodyRecord",
            "ProductionRestoreAdmissibleBackupBundle",
        ),
        (
            "lineage_projection_cannot_mint_primary_serve_lease",
            "ReplicaPromotionReceipt",
            "PrimaryServeLease",
        ),
        (
            "counter_receipt_cannot_construct_execution_ready_plan",
            "OperationalCounterReceipt",
            "ExecutionReadyRepair",
        ),
    ] {
        let output = Command::new(env!("CARGO"))
            .args(["check", "--quiet", "--bin", binary])
            .current_dir(case_root())
            .env("CARGO_TARGET_DIR", target_root())
            .output()
            .expect("compile-fail fixture invokes cargo");
        assert!(!output.status.success(), "{binary} unexpectedly compiled");
        let stderr = String::from_utf8_lossy(&output.stderr);
        for expected in [source, authority] {
            assert!(
                stderr.contains(expected),
                "{binary} failed at the wrong boundary; missing {expected}:\n{stderr}"
            );
        }
        for setup_failure in [
            "failed to load manifest",
            "no matching package",
            "can't find crate",
        ] {
            assert!(
                !stderr.contains(setup_failure),
                "{binary} hit fixture setup failure: {stderr}"
            );
        }
    }
}

fn case_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/compile_fail/operational_recovery/cases/adoption_reverse_flow")
}

fn target_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/s10-adoption-reverse-flow-compile-fail")
}
