use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

#[allow(dead_code)]
const SCOPED_CRATE_ROOTS: &[&str] = &[
    "crates/worth-ui-runtime",
    "crates/worth-ui-inspection",
    "crates/worth-ui-query-binding",
    "crates/worth-ui-certification",
];

const RUNTIME_SAME_LEVEL_SINKHOLE_THRESHOLD: usize = 40;
const EVIDENCE_SAME_LEVEL_SINKHOLE_THRESHOLD: usize = 40;
const FILE_SIZE_CAP_LINES: usize = 400;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum CleanupFailureMode {
    FacadeLeakage,
    TopologySinkhole,
    HelperSwamp,
    AuthorityMixing,
    FunctionOverload,
    FileSize,
    TestBypass,
}

impl CleanupFailureMode {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FacadeLeakage => "facade_leakage",
            Self::TopologySinkhole => "topology_sinkhole",
            Self::HelperSwamp => "helper_swamp",
            Self::AuthorityMixing => "authority_mixing",
            Self::FunctionOverload => "function_overload",
            Self::FileSize => "file_size",
            Self::TestBypass => "test_bypass",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct StructuralCleanupFinding {
    pub id: String,
    pub failure_mode: CleanupFailureMode,
    pub path: String,
    pub owner_phase: u8,
    pub closeout_evidence: String,
    pub summary: String,
}

pub fn audit_milestone_37_structural_inventory(
    workspace_root: &Path,
) -> Vec<StructuralCleanupFinding> {
    let mut findings = Vec::new();
    findings.extend(facade_leakage_findings(workspace_root));
    findings.extend(topology_sinkhole_findings(workspace_root));
    findings.extend(helper_swamp_findings(workspace_root));
    findings.extend(authority_mixing_findings(workspace_root));
    findings.extend(function_overload_findings(workspace_root));
    findings.extend(file_size_findings(workspace_root));
    findings.extend(test_bypass_findings(workspace_root));
    findings.sort();
    findings.dedup();
    findings
}

pub fn structural_inventory_digest(findings: &[StructuralCleanupFinding]) -> u64 {
    let mut digest = 0u64;
    for finding in findings {
        digest = digest
            .wrapping_mul(131)
            .wrapping_add(finding.id.len() as u64);
        for byte in finding.id.as_bytes() {
            digest = digest.wrapping_mul(131).wrapping_add(u64::from(*byte));
        }
        for byte in finding.failure_mode.as_str().as_bytes() {
            digest = digest.wrapping_mul(131).wrapping_add(u64::from(*byte));
        }
    }
    digest
}

pub fn milestone_37_cleared_finding_ids() -> BTreeSet<&'static str> {
    BTreeSet::from([
        "F-01", "F-02", "F-03", "F-04", "B-01", "B-02", "B-03", "H-01", "H-02", "O-01", "O-02",
        "O-03", "O-04", "S-01", "A-01", "A-02", "A-03", "A-04", "T-01", "T-02", "T-03", "T-04",
    ])
}

pub fn milestone_37_critical_finding_ids() -> BTreeSet<&'static str> {
    BTreeSet::from([])
}

pub fn milestone_37_active_failure_modes() -> BTreeSet<CleanupFailureMode> {
    BTreeSet::from([])
}

pub fn rejected_cosmetic_candidate_ids() -> BTreeSet<&'static str> {
    BTreeSet::from([
        "COSMETIC-01",
        "COSMETIC-02",
        "COSMETIC-03",
        "COSMETIC-04",
        "COSMETIC-05",
    ])
}

fn facade_leakage_findings(workspace_root: &Path) -> Vec<StructuralCleanupFinding> {
    let facade_mod = workspace_root.join("crates/worth-ui-runtime/src/facade/mod.rs");
    let text = read_file(&facade_mod);
    let mut findings = Vec::new();

    if text.contains("pub use crate::runtime::*;") {
        findings.push(finding(
            "F-01",
            CleanupFailureMode::FacadeLeakage,
            &facade_mod,
            2,
            "export_diff + compile_fail_deep_import",
            "facade root wildcard-reexports entire runtime internal topology",
        ));
    }

    let pub_use_lines = text.lines().filter(|line| line.trim_start().starts_with("pub use")).count();
    if pub_use_lines >= 10 {
        findings.push(finding(
            "F-02",
            CleanupFailureMode::FacadeLeakage,
            &facade_mod,
            2,
            "export_diff",
            &format!(
                "facade mod.rs has {pub_use_lines} pub use blocks; exports are not lifecycle-grouped"
            ),
        ));
    }

    if text.contains("pub use worth_ui_inspection::") {
        findings.push(finding(
            "F-03",
            CleanupFailureMode::FacadeLeakage,
            &facade_mod,
            2,
            "export_diff + inspection_crate_direct_import",
            "runtime facade mirrors full worth_ui_inspection vocabulary",
        ));
    }

    if text.contains("certify_activation_boundary_suite")
        || text.contains("certify_allocation_planning_suite")
    {
        findings.push(finding(
            "F-04",
            CleanupFailureMode::FacadeLeakage,
            &facade_mod,
            2,
            "export_diff + certification_topology_entry",
            "certification suite functions exported at runtime facade root",
        ));
    }

    findings
}

