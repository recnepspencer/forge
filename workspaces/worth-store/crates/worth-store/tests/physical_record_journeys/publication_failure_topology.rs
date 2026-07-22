use std::path::Path;

use worth_store::physical_runtime::RecordAppendBatch;
use worth_store_physical_backend::MediaOperationRole;

use super::scenario_evidence::{ScenarioEvidence, ScenarioPredicate, ScenarioProcessEvidence};
use super::{configuration, serving_from_open};

const DEATH_CASE_ENV: &str = "WORTH_STORE_C5_DEATH_CASE";

#[test]
fn publication_cutover_never_invents_current_truth() {
    for case in failure_cases() {
        exercise_process_death(case);
    }
}

#[derive(Clone, Copy)]
struct FailureCase {
    name: &'static str,
    role: MediaOperationRole,
    role_name: &'static str,
    append_ordinal: u64,
    payload_bytes: usize,
    directive: &'static str,
    expected_generation: u64,
    expected_records: usize,
    expected_residue: bool,
}

fn failure_cases() -> [FailureCase; 6] {
    [
        failure(
            "short-data-write",
            MediaOperationRole::PositionedWrite,
            "positioned-write",
            1,
            9,
            "prefix",
        ),
        failure(
            "extent-truncation",
            MediaOperationRole::PositionedWrite,
            "positioned-write",
            1,
            20_000,
            "prefix",
        ),
        failure(
            "data-sync",
            MediaOperationRole::SynchronizeFileState,
            "file-sync",
            1,
            9,
            "before",
        ),
        failure(
            "manifest-write",
            MediaOperationRole::PositionedWrite,
            "positioned-write",
            2,
            9,
            "before",
        ),
        failure(
            "post-manifest-pre-catalog",
            MediaOperationRole::PositionedWrite,
            "positioned-write",
            7,
            9,
            "before",
        ),
        FailureCase {
            name: "post-catalog-pre-directory-sync",
            role: MediaOperationRole::AtomicReplace,
            role_name: "atomic-replace",
            append_ordinal: 1,
            payload_bytes: 9,
            directive: "after",
            expected_generation: 3,
            expected_records: 2,
            expected_residue: false,
        },
    ]
}

const fn failure(
    name: &'static str,
    role: MediaOperationRole,
    role_name: &'static str,
    append_ordinal: u64,
    payload_bytes: usize,
    directive: &'static str,
) -> FailureCase {
    FailureCase {
        name,
        role,
        role_name,
        append_ordinal,
        payload_bytes,
        directive,
        expected_generation: 2,
        expected_records: 1,
        expected_residue: true,
    }
}

