use std::fs;
use std::path::Path;

use worth_store_formal_models::runner::PinnedTlcToolchain;

#[test]
fn pinned_toolchain_manifest_matches_the_compiled_invocation_contract() {
    let crate_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let manifest = fs::read_to_string(crate_root.join("formal-toolchain.toml"))
        .expect("formal toolchain manifest is committed");

    assert!(manifest.contains(&format!("version = \"{}\"", PinnedTlcToolchain::VERSION)));
    assert!(manifest.contains(&format!(
        "download_url = \"{}\"",
        PinnedTlcToolchain::DOWNLOAD_URL
    )));
    assert!(manifest.contains(&format!("sha256 = \"{}\"", PinnedTlcToolchain::SHA256)));
}

#[test]
fn toolchain_smoke_artifacts_are_real_checked_inputs() {
    let crate_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let model = crate_root.join("src/runner/toolchain_smoke/ToolchainSmoke.tla");
    let configuration = crate_root.join("src/runner/toolchain_smoke/ToolchainSmoke.cfg");
    let model_text = fs::read_to_string(model).expect("smoke model is committed");
    let configuration_text =
        fs::read_to_string(configuration).expect("smoke configuration is committed");

    assert!(model_text.contains("TypeInvariant"));
    assert!(model_text.contains("Spec =="));
    assert!(configuration_text.contains("SPECIFICATION Spec"));
    assert!(configuration_text.contains("INVARIANT TypeInvariant"));
}

#[test]
fn toolchain_smoke_keeps_checker_state_out_of_source() {
    let crate_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let repo_root = crate_root.join("../../../..");
    for script in [
        "scripts/ci/verify_worth_store_formal_toolchain.ps1",
        "scripts/ci/verify_worth_store_formal_toolchain.sh",
    ] {
        let script = fs::read_to_string(repo_root.join(script)).expect("smoke script is committed");
        assert!(script.contains("-metadir"));
        assert!(script.contains("states/"));
        assert!(script.contains("toolchain-smoke"));
        assert!(script.contains("src/protocols"));
        assert!(script.contains("*.tla"));
        assert!(script.contains("worth-store-certification"));
        assert!(script.contains("worth_store_protocol_closeout"));
        assert!(!script.contains("worth_store_mutant_check"));
        assert!(!script.contains("worth_store_protocol_check"));
    }
    assert!(!crate_root
        .join("src/runner/toolchain_smoke/states")
        .exists());
}

#[test]
fn bounded_checker_verdict_requires_separate_trace_adjudication() {
    let crate_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let verdict = fs::read_to_string(crate_root.join("src/runner/verdict.rs"))
        .expect("runner verdict vocabulary exists");
    let adjudication = fs::read_to_string(crate_root.join("src/runner/adjudication.rs"))
        .expect("trace adjudication vocabulary exists");
    let localization =
        fs::read_to_string(crate_root.join("src/protocols/compaction_visibility/localization.rs"))
            .expect("counterexample localization vocabulary exists");

    assert!(verdict.contains("CheckedWithinBounds"));
    assert!(!verdict.contains("LegalProtocolExecution"));
    assert!(adjudication.contains("LegalProtocolExecution"));
    assert!(adjudication.contains("adjudicate_shared_frontier_trace"));
    assert!(localization.contains("CompactionVisibilityAbstractionFunction"));
    assert!(!localization.contains("abstraction_function: &'static str"));
}

#[test]
fn placeholder_and_public_facade_dependencies_are_removed() {
    let crate_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let lib = fs::read_to_string(crate_root.join("src/lib.rs")).expect("crate facade exists");
    let readme = fs::read_to_string(crate_root.join("README.md")).expect("crate readme exists");
    let public_store_manifest = fs::read_to_string(crate_root.join("../worth-store/Cargo.toml"))
        .expect("public Store manifest exists");

    assert!(!lib.contains("ModeledStateMachine"));
    assert!(!readme.contains("Roadmap 2 S.9"));
    assert!(!public_store_manifest.contains("worth-store-formal-models"));
}

#[test]
fn dependency_direction_points_from_certification_to_formal_models_to_owners() {
    let crate_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let certification =
        fs::read_to_string(crate_root.join("../worth-store-certification/Cargo.toml"))
            .expect("certification manifest exists");
    let physical_certification =
        fs::read_to_string(crate_root.join("../worth-store-physical-certification/Cargo.toml"))
            .expect("physical certification manifest exists");
    assert!(certification.contains("worth-store-formal-models.workspace = true"));
    assert!(!physical_certification.contains("worth-store-formal-models"));

    for owner in [
        "worth-store-layout-indexes",
        "worth-store-lsm-authority",
        "worth-store-operations",
        "worth-store-physical-backend",
        "worth-store-physical-integrity",
        "worth-store-physical-isolation",
        "worth-store-recovery-physics",
        "worth-store-security",
        "worth-store-wal",
    ] {
        let manifest = fs::read_to_string(crate_root.join(format!("../{owner}/Cargo.toml")))
            .expect("runtime owner manifest exists");
        assert!(
            !manifest.contains("worth-store-formal-models"),
            "runtime owner {owner} points back into formal models"
        );
    }
}

#[test]
fn phase_topology_has_named_responsibilities_without_placeholder_models() {
    let source = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    for path in [
        "protocol_bindings/manifest/mod.rs",
        "protocol_bindings/manifest/protocol_manifest.rs",
        "protocol_bindings/manifest/current.rs",
        "protocol_bindings/evidence_class.rs",
        "protocol_bindings/compaction_visibility/completeness.rs",
        "protocol_bindings/capability_gap.rs",
        "assumptions/backend.rs",
        "assumptions/atomicity.rs",
        "assumptions/clock.rs",
        "runner/invocation.rs",
        "runner/verdict.rs",
        "runner/bounds.rs",
        "runner/counterexample.rs",
        "runner/canonical_trace.rs",
        "runner/execution.rs",
        "runner/localization.rs",
        "runner/output.rs",
        "runner/receipt_loss.rs",
        "runner/statistics.rs",
        "protocols/compaction_visibility/localization.rs",
        "protocols/compaction_visibility/physical_mapping.rs",
    ] {
        assert!(source.join(path).is_file(), "missing responsibility {path}");
    }

    let certification = source.join(
        "../../worth-store-certification/src/courtroom/protocol_models/compaction_visibility",
    );
    for path in [
        "adjudication.rs",
        "evidence.rs",
        "scenarios/ordinary_owner_execution.rs",
        "mutants/omitted_mapping.rs",
    ] {
        assert!(
            certification.join(path).is_file(),
            "missing certification responsibility {path}"
        );
    }
}