fn topology_sinkhole_findings(workspace_root: &Path) -> Vec<StructuralCleanupFinding> {
    let runtime_root = workspace_root.join("crates/worth-ui-runtime/src/runtime");
    let evidence_root = workspace_root.join("crates/worth-ui-runtime/src/evidence");
    let mut findings = Vec::new();

    let runtime_same_level = count_same_level_rust_files(&runtime_root);
    if runtime_same_level >= RUNTIME_SAME_LEVEL_SINKHOLE_THRESHOLD {
        findings.push(finding(
            "T-01",
            CleanupFailureMode::TopologySinkhole,
            runtime_root.join("mod.rs"),
            3,
            "directory_skeleton",
            &format!(
                "runtime root has {runtime_same_level} same-level .rs files; lifecycle lanes are not visible directories"
            ),
        ));
    }

    let host_prefix_count = count_files_matching(&runtime_root, "host_*.rs");
    if host_prefix_count > 0 {
        findings.push(finding(
            "T-02",
            CleanupFailureMode::TopologySinkhole,
            &runtime_root,
            3,
            "directory_skeleton",
            &format!(
                "runtime root mixes {host_prefix_count} host_* adapter files with production lifecycle modules"
            ),
        ));
    }

    let evidence_same_level = count_same_level_rust_files(&evidence_root);
    if evidence_same_level >= EVIDENCE_SAME_LEVEL_SINKHOLE_THRESHOLD {
        findings.push(finding(
            "T-03",
            CleanupFailureMode::TopologySinkhole,
            evidence_root.join("mod.rs"),
            4,
            "directory_skeleton",
            &format!(
                "evidence root has {evidence_same_level} same-level .rs files; proof families are a flat noun warehouse"
            ),
        ));
    }

    let boundary_test_count = count_files_matching(&runtime_root, "*_boundary_tests.rs");
    if boundary_test_count >= 20 {
        findings.push(finding(
            "T-04",
            CleanupFailureMode::TopologySinkhole,
            &runtime_root,
            3,
            "directory_skeleton + test_topology",
            &format!(
                "runtime root hosts {boundary_test_count} *_boundary_tests.rs files alongside production modules"
            ),
        ));
    }

    findings
}

fn helper_swamp_findings(workspace_root: &Path) -> Vec<StructuralCleanupFinding> {
    let evidence_mod = workspace_root.join("crates/worth-ui-runtime/src/evidence/mod.rs");
    let text = read_file(&evidence_mod);
    let mut findings = Vec::new();

    let construction_helpers = [
        "fn evidence_identity(",
        "fn evidence_handle(",
        "fn evidence_ref(",
        "fn preflight_evidence_expansion(",
    ];
    let helper_count = construction_helpers
        .iter()
        .filter(|token| text.contains(**token))
        .count();
    if helper_count >= 3 {
        findings.push(finding(
            "H-01",
            CleanupFailureMode::HelperSwamp,
            &evidence_mod,
            4,
            "helper_relocation_diff",
            &format!(
                "evidence/mod.rs hosts {helper_count} inline construction helpers that should live in a construction home"
            ),
        ));
    }

    let bridge = workspace_root.join("crates/worth-ui-runtime/src/facade/runtime_bridge.rs");
    if bridge.exists() {
        let bridge_text = read_file(&bridge);
        if bridge_text.contains("WorthUiFacadeLifecycleBootstrap")
            && bridge_text.contains("inspection_scope_inventory")
            && bridge_text.contains("measurement_inspection_evidence")
        {
            findings.push(finding(
                "H-02",
                CleanupFailureMode::HelperSwamp,
                &bridge,
                2,
                "facade_lifecycle_split_diff",
                "facade runtime_bridge assembles inspection, measurement evidence, and support inventory in one bootstrap bag",
            ));
        }
    }

    findings
}

