use std::path::Path;

use super::workspace_source::{
    occurrence_count, production_rust_sources, read, workspace_relative,
};
use crate::workspace_root;

const PHYSICAL_RUNTIME_ROOT: &str = "crates/worth-store/src/physical_runtime";

const CONSTRUCTION_AUTHORITIES: &[(&str, &str)] = &[
    (
        "PhysicalResidencyPoolOwner::open",
        "crates/worth-store/src/physical_runtime/record_serving/residency/frame_ports.rs",
    ),
    (
        "PhysicalStoreWorkRuntime::new",
        "crates/worth-store/src/physical_runtime/instance/construction/work_runtime.rs",
    ),
    (
        "PhysicalSchedulerAdmissionOwner::new",
        "crates/worth-store/src/physical_runtime/instance/construction/work_runtime.rs",
    ),
    (
        "SignalRuntime::build_for",
        "crates/worth-store/src/physical_runtime/instance/signal_owner/graph.rs",
    ),
];

const LOCAL_RUNTIME_FRAGMENTS: &[(&str, &str)] = &[
    ("PendingWorkRegistry", "pending-work registry"),
    (
        "HashMap<PhysicalWorkIdentity",
        "identity-keyed pending-work registry",
    ),
    ("OnceLock", "process-global runtime owner"),
    ("std::thread", "local worker thread"),
    ("thread::spawn", "local worker thread"),
    ("mpsc::", "local work channel"),
    ("crossbeam", "local work channel"),
    ("async_channel", "local work channel"),
    ("tokio::spawn", "local async worker"),
    ("TimerWheel", "local timer wheel"),
    ("RetryQueue", "local retry queue"),
    ("CallbackSettlement", "callback settlement route"),
    ("VecDeque", "local work queue"),
    ("BinaryHeap", "local scheduling queue"),
    ("Condvar", "local worker coordination"),
];

const CONSEQUENTIAL_LOCAL_RUNTIME_FRAGMENTS: &[(&str, &str)] = &[
    ("PendingWorkRegistry", "pending-work registry"),
    (
        "HashMap<PhysicalWorkIdentity",
        "identity-keyed pending-work registry",
    ),
    ("OnceLock", "process-global runtime owner"),
    ("std::thread", "local worker thread"),
    ("thread::spawn", "local worker thread"),
    ("mpsc::", "local work channel"),
    ("crossbeam", "local work channel"),
    ("async_channel", "local work channel"),
    ("tokio::spawn", "local async worker"),
    ("TimerWheel", "local timer wheel"),
    ("RetryQueue", "local retry queue"),
    ("CallbackSettlement", "callback settlement route"),
];

#[test]
fn production_has_one_constructor_for_each_runtime_authority() {
    let root = workspace_root().join(PHYSICAL_RUNTIME_ROOT);
    let sources = production_rust_sources(&root).expect("discover physical runtime sources");
    for &(constructor, expected_path) in CONSTRUCTION_AUTHORITIES {
        let mut sites = Vec::new();
        for source in &sources {
            let text = read(source).expect("read physical runtime source");
            let count = occurrence_count(&text, constructor);
            sites.extend(
                std::iter::repeat_n(workspace_relative(source), count)
                    .map(|path| format!("{path}:{constructor}")),
            );
        }
        assert_eq!(
            sites,
            [format!("{expected_path}:{constructor}")],
            "{constructor} must have one production construction authority"
        );
    }
}

#[test]
fn residency_composition_contains_no_local_work_runtime() {
    for relative in [
        "crates/worth-store/src/physical_runtime/record_serving/residency",
        "crates/worth-store-buffer-pool/src/physical_residency/speculation",
    ] {
        let root = workspace_root().join(relative);
        for source in production_rust_sources(&root).expect("discover residency composition") {
            let text = read(&source).expect("read residency composition");
            inspect_local_runtime_source(&source, &text)
                .unwrap_or_else(|denial| panic!("{denial}"));
        }
    }
}

#[test]
fn lower_owner_trees_contain_no_relocated_local_work_runtime() {
    for relative in [
        "crates/worth-store/src/physical_runtime/record_serving",
        "crates/worth-store-buffer-pool/src",
    ] {
        let root = workspace_root().join(relative);
        for source in production_rust_sources(&root).expect("discover lower-owner sources") {
            let text = read(&source).expect("read lower-owner source");
            inspect_consequential_local_runtime_source(&source, &text)
                .unwrap_or_else(|denial| panic!("{denial}"));
        }
    }
}

