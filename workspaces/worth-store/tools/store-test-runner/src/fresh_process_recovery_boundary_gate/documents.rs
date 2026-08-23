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

#[test]
fn public_recovery_docs_name_the_exact_terminal_variant() {
    for path in [
        "_docs/worth-store/physical-recovery-and-reopen.md",
        "workspaces/worth-store/crates/worth-store-recovery-runtime/README.md",
    ] {
        let document = read_repository_document(path).expect("read recovery documentation");
        assert!(
            document.contains("PublicationIndeterminate"),
            "{path} must name the public PublicationIndeterminate terminal"
        );
    }
}

#[test]
fn documented_recovery_and_observer_commands_are_extractable() {
    let document = read_repository_document("_docs/worth-store/physical-recovery-and-reopen.md")
        .expect("read operator recovery guide");
    let commands = extract_real_example_commands(&document);
    assert_eq!(commands.len(), 2);
    assert_eq!(commands[0].program, "physical_store_recover");
    assert_eq!(commands[1].program, "physical_store_offline_observer");
    assert!(commands[0]
        .arguments
        .iter()
        .any(|argument| argument == "--bounded-profile=c8-phase2-admission-v1"));
    assert!(commands[1]
        .arguments
        .iter()
        .any(|argument| argument == "c8-recovery-observe"));
}

#[derive(Debug, PartialEq, Eq)]
struct DocumentedCommand {
    program: String,
    arguments: Vec<String>,
}

fn extract_real_example_commands(document: &str) -> Vec<DocumentedCommand> {
    let start = document
        .find("## Real Example")
        .expect("operator guide real example heading")
        + "## Real Example".len();
    let end = document[start..]
        .find("## How It Relates To Other Features")
        .map(|offset| start + offset)
        .expect("operator guide real example boundary");
    let mut commands = Vec::new();
    let mut logical = String::new();
    let mut in_code_block = false;
    for line in document[start..end].lines() {
        let line = line.trim();
        if line == "```text" {
            in_code_block = true;
            continue;
        }
        if line == "```" {
            in_code_block = false;
            continue;
        }
        if !in_code_block || line.is_empty() {
            continue;
        }
        let continued = line.ends_with('\\');
        let fragment = line.strip_suffix('\\').unwrap_or(line).trim_end();
        if !logical.is_empty() {
            logical.push(' ');
        }
        logical.push_str(fragment);
        if !continued {
            let mut fields = logical.split_whitespace();
            let Some(program) = fields.next() else {
                logical.clear();
                continue;
            };
            commands.push(DocumentedCommand {
                program: program.to_owned(),
                arguments: fields.map(str::to_owned).collect(),
            });
            logical.clear();
        }
    }
    assert!(!in_code_block, "unterminated documented code block");
    assert!(logical.is_empty(), "unterminated documented command");
    commands
}