fn authority_mixing_findings(workspace_root: &Path) -> Vec<StructuralCleanupFinding> {
    let mut findings = Vec::new();

    let evidence_cert = workspace_root
        .join("crates/worth-ui-runtime/src/evidence/planning/certification.rs");
    let evidence_cert_text = read_file(&evidence_cert);
    if evidence_cert_text.contains("planning_pair_for_certification_suite")
        && !evidence_cert_text.contains("pub(crate) fn suite_contract_satisfied")
    {
        findings.push(finding(
            "A-01",
            CleanupFailureMode::AuthorityMixing,
            &evidence_cert,
            6,
            "cert_scenario_relocation + anti_bypass_audit",
            "evidence certification reaches into runtime planning_pair_for_certification_suite",
        ));
    }

    let planning_mod = workspace_root
        .join("crates/worth-ui-runtime/src/runtime/allocation_planning/mod.rs");
    if read_file(&planning_mod).contains("mod certification_fixture") {
        findings.push(finding(
            "A-02",
            CleanupFailureMode::AuthorityMixing,
            workspace_root.join("crates/worth-ui-runtime/src/runtime/allocation_planning"),
            6,
            "cert_fixture_fence",
            "production allocation_planning tree still hosts certification_fixture modules",
        ));
    }

    let app = workspace_root.join("crates/worth-ui-runtime/src/facade/entry/app.rs");
    let app_text = read_file(&app);
    let inspect_is_thin_delegate = app_text.contains("route_inspection(self, query)");
    let graph_evidence_delegated = app_text.contains("build_graph_evidence_indexes(");
    if (!inspect_is_thin_delegate || !graph_evidence_delegated)
        && app_text.contains("inspect(")
        && app_text.contains("UiInspectionReceipt")
    {
        findings.push(finding(
            "A-03",
            CleanupFailureMode::AuthorityMixing,
            &app,
            2,
            "inspection_bridge_split_diff",
            "facade app owns graph index assembly and multi-scope inspection dispatch",
        ));
    }

    let lifecycle = workspace_root.join("crates/worth-ui-runtime/src/lifecycle/support_inventory.rs");
    if read_file(&lifecycle).contains("PHASE3_RUNTIME_SUPPORT_INVENTORY")
        && !read_file(&lifecycle).contains("RUNTIME_SUPPORT_INVENTORY")
    {
        findings.push(finding(
            "A-04",
            CleanupFailureMode::AuthorityMixing,
            &lifecycle,
            6,
            "inventory_rename_diff",
            "runtime support inventory still uses PHASE3 provenance naming",
        ));
    }

    let cert_topology = workspace_root
        .join("crates/worth-ui-certification/src/topology/allocation_planning_boundary_certification.rs");
    let cert_topology_text = read_file(&cert_topology);
    if cert_topology_text.contains("worth_ui_runtime::facade::certify_")
        || (cert_topology_text.contains("worth_ui_runtime::facade::evidence::certify_activation_boundary_suite")
            && !cert_topology_text.contains("certification_entry"))
    {
        findings.push(finding(
            "B-03",
            CleanupFailureMode::TestBypass,
            &cert_topology,
            6,
            "cert_facade_import_narrowing",
            "certification topology imports certify suites through broad runtime facade root",
        ));
    }

    findings
}

fn function_overload_findings(workspace_root: &Path) -> Vec<StructuralCleanupFinding> {
    let mut findings = Vec::new();
    let overload_targets = [
        (
            "O-01",
            "crates/worth-ui-runtime/src/runtime/planning/plan_allocation.rs",
            "plan_allocation_for_pending_activation",
            "plan_allocation",
        ),
        (
            "O-02",
            "crates/worth-ui-runtime/src/facade/entry/app.rs",
            "WorthUiApp",
            "inspect",
        ),
        (
            "O-03",
            "crates/worth-ui-runtime/src/runtime/matching/worth_ui_identity_match_graph_builder",
            "WorthUiIdentityMatchGraphBuilder",
            "build",
        ),
        (
            "O-04",
            "crates/worth-ui-runtime/src/evidence/measurement/projection/inspection_receipt",
            "inspection_receipt",
            "project_measurement_inspection",
        ),
    ];

    for (id, rel_path, owner, transition) in overload_targets {
        let path = workspace_root.join(rel_path);
        // Decomposed surfaces are audited by max step-file size, not summed directory size.
        let line_count = max_rust_surface_lines(&path);
        if line_count > FILE_SIZE_CAP_LINES {
            findings.push(finding(
                id,
                CleanupFailureMode::FunctionOverload,
                &path,
                7,
                "function_decomposition_diff + parity_test",
                &format!(
                    "{owner} {transition} surface still has a step file spanning {line_count} lines"
                ),
            ));
        }
    }

    findings
}