#[test]
fn constructor_gate_rejects_a_second_or_raw_residency_pool() {
    let denial = inspect_constructor_site(
        Path::new("crates/worth-store/src/physical_runtime/record_serving/residency/alternate.rs"),
        "let owner = PhysicalResidencyPoolOwner::open(store, limits)?;",
    )
    .expect_err("a second pool construction must be denied");
    assert!(denial.contains("second PhysicalResidencyPoolOwner::open"));

    let denial = inspect_constructor_site(
        Path::new(
            "crates/worth-store/src/physical_runtime/record_serving/residency/frame_ports.rs",
        ),
        "let pool = PhysicalResidencyPool::open(store, limits)?;",
    )
    .expect_err("raw pool construction must not issue Store cleaning authority");
    assert!(denial.contains("raw PhysicalResidencyPool::open"));

    inspect_constructor_site(
        Path::new(
            "crates/worth-store/src/physical_runtime/record_serving/residency/frame_ports.rs",
        ),
        "let owner = PhysicalResidencyPoolOwner::open(store, limits)?;",
    )
    .expect("the instance-owned pool construction must remain admitted");
}

#[test]
fn local_runtime_gate_rejects_consequential_runtime_families() {
    for (source, expected) in [
        (
            "static PENDING: OnceLock<Mutex<HashMap<PhysicalWorkIdentity, Work>>> = OnceLock::new();",
            "identity-keyed pending-work registry",
        ),
        (
            "let (sender, receiver) = std::sync::mpsc::channel();",
            "local work channel",
        ),
        (
            "std::thread::spawn(move || run_residency_work(receiver));",
            "local worker thread",
        ),
    ] {
        let denial = inspect_local_runtime_source(Path::new("controlled_mutant.rs"), source)
            .expect_err("a local runtime mutant must be denied");
        assert!(denial.contains(expected), "wrong denial: {denial}");
    }
}

#[test]
fn relocated_local_runtime_mutants_fail_outside_speculation_directories() {
    for (path, source, expected) in [
        (
            "crates/worth-store-buffer-pool/src/physical_residency/pool/background.rs",
            "std::thread::spawn(move || run_pool_background_work());",
            "local worker thread",
        ),
        (
            "crates/worth-store/src/physical_runtime/record_serving/access/pending.rs",
            "static PENDING: OnceLock<Mutex<HashMap<PhysicalWorkIdentity, Work>>> = OnceLock::new();",
            "identity-keyed pending-work registry",
        ),
        (
            "crates/worth-store-buffer-pool/src/physical_residency/pool/retry.rs",
            "let (sender, receiver) = std::sync::mpsc::channel();",
            "local work channel",
        ),
    ] {
        let denial = inspect_consequential_local_runtime_source(Path::new(path), source)
            .expect_err("relocated local runtime machinery must be denied");
        assert!(denial.contains(expected), "wrong denial: {denial}");
    }
}

fn inspect_constructor_site(path: &Path, source: &str) -> Result<(), String> {
    let expected =
        "crates/worth-store/src/physical_runtime/record_serving/residency/frame_ports.rs";
    if source.contains("PhysicalResidencyPool::open") {
        return Err(format!(
            "physical residency boundary: raw PhysicalResidencyPool::open at {}",
            path.display()
        ));
    }
    if source.contains("PhysicalResidencyPoolOwner::open")
        && path.to_string_lossy().replace('\\', "/") != expected
    {
        return Err(format!(
            "physical residency boundary: second PhysicalResidencyPoolOwner::open at {}",
            path.display()
        ));
    }
    Ok(())
}

fn inspect_local_runtime_source(path: &Path, source: &str) -> Result<(), String> {
    inspect_runtime_fragments(path, source, LOCAL_RUNTIME_FRAGMENTS)
}

fn inspect_consequential_local_runtime_source(path: &Path, source: &str) -> Result<(), String> {
    inspect_runtime_fragments(path, source, CONSEQUENTIAL_LOCAL_RUNTIME_FRAGMENTS)
}

fn inspect_runtime_fragments(
    path: &Path,
    source: &str,
    fragments: &[(&str, &str)],
) -> Result<(), String> {
    for &(fragment, authority) in fragments {
        if source.contains(fragment) {
            return Err(format!(
                "physical residency boundary: {authority} at {}",
                path.display()
            ));
        }
    }
    Ok(())
}
