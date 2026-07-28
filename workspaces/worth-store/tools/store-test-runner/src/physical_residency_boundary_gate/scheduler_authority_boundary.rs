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
    "S7PlacementIoReadinessSeed",
    "BackgroundPacingProgressionEvidence",
    "BackgroundPacingProgressionDrift",
];

const FORBIDDEN_SCHEDULER_PROOF_FRAGMENTS: &[&str] = &[
    "AuthorityMarker",
    "AuthorityWitness",
    "CapabilityMarker",
    "CapabilityWitness",
];

const FORBIDDEN_PLACEMENT_READINESS_FRAGMENTS: &[&str] = &[
    "worth_store_io_scheduler",
    "IoScheduler",
    "BackgroundPacing",
    "SchedulerReadiness",
    "scheduler_readiness",
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
        (
            "crates/worth-store-physical-isolation/Cargo.toml",
            &["worth-store-io-scheduler"][..],
        ),
    ] {
        let path = workspace_root().join(relative);
        let manifest = read(&path).expect("read authority-boundary manifest");
        inspect_manifest(&path, &manifest, forbidden).unwrap_or_else(|denial| panic!("{denial}"));
    }
}

#[test]
fn scheduler_authority_gate_rejects_deleted_identifier_mutants() {
    for identifier in FORBIDDEN_AUTHORITY_IDENTIFIERS {
        let mutant = format!("pub struct {identifier};");
        let denial = inspect_source(Path::new("crates/mutant/src/lib.rs"), &mutant)
            .expect_err("deleted scheduler authority must be denied");
        assert!(denial.contains(identifier), "wrong denial: {denial}");
    }
}

#[test]
fn scheduler_authority_gate_rejects_renamed_background_authority() {
    let renamed_background_authority = r#"
use worth_proof::{AuthorityMarker, AuthorityWitness};

pub struct RenamedBackgroundPacingAuthority;
impl AuthorityMarker for RenamedBackgroundPacingAuthority {}

pub fn mint_from_policy() -> AuthorityWitness<RenamedBackgroundPacingAuthority> {
    AuthorityWitness::from_authority_marker(RenamedBackgroundPacingAuthority)
}
"#;
    let denial = inspect_source(
        Path::new("crates/worth-store-io-scheduler/src/background_pacing/renamed_authority.rs"),
        renamed_background_authority,
    )
    .expect_err("renamed background-pacing proof authority must be denied");
    assert!(
        denial.contains("scheduler policy or execution"),
        "wrong denial: {denial}"
    );
}

#[test]
fn scheduler_authority_gate_rejects_renamed_queue_execution_authority() {
    let renamed_queue_execution_authority = r#"
use worth_proof::{AuthorityMarker, AuthorityWitness};

pub struct RenamedQueueExecutionAuthority;
impl AuthorityMarker for RenamedQueueExecutionAuthority {}

pub fn mint_from_admission() -> AuthorityWitness<RenamedQueueExecutionAuthority> {
    AuthorityWitness::from_authority_marker(RenamedQueueExecutionAuthority)
}
"#;
    let denial = inspect_source(
        Path::new("crates/worth-store-io-scheduler/src/queue_execution/renamed_authority.rs"),
        renamed_queue_execution_authority,
    )
    .expect_err("renamed queue-execution proof authority must be denied");
    assert!(
        denial.contains("scheduler policy or execution"),
        "wrong denial: {denial}"
    );
}

#[test]
fn scheduler_authority_gate_allows_concrete_backend_witnesses() {
    let concrete_backend_witness = r#"
use worth_store_physical_backend::AdmittedBackendCapabilityWitness;

pub fn inspect_backend(_: &AdmittedBackendCapabilityWitness) {}
"#;
    inspect_source(
        Path::new("crates/worth-store-io-scheduler/src/backend_capability/admission.rs"),
        concrete_backend_witness,
    )
    .expect("concrete platform witness names must not match generic proof identifiers");
}

