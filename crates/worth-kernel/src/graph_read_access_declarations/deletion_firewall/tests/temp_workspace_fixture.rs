use std::path::{Path, PathBuf};

use super::super::source_firewall::SourceFirewallRegion;
use super::deletion_fingerprints::old_graph_read_source_path;

pub(crate) fn temp_workspace_with_old_graph_read_adoption_residue() -> PathBuf {
    let root = temp_phase_six_workspace("residue");
    let old_path = root.join(old_graph_read_source_path());
    std::fs::create_dir_all(&old_path).expect("temp old graph-read path should be creatable");
    root
}

pub(crate) fn temp_workspace_with_topology_declaration_helper_residue() -> PathBuf {
    let root = temp_phase_six_workspace("topology_helper");
    let helper_path = root
        .join("crates")
        .join("worth-topo")
        .join("src")
        .join("projection")
        .join("local_read_helper.rs");
    std::fs::create_dir_all(
        helper_path
            .parent()
            .expect("helper path should have parent"),
    )
    .expect("temp topology projection path should be creatable");
    std::fs::write(
        helper_path,
        "fn fallback_graph_walk() { let _residue = true; }\n",
    )
    .expect("temp topology helper residue should be writable");
    root
}

pub(crate) fn temp_workspace_with_forbidden_pattern(
    region: SourceFirewallRegion,
    pattern: &'static str,
) -> PathBuf {
    let root = temp_phase_six_workspace(region.digest_part());
    let path = source_file_for_region(&root, region);
    std::fs::create_dir_all(
        path.parent()
            .expect("forbidden source file should have parent"),
    )
    .expect("forbidden pattern source root should be creatable");
    std::fs::write(
        path,
        format!("fn forbidden_pattern_probe() {{ {pattern}; }}\n"),
    )
    .expect("forbidden pattern source file should be writable");
    root
}

fn source_file_for_region(root: &Path, region: SourceFirewallRegion) -> PathBuf {
    match region {
        SourceFirewallRegion::DeclarationAuthority => root
            .join("crates")
            .join("worth-kernel")
            .join("src")
            .join("graph_read_access_declarations")
            .join("hostile_region_probe.rs"),
        SourceFirewallRegion::WorthKernelAdoptionAuthority => root
            .join(old_graph_read_source_path())
            .join("hostile_region_probe.rs"),
        SourceFirewallRegion::TopologySpatialReadHelpers => root
            .join("crates")
            .join("worth-topo")
            .join("src")
            .join("projection")
            .join("hostile_region_probe.rs"),
    }
}

fn temp_phase_six_workspace(label: &str) -> PathBuf {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system time should be after unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "worth_graph_read_phase_six_{label}_{}_{}",
        std::process::id(),
        timestamp
    ))
}