fn exercise_process_death(case: FailureCase) {
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join(case.name);
    seed_prior_root(&root);
    let baseline = serving_from_open(&root);
    let role_ordinal = baseline
        .media_counters()
        .attempts_for(case.role)
        .saturating_add(case.append_ordinal);
    baseline.close();

    let writer_stdout = run_death_writer(&root, case, role_ordinal);
    let boundary = parse_death_boundary(&writer_stdout);
    assert_eq!(boundary.role, case.role.metric_name(), "{}", case.name);
    assert_eq!(boundary.ordinal, role_ordinal, "{}", case.name);
    let expected_partial_artifacts = usize::from(case.directive == "prefix");
    let partial_artifacts = one_byte_candidate_artifacts(&root);
    assert_eq!(
        partial_artifacts.len(),
        expected_partial_artifacts,
        "{} completed byte prefix",
        case.name
    );
    let expected_partial_role = expected_partial_role(case);
    let observed_partial_role = partial_artifacts.first().map(|path| {
        if path.contains("/segments/") && path.ends_with(".pages") {
            "segment-page"
        } else if path.contains("/extents/") && path.ends_with(".data") {
            "extent-data"
        } else {
            "unexpected-artifact"
        }
    });
    assert_eq!(
        observed_partial_role, expected_partial_role,
        "{} partial write reached the wrong artifact family",
        case.name
    );
    let writer = ScenarioProcessEvidence::parse_child(&writer_stdout, "faulting-writer");

    let catalog_before_reopen = std::fs::read(catalog_path(&root)).unwrap();
    let reopener_stdout = super::child_process::run_child("publication_reopener", &root, None);
    let reopener = ScenarioProcessEvidence::parse_child(&reopener_stdout, "fresh-reopener");
    let reopened = parse_reopener(&reopener_stdout);
    assert_eq!(
        std::fs::read(catalog_path(&root)).unwrap(),
        catalog_before_reopen
    );

    let observer_stdout = super::observer::run(&root);
    let observer =
        ScenarioProcessEvidence::offline_process(&observer_stdout, &super::observer::binary_path());
    let offline = parse_offline(&observer_stdout);
    let (format, placement, _) = configuration();
    let walk = worth_store_offline_verifier::walk_current_durable_record_manifest(
        &root,
        format.declaration(),
    )
    .unwrap();

    assert_eq!(
        reopened.generation, case.expected_generation,
        "C5_PREDICATE:independent-decision-path {}",
        case.name
    );
    assert_eq!(
        reopened.records, case.expected_records,
        "C5_PREDICATE:independent-decision-path {}",
        case.name
    );
    assert_eq!(reopened.residue, case.expected_residue, "{}", case.name);
    assert_eq!(
        offline.generation, case.expected_generation,
        "{}",
        case.name
    );
    assert_eq!(offline.records, case.expected_records, "{}", case.name);
    assert_eq!(
        walk.root_generation(),
        case.expected_generation,
        "{}",
        case.name
    );
    assert_eq!(
        walk.placements().len(),
        case.expected_records,
        "{}",
        case.name
    );

    let processes = [writer, reopener, observer];
    let predicates = [
        ScenarioPredicate::equality(
            "runtime_root",
            case.expected_generation,
            reopened.generation,
        ),
        ScenarioPredicate::equality("offline_root", case.expected_generation, offline.generation),
        ScenarioPredicate::equality(
            "runtime_record_count",
            case.expected_records as u64,
            reopened.records as u64,
        ),
        ScenarioPredicate::equality(
            "offline_record_count",
            case.expected_records as u64,
            offline.records as u64,
        ),
        ScenarioPredicate::equality("residue_posture", case.expected_residue, reopened.residue),
        ScenarioPredicate::equality(
            "close_added_no_publication_effect",
            catalog_before_reopen,
            std::fs::read(catalog_path(&root)).unwrap(),
        ),
        ScenarioPredicate::equality("interposer_role", case.role.metric_name(), boundary.role),
        ScenarioPredicate::equality("interposer_ordinal", role_ordinal, boundary.ordinal),
        ScenarioPredicate::equality(
            "completed_byte_prefix",
            expected_partial_artifacts as u64,
            partial_artifacts.len() as u64,
        ),
        ScenarioPredicate::equality(
            "completed_prefix_role",
            expected_partial_role,
            observed_partial_role,
        ),
    ];
    super::scenario_evidence::emit(ScenarioEvidence {
        courtroom: "publication_cutover_never_invents_current_truth",
        world: case.name,
        root: &root,
        seed: 0xC5C5_0000_0000_0001,
        action_trace: &[
            "seed-prior",
            "kill-at-interposer",
            "fresh-reopen",
            "offline-process",
        ],
        authority_transitions: &[
            "prior-root-published",
            "writer-process-died",
            "fresh-runtime-readmitted",
        ],
        walk: &walk,
        placement,
        publication_identity: None,
        processes: &processes,
        counters: serde_json::json!({
            "killed_role": case.role.metric_name(),
            "killed_ordinal": role_ordinal,
            "payload_bytes": case.payload_bytes,
            "requested_bytes_at_death": boundary.requested_bytes,
            "completed_prefix_bytes": expected_partial_artifacts,
            "completed_prefix_artifacts": partial_artifacts,
        }),
        runtime_result: serde_json::json!({
            "root_generation": reopened.generation,
            "records": reopened.records,
            "residue": reopened.residue,
        }),
        oracle_result: serde_json::json!({
            "root_generation": case.expected_generation,
            "records": case.expected_records,
            "residue": case.expected_residue,
        }),
        mutant_posture: "production-interposer-process-death",
        predicates: &predicates,
    });
}

