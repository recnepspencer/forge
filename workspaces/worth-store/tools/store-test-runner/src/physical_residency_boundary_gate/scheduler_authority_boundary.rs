use std::path::Path;

use super::workspace_source::{read, rust_sources, workspace_relative};
use crate::workspace_root;

const FORBIDDEN_AUTHORITY_IDENTIFIERS: &[&str] = &[
    "SchedulerIsolationCapability",
    "SchedulerIsolationProof",
    "SchedulerIsolationPublication",
    "IoSchedulerIsolationAdmission",
    "IoSchedulerIsolationCounterSnapshot",
    "TierPlacementIoAdmission",
    "admit_tier_placement_io",
    "BackgroundPacingProgressionEvidence",
    "BackgroundPacingProgressionDrift",
];

#[test]
fn crates_have_no_scheduler_authority_derived_from_physical_isolation() {
    for source in rust_sources(&workspace_root().join("crates"))
        .expect("discover Store workspace crate sources")
    {
        let text = read(&source).expect("read Store workspace source");
        inspect_source(&source, &text).unwrap_or_else(|denial| panic!("{denial}"));
    }
}

#[test]
fn scheduler_and_tiering_manifests_preserve_one_way_authority() {
    for (relative, forbidden) in [
        (
            "crates/worth-store-io-scheduler/Cargo.toml",
            &[
                "worth-store-physical-isolation",
                "worth-store-recovery-physics",
            ][..],
        ),
        (
            "crates/worth-store-tiering/Cargo.toml",
            &["worth-store-io-scheduler", "worth-store-physical-isolation"][..],
        ),
    ] {
        let path = workspace_root().join(relative);
        let manifest = read(&path).expect("read authority-boundary manifest");
        inspect_manifest(&path, &manifest, forbidden).unwrap_or_else(|denial| panic!("{denial}"));
    }
}

#[test]
fn scheduler_authority_gate_rejects_publication_and_dependency_mutants() {
    for identifier in FORBIDDEN_AUTHORITY_IDENTIFIERS {
        let mutant = format!("pub struct {identifier};");
        let denial = inspect_source(Path::new("crates/mutant/src/lib.rs"), &mutant)
            .expect_err("deleted scheduler authority must be denied");
        assert!(denial.contains(identifier), "wrong denial: {denial}");
    }

    for dependency in [
        "worth-store-physical-isolation",
        "worth-store-recovery-physics",
        "worth-store-io-scheduler",
    ] {
        let mutant = format!("[dependencies]\n{dependency}.workspace = true\n");
        inspect_manifest(Path::new("Cargo.toml"), &mutant, &[dependency])
            .expect_err("forbidden authority edge must be denied");
    }
}

fn inspect_source(path: &Path, source: &str) -> Result<(), String> {
    for identifier in FORBIDDEN_AUTHORITY_IDENTIFIERS {
        if source.contains(identifier) {
            return Err(format!(
                "Phase 8 scheduler-authority boundary: deleted `{identifier}` appears at {}",
                workspace_relative(path)
            ));
        }
    }
    Ok(())
}

fn inspect_manifest(path: &Path, manifest: &str, forbidden: &[&str]) -> Result<(), String> {
    for dependency in forbidden {
        if manifest.lines().any(|line| line.contains(dependency)) {
            return Err(format!(
                "Phase 8 scheduler-authority boundary: forbidden dependency `{dependency}` appears at {}",
                workspace_relative(path)
            ));
        }
    }
    Ok(())
}
