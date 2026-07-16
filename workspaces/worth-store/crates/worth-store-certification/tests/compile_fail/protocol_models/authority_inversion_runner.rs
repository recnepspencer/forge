use std::path::{Path, PathBuf};
use std::process::Command;

#[test]
fn diagnostic_model_surfaces_cannot_open_production_authority_doors() {
    for (binary, expected_types) in [
        (
            "model_action_as_publication_authority",
            ["ImportPublicationAction", "ImportPublicationReadiness"],
        ),
        (
            "owner_observation_as_publication_authority",
            [
                "CompactionOwnerCaseObservation",
                "ImportPublicationReadiness",
            ],
        ),
        (
            "binding_manifest_as_publication_authority",
            ["ProtocolBindingManifest", "ImportPublicationReadiness"],
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
        for expected in expected_types {
            assert!(
                stderr.contains(expected),
                "{binary} failed for the wrong reason; missing {expected}:\n{stderr}"
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
        .join("tests/compile_fail/protocol_models/cases/authority_inversion")
}

fn target_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/protocol-model-authority-compile-fail")
}
