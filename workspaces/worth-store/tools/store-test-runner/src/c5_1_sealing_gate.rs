use std::{
    path::{Path, PathBuf},
    process::Command,
};

use super::workspace_root;

const ORDINARY_SOURCE_ROOTS: [&str; 4] = [
    "crates/worth-store/src/physical_runtime",
    "crates/worth-store-io-scheduler/src",
    "crates/worth-store-buffer-pool/src",
    "crates/worth-store-physical-backend/src",
];

const ORDINARY_MANIFESTS: [&str; 4] = [
    "crates/worth-store/Cargo.toml",
    "crates/worth-store-io-scheduler/Cargo.toml",
    "crates/worth-store-buffer-pool/Cargo.toml",
    "crates/worth-store-physical-backend/Cargo.toml",
];

const FORBIDDEN_TYPED_BOUNDARY_FRAGMENTS: &[(&str, &str)] = &[
    ("serde_json", "internal JSON carrier"),
    ("serde_json::Value", "internal JSON value"),
    ("worth_query", "Query dependency"),
    ("worth_relational", "Relational dependency"),
    ("BranchHead", "branch-head authority"),
    ("branch_head", "branch-head authority"),
    ("BranchWriter", "branch writer authority"),
    ("branch_writer", "branch writer authority"),
    ("WriterGeneration", "semantic writer-generation authority"),
    ("Mvcc", "MVCC authority"),
    ("MVCC", "MVCC authority"),
    (
        "SerializedSignalReopenState",
        "serialized Signal state used for reopen",
    ),
    (
        "ResourceNodeDeclaration",
        "legacy Signal resource-node construction",
    ),
    (
        "RawSignalSlotSemanticAuthority",
        "raw Signal slot used as semantic authority",
    ),
    (
        "FoundationalMaskSubstitution",
        "Foundational mask substituted for native binding",
    ),
    (
        "PhysicalAspectPartitionBroadening",
        "caller aspect or partition broadening",
    ),
    (
        "InternalPhysicalWorkJsonCarrier",
        "internal physical-work JSON carrier",
    ),
    (
        "BranchLabelPhysicalDisjointness",
        "branch label used for physical disjointness",
    ),
];

const FORBIDDEN_DUPLICATE_RUNTIME_FRAGMENTS: &[(&str, &str)] = &[
    ("TimerWheel", "Store-local timer wheel"),
    ("RetryQueue", "Store-local retry queue"),
    ("TimeoutRegistry", "Store-local timeout registry"),
    ("PolicyRegistry", "Store-local async policy registry"),
    ("PendingWorkRegistry", "duplicate pending-work registry"),
    ("C6LocalScheduler", "C.6-local scheduler"),
    ("CallbackSettlement", "callback settlement route"),
    ("BranchWriterRegistry", "Store-local branch writer registry"),
    (
        "BranchWriteToken",
        "branch token used as physical authority",
    ),
    (
        "PhysicalEffectRetryAfterDerivedRollback",
        "physical effect retry after derived rollback",
    ),
    ("DuplicatePhysicalLifecycle", "duplicate physical lifecycle"),
];

const RAW_SIGNAL_CONSTRUCTORS: &[&str] = &[
    "Aspect::new(",
    "Aspect::try_new(",
    "AspectMask::from_bits",
    "AspectMask::from_aspect",
];

#[test]
fn ordinary_feature_graph_excludes_legacy_and_certification_authority() {
    let output = Command::new(std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into()))
        .current_dir(workspace_root())
        .args([
            "tree",
            "--manifest-path",
            "Cargo.toml",
            "-p",
            "worth-store",
            "-e",
            "normal,build",
            "-f",
            "{p} [{f}]",
        ])
        .output()
        .expect("run ordinary Worth Store feature-tree audit");
    assert!(
        output.status.success(),
        "ordinary feature-tree audit failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let tree = String::from_utf8(output.stdout).expect("cargo tree output is UTF-8");
    let pool_rows = tree
        .lines()
        .filter(|line| line.contains("worth-store-buffer-pool "))
        .collect::<Vec<_>>();
    assert!(
        !pool_rows.is_empty(),
        "ordinary Store omitted its canonical residency dependency"
    );
    assert!(
        pool_rows
            .iter()
            .all(|line| !line.contains("legacy-s2-models")),
        "ordinary Store activated legacy S.2 residency: {pool_rows:?}"
    );
    assert!(
        !tree.contains("[certification-test-authority")
            && !tree.contains(",certification-test-authority")
            && !tree.contains("worth-store-certification "),
        "ordinary Store activated certification-only authority:\n{tree}"
    );
}

#[test]
fn ordinary_work_sources_keep_aspect_and_semantic_authority_at_typed_boundaries() {
    for root in ORDINARY_SOURCE_ROOTS {
        for source in rust_sources(&workspace_root().join(root)).expect("read production sources") {
            let text = std::fs::read_to_string(&source).expect("read production source");
            inspect_ordinary_source(&source, &text).unwrap_or_else(|denial| panic!("{denial}"));
        }
    }
}

#[test]
fn canonical_crates_have_no_forbidden_ordinary_dependency() {
    for manifest in ORDINARY_MANIFESTS {
        let path = workspace_root().join(manifest);
        let text = std::fs::read_to_string(&path).expect("read canonical manifest");
        inspect_ordinary_dependencies(&path, &text).unwrap_or_else(|denial| panic!("{denial}"));
    }
}

