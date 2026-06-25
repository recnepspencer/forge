use std::fs;
use std::path::{Path, PathBuf};

use super::super::source_firewall::WorthGraphReadAccessPlanAdoptionSourceFirewallReport;

#[test]
fn phase_two_source_firewall_rejects_execution_shaped_residue() {
    let report = WorthGraphReadAccessPlanAdoptionSourceFirewallReport::from_sources([(
        "hostile.rs",
        "fn local_adjacency_map() { let _ = \"fabricated_receipt\"; }",
    )]);

    assert_eq!(report.scanned_file_count(), 1);
    assert!(report.violation_count() >= 2);
    assert!(report.violations().iter().any(|violation| {
        violation.file_path() == "hostile.rs"
            && violation.forbidden_pattern() == "local_adjacency_map"
    }));
}

#[test]
fn phase_two_production_sources_do_not_reintroduce_local_execution_authority() {
    let phase_two_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("graph_read_access_plan_adoption")
        .join("phase_two_adoption");
    let sources = rust_sources_under(&phase_two_root)
        .into_iter()
        .filter(|path| !has_component(path, "tests"))
        .filter(|path| !has_component(path, "source_firewall"))
        .map(|path| {
            let contents = fs::read_to_string(&path).expect("source file should be readable");
            (path.display().to_string(), contents)
        })
        .collect::<Vec<_>>();

    let report = WorthGraphReadAccessPlanAdoptionSourceFirewallReport::from_sources(sources);

    assert!(report.scanned_file_count() > 0);
    assert_eq!(
        report.violation_count(),
        0,
        "Phase 2 production sources must not contain local execution authority residue: {:?}",
        report.violations()
    );
}

fn rust_sources_under(root: &Path) -> Vec<PathBuf> {
    let mut sources = Vec::new();
    collect_rust_sources(root, &mut sources);
    sources
}

fn has_component(path: &Path, component_name: &str) -> bool {
    path.components()
        .any(|component| component.as_os_str().to_string_lossy() == component_name)
}

fn collect_rust_sources(root: &Path, sources: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(root).expect("source directory should be readable") {
        let path = entry.expect("directory entry should be readable").path();
        if path.is_dir() {
            collect_rust_sources(&path, sources);
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            sources.push(path);
        }
    }
}
