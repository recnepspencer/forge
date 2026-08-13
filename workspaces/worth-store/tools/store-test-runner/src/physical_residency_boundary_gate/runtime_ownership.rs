use std::path::Path;

use super::constructor_syntax::{constructor_calls, ConstructorSpec};
use super::production_source_graph::production_rust_sources as reachable_production_sources;
use super::workspace_source::{production_rust_sources, read, workspace_relative};
use crate::workspace_root;

const ALTERNATE_RESIDENCY_PATH: &str =
    "crates/worth-store/src/physical_runtime/record_serving/residency/alternate.rs";
const FRAME_PORTS_PATH: &str =
    "crates/worth-store/src/physical_runtime/record_serving/residency/frame_ports.rs";

const CONSTRUCTION_AUTHORITIES: &[(ConstructorSpec, &str)] = &[
    (
        ConstructorSpec {
            owner: "PhysicalResidencyPoolOwner",
            method: "open",
        },
        "crates/worth-store/src/physical_runtime/record_serving/residency/frame_ports.rs",
    ),
    (
        ConstructorSpec {
            owner: "PhysicalStoreWorkRuntime",
            method: "new",
        },
        "crates/worth-store/src/physical_runtime/instance/construction/work_runtime.rs",
    ),
    (
        ConstructorSpec {
            owner: "PhysicalSchedulerAdmissionOwner",
            method: "new",
        },
        "crates/worth-store/src/physical_runtime/instance/construction/work_runtime.rs",
    ),
    (
        ConstructorSpec {
            owner: "PhysicalSchedulerAdmissionOwner",
            method: "new_recovery",
        },
        "crates/worth-store/src/physical_runtime/recovery_coordination/owner.rs",
    ),
    (
        ConstructorSpec {
            owner: "SignalRuntime",
            method: "build_for",
        },
        "crates/worth-store/src/physical_runtime/instance/signal_owner/graph.rs",
    ),
];