#[test]
fn scheduler_authority_gate_rejects_placement_readiness_mutants() {
    let renamed_tiering_readiness = "pub struct PlacementExecutionPermit;";
    let denial = inspect_source(
        Path::new("crates/worth-store-tiering/src/io_readiness/placement_permit.rs"),
        renamed_tiering_readiness,
    )
    .expect_err("renamed tiering readiness module must be denied");
    assert!(
        denial.contains("tier placement readiness module"),
        "wrong denial: {denial}"
    );

    let renamed_blob_readiness = r#"
use worth_store_io_scheduler::BackgroundPacingAdmissionBasis as PlacementExecutionPermit;

pub enum BlobPlacementIntent {
    Inline { permit: PlacementExecutionPermit },
}
"#;
    let denial = inspect_source(
        Path::new(
            "crates/worth-store-blob-chunks/src/placement/admission/renamed_placement_permit.rs",
        ),
        renamed_blob_readiness,
    )
    .expect_err("renamed blob placement scheduler readiness must be denied");
    assert!(
        denial.contains("placement admission contains scheduler readiness"),
        "wrong denial: {denial}"
    );
}

#[test]
fn scheduler_authority_gate_rejects_dependency_mutants() {
    for (manifest, dependency) in [
        (
            "crates/worth-store-io-scheduler/Cargo.toml",
            "worth-store-physical-isolation",
        ),
        (
            "crates/worth-store-io-scheduler/Cargo.toml",
            "worth-store-recovery-physics",
        ),
        (
            "crates/worth-store-physical-isolation/Cargo.toml",
            "worth-store-io-scheduler",
        ),
        (
            "crates/worth-store-tiering/Cargo.toml",
            "worth-store-io-scheduler",
        ),
        (
            "crates/worth-store-tiering/Cargo.toml",
            "worth-store-physical-isolation",
        ),
    ] {
        let mutant = format!("[dependencies]\n{dependency}.workspace = true\n");
        inspect_manifest(Path::new(manifest), &mutant, &[dependency])
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

    let relative = workspace_relative(path).replace('\\', "/");
    if relative.starts_with("crates/worth-store-tiering/src/io_readiness/") {
        return Err(format!(
            "Phase 8 scheduler-authority boundary: deleted tier placement readiness module appears at {relative}"
        ));
    }

    if relative.starts_with("crates/worth-store-tiering/src/")
        || relative.starts_with("crates/worth-store-blob-chunks/src/placement/admission/")
    {
        for fragment in FORBIDDEN_PLACEMENT_READINESS_FRAGMENTS {
            if source.contains(fragment) {
                return Err(format!(
                    "Phase 8 scheduler-authority boundary: placement admission contains scheduler readiness fragment `{fragment}` at {relative}"
                ));
            }
        }
    }

    if relative.starts_with("crates/worth-store-io-scheduler/src/") {
        for fragment in FORBIDDEN_SCHEDULER_PROOF_FRAGMENTS {
            if contains_rust_identifier(source, fragment) {
                return Err(format!(
                    "Phase 8 scheduler-authority boundary: scheduler policy or execution contains forbidden proof-authority fragment `{fragment}` at {relative}"
                ));
            }
        }
    }

    Ok(())
}

fn contains_rust_identifier(source: &str, identifier: &str) -> bool {
    source.match_indices(identifier).any(|(start, _)| {
        let end = start + identifier.len();
        let before_is_identifier = start
            .checked_sub(1)
            .and_then(|index| source.as_bytes().get(index))
            .is_some_and(|byte| byte.is_ascii_alphanumeric() || *byte == b'_');
        let after_is_identifier = source
            .as_bytes()
            .get(end)
            .is_some_and(|byte| byte.is_ascii_alphanumeric() || *byte == b'_');
        !before_is_identifier && !after_is_identifier
    })
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
