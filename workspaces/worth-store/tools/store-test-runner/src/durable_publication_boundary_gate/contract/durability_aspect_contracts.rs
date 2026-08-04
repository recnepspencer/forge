use super::super::read_repository_document;

const SEMANTICS: &str =
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/record_serving/work_semantics/mod.rs";
const POLICY: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/record_serving/work_semantics/durability/policy_binding_basis.rs";
const MUTATIONS: &[(&str, &str, &str)] = &[
    (
        "wal append",
        "workspaces/worth-store/crates/worth-store/src/physical_runtime/record_serving/work_semantics/durability/wal_append_basis.rs",
        "store.physical.durability.wal-append-basis",
    ),
    (
        "WAL barrier",
        "workspaces/worth-store/crates/worth-store/src/physical_runtime/record_serving/work_semantics/durability/wal_barrier_basis.rs",
        "store.physical.durability.wal-barrier-basis",
    ),
    (
        "checkpoint capture",
        "workspaces/worth-store/crates/worth-store/src/physical_runtime/record_serving/work_semantics/durability/checkpoint_capture_basis.rs",
        "store.physical.durability.checkpoint-capture-basis",
    ),
    (
        "root publication",
        "workspaces/worth-store/crates/worth-store/src/physical_runtime/record_serving/work_semantics/durability/root_publication_basis.rs",
        "store.physical.durability.root-publication-basis",
    ),
    (
        "WAL reclamation",
        "workspaces/worth-store/crates/worth-store/src/physical_runtime/record_serving/work_semantics/durability/wal_reclamation_basis.rs",
        "store.physical.durability.wal-reclamation-basis",
    ),
];

#[test]
fn c7_durability_aspects_are_six_exact_store_partitioned_contracts() {
    inspect(&sources()).unwrap_or_else(|denial| panic!("{denial}"));
}

#[test]
fn durability_aspect_gate_rejects_unpartitioned_ephemeral_missing_and_wrong_key_mutants() {
    let source = sources();
    let mut unpartitioned = source.clone();
    unpartitioned.semantics =
        unpartitioned
            .semantics
            .replacen(".with_partition(", ".without_partition(", 1);
    inspect(&unpartitioned).expect_err("unpartitioned mutation aspects must fail");

    let mut ephemeral = source.clone();
    ephemeral.semantics = ephemeral.semantics.replacen(
        "format!(\"physical-durability-store/{store}\")",
        "format!(\"physical-durability-store/{store}/{}\", durability.runtime_identity().get())",
        1,
    );
    inspect(&ephemeral).expect_err("ephemeral runtime identity must not destabilize the profile");

    let mut missing = source.clone();
    missing.mutations.pop();
    inspect(&missing).expect_err("a missing durability aspect contract must fail");

    let mut wrong_key = source;
    wrong_key.mutations[1].1 =
        wrong_key.mutations[1]
            .1
            .replacen("wal-barrier-basis", "barrier-basis", 1);
    inspect(&wrong_key).expect_err("a generic barrier aspect key must fail");
}

#[derive(Clone)]
struct Sources {
    semantics: String,
    policy: String,
    mutations: Vec<(&'static str, String)>,
}

fn sources() -> Sources {
    Sources {
        semantics: read(SEMANTICS),
        policy: read(POLICY),
        mutations: MUTATIONS
            .iter()
            .map(|(name, path, _)| (*name, read(path)))
            .collect(),
    }
}

fn read(path: &str) -> String {
    read_repository_document(path).unwrap_or_else(|error| panic!("{error}"))
}

fn inspect(source: &Sources) -> Result<(), &'static str> {
    let semantics = compact(&source.semantics);
    for required in [
        "letstore_partition=durability_store_partition(durability)",
        "physical-durability-store/{store}",
        "fnpartitioned_dependency_and_output_declaration(",
        ".with_partition(worth_signal::facade::PartitionSubscription::whole_partition(partition))",
    ] {
        if !semantics.contains(required) {
            return Err("C7 mutation aspect partition ownership is incomplete");
        }
    }
    if semantics.contains("runtime_identity().get()") {
        return Err("C7 mutation aspect partition contains ephemeral runtime identity");
    }

    let policy = compact(&source.policy);
    for required in [
        "store.physical.durability.policy-binding-basis",
        "PhysicalSignalAspectRole::Dependency",
        ".with_partition(PartitionSubscription::whole_partition(policy_partition))",
    ] {
        if !policy.contains(required) {
            return Err("durability policy aspect lost its exact projection partition");
        }
    }

    if source.mutations.len() != MUTATIONS.len() {
        return Err("C7 durability aspect inventory is incomplete");
    }
    for ((expected_name, _, key), (actual_name, source)) in MUTATIONS.iter().zip(&source.mutations)
    {
        if expected_name != actual_name {
            return Err("C7 durability aspect inventory order drifted");
        }
        let source = compact(source);
        for required in [
            *key,
            "PhysicalSignalAspectRole::DependencyAndOutput",
            "partitioned_dependency_and_output_declaration(admission,",
            "installed.declaration.partition().unwrap().partition.0",
        ] {
            if !source.contains(required) {
                return Err("a C7 mutation aspect lost exact key role or partition proof");
            }
        }
    }
    Ok(())
}

fn compact(source: &str) -> String {
    source
        .chars()
        .filter(|character| !character.is_whitespace())
        .collect()
}
