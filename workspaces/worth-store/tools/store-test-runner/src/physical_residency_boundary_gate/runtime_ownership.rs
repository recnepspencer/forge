use std::path::Path;

use super::workspace_source::{
    occurrence_count, production_rust_sources, read, workspace_relative,
};
use crate::workspace_root;

const PHYSICAL_RUNTIME_ROOT: &str = "crates/worth-store/src/physical_runtime";

const CONSTRUCTION_AUTHORITIES: &[(&str, &str)] = &[
    (
        "PhysicalResidencyPool::open",
        "crates/worth-store/src/physical_runtime/record_serving/residency/frame_ports.rs",
    ),
    (
        "PhysicalStoreWorkRuntime::new",
        "crates/worth-store/src/physical_runtime/instance/construction.rs",
    ),
    (
        "PhysicalSchedulerAdmissionOwner::new",
        "crates/worth-store/src/physical_runtime/instance/construction.rs",
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
        "crates/worth-store/src/physical_runtime/record_serving/c6_handoff",
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
fn constructor_gate_rejects_a_second_residency_pool() {
    let denial = inspect_constructor_site(
        Path::new("crates/worth-store/src/physical_runtime/record_serving/residency/alternate.rs"),
        "let pool = PhysicalResidencyPool::open(store, limits)?;",
    )
    .expect_err("a second pool construction must be denied");
    assert!(denial.contains("second PhysicalResidencyPool::open"));

    inspect_constructor_site(
        Path::new(
            "crates/worth-store/src/physical_runtime/record_serving/residency/frame_ports.rs",
        ),
        "let pool = PhysicalResidencyPool::open(store, limits)?;",
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

fn inspect_constructor_site(path: &Path, source: &str) -> Result<(), String> {
    let expected =
        "crates/worth-store/src/physical_runtime/record_serving/residency/frame_ports.rs";
    if source.contains("PhysicalResidencyPool::open")
        && path.to_string_lossy().replace('\\', "/") != expected
    {
        return Err(format!(
            "physical residency boundary: second PhysicalResidencyPool::open at {}",
            path.display()
        ));
    }
    Ok(())
}

fn inspect_local_runtime_source(path: &Path, source: &str) -> Result<(), String> {
    for &(fragment, authority) in LOCAL_RUNTIME_FRAGMENTS {
        if source.contains(fragment) {
            return Err(format!(
                "physical residency boundary: {authority} at {}",
                path.display()
            ));
        }
    }
    Ok(())
}