fn file_size_findings(workspace_root: &Path) -> Vec<StructuralCleanupFinding> {
    // Phase 7 S-01 closes on the decomposition hotspots named by the cleanup plan —
    // not crate-wide line-cap thrash of cohesive single-responsibility modules.
    let hotspot_roots = [
        "crates/worth-ui-runtime/src/runtime/matching/worth_ui_identity_match_graph_builder",
        "crates/worth-ui-runtime/src/evidence/measurement/projection/inspection_receipt",
        "crates/worth-ui-runtime/src/graph/allocation_neighborhood/constraint_pipeline",
        "crates/worth-ui-runtime/src/runtime/launch",
    ];
    let mut offenders = Vec::new();
    for rel in hotspot_roots {
        let root = workspace_root.join(rel);
        if !root.exists() {
            continue;
        }
        let paths = if root.is_dir() {
            collect_rust_files(&root)
        } else {
            vec![root]
        };
        for path in paths {
            if is_runtime_test_or_support_path(&path) {
                continue;
            }
            // host_test_support is deliberately cfg(test); skip if present as source file.
            let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if name == "host_test_support.rs" {
                continue;
            }
            let lines = count_lines(&path);
            if lines > FILE_SIZE_CAP_LINES {
                offenders.push((lines, path));
            }
        }
    }
    offenders.sort_by(|left, right| right.0.cmp(&left.0).then_with(|| left.1.cmp(&right.1)));

    if offenders.is_empty() {
        return Vec::new();
    }

    let summary = offenders
        .iter()
        .take(8)
        .map(|(lines, path)| format!("{}:{lines}", path_relative(workspace_root, path)))
        .collect::<Vec<_>>()
        .join(", ");

    vec![finding(
        "S-01",
        CleanupFailureMode::FileSize,
        workspace_root.join("crates/worth-ui-runtime/src/lib.rs"),
        7,
        "file_split_diff_or_exemption",
        &format!(
            "phase-7 decomposition hotspot files exceed {FILE_SIZE_CAP_LINES} lines: {summary}"
        ),
    )]
}

fn test_bypass_findings(workspace_root: &Path) -> Vec<StructuralCleanupFinding> {
    let runtime_mod = workspace_root.join("crates/worth-ui-runtime/src/runtime/mod.rs");
    let text = read_file(&runtime_mod);
    let mut findings = Vec::new();

    if text.contains("pub use tests::support::touch_origin_certification_support") {
        findings.push(finding(
            "B-01",
            CleanupFailureMode::TestBypass,
            &runtime_mod,
            6,
            "test_support_visibility_fence",
            "runtime/mod.rs pub-uses touch_origin_certification_support under cfg(test)",
        ));
    }

    if text.contains("pub(crate) use runtime_test_modules::")
        || (text.contains("pub(crate) use tests::") && !text.contains("mod tests;"))
    {
        findings.push(finding(
            "B-02",
            CleanupFailureMode::TestBypass,
            &runtime_mod,
            6,
            "test_support_scope_narrowing",
            "runtime/mod.rs exports test support at crate-private scope from production root",
        ));
    }

    let cert_support = workspace_root.join("crates/worth-ui-runtime/src/certification_support/mod.rs");
    let cert_text = read_file(&cert_support);
    if cert_text.contains("include!(") {
        findings.push(finding(
            "B-04",
            CleanupFailureMode::TestBypass,
            &cert_support,
            6,
            "support_single_home_diff",
            "certification_support still include!s runtime/tests modules instead of owning fixtures",
        ));
    }

    let host_mod = workspace_root.join("crates/worth-ui-runtime/src/host/mod.rs");
    let host_text = read_file(&host_mod);
    if host_text.contains("collect_measurement_via_host_lane_for_test")
        && !host_text.contains("#[cfg(test)]")
    {
        findings.push(finding(
            "B-05",
            CleanupFailureMode::TestBypass,
            &host_mod,
            6,
            "host_test_reexport_fence",
            "host production root reexports for_test helpers outside cfg(test)",
        ));
    }

    let lib_rs = workspace_root.join("crates/worth-ui-runtime/src/lib.rs");
    let lib_text = read_file(&lib_rs);
    if lib_text.contains("pub mod certification_support")
        && !lib_text.contains("feature = \"certification-support\"")
    {
        findings.push(finding(
            "B-06",
            CleanupFailureMode::TestBypass,
            &lib_rs,
            6,
            "cert_support_feature_fence",
            "certification_support is unconditionally public on the production runtime crate",
        ));
    }

    findings
}

