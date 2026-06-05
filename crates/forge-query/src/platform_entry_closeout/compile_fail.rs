use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use crate::identity::hash_parts;
use crate::public_doc_coverage::forge_query_public_doc_coverage_golden_transcripts;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryPlatformEntryUiProofKind {
    Golden,
    CompileFailBoundary,
}

impl ForgeQueryPlatformEntryUiProofKind {
    fn as_str(self) -> &'static str {
        match self {
            Self::Golden => "golden",
            Self::CompileFailBoundary => "compile_fail_boundary",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ForgeQueryPlatformEntryUiProofRow {
    label: &'static str,
    path: &'static str,
    kind: ForgeQueryPlatformEntryUiProofKind,
}

impl ForgeQueryPlatformEntryUiProofRow {
    const fn new(
        label: &'static str,
        path: &'static str,
        kind: ForgeQueryPlatformEntryUiProofKind,
    ) -> Self {
        Self { label, path, kind }
    }

    pub fn label(&self) -> &'static str {
        self.label
    }

    pub fn path(&self) -> &'static str {
        self.path
    }

    pub fn kind(&self) -> ForgeQueryPlatformEntryUiProofKind {
        self.kind
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryPlatformEntryCompileFailManifest {
    rows: Vec<ForgeQueryPlatformEntryUiProofRow>,
    manifest_digest: String,
    boundary_digest: String,
}

impl ForgeQueryPlatformEntryCompileFailManifest {
    fn new(rows: Vec<ForgeQueryPlatformEntryUiProofRow>) -> Self {
        let manifest_digest = hash_parts(
            &rows
                .iter()
                .map(|row| format!("{}|{}|{}", row.label(), row.path(), row.kind().as_str()))
                .collect::<Vec<_>>(),
        );
        let boundary_digest = hash_parts(
            &rows
                .iter()
                .filter(|row| row.kind() == ForgeQueryPlatformEntryUiProofKind::CompileFailBoundary)
                .map(|row| format!("{}|{}", row.label(), row.path()))
                .collect::<Vec<_>>(),
        );
        Self {
            rows,
            manifest_digest,
            boundary_digest,
        }
    }

    pub fn rows(&self) -> &[ForgeQueryPlatformEntryUiProofRow] {
        &self.rows
    }

    pub fn manifest_digest(&self) -> &str {
        &self.manifest_digest
    }

    pub fn boundary_digest(&self) -> &str {
        &self.boundary_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryPlatformEntryCompileFailAudit {
    manifest_digest: String,
    missing_surfaces: Vec<String>,
    missing_paths: Vec<String>,
    orphan_rows: Vec<String>,
}

impl ForgeQueryPlatformEntryCompileFailAudit {
    pub fn current() -> Self {
        Self::from_manifest(&forge_query_platform_entry_compile_fail_manifest())
    }

    pub fn from_manifest(manifest: &ForgeQueryPlatformEntryCompileFailManifest) -> Self {
        let expected = manifest
            .rows()
            .iter()
            .map(|row| row.path().to_string())
            .collect::<BTreeSet<_>>();
        let actual = manifest_paths();
        let mut missing_surfaces = required_ui_proof_labels()
            .into_iter()
            .filter(|label| !manifest.rows().iter().any(|row| row.label() == *label))
            .map(str::to_string)
            .collect::<Vec<_>>();
        let mut missing_paths = manifest
            .rows()
            .iter()
            .filter(|row| !crate_root().join(row.path()).is_file())
            .map(|row| row.path().to_string())
            .collect::<Vec<_>>();
        let mut orphan_rows = actual.difference(&expected).cloned().collect::<Vec<_>>();
        missing_surfaces.sort();
        missing_paths.sort();
        orphan_rows.sort();
        Self {
            manifest_digest: manifest.manifest_digest().to_string(),
            missing_surfaces,
            missing_paths,
            orphan_rows,
        }
    }

    pub fn manifest_digest(&self) -> &str {
        &self.manifest_digest
    }

    pub fn missing_surfaces(&self) -> &[String] {
        &self.missing_surfaces
    }

    pub fn missing_paths(&self) -> &[String] {
        &self.missing_paths
    }

    pub fn orphan_rows(&self) -> &[String] {
        &self.orphan_rows
    }
}

const REQUIRED_HANDLE_BASE_GOLDENS: &[&str] = &[
    "checked_configured_handle",
    "ordinary_configured_handle",
    "proof_configured_handle",
    "configured_handle_real_example",
];

const REQUIRED_BOUNDARY_LABELS: &[&str] = &[
    "configured_handle_rejects_bool_shortcut_context",
    "family_helper_rejects_non_geometry_family",
    "configured_handle_constructors_private",
    "continuation_pipeline_artifact_constructors_private",
    "signal_compatibility_orchestration_artifact_constructors_private",
    "contribution_composed_orchestration_artifact_constructors_private",
];

fn required_ui_proof_labels() -> BTreeSet<&'static str> {
    REQUIRED_HANDLE_BASE_GOLDENS
        .iter()
        .copied()
        .chain(
            forge_query_public_doc_coverage_golden_transcripts()
                .iter()
                .map(|row| row.label()),
        )
        .chain(REQUIRED_BOUNDARY_LABELS.iter().copied())
        .collect()
}

pub fn forge_query_platform_entry_compile_fail_manifest(
) -> ForgeQueryPlatformEntryCompileFailManifest {
    use ForgeQueryPlatformEntryUiProofKind::{CompileFailBoundary as B, Golden as G};

    ForgeQueryPlatformEntryCompileFailManifest::new(vec![
        ForgeQueryPlatformEntryUiProofRow::new("checked_configured_handle", "tests/ui/domain_handle/golden/checked_configured_handle_compiles.rs", G),
        ForgeQueryPlatformEntryUiProofRow::new("admitted_world_basis_surface_readout", "tests/ui/domain_handle/golden/admitted_world_basis_surface_readout_compiles.rs", G),
        ForgeQueryPlatformEntryUiProofRow::new("configured_handle_real_example", "tests/ui/domain_handle/golden/configured_handle_real_example_compiles.rs", G),
        ForgeQueryPlatformEntryUiProofRow::new("continuation_pipeline_surface_readout", "tests/ui/domain_handle/golden/continuation_pipeline_surface_readout_compiles.rs", G),
        ForgeQueryPlatformEntryUiProofRow::new("contribution_composed_surface_readout", "tests/ui/domain_handle/golden/contribution_composed_surface_readout_compiles.rs", G),
        ForgeQueryPlatformEntryUiProofRow::new("declaration_entry_orchestration_surface_readout", "tests/ui/domain_handle/golden/declaration_entry_orchestration_surface_readout_compiles.rs", G),
        ForgeQueryPlatformEntryUiProofRow::new("family_helper_surface_readout", "tests/ui/domain_handle/golden/family_helper_surface_readout_compiles.rs", G),
        ForgeQueryPlatformEntryUiProofRow::new("grouped_authoring_surface_readout", "tests/ui/domain_handle/golden/grouped_authoring_surface_readout_compiles.rs", G),
        ForgeQueryPlatformEntryUiProofRow::new("ordinary_configured_handle", "tests/ui/domain_handle/golden/ordinary_configured_handle_compiles.rs", G),
        ForgeQueryPlatformEntryUiProofRow::new("proof_configured_handle", "tests/ui/domain_handle/golden/proof_configured_handle_compiles.rs", G),
        ForgeQueryPlatformEntryUiProofRow::new("public_doc_coverage_surface_readout", "tests/ui/domain_handle/golden/public_doc_coverage_surface_readout_compiles.rs", G),
        ForgeQueryPlatformEntryUiProofRow::new("recovery_boundary_surface_readout", "tests/ui/domain_handle/golden/recovery_boundary_surface_readout_compiles.rs", G),
        ForgeQueryPlatformEntryUiProofRow::new("signal_compatibility_surface_readout", "tests/ui/domain_handle/golden/signal_compatibility_surface_readout_compiles.rs", G),
        ForgeQueryPlatformEntryUiProofRow::new("configured_handle_rejects_bool_shortcut_context", "tests/ui/domain_handle/boundaries/configured_handle_rejects_bool_shortcut_context.rs", B),
        ForgeQueryPlatformEntryUiProofRow::new("configured_handle_rejects_callback_context", "tests/ui/domain_handle/boundaries/configured_handle_rejects_callback_context.rs", B),
        ForgeQueryPlatformEntryUiProofRow::new("configured_handle_rejects_raw_string_context", "tests/ui/domain_handle/boundaries/configured_handle_rejects_raw_string_context.rs", B),
        ForgeQueryPlatformEntryUiProofRow::new("family_helper_rejects_non_geometry_family", "tests/ui/domain_handle/boundaries/family_helper_rejects_non_geometry_family.rs", B),
        ForgeQueryPlatformEntryUiProofRow::new("unvalidated_draft_cannot_masquerade_as_admitted_handle", "tests/ui/domain_handle/boundaries/unvalidated_draft_cannot_masquerade_as_admitted_handle.rs", B),
        ForgeQueryPlatformEntryUiProofRow::new("binding_context_witness_constructor_private", "tests/ui/domain_handle/boundaries/binding/binding_context_witness_constructor_private.rs", B),
        ForgeQueryPlatformEntryUiProofRow::new("binding_linked_artifacts_constructor_private", "tests/ui/domain_handle/boundaries/binding/binding_linked_artifacts_constructor_private.rs", B),
        ForgeQueryPlatformEntryUiProofRow::new("binding_witness_constructors_private", "tests/ui/domain_handle/boundaries/binding/binding_witness_constructors_private.rs", B),
        ForgeQueryPlatformEntryUiProofRow::new("ordinary_checked_topology_constructors_private", "tests/ui/domain_handle/boundaries/binding/ordinary_checked_topology_constructors_private.rs", B),
        ForgeQueryPlatformEntryUiProofRow::new("admitted_world_basis_constructors_private", "tests/ui/domain_handle/boundaries/construction/admitted_world_basis_constructors_private.rs", B),
        ForgeQueryPlatformEntryUiProofRow::new("configured_handle_constructors_private", "tests/ui/domain_handle/boundaries/construction/configured_handle_constructors_private.rs", B),
        ForgeQueryPlatformEntryUiProofRow::new("continuation_pipeline_artifact_constructors_private", "tests/ui/domain_handle/boundaries/construction/continuation_pipeline_artifact_constructors_private.rs", B),
        ForgeQueryPlatformEntryUiProofRow::new("contribution_composed_orchestration_artifact_constructors_private", "tests/ui/domain_handle/boundaries/construction/contribution_composed_orchestration_artifact_constructors_private.rs", B),
        ForgeQueryPlatformEntryUiProofRow::new("signal_compatibility_orchestration_artifact_constructors_private", "tests/ui/domain_handle/boundaries/construction/signal_compatibility_orchestration_artifact_constructors_private.rs", B),
    ])
}

pub fn forge_query_platform_entry_compile_fail_boundary_digest() -> String {
    forge_query_platform_entry_compile_fail_manifest()
        .boundary_digest()
        .to_string()
}

fn manifest_paths() -> BTreeSet<String> {
    collect_rs_paths(&crate_root().join("tests/ui/domain_handle/golden"))
        .into_iter()
        .chain(collect_rs_paths(
            &crate_root().join("tests/ui/domain_handle/boundaries"),
        ))
        .collect()
}

fn collect_rs_paths(root: &Path) -> BTreeSet<String> {
    let mut results = BTreeSet::new();
    for entry in fs::read_dir(root).expect("ui proof directory should exist") {
        let entry = entry.expect("directory entry should be readable");
        let path = entry.path();
        if path.is_dir() {
            results.extend(collect_rs_paths(&path));
            continue;
        }
        if path.extension().and_then(|ext| ext.to_str()) != Some("rs") {
            continue;
        }
        results.insert(
            path.strip_prefix(crate_root())
                .expect("proof path should live under crate root")
                .to_string_lossy()
                .replace('\\', "/"),
        );
    }
    results
}

fn crate_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manifest_matches_checked_in_domain_handle_ui_suite() {
        let manifest = forge_query_platform_entry_compile_fail_manifest();
        let expected = manifest
            .rows()
            .iter()
            .map(|row| row.path())
            .collect::<BTreeSet<_>>();
        let actual = manifest_paths();

        assert_eq!(expected, actual.iter().map(String::as_str).collect());
        assert_eq!(manifest.rows().len(), 27);
        assert!(!manifest.manifest_digest().is_empty());
        assert!(!manifest.boundary_digest().is_empty());
        for label in required_ui_proof_labels() {
            assert!(manifest.rows().iter().any(|row| row.label() == label));
        }
    }

    #[test]
    fn compile_fail_audit_is_green_for_current_manifest() {
        let audit = ForgeQueryPlatformEntryCompileFailAudit::current();

        assert!(audit.missing_surfaces().is_empty());
        assert!(audit.missing_paths().is_empty());
        assert!(audit.orphan_rows().is_empty());
    }
}