struct DeathBoundary<'output> {
    role: &'output str,
    ordinal: u64,
    requested_bytes: u64,
}

fn parse_death_boundary(stdout: &str) -> DeathBoundary<'_> {
    let fields = completion_fields(stdout, "C5_PUBLICATION_DEATH ");
    assert_eq!(fields.len(), 3);
    DeathBoundary {
        role: fields[0],
        ordinal: fields[1].parse().unwrap(),
        requested_bytes: fields[2].parse().unwrap(),
    }
}

fn expected_partial_role(case: FailureCase) -> Option<&'static str> {
    match (case.directive, case.payload_bytes > 8 * 1024) {
        ("prefix", false) => Some("segment-page"),
        ("prefix", true) => Some("extent-data"),
        _ => None,
    }
}

fn one_byte_candidate_artifacts(root: &Path) -> Vec<String> {
    let mut pending = vec![root.join("families/records")];
    let mut paths = Vec::new();
    while let Some(directory) = pending.pop() {
        for entry in std::fs::read_dir(directory).unwrap() {
            let entry = entry.unwrap();
            if entry.file_type().unwrap().is_dir() {
                pending.push(entry.path());
            } else if entry.metadata().unwrap().len() == 1 {
                paths.push(
                    entry
                        .path()
                        .strip_prefix(root)
                        .unwrap()
                        .to_string_lossy()
                        .replace('\\', "/"),
                );
            }
        }
    }
    paths.sort();
    paths
}

fn seed_prior_root(root: &Path) {
    let (_, placement, _) = configuration();
    let mut serving = super::serving_from_initialization(root);
    serving
        .records_mut()
        .append_batch(
            RecordAppendBatch::try_from_iter([b"prior".as_slice()]).unwrap(),
            placement,
        )
        .unwrap();
    serving.close();
}

fn run_death_writer(root: &Path, case: FailureCase, ordinal: u64) -> String {
    let encoded = format!(
        "{},{},{},{}",
        case.role_name, ordinal, case.directive, case.payload_bytes
    );
    let output = std::process::Command::new(std::env::current_exe().unwrap())
        .args(["--exact", "child_process::c5_child_role", "--nocapture"])
        .env("WORTH_STORE_C5_CHILD_ROLE", "publication_death_writer")
        .env("WORTH_STORE_C5_CHILD_ROOT", root)
        .env(DEATH_CASE_ENV, encoded)
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(86), "{}", case.name);
    String::from_utf8(output.stdout).unwrap()
}

struct ReopenedResult {
    generation: u64,
    records: usize,
    residue: bool,
}

fn parse_reopener(stdout: &str) -> ReopenedResult {
    let fields = completion_fields(stdout, "C5_PUBLICATION_REOPEN ");
    assert_eq!(fields.len(), 3);
    ReopenedResult {
        generation: fields[0].parse().unwrap(),
        records: fields[1].parse().unwrap(),
        residue: fields[2].parse().unwrap(),
    }
}

struct OfflineResult {
    generation: u64,
    records: usize,
}

fn parse_offline(stdout: &str) -> OfflineResult {
    let fields = completion_fields(stdout, "C5_OFFLINE ");
    assert_eq!(fields.len(), 10);
    OfflineResult {
        generation: fields[1].parse().unwrap(),
        records: fields[2].parse().unwrap(),
    }
}

fn completion_fields<'output>(stdout: &'output str, prefix: &str) -> Vec<&'output str> {
    stdout
        .lines()
        .find_map(|line| line.strip_prefix(prefix))
        .unwrap_or_else(|| panic!("child output omitted `{prefix}` completion"))
        .split_whitespace()
        .collect()
}

fn catalog_path(root: &Path) -> std::path::PathBuf {
    root.join("families/records/bootstrap.catalog")
}