#[test]
fn sealing_gate_rejects_each_authority_bypass_family() {
    for (source, expected) in [
        (
            "let slot = Aspect::try_new(7)?;",
            "raw Signal aspect construction",
        ),
        (
            "let mask = AspectMask::from_bits(bits)?;",
            "raw Signal aspect construction",
        ),
        (
            "let packet: serde_json::Value = decode(bytes)?;",
            "internal JSON",
        ),
        (
            "let writers = BranchWriterRegistry::new();",
            "branch writer authority",
        ),
        (
            "let resources = ResourceNodeDeclaration::new();",
            "legacy Signal resource-node construction",
        ),
        (
            "let retries = RetryQueue::new();",
            "Store-local retry queue",
        ),
        (
            "type PhysicalEffectRetryAfterDerivedRollback = PhysicalExecutorCommand;",
            "physical effect retry after derived rollback",
        ),
        (
            "struct SerializedSignalReopenState;",
            "serialized Signal state used for reopen",
        ),
        (
            "struct RawSignalSlotSemanticAuthority;",
            "raw Signal slot used as semantic authority",
        ),
        (
            "struct FoundationalMaskSubstitution;",
            "Foundational mask substituted for native binding",
        ),
        (
            "struct PhysicalAspectPartitionBroadening;",
            "caller aspect or partition broadening",
        ),
        (
            "struct InternalPhysicalWorkJsonCarrier(String);",
            "internal physical-work JSON carrier",
        ),
        (
            "struct BranchLabelPhysicalDisjointness;",
            "branch label used for physical disjointness",
        ),
        (
            "struct DuplicatePhysicalLifecycle;",
            "duplicate physical lifecycle",
        ),
        ("struct C6LocalScheduler;", "C.6-local scheduler"),
    ] {
        let denial = inspect_ordinary_source(Path::new("controlled_mutant.rs"), source)
            .expect_err("controlled authority bypass must be denied");
        assert!(denial.contains(expected), "wrong denial: {denial}");
    }
}

fn inspect_ordinary_source(path: &Path, source: &str) -> Result<(), String> {
    let code = without_line_comments(source);
    let binding_owner =
        path.ends_with("worth-store/src/physical_runtime/work/profile/aspect_bindings.rs");
    if !binding_owner {
        for &constructor in RAW_SIGNAL_CONSTRUCTORS {
            if let Some(offset) = code.find(constructor) {
                return Err(localized_denial(
                    path,
                    &code,
                    offset,
                    "raw Signal aspect construction outside PhysicalSignalAspectBindingSet",
                ));
            }
        }
    }
    for (fragment, authority) in FORBIDDEN_TYPED_BOUNDARY_FRAGMENTS
        .iter()
        .copied()
        .chain(FORBIDDEN_DUPLICATE_RUNTIME_FRAGMENTS.iter().copied())
    {
        if let Some(offset) = code.find(fragment) {
            return Err(localized_denial(path, &code, offset, authority));
        }
    }
    Ok(())
}

fn inspect_ordinary_dependencies(path: &Path, manifest: &str) -> Result<(), String> {
    let dependencies = ordinary_dependency_tables(manifest);
    for (fragment, authority) in [
        ("serde_json", "JSON"),
        ("worth-query", "Query"),
        ("worth_query", "Query"),
        ("worth-relational", "Relational"),
        ("worth_relational", "Relational"),
    ] {
        if dependencies.contains(fragment) {
            return Err(format!(
                "{} has forbidden ordinary {authority} dependency `{fragment}`",
                path.display()
            ));
        }
    }
    Ok(())
}

fn ordinary_dependency_tables(manifest: &str) -> String {
    let mut dependencies = String::new();
    let mut ordinary = false;
    for line in manifest.lines() {
        let trimmed = line.trim();
        if let Some(table) = trimmed
            .strip_prefix('[')
            .and_then(|table| table.strip_suffix(']'))
        {
            let table = table.trim();
            ordinary = table == "dependencies"
                || (table.starts_with("target.") && table.ends_with(".dependencies"));
        } else if ordinary {
            dependencies.push_str(line);
            dependencies.push('\n');
        }
    }
    dependencies
}

fn without_line_comments(source: &str) -> String {
    let mut code = String::with_capacity(source.len());
    for line in source.lines() {
        code.push_str(line.split_once("//").map_or(line, |(before, _)| before));
        code.push('\n');
    }
    code
}

fn localized_denial(path: &Path, source: &str, offset: usize, authority: &str) -> String {
    let line = source[..offset]
        .bytes()
        .filter(|byte| *byte == b'\n')
        .count()
        + 1;
    format!(
        "C.5.1 sealing gate: {authority} at {}:{line}",
        path.display()
    )
}

fn rust_sources(root: &Path) -> Result<Vec<PathBuf>, std::io::Error> {
    let mut pending = vec![root.to_path_buf()];
    let mut sources = Vec::new();
    while let Some(directory) = pending.pop() {
        for entry in std::fs::read_dir(directory)? {
            let path = entry?.path();
            if path.is_dir() {
                pending.push(path);
            } else if path.extension().and_then(|value| value.to_str()) == Some("rs") {
                sources.push(path);
            }
        }
    }
    sources.sort();
    Ok(sources)
}

#[test]
fn manifest_gate_covers_target_dependencies_but_not_dev_dependencies() {
    let target_forbidden = r#"
[dependencies]
worth-proof.workspace = true

[target.'cfg(windows)'.dependencies]
json-carrier = { package = "serde_json", version = "1" }
"#;
    assert!(inspect_ordinary_dependencies(Path::new("targeted.toml"), target_forbidden).is_err());

    let dev_only = r#"
[dependencies]
worth-proof.workspace = true

[dev-dependencies]
serde_json.workspace = true
"#;
    assert!(inspect_ordinary_dependencies(Path::new("dev-only.toml"), dev_only).is_ok());
}
