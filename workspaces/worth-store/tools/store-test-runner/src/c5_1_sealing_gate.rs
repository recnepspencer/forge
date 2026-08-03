use std::path::{Path, PathBuf};

use super::workspace_root;

mod ordinary_product_graph;

const ORDINARY_SOURCE_ROOTS: [&str; 9] = [
    "crates/worth-store/src/physical_runtime",
    "crates/worth-store-blob-chunks/src",
    "crates/worth-store-maintenance/src",
    "crates/worth-store-test-support/src",
    "crates/worth-store-io-scheduler/src",
    "crates/worth-store-buffer-pool/src",
    "crates/worth-store-physical-backend/src",
    "crates/worth-store-physical-integrity/src",
    "crates/worth-store-recovery-physics/src",
];

const ORDINARY_SOURCE_ALLOWLIST: [&str; 1] =
    ["crates/worth-store-test-support/src/compiler_boundary"];

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
    (
        "LocalPhysicalWorkScheduler",
        "local physical-work scheduler",
    ),
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
fn ordinary_work_sources_keep_aspect_and_semantic_authority_at_typed_boundaries() {
    for root in ORDINARY_SOURCE_ROOTS {
        for source in rust_sources(&workspace_root().join(root)).expect("read production sources") {
            if source_is_allowlisted(&source) {
                continue;
            }
            let text = std::fs::read_to_string(&source).expect("read production source");
            inspect_ordinary_source(&source, &text).unwrap_or_else(|denial| panic!("{denial}"));
        }
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
        (
            "struct LocalPhysicalWorkScheduler;",
            "local physical-work scheduler",
        ),
    ] {
        let denial = inspect_ordinary_source(Path::new("controlled_mutant.rs"), source)
            .expect_err("controlled authority bypass must be denied");
        assert!(denial.contains(expected), "wrong denial: {denial}");
    }
}

#[test]
fn every_ordinary_source_product_rejects_representative_bypasses() {
    for root in ORDINARY_SOURCE_ROOTS {
        let path = Path::new(root).join("hostile_insertion.rs");
        for source in [
            "let aspect = Aspect::try_new(7)?;",
            "let packet: serde_json::Value = decode(bytes)?;",
            "let writers = BranchWriterRegistry::new();",
        ] {
            inspect_ordinary_source(&path, source)
                .expect_err("hostile source insertion must be denied in every ordinary product");
        }
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

fn without_line_comments(source: &str) -> String {
    let mut code = String::with_capacity(source.len());
    for line in source.lines() {
        code.push_str(line.split_once("//").map_or(line, |(before, _)| before));
        code.push('\n');
    }
    code
}

fn source_is_allowlisted(source: &Path) -> bool {
    ORDINARY_SOURCE_ALLOWLIST
        .iter()
        .map(|relative| workspace_root().join(relative))
        .any(|allowed| source.starts_with(allowed))
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
