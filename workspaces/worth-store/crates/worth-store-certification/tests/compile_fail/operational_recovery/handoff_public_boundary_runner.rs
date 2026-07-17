use std::path::{Path, PathBuf};
use std::process::Command;

#[test]
fn later_milestone_consumers_compile_but_cannot_reinterpret_handoffs_as_authority() {
    for binary in ["s11_public_consumer", "s12_public_consumer"] {
        let output = cargo_check(binary);
        assert!(
            output.status.success(),
            "{binary} public-facade consumer did not compile:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    for (binary, handoff, authority) in [
        (
            "s11_handoff_cannot_mint_current_authority",
            "S11StructuredAuditHardeningHandoff",
            "StoreCurrentAuthorityWitness",
        ),
        (
            "s12_handoff_cannot_mint_control_state",
            "S12PhysicalQualificationHandoff",
            "SelectedOperationalControlState",
        ),
        (
            "s11_handoff_fields_are_not_reinterpretable",
            "S11StructuredAuditHardeningHandoff",
            "private",
        ),
        (
            "s12_handoff_fields_are_not_reinterpretable",
            "S12PhysicalQualificationHandoff",
            "private",
        ),
    ] {
        let output = cargo_check(binary);
        assert!(!output.status.success(), "{binary} unexpectedly compiled");
        let stderr = String::from_utf8_lossy(&output.stderr);
        for expected in [handoff, authority] {
            assert!(
                stderr.contains(expected),
                "{binary} failed at the wrong boundary; missing {expected}:\n{stderr}"
            );
        }
        assert!(!stderr.contains("failed to load manifest"), "{stderr}");
        assert!(!stderr.contains("can't find crate"), "{stderr}");
    }
}

fn cargo_check(binary: &str) -> std::process::Output {
    Command::new(env!("CARGO"))
        .args(["check", "--quiet", "--bin", binary])
        .current_dir(case_root())
        .env("CARGO_TARGET_DIR", target_root())
        .output()
        .expect("handoff boundary fixture invokes cargo")
}

fn case_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/compile_fail/operational_recovery/cases/handoff_public_boundary")
}

fn target_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/s10-handoff-public-boundary")
}
