use std::path::{Path, PathBuf};

use worth_store::physical_runtime::{AdmittedRecordPlacementPolicy, RecordAppendBatch};
use worth_store_offline_verifier::OfflineDurableManifestWalk;

use super::super::scenario_evidence::ScenarioProcessEvidence;
use super::result_codec::{parse_offline, parse_reopener, OfflineResult, ReopenedResult};
use super::FailureCase;

const DEATH_CASE_ENV: &str = "WORTH_STORE_C5_DEATH_CASE";

pub(super) struct ObservedPublicationDeath {
    _parent: tempfile::TempDir,
    pub(super) root: PathBuf,
    pub(super) case: FailureCase,
    pub(super) boundary: DeathBoundary,
    pub(super) artifacts_after_death: Vec<(String, u64)>,
    pub(super) partial_artifacts: Vec<String>,
    pub(super) observed_partial_role: Option<&'static str>,
    pub(super) catalog_before_reopen: Vec<u8>,
    pub(super) catalog_after_reopen: Vec<u8>,
    pub(super) reopened: ReopenedResult,
    pub(super) offline: OfflineResult,
    pub(super) walk: OfflineDurableManifestWalk,
    pub(super) placement: AdmittedRecordPlacementPolicy,
    pub(super) processes: [ScenarioProcessEvidence; 3],
}

pub(super) struct DeathBoundary {
    pub(super) role: String,
    pub(super) raw_ordinal: u64,
    pub(super) identified_ordinal: u64,
    pub(super) requested_bytes: u64,
}

pub(super) fn observe(case: FailureCase) -> ObservedPublicationDeath {
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join(case.name);
    seed_prior_root(&root);

    let writer_stdout = run_death_writer(&root, case);
    let boundary = parse_death_boundary(&writer_stdout);
    let artifacts_after_death = record_artifacts(&root);
    let partial_artifacts = one_byte_candidate_artifacts(&root);
    let observed_partial_role = partial_artifacts.first().map(|path| {
        if path.contains("/segments/") && path.ends_with(".pages") {
            "segment-page"
        } else if path.contains("/extents/") && path.ends_with(".data") {
            "extent-data"
        } else {
            "unexpected-artifact"
        }
    });
    let writer = ScenarioProcessEvidence::parse_child(&writer_stdout, "faulting-writer");

    let catalog_before_reopen = std::fs::read(catalog_path(&root)).unwrap();
    let reopener_stdout =
        super::super::child_process::run_child("publication_reopener", &root, None);
    let reopener = ScenarioProcessEvidence::parse_child(&reopener_stdout, "fresh-reopener");
    let reopened = parse_reopener(&reopener_stdout);
    let catalog_after_reopen = std::fs::read(catalog_path(&root)).unwrap();

    let observer_stdout = super::super::observer::run(&root);
    let observer = ScenarioProcessEvidence::offline_process(
        &observer_stdout,
        &super::super::observer::binary_path(),
    );
    let offline = parse_offline(&observer_stdout);
    let (format, placement, _) = super::super::configuration();
    let walk = worth_store_offline_verifier::walk_current_durable_record_manifest(
        &root,
        format.declaration(),
    )
    .unwrap();

    ObservedPublicationDeath {
        _parent: parent,
        root,
        case,
        boundary,
        artifacts_after_death,
        partial_artifacts,
        observed_partial_role,
        catalog_before_reopen,
        catalog_after_reopen,
        reopened,
        offline,
        walk,
        placement,
        processes: [writer, reopener, observer],
    }
}

fn parse_death_boundary(stdout: &str) -> DeathBoundary {
    let fields = completion_fields(stdout, "C5_PUBLICATION_DEATH ");
    assert_eq!(fields.len(), 4);
    DeathBoundary {
        role: fields[0].to_owned(),
        raw_ordinal: fields[1].parse().unwrap(),
        identified_ordinal: fields[2].parse().unwrap(),
        requested_bytes: fields[3].parse().unwrap(),
    }
}

fn one_byte_candidate_artifacts(root: &Path) -> Vec<String> {
    record_artifacts(root)
        .into_iter()
        .filter_map(|(path, bytes)| (bytes == 1).then_some(path))
        .collect()
}

fn record_artifacts(root: &Path) -> Vec<(String, u64)> {
    let mut pending = vec![root.join("families/records")];
    let mut paths = Vec::new();
    while let Some(directory) = pending.pop() {
        for entry in std::fs::read_dir(directory).unwrap() {
            let entry = entry.unwrap();
            if entry.file_type().unwrap().is_dir() {
                pending.push(entry.path());
            } else {
                paths.push((
                    entry
                        .path()
                        .strip_prefix(root)
                        .unwrap()
                        .to_string_lossy()
                        .replace('\\', "/"),
                    entry.metadata().unwrap().len(),
                ));
            }
        }
    }
    paths.sort();
    paths
}

fn seed_prior_root(root: &Path) {
    let (_, placement, _) = super::super::configuration();
    let serving = super::super::serving_from_initialization(root);
    serving
        .record_submission()
        .append_batch(
            RecordAppendBatch::try_from_iter([b"prior".as_slice()]).unwrap(),
            placement,
        )
        .unwrap();
    serving.close();
}

fn run_death_writer(root: &Path, case: FailureCase) -> String {
    let encoded = format!(
        "{},{},{},{}",
        case.role_name, case.append_ordinal, case.directive, case.payload_bytes
    );
    let output = super::super::child_process::child_command("publication_death_writer", root)
        .env(DEATH_CASE_ENV, encoded)
        .output()
        .unwrap();
    assert_eq!(
        output.status.code(),
        Some(86),
        "{}\nstdout:\n{}\nstderr:\n{}",
        case.name,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
    String::from_utf8(output.stdout).unwrap()
}

fn completion_fields<'output>(stdout: &'output str, prefix: &str) -> Vec<&'output str> {
    stdout
        .lines()
        .find_map(|line| line.strip_prefix(prefix))
        .unwrap_or_else(|| panic!("child output omitted `{prefix}` completion"))
        .split_whitespace()
        .collect()
}

fn catalog_path(root: &Path) -> PathBuf {
    root.join("families/records/bootstrap.catalog")
}