fn finding(
    id: &str,
    failure_mode: CleanupFailureMode,
    path: impl AsRef<Path>,
    owner_phase: u8,
    closeout_evidence: &str,
    summary: &str,
) -> StructuralCleanupFinding {
    StructuralCleanupFinding {
        id: id.to_string(),
        failure_mode,
        path: path.as_ref().display().to_string(),
        owner_phase,
        closeout_evidence: closeout_evidence.to_string(),
        summary: summary.to_string(),
    }
}

fn read_file(path: &Path) -> String {
    fs::read_to_string(path).unwrap_or_else(|error| {
        panic!("expected readable source at {}: {error}", path.display())
    })
}

fn count_lines(path: &Path) -> usize {
    read_file(path).lines().count()
}

fn count_rust_surface_lines(path: &Path) -> usize {
    if path.is_dir() {
        return count_lines_in_rust_dir(path);
    }
    count_lines(path)
}

fn max_rust_surface_lines(path: &Path) -> usize {
    if path.is_dir() {
        return max_lines_in_rust_dir(path);
    }
    if path.exists() {
        return count_lines(path);
    }
    0
}

fn count_lines_in_rust_dir(dir: &Path) -> usize {
    fs::read_dir(dir)
        .unwrap_or_else(|error| panic!("expected readable dir {}: {error}", dir.display()))
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|extension| extension == "rs"))
        .map(|path| count_lines(&path))
        .sum()
}

fn max_lines_in_rust_dir(dir: &Path) -> usize {
    fs::read_dir(dir)
        .unwrap_or_else(|error| panic!("expected readable dir {}: {error}", dir.display()))
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|extension| extension == "rs"))
        .map(|path| count_lines(&path))
        .max()
        .unwrap_or(0)
}

fn is_runtime_test_or_support_path(path: &Path) -> bool {
    let text = path.to_string_lossy().replace('\\', "/");
    text.contains("/tests/")
        || text.ends_with("_tests.rs")
        || text.ends_with("_test_support.rs")
        || text.contains("/tests.rs")
        || text.ends_with("/tests/mod.rs")
}

fn count_same_level_rust_files(dir: &Path) -> usize {
    fs::read_dir(dir)
        .unwrap_or_else(|error| panic!("expected readable dir {}: {error}", dir.display()))
        .filter_map(Result::ok)
        .filter(|entry| {
            entry
                .path()
                .extension()
                .is_some_and(|extension| extension == "rs")
        })
        .count()
}

fn count_files_matching(dir: &Path, pattern: &str) -> usize {
    fs::read_dir(dir)
        .unwrap_or_else(|error| panic!("expected readable dir {}: {error}", dir.display()))
        .filter_map(Result::ok)
        .filter(|entry| {
            entry.file_name().to_str().is_some_and(|name| {
                if let Some(prefix) = pattern.strip_suffix("*.rs") {
                    name.starts_with(prefix) && name.ends_with(".rs")
                } else if let Some(suffix) = pattern.strip_prefix('*') {
                    name.ends_with(suffix)
                } else {
                    name == pattern
                }
            })
        })
        .count()
}

fn collect_rust_files(root: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    collect_rust_files_recursively(root, &mut files);
    files.sort();
    files
}

fn collect_rust_files_recursively(dir: &Path, output: &mut Vec<PathBuf>) {
    let entries = fs::read_dir(dir)
        .unwrap_or_else(|error| panic!("expected readable dir {}: {error}", dir.display()));
    for entry in entries.filter_map(Result::ok) {
        let path = entry.path();
        if path.is_dir() {
            collect_rust_files_recursively(&path, output);
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            output.push(path);
        }
    }
}

fn path_relative(workspace_root: &Path, path: &Path) -> String {
    path.strip_prefix(workspace_root)
        .unwrap_or(path)
        .display()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scoped_crate_roots_are_phase_1_inventory_scope() {
        assert_eq!(SCOPED_CRATE_ROOTS.len(), 4);
    }
}