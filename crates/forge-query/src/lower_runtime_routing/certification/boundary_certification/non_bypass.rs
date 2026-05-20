use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use crate::identity::hash_parts;

use super::public_surface::{
    forge_query_lower_runtime_public_surface_inventory, ForgeQueryLowerRuntimePublicSurfaceKind,
};
use crate::lower_runtime_routing::forge_query_lower_runtime_direct_import_audit;

const LOWER_RUNTIME_IMPORT_MARKERS: &[&str] = &[
    "forge_runtime_bridge::facade",
    "forge_relational::facade",
    "forge_signal::facade",
];

const COMPILE_FAIL_TARGETS: &[&str] = &[
    "tests/ui/lower_runtime_routing/inventory/crossing_row_constructor_private.rs",
    "tests/ui/lower_runtime_routing/inventory/crossing_inventory_constructor_private.rs",
    "tests/ui/lower_runtime_routing/gaps/gap_registry_row_constructor_private.rs",
    "tests/ui/lower_runtime_routing/audit/audit_row_constructor_private.rs",
    "tests/ui/lower_runtime_routing/closeout/closeout_row_constructor_private.rs",
    "tests/ui/lower_runtime_routing/closeout/closeout_registry_constructor_private.rs",
    "tests/ui/lower_runtime_routing/envelopes/boundary_envelope_constructor_private.rs",
    "tests/ui/lower_runtime_routing/protocol/capability_request_constructor_private.rs",
    "tests/ui/lower_runtime_routing/protocol/route_plan_constructor_private.rs",
    "tests/ui/lower_runtime_routing/protocol/boundary_execution_receipt_constructor_private.rs",
    "tests/ui/lower_runtime_routing/certification/public_surface/public_surface_row_constructor_private.rs",
    "tests/ui/lower_runtime_routing/certification/public_surface/public_surface_inventory_constructor_private.rs",
    "tests/ui/lower_runtime_routing/certification/non_bypass/non_bypass_audit_constructor_private.rs",
    "tests/ui/lower_runtime_routing/certification/phase_manifest/phase_manifest_row_constructor_private.rs",
    "tests/ui/lower_runtime_routing/certification/phase_manifest/phase_manifest_constructor_private.rs",
    "tests/ui/lower_runtime_routing/certification/bundle/certification_row_constructor_private.rs",
    "tests/ui/lower_runtime_routing/certification/bundle/certification_bundle_constructor_private.rs",
    "tests/ui/lower_runtime_routing/certification/acceptance/acceptance_suite_constructor_private.rs",
    "tests/ui/lower_runtime_routing/certification/reconciliation/boundary_reconciliation_report_constructor_private.rs",
    "tests/ui/lower_runtime_routing/certification/closeout/closeout_report_constructor_private.rs",
    "tests/ui/lower_runtime_routing/certification/closeout/closure_test_constructor_private.rs",
    "tests/ui/lower_runtime_routing/certification/proof_shape/proof_shape_audit_constructor_private.rs",
    "tests/ui/lower_runtime_routing/certification/synthetic_tail/synthetic_tail_report_constructor_private.rs",
];

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryLowerRuntimeNonBypassAudit {
    route_public_surface_digest: String,
    route_non_bypass_digest: String,
    compile_fail_boundary_digest: String,
    checked_file_count: usize,
}

impl ForgeQueryLowerRuntimeNonBypassAudit {
    pub(crate) fn new(
        route_public_surface_digest: String,
        route_non_bypass_digest: String,
        compile_fail_boundary_digest: String,
        checked_file_count: usize,
    ) -> Self {
        Self {
            route_public_surface_digest,
            route_non_bypass_digest,
            compile_fail_boundary_digest,
            checked_file_count,
        }
    }

    pub fn route_public_surface_digest(&self) -> &str {
        &self.route_public_surface_digest
    }

    pub fn route_non_bypass_digest(&self) -> &str {
        &self.route_non_bypass_digest
    }

    pub fn compile_fail_boundary_digest(&self) -> &str {
        &self.compile_fail_boundary_digest
    }

    pub fn checked_file_count(&self) -> usize {
        self.checked_file_count
    }
}

pub fn certify_lower_runtime_non_bypass() -> Result<ForgeQueryLowerRuntimeNonBypassAudit, String> {
    let inventory = forge_query_lower_runtime_public_surface_inventory();
    let workspace_root = workspace_root()?;
    let mut violations = Vec::new();
    let mut checked_files = 0usize;

    for (relative, allow_imports) in routed_surface_scan_targets() {
        checked_files +=
            scan_surface_target(&workspace_root, relative, allow_imports, &mut violations)?;
    }

    checked_files += scan_tree(
        &workspace_root,
        "crates/worth-topo/src/projection",
        Some("crates/worth-topo/src/projection/runtime_boundary"),
        &mut violations,
    )?;

    for row in forge_query_lower_runtime_direct_import_audit().rows() {
        if row.seam_key().as_str() == "downstream-query-runtime-boundary-subtree" {
            continue;
        }
        checked_files +=
            scan_allowed_query_boundary(&workspace_root, row.module_path(), &mut violations)?;
    }

    if !violations.is_empty() {
        return Err(violations.join("\n"));
    }

    let route_public_surface_digest = inventory.public_surface_digest();
    let compile_fail_boundary_digest = compile_fail_boundary_digest();
    let route_non_bypass_digest = hash_parts(&[
        route_public_surface_digest.clone(),
        compile_fail_boundary_digest.clone(),
        checked_files.to_string(),
        "query-runtime-routed-surfaces:no-lower-runtime-imports".to_string(),
        "worth-topo-projection:imports-confined-to-runtime-boundary".to_string(),
    ]);
    Ok(ForgeQueryLowerRuntimeNonBypassAudit::new(
        route_public_surface_digest,
        route_non_bypass_digest,
        compile_fail_boundary_digest,
        checked_files,
    ))
}

