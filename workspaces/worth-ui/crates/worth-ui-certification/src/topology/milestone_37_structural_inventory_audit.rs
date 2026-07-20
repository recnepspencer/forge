use std::path::Path;

use super::workspace_source_inventory::WorkspaceSourceInventory;

mod milestone_37_filesystem_findings;
mod milestone_37_finding_catalog;

use milestone_37_filesystem_findings::{
    count_files_matching, count_same_level_rust_files, read_file,
};
pub use milestone_37_finding_catalog::{
    milestone_37_active_failure_modes, milestone_37_cleared_finding_ids,
    milestone_37_critical_finding_ids, rejected_cosmetic_candidate_ids,
};

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
    inventory: &WorkspaceSourceInventory,
) -> Vec<StructuralCleanupFinding> {
    let mut findings = Vec::new();
    findings.extend(facade_leakage_findings(inventory));
    findings.extend(topology_sinkhole_findings(inventory));
    findings.extend(helper_swamp_findings(inventory));
    findings.extend(authority_mixing_findings(inventory));
    findings.extend(function_overload_findings(inventory));
    findings.extend(milestone_37_filesystem_findings::file_size_findings(
        inventory,
    ));
    findings.extend(milestone_37_filesystem_findings::test_bypass_findings(
        inventory,
    ));
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

fn facade_leakage_findings(inventory: &WorkspaceSourceInventory) -> Vec<StructuralCleanupFinding> {
    let facade_mod = inventory.absolute_path("crates/worth-ui-runtime/src/facade/mod.rs");
    let text = read_file(inventory, &facade_mod);
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

    let pub_use_lines = text
        .lines()
        .filter(|line| line.trim_start().starts_with("pub use"))
        .count();
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

fn topology_sinkhole_findings(
    inventory: &WorkspaceSourceInventory,
) -> Vec<StructuralCleanupFinding> {
    let runtime_root = inventory.absolute_path("crates/worth-ui-runtime/src/runtime");
    let evidence_root = inventory.absolute_path("crates/worth-ui-runtime/src/evidence");
    let mut findings = Vec::new();

    let runtime_same_level = count_same_level_rust_files(inventory, &runtime_root);
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

    let host_prefix_count = count_files_matching(inventory, &runtime_root, "host_*.rs");
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

    let evidence_same_level = count_same_level_rust_files(inventory, &evidence_root);
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

    let boundary_test_count = count_files_matching(inventory, &runtime_root, "*_boundary_tests.rs");
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

fn helper_swamp_findings(inventory: &WorkspaceSourceInventory) -> Vec<StructuralCleanupFinding> {
    let evidence_mod = inventory.absolute_path("crates/worth-ui-runtime/src/evidence/mod.rs");
    let text = read_file(inventory, &evidence_mod);
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

    let bridge = inventory.absolute_path("crates/worth-ui-runtime/src/facade/runtime_bridge.rs");
    if inventory.source(&bridge).is_some() {
        let bridge_text = read_file(inventory, &bridge);
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

fn authority_mixing_findings(
    inventory: &WorkspaceSourceInventory,
) -> Vec<StructuralCleanupFinding> {
    let mut findings = Vec::new();

    let evidence_cert =
        inventory.absolute_path("crates/worth-ui-runtime/src/evidence/planning/certification.rs");
    let evidence_cert_text = read_file(inventory, &evidence_cert);
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

    let planning_mod =
        inventory.absolute_path("crates/worth-ui-runtime/src/runtime/allocation_planning/mod.rs");
    if read_file(inventory, &planning_mod).contains("mod certification_fixture") {
        findings.push(finding(
            "A-02",
            CleanupFailureMode::AuthorityMixing,
            inventory.absolute_path("crates/worth-ui-runtime/src/runtime/allocation_planning"),
            6,
            "cert_fixture_fence",
            "production allocation_planning tree still hosts certification_fixture modules",
        ));
    }

    let app = inventory.absolute_path("crates/worth-ui-runtime/src/facade/entry/app.rs");
    let app_text = read_file(inventory, &app);
    let inspect_is_thin_delegate = app_text.contains("route_inspection(self, query)");
    let app_assembles_graph_evidence = app_text.contains("build_graph_evidence_indexes(");
    if (!inspect_is_thin_delegate || app_assembles_graph_evidence)
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

    let lifecycle =
        inventory.absolute_path("crates/worth-ui-runtime/src/lifecycle/support_inventory.rs");
    if read_file(inventory, &lifecycle).contains("PHASE3_RUNTIME_SUPPORT_INVENTORY")
        && !read_file(inventory, &lifecycle).contains("RUNTIME_SUPPORT_INVENTORY")
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

    let cert_topology = inventory.absolute_path(
        "crates/worth-ui-certification/src/topology/allocation_planning_boundary_certification.rs",
    );
    let cert_topology_text = read_file(inventory, &cert_topology);
    if cert_topology_text.contains("worth_ui_runtime::facade::certify_")
        || (cert_topology_text
            .contains("worth_ui_runtime::facade::evidence::certify_activation_boundary_suite")
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

fn function_overload_findings(
    inventory: &WorkspaceSourceInventory,
) -> Vec<StructuralCleanupFinding> {
    milestone_37_filesystem_findings::function_overload_findings(inventory)
}

// Phase 7 S-01 closes on the decomposition hotspots named by the cleanup plan —
pub(super) fn finding(
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

#[cfg(test)]
#[path = "milestone_37_structural_inventory_audit/scope_tests.rs"]
mod scope_tests;
