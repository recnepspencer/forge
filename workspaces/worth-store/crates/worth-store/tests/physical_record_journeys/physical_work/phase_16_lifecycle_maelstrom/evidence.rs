use std::{
    fmt::Write,
    num::NonZeroU32,
    path::{Path, PathBuf},
};

use sha2::{Digest, Sha256};
use worth_store::physical_runtime::{
    PhysicalWorkArtifactBinding, PhysicalWorkCourtroomBinding, PhysicalWorkCourtroomEvidence,
    PhysicalWorkCourtroomFinding, PhysicalWorkCourtroomRunBinding, PhysicalWorkEvidenceDigest,
    PhysicalWorkExecutionContext, PhysicalWorkOracleEvidence, PhysicalWorkProcessEvidence,
    PhysicalWorkRunEnvironmentEvidence, PhysicalWorkSourceBinding,
};

use super::{fresh_process::FreshReopenObservation, mutant_report};

const SEED: u64 = 0x0c50_116a;
const SCHEDULE: &str =
    "read-patch,policy-denial,predispatch-cancel,reverse-read-completion,clock-retry,writeback,capacity-siege,dispatched-close,fresh-reopen";
const SOURCE_MANIFEST_SCHEMA: &str = "worth.store.c5_1.phase16-a.source-manifest.v1";
const SOURCE_FILES: [(&str, &[u8]); 15] = [
    (
        "../phase_16_lifecycle_maelstrom.rs",
        include_bytes!("../phase_16_lifecycle_maelstrom.rs"),
    ),
    (
        "append_preparation.rs",
        include_bytes!("append_preparation.rs"),
    ),
    (
        "../courtroom_environment.rs",
        include_bytes!("../courtroom_environment.rs"),
    ),
    ("evidence.rs", include_bytes!("evidence.rs")),
    ("fixture.rs", include_bytes!("fixture.rs")),
    ("fresh_process.rs", include_bytes!("fresh_process.rs")),
    ("joined_execution.rs", include_bytes!("joined_execution.rs")),
    ("mutant_report.rs", include_bytes!("mutant_report.rs")),
    (
        "mutant_report/campaign_source.rs",
        include_bytes!("mutant_report/campaign_source.rs"),
    ),
    (
        "mutant_report/decoding.rs",
        include_bytes!("mutant_report/decoding.rs"),
    ),
    ("shutdown_trace.rs", include_bytes!("shutdown_trace.rs")),
    ("terminal_labels.rs", include_bytes!("terminal_labels.rs")),
    (
        "terminal_projection.rs",
        include_bytes!("terminal_projection.rs"),
    ),
    ("workflows.rs", include_bytes!("workflows.rs")),
    ("world.rs", include_bytes!("world.rs")),
];

fn run_binding(
    child: NonZeroU32,
    environment: PhysicalWorkRunEnvironmentEvidence,
) -> PhysicalWorkCourtroomRunBinding {
    let source_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/physical_record_journeys/physical_work/phase_16_lifecycle_maelstrom/source_manifest.txt");
    let source = PhysicalWorkSourceBinding::new(
        source_path.display().to_string(),
        digest(source_manifest()),
    )
    .unwrap();
    let binary_path = std::env::current_exe().unwrap();
    let binary = PhysicalWorkSourceBinding::new(
        binary_path.display().to_string(),
        digest(&std::fs::read(&binary_path).unwrap()),
    )
    .unwrap();
    let execution = PhysicalWorkExecutionContext::new(
        SEED,
        SCHEDULE,
        [
            PhysicalWorkProcessEvidence::active_evidence_producer(
                "maelstrom-evidence-producer",
                NonZeroU32::new(std::process::id()).unwrap(),
            )
            .unwrap(),
            PhysicalWorkProcessEvidence::exited_success("fresh-reopener", child).unwrap(),
        ],
    )
    .unwrap();
    PhysicalWorkCourtroomRunBinding::new(source, binary, execution, environment)
}

