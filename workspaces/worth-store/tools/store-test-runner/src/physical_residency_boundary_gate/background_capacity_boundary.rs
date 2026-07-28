use std::path::Path;

use super::workspace_source::{read, rust_sources, workspace_relative};
use crate::workspace_root;

const DELETED_CAPACITY_IDENTIFIERS: &[&str] = &[
    "BackgroundPacingCapability",
    "BackgroundPacingAuthority",
    "BackgroundPacingReady",
    "BackgroundPacingProgressionOutcome",
    "prove_background_pacing_current",
    "from_scheduler_capability",
    "with_pacing_admission",
    "io_readmission_satisfied",
    "admitted_compaction(",
];

const FORBIDDEN_COMPACTION_BYPASS_FRAGMENTS: &[&str] = &[
    "BackgroundPressureDeclaration",
    "BackgroundPacingCapability",
    "from_scheduler_capability",
    "with_pacing_admission",
    "io_readmission_satisfied",
];

#[test]
fn production_has_no_deleted_or_renamed_background_capacity_bypass() {
    for source in rust_sources(&workspace_root().join("crates"))
        .expect("discover Store workspace crate sources")
    {
        let text = read(&source).expect("read Store workspace source");
        inspect_source(&source, &text).unwrap_or_else(|denial| panic!("{denial}"));
    }
}

#[test]
fn scheduler_and_compaction_capacity_types_are_move_owned_and_lease_backed() {
    for (relative, type_name) in [
        (
            "crates/worth-store-io-scheduler/src/background_pacing/lease.rs",
            "BackgroundIdleCapacityLease",
        ),
        (
            "crates/worth-store-blob-chunks/src/compaction/types/pacing.rs",
            "BlobCompactionPacingAdmission",
        ),
        (
            "crates/worth-store-blob-chunks/src/compaction/types/intent.rs",
            "BlobCompactionIntent",
        ),
        (
            "crates/worth-store-blob-chunks/src/compaction/types/rewrite_plan.rs",
            "BlobCompactionRewritePlan",
        ),
    ] {
        let path = workspace_root().join(relative);
        let source = read(&path).expect("read move-owned capacity source");
        reject_clone_or_copy(&path, &source, type_name).unwrap_or_else(|denial| panic!("{denial}"));
    }

    let pacing_path =
        workspace_root().join("crates/worth-store-blob-chunks/src/compaction/types/pacing.rs");
    let pacing = read(&pacing_path).expect("read blob compaction pacing source");
    inspect_compaction_pacing(&pacing_path, &pacing).unwrap_or_else(|denial| panic!("{denial}"));
}

#[test]
fn background_capacity_gate_rejects_exact_and_renamed_bypass_mutants() {
    for identifier in DELETED_CAPACITY_IDENTIFIERS {
        let mutant = format!("pub fn resurrected() {{ let _ = {identifier}; }}");
        let denial = inspect_source(Path::new("crates/mutant/src/lib.rs"), &mutant)
            .expect_err("deleted capacity surface must be denied");
        assert!(denial.contains(identifier), "wrong denial: {denial}");
    }

    let cloneable_lease = r#"
#[derive(Clone, Copy, Debug)]
pub struct BackgroundIdleCapacityLease;
"#;
    let denial = reject_clone_or_copy(
        Path::new("crates/worth-store-io-scheduler/src/background_pacing/lease.rs"),
        cloneable_lease,
        "BackgroundIdleCapacityLease",
    )
    .expect_err("cloneable scheduler lease must be denied");
    assert!(denial.contains("move-owned"), "wrong denial: {denial}");

    let renamed_boolean_pacing = r#"
#[derive(Debug)]
pub(crate) struct BlobCompactionPacingAdmission {
    declaration_admitted: bool,
}

impl BlobCompactionPacingAdmission {
    pub(crate) fn from_declaration() -> Self {
        Self { declaration_admitted: true }
    }
}
"#;
    let pacing_path = Path::new("crates/worth-store-blob-chunks/src/compaction/types/pacing.rs");
    let denial = inspect_compaction_pacing(pacing_path, renamed_boolean_pacing)
        .expect_err("renamed boolean pacing admission must be denied");
    assert!(denial.contains("scheduler lease"), "wrong denial: {denial}");

    let renamed_declaration_bypass = r#"
use worth_store_contracts::BackgroundPressureDeclaration as CompactionExecutionBasis;
pub fn lower_without_scheduler(_: CompactionExecutionBasis) {}
"#;
    let denial = inspect_source(
        Path::new("crates/worth-store-blob-chunks/src/compaction/types/renamed_basis.rs"),
        renamed_declaration_bypass,
    )
    .expect_err("renamed declaration-based self-admission must be denied");
    assert!(
        denial.contains("compaction pacing bypass"),
        "wrong denial: {denial}"
    );
}

fn inspect_source(path: &Path, source: &str) -> Result<(), String> {
    for identifier in DELETED_CAPACITY_IDENTIFIERS {
        if source.contains(identifier) {
            return Err(format!(
                "Phase 8 background-capacity boundary: deleted `{identifier}` appears at {}",
                workspace_relative(path)
            ));
        }
    }

    let relative = workspace_relative(path).replace('\\', "/");
    if relative.starts_with("crates/worth-store-blob-chunks/src/compaction/") {
        for fragment in FORBIDDEN_COMPACTION_BYPASS_FRAGMENTS {
            if source.contains(fragment) {
                return Err(format!(
                    "Phase 8 background-capacity boundary: compaction pacing bypass fragment `{fragment}` appears at {relative}"
                ));
            }
        }
    }
    Ok(())
}

fn inspect_compaction_pacing(path: &Path, source: &str) -> Result<(), String> {
    inspect_source(path, source)?;
    let normalized = source.split_whitespace().collect::<Vec<_>>().join(" ");
    if !normalized.contains(
        "pub(crate) struct BlobCompactionPacingAdmission { lease: BackgroundIdleCapacityLease, }",
    ) {
        return Err(format!(
            "Phase 8 background-capacity boundary: blob compaction pacing must contain only a scheduler lease at {}",
            workspace_relative(path)
        ));
    }
    if !normalized.contains(
        "pub(crate) fn from_scheduler_lease( lease: BackgroundIdleCapacityLease, ) -> Result<Self, BlobCompactionPacingDenial>",
    ) {
        return Err(format!(
            "Phase 8 background-capacity boundary: blob compaction pacing must consume a scheduler lease at {}",
            workspace_relative(path)
        ));
    }
    Ok(())
}

fn reject_clone_or_copy(path: &Path, source: &str, type_name: &str) -> Result<(), String> {
    let marker = format!("struct {type_name}");
    let type_offset = source.find(&marker).ok_or_else(|| {
        format!(
            "Phase 8 background-capacity boundary: `{type_name}` is absent at {}",
            workspace_relative(path)
        )
    })?;
    let before = &source[..type_offset];
    let derive_offset = before.rfind("#[derive(").ok_or_else(|| {
        format!(
            "Phase 8 background-capacity boundary: `{type_name}` has no auditable derive list at {}",
            workspace_relative(path)
        )
    })?;
    let derive = &before[derive_offset..];
    if derive.contains("Clone") || derive.contains("Copy") {
        return Err(format!(
            "Phase 8 background-capacity boundary: `{type_name}` must remain move-owned at {}",
            workspace_relative(path)
        ));
    }
    Ok(())
}