const RAW_RESIDENCY_POOL: ConstructorSpec = ConstructorSpec {
    owner: "PhysicalResidencyPool",
    method: "open",
};

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
    let sources = reachable_production_sources(&workspace_root())
        .expect("discover Cargo-reachable Worth Store production sources");
    let specs: Vec<_> = CONSTRUCTION_AUTHORITIES
        .iter()
        .map(|(spec, _)| *spec)
        .collect();
    for &(constructor, expected_path) in CONSTRUCTION_AUTHORITIES {
        let mut sites = Vec::new();
        for source in &sources {
            let text = read(source).expect("read physical runtime source");
            let calls = constructor_calls(&text, &specs).unwrap_or_else(|denial| {
                panic!(
                    "constructor syntax at {}: {denial}",
                    workspace_relative(source)
                )
            });
            let expected = format!("{}::{}", constructor.owner, constructor.method);
            sites.extend(
                calls
                    .into_iter()
                    .filter(|call| call == &expected)
                    .map(|call| format!("{}:{call}", workspace_relative(source))),
            );
        }
        assert_eq!(
            sites,
            [format!(
                "{expected_path}:{}::{}",
                constructor.owner, constructor.method
            )],
            "{}::{} must have one production construction authority",
            constructor.owner,
            constructor.method
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
fn constructor_gate_rejects_direct_raw_and_alias_mutants() {
    let denial = inspect_constructor_site(
        Path::new(ALTERNATE_RESIDENCY_PATH),
        "let owner = PhysicalResidencyPoolOwner::open(store, limits)?;",
    )
    .expect_err("a second pool construction must be denied");
    assert!(denial.contains("second PhysicalResidencyPoolOwner::open"));

    let denial = inspect_constructor_site(
        Path::new(FRAME_PORTS_PATH),
        "let pool = PhysicalResidencyPool::open(store, limits)?;",
    )
    .expect_err("raw pool construction must not issue Store cleaning authority");
    assert!(denial.contains("raw PhysicalResidencyPool::open"));

    inspect_constructor_site(
        Path::new(FRAME_PORTS_PATH),
        "let owner = PhysicalResidencyPoolOwner::open(store, limits)?;",
    )
    .expect("the instance-owned pool construction must remain admitted");

    let denial = inspect_constructor_site(
        Path::new(ALTERNATE_RESIDENCY_PATH),
        "let owner = PhysicalResidencyPoolOwner /* alternate */ :: open (store, limits)?;",
    )
    .expect_err("token spacing and comments must not hide a second construction");
    assert!(denial.contains("second PhysicalResidencyPoolOwner::open"));

    let denial = inspect_constructor_site(
        Path::new(ALTERNATE_RESIDENCY_PATH),
        "use crate::PhysicalResidencyPoolOwner as PoolOwner; let owner = PoolOwner::open(store, limits)?;",
    )
    .expect_err("an imported constructor-owner alias must fail closed");
    assert!(denial.contains("owner alias"), "wrong denial: {denial}");

    let denial = inspect_constructor_site(
        Path::new("crates/worth-store-buffer-pool/src/physical_residency/alternate_owner.rs"),
        "pub fn alternate(store: StoreIdentity, limits: PhysicalResidencyLimits) { let _ = PhysicalResidencyPoolOwner::open(store, limits); }",
    )
    .expect_err("an out-of-runtime production owner must be denied");
    assert!(denial.contains("second PhysicalResidencyPoolOwner::open"));

    inspect_constructor_site(
        Path::new(ALTERNATE_RESIDENCY_PATH),
        "#[cfg(test)] fn controlled_only() { let _ = PhysicalResidencyPoolOwner::open(store, limits); }",
    )
    .expect("an exact test-only constructor must not become production evidence");
}

#[test]
fn constructor_gate_rejects_grouped_self_alias_mutants() {
    for grouped_alias in [
        "use PhysicalResidencyPoolOwner::{self as PoolOwner}; let owner = PoolOwner::open(store, limits)?;",
        "use crate::PhysicalResidencyPoolOwner::{self as PoolOwner}; let owner = PoolOwner::open(store, limits)?;",
        "use crate::{PhysicalResidencyPoolOwner::{self as PoolOwner}}; let owner = PoolOwner::open(store, limits)?;",
    ] {
        let denial = inspect_constructor_site(Path::new(ALTERNATE_RESIDENCY_PATH), grouped_alias)
            .expect_err("a grouped constructor-owner self alias must fail closed");
        assert!(denial.contains("owner alias"), "wrong denial: {denial}");
    }

    constructor_calls(
        "use UnrelatedModule::{self as Other}; let _ = Other::value();",
        &[CONSTRUCTION_AUTHORITIES[0].0],
    )
    .expect("an unrelated grouped self alias must remain admitted");
}

#[test]
fn constructor_gate_rejects_macro_expansion_mutants() {
    let denial = inspect_constructor_site(
        Path::new(ALTERNATE_RESIDENCY_PATH),
        "include!(concat!(env!(\"OUT_DIR\"), \"/alternate_owner.rs\"));",
    )
    .expect_err("generated Rust must not hide a constructor");
    assert!(denial.contains("computed Rust include"));

    let denial = inspect_constructor_site(
        Path::new(ALTERNATE_RESIDENCY_PATH),
        "macro_rules! open_pool { () => { PhysicalResidencyPoolOwner::open(store, limits) } } open_pool!();",
    )
    .expect_err("a macro-hidden governed constructor must fail closed");
    assert!(denial.contains("macro-carried"), "wrong denial: {denial}");

    for split_macro in [
        "macro_rules! call_open { ($owner:ident) => { $owner::open(store, limits) } } call_open!(PhysicalResidencyPoolOwner);",
        "macro_rules! call_method { ($method:ident) => { PhysicalResidencyPoolOwner::$method(store, limits) } } call_method!(open);",
    ] {
        let denial = inspect_constructor_site(
            Path::new(ALTERNATE_RESIDENCY_PATH),
            split_macro,
        )
        .expect_err("split macro constructor evidence must fail closed");
        assert!(denial.contains("macro-carried"), "wrong denial: {denial}");
    }

    let denial = inspect_constructor_site(
        Path::new(ALTERNATE_RESIDENCY_PATH),
        "impl PhysicalResidencyPoolOwner { fn alternate() { call_hidden_open!(); } }",
    )
    .expect_err("macros inside a governed owner impl must fail closed");
    assert!(
        denial.contains("cannot prove expansion"),
        "wrong denial: {denial}"
    );
}

#[test]
fn constructor_gate_rejects_associated_type_projections() {
    for projection_call in [
        "trait Types { type Owner; } struct Marker; impl Types for Marker { type Owner = PhysicalResidencyPoolOwner; } type Hidden = <Marker as Types>::Owner; fn attack() { let _ = Hidden::open(store, limits); }",
        "trait Types { type Owner; } struct Marker; impl Types for Marker { type Owner = PhysicalResidencyPoolOwner; } fn attack() { let _ = <Marker as Types>::Owner::open(store, limits); }",
    ] {
        let denial = inspect_constructor_site(
            Path::new(ALTERNATE_RESIDENCY_PATH),
            projection_call,
        )
        .expect_err("an associated-type constructor projection must fail closed");
        assert!(
            denial.contains("associated type") || denial.contains("projection"),
            "wrong denial: {denial}"
        );
    }
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
    let calls = constructor_calls(source, &[CONSTRUCTION_AUTHORITIES[0].0, RAW_RESIDENCY_POOL])?;
    if calls
        .iter()
        .any(|call| call == "PhysicalResidencyPool::open")
    {
        return Err(format!(
            "physical residency boundary: raw PhysicalResidencyPool::open at {}",
            path.display()
        ));
    }
    if calls
        .iter()
        .any(|call| call == "PhysicalResidencyPoolOwner::open")
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