pub(super) fn finish(
    binding: PhysicalWorkCourtroomBinding,
    environment: PhysicalWorkRunEnvironmentEvidence,
    root: &Path,
    fresh: &FreshReopenObservation,
    expected_generation: u64,
    expected_records: &[&[u8]],
) -> PhysicalWorkCourtroomEvidence {
    let accepted = fresh.root_generation == expected_generation
        && sorted(fresh.records.clone())
            == sorted(
                expected_records
                    .iter()
                    .map(|bytes| bytes.to_vec())
                    .collect(),
            );
    let oracle = PhysicalWorkOracleEvidence::new(
        "predeclared-root-generation-and-record-set",
        accepted,
        oracle_digest(expected_generation, expected_records),
    )
    .unwrap();
    let mutants = mutant_report::load();
    let sealing = mutants.is_some();
    let evidence = binding
        .finish(
            run_binding(fresh.process, environment),
            artifact_bindings(root),
            oracle,
            mutants.unwrap_or_default(),
        )
        .unwrap();
    if sealing {
        assert_eq!(
            evidence.mutants().len(),
            mutant_report::complete_mutant_count()
        );
        assert!(
            evidence.verdict().accepted(),
            "sealing Courtroom A must produce accepted physical-work evidence: {:?}",
            evidence.verdict().findings()
        );
    } else {
        assert_eq!(
            evidence.verdict().findings(),
            &[PhysicalWorkCourtroomFinding::MissingMutantLocalization],
            "focused Courtroom A must leave only the phase-level mutation join open"
        );
    }
    assert_eq!(evidence.causal_overflow(), 0);
    assert!(evidence.backend_profile().is_some());
    assert!(evidence
        .run()
        .execution()
        .processes()
        .iter()
        .any(|process| process.process() == fresh.process));
    assert_exact_shutdown(&evidence);
    super::terminal_projection::publish_if_requested(&evidence);
    evidence
}

fn assert_exact_shutdown(evidence: &PhysicalWorkCourtroomEvidence) {
    let shutdown = evidence.shutdown();
    assert_eq!(shutdown.stage_counts(), [0; 6]);
    assert_eq!(shutdown.residual(), 0);
    assert_eq!(shutdown.unaccounted_terminal(), 0);
    assert_eq!(shutdown.drain_residual(), 0);
    assert_eq!(shutdown.drain_evidence_overflow(), 0);
    assert_eq!(
        shutdown.declared(),
        shutdown.drain_counts().into_iter().sum::<u64>()
    );
}

fn artifact_bindings(root: &Path) -> Vec<PhysicalWorkArtifactBinding> {
    let mut pending = vec![root.to_path_buf()];
    let mut files = Vec::new();
    while let Some(directory) = pending.pop() {
        let mut entries = std::fs::read_dir(directory)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .collect::<Vec<_>>();
        entries.sort();
        for path in entries {
            if path.is_dir() {
                pending.push(path);
            } else {
                files.push(path);
            }
        }
    }
    files.sort();
    files
        .into_iter()
        .map(|path| artifact_binding(root, path))
        .collect()
}

fn artifact_binding(root: &Path, path: PathBuf) -> PhysicalWorkArtifactBinding {
    let bytes = std::fs::read(&path).unwrap();
    let relative = path
        .strip_prefix(root)
        .unwrap()
        .to_string_lossy()
        .replace('\\', "/");
    PhysicalWorkArtifactBinding::new(relative, bytes.len() as u64, digest(&bytes)).unwrap()
}

fn oracle_digest(generation: u64, records: &[&[u8]]) -> PhysicalWorkEvidenceDigest {
    let mut records = records.to_vec();
    records.sort();
    let mut hasher = Sha256::new();
    hasher.update(generation.to_le_bytes());
    for record in records {
        hasher.update((record.len() as u64).to_le_bytes());
        hasher.update(record);
    }
    PhysicalWorkEvidenceDigest::new(hasher.finalize().into()).unwrap()
}

fn source_manifest() -> &'static [u8] {
    let manifest = include_str!("source_manifest.txt");
    let mut expected = format!("{SOURCE_MANIFEST_SCHEMA}\n");
    for (path, bytes) in SOURCE_FILES {
        let source = std::str::from_utf8(bytes).unwrap().replace("\r\n", "\n");
        writeln!(
            &mut expected,
            "{:x}  {path}",
            Sha256::digest(source.as_bytes())
        )
        .unwrap();
    }
    assert_eq!(
        manifest.replace("\r\n", "\n"),
        expected,
        "Courtroom A source manifest must match every execution-bearing module"
    );
    manifest.as_bytes()
}

fn digest(bytes: &[u8]) -> PhysicalWorkEvidenceDigest {
    PhysicalWorkEvidenceDigest::new(Sha256::digest(bytes).into()).unwrap()
}

fn sorted(mut records: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    records.sort();
    records
}

#[test]
fn source_manifest_matches_every_courtroom_a_module() {
    assert!(!source_manifest().is_empty());
}