pub fn forge_query_lower_runtime_compile_fail_boundary_digest() -> String {
    compile_fail_boundary_digest()
}

pub fn forge_query_lower_runtime_compile_fail_boundary_target_count() -> usize {
    COMPILE_FAIL_TARGETS.len()
}

fn routed_surface_scan_targets() -> Vec<(&'static str, bool)> {
    let mut seen = BTreeMap::new();
    for row in forge_query_lower_runtime_public_surface_inventory().rows() {
        if row.surface_kind() == ForgeQueryLowerRuntimePublicSurfaceKind::DownstreamRuntimeBoundary
        {
            continue;
        }
        let allow_imports = matches!(
            row.surface_kind(),
            ForgeQueryLowerRuntimePublicSurfaceKind::AllowedBoundaryAdapter
                | ForgeQueryLowerRuntimePublicSurfaceKind::RuntimeBackendBoundary
        );
        seen.entry(row.implementation_path())
            .and_modify(|existing| *existing |= allow_imports)
            .or_insert(allow_imports);
    }
    seen.into_iter().collect()
}

fn workspace_root() -> Result<PathBuf, String> {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest_dir
        .parent()
        .and_then(Path::parent)
        .map(Path::to_path_buf)
        .ok_or_else(|| "could not derive workspace root from CARGO_MANIFEST_DIR".to_string())
}

fn compile_fail_boundary_digest() -> String {
    hash_parts(
        &COMPILE_FAIL_TARGETS
            .iter()
            .map(|target| target.to_string())
            .collect::<Vec<_>>(),
    )
}

fn scan_allowed_query_boundary(
    workspace_root: &Path,
    relative: &str,
    violations: &mut Vec<String>,
) -> Result<usize, String> {
    if relative.ends_with("/*") {
        return scan_tree(
            workspace_root,
            relative.trim_end_matches("/*"),
            None,
            violations,
        );
    }
    scan_surface_file(workspace_root, relative, true, violations)
}

fn scan_tree(
    workspace_root: &Path,
    relative_root: &str,
    allowed_prefix: Option<&str>,
    violations: &mut Vec<String>,
) -> Result<usize, String> {
    let root = workspace_root.join(relative_root);
    let mut count = 0usize;
    for path in rust_files_under(&root)? {
        count += 1;
        let relative = slash_path(
            path.strip_prefix(workspace_root)
                .map_err(|error| error.to_string())?,
        );
        let allow_imports = allowed_prefix
            .map(|prefix| relative.starts_with(prefix))
            .unwrap_or(true);
        scan_file_contents(&path, &relative, allow_imports, violations)?;
    }
    Ok(count)
}

fn scan_surface_file(
    workspace_root: &Path,
    relative: &str,
    allow_imports: bool,
    violations: &mut Vec<String>,
) -> Result<usize, String> {
    let path = workspace_root.join(relative);
    scan_file_contents(&path, relative, allow_imports, violations)?;
    Ok(1)
}

fn scan_surface_target(
    workspace_root: &Path,
    relative: &str,
    allow_imports: bool,
    violations: &mut Vec<String>,
) -> Result<usize, String> {
    if relative.ends_with("/*") {
        return scan_tree(
            workspace_root,
            relative.trim_end_matches("/*"),
            None,
            violations,
        );
    }
    scan_surface_file(workspace_root, relative, allow_imports, violations)
}

fn scan_file_contents(
    path: &Path,
    relative: &str,
    allow_imports: bool,
    violations: &mut Vec<String>,
) -> Result<(), String> {
    let source = fs::read_to_string(path)
        .map_err(|error| format!("failed to read `{}`: {error}", path.display()))?;
    for (line_number, line) in source.lines().enumerate() {
        if let Some(marker) = LOWER_RUNTIME_IMPORT_MARKERS
            .iter()
            .find(|marker| line.contains(**marker))
        {
            if !allow_imports {
                violations.push(format!(
                    "{relative}:{} imports `{marker}` outside the declared routed boundary",
                    line_number + 1
                ));
            }
        }
    }
    Ok(())
}

fn rust_files_under(root: &Path) -> Result<Vec<PathBuf>, String> {
    let mut files = Vec::new();
    for entry in fs::read_dir(root)
        .map_err(|error| format!("failed to read `{}`: {error}", root.display()))?
    {
        let entry = entry.map_err(|error| error.to_string())?;
        let path = entry.path();
        if path.is_dir() {
            files.extend(rust_files_under(&path)?);
        } else if path.extension().and_then(|value| value.to_str()) == Some("rs") {
            files.push(path);
        }
    }
    Ok(files)
}

fn slash_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

#[cfg(test)]
mod non_bypass_tests;
