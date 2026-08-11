use std::path::Path;

use super::repository_root;

pub(super) const SPECIFICATION: &str =
    "_docs/worth-store/physical-reconstruction-c8-fresh-process-recovery-and-reopen.md";
pub(super) const API_INVENTORY: &str =
    "_docs/worth-store/physical-reconstruction-c8-public-api.csv";
pub(super) const AUTHORITY_TRACE: &str =
    "_docs/worth-store/physical-reconstruction-c8-authority-trace.csv";
pub(super) const CARGO_GRAPH: &str = "_docs/worth-store/physical-reconstruction-c8-cargo-graph.csv";
pub(super) const DESTINATION_TOPOLOGY: &str =
    "_docs/worth-store/physical-reconstruction-c8-destination-topology.csv";
pub(super) const CUTOVER_INVENTORY: &str =
    "_docs/worth-store/physical-reconstruction-c8-cutover-inventory.csv";
pub(super) const CLOSURE_LEDGER: &str =
    "_docs/worth-store/physical-reconstruction-c8-closure-ledger.md";
pub(super) const PERSISTED_INPUTS: &str =
    "_docs/worth-store/physical-reconstruction-c8-persisted-inputs.csv";
pub(super) const QA_AUDITS: &str = "_docs/worth-store/physical-reconstruction-c8-qa-audits.csv";
pub(super) const QA_SOURCE_MANIFESTS: &str =
    "_docs/worth-store/physical-reconstruction-c8-qa-source-manifests.csv";

pub(super) fn read_repository_document(path: &str) -> Result<String, String> {
    let path = repository_root().join(path);
    std::fs::read_to_string(&path)
        .map_err(|error| format!("cannot read {}: {error}", path.display()))
}

pub(super) fn workspace_relative(path: &Path) -> Result<String, String> {
    let workspace = crate::workspace_root();
    path.strip_prefix(&workspace)
        .map(|relative| relative.to_string_lossy().replace('\\', "/"))
        .map_err(|error| {
            format!(
                "{} is outside {}: {error}",
                path.display(),
                workspace.display()
            )
        })
}

pub(super) fn split_csv<'a>(line: &'a str, columns: usize) -> Result<Vec<&'a str>, String> {
    let values = line.split(',').map(str::trim).collect::<Vec<_>>();
    if values.len() != columns || values.iter().any(|value| value.is_empty()) {
        return Err(format!("expected {columns} nonempty columns in `{line}`"));
    }
    Ok(values)
}

#[test]
fn authoritative_roadmap_c8_specification_link_exists() {
    let roadmap =
        read_repository_document("_docs/worth-store/physical-foundation-reconstruction-roadmap.md")
            .expect("read reconstruction roadmap");
    let target = "physical-reconstruction-c8-fresh-process-recovery-and-reopen.md";
    assert!(roadmap.contains(target));
    assert!(repository_root()
        .join("_docs/worth-store")
        .join(target)
        .is_file());
}

#[test]
fn roadmap_and_specification_share_exact_fates_and_entry_inputs() {
    let roadmap =
        read_repository_document("_docs/worth-store/physical-foundation-reconstruction-roadmap.md")
            .expect("read reconstruction roadmap");
    let trace = read_repository_document(AUTHORITY_TRACE).expect("read C.8 authority trace");
    for fate in [
        "AcknowledgedDurable",
        "ProvenNoEffect",
        "DurableUnacknowledged",
        "Indeterminate",
    ] {
        assert!(roadmap.contains(fate), "roadmap omitted exact fate {fate}");
        assert!(trace.contains(&format!("operation-fate,{fate},")));
    }
    assert!(!roadmap.contains("unacknowledged-not-durable"));
    assert!(roadmap.contains("physical recovery limits"));
    assert!(trace.contains("entry-input,physical-recovery-limits,"));
    assert!(roadmap.contains("platform authority is minted inside"));
}
