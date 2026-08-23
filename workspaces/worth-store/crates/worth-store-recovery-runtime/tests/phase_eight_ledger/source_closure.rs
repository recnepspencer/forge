use std::collections::BTreeSet;
use std::path::Path;

use sha2::{Digest, Sha256};

pub(super) const SOURCE_CLOSURE_SHA256: &str =
    "eb9e48f282f5c65467ef403d7ba5e666dc8b0b8774a2a70d57ffe86b91787572";
const RETAINED_MUTATION_REPORT: &str =
    "_docs/worth-store/physical-reconstruction-c8-phase-8-mutants.json";

const REVERSE_CLOSURE_ROOTS: [&str; 11] = [
    "workspaces/worth-store/crates/worth-store-offline-verifier/src/c8_recovery_observation",
    "workspaces/worth-store/crates/worth-store-recovery-physics/src",
    "workspaces/worth-store/crates/worth-store-recovery-runtime/src/observation",
    "workspaces/worth-store/crates/worth-store-recovery-runtime/tests/phase_eight_process",
    "workspaces/worth-store/crates/worth-store/src/bin/physical_store_c8_writer",
    "workspaces/worth-store/crates/worth-store-physical-format/src/checkpoint",
    "workspaces/worth-store/crates/worth-store-formal-models/src/protocol_bindings",
    "workspaces/worth-store/tools/store-test-runner/src/fresh_process_recovery_boundary_gate",
    "workspaces/worth-store/tools/store-process-bundle/src",
    "workspaces/worth-store/crates/worth-store-recovery-runtime/tests/phase_eight_ledger",
    "workspaces/worth-store/crates/worth-store/tests/physical_record_journeys/physical_work/phase_16_lifecycle_maelstrom/mutant_report",
];

const REVERSE_CLOSURE_FILES: [&str; 59] = [
    "workspaces/worth-store/crates/worth-store-recovery-runtime/src/bin/physical_store_recover.rs",
    "workspaces/worth-store/crates/worth-store-recovery-runtime/src/bin/physical_store_recover/arguments.rs",
    "workspaces/worth-store/crates/worth-store-recovery-runtime/src/bin/physical_store_recover/arguments/options.rs",
    "workspaces/worth-store/crates/worth-store-recovery-runtime/src/bin/physical_store_recover/terminal.rs",
    "workspaces/worth-store/crates/worth-store-offline-verifier/src/bin/physical_store_offline_observer.rs",
    "workspaces/worth-store/crates/worth-store-offline-verifier/src/bin/physical_store_offline_observer/recovery_report.rs",
    "workspaces/worth-store/crates/worth-store/src/bin/physical_store_c8_writer.rs",
    "workspaces/worth-store/crates/worth-store/src/bin/physical_store_c8_writer/admission.rs",
    "workspaces/worth-store/crates/worth-store/src/bin/physical_store_c8_writer/configuration.rs",
    "workspaces/worth-store/crates/worth-store/src/bin/physical_store_c8_writer/operation_program.rs",
    "workspaces/worth-store/crates/worth-store-offline-verifier/src/c8_recovery_observation/report_wire.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/durability/checkpoint/yieldpoint.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/durability/lifecycle/yieldpoint.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/record_serving/publication/director/durable_data/effect_progression.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/record_serving/residency/publication_artifacts.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/record_serving/residency/candidate_frame_residency/write_progression.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/record_serving/residency/candidate_frame_residency/writeback_progression.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/recovery_yieldpoint.rs",
    "workspaces/worth-store/crates/worth-store-formal-models/tests/binding_completeness.rs",
    "workspaces/worth-store/crates/worth-store-recovery-runtime/tests/phase_eight_process.rs",
    "workspaces/worth-store/crates/worth-store-recovery-runtime/tests/phase_eight_process/support_binaries.rs",
    "workspaces/worth-store/crates/worth-store-recovery-runtime/tests/phase_eight_process/support_binary_feature_sets.rs",
    "workspaces/worth-store/crates/worth-store-recovery-runtime/tests/phase_eight_process/support_binary_freshness.rs",
    "workspaces/worth-store/tools/store-test-runner/src/courtroom_campaign/executable_binding.rs",
    "workspaces/worth-store/tools/store-test-runner/src/courtroom_campaign/bounded_residency_siege.rs",
    "workspaces/worth-store/tools/store-test-runner/src/courtroom_campaign/bounded_residency_siege/execution.rs",
    "workspaces/worth-store/crates/worth-store/src/bin/physical_store_work_courtroom/bounded_residency/producer.rs",
    "workspaces/worth-store/crates/worth-store/src/bin/physical_store_work_courtroom/bounded_residency/serving.rs",
    "workspaces/worth-store/crates/worth-store/src/bin/physical_store_work_courtroom/bounded_residency/writeback_pressure.rs",
    "workspaces/worth-store/crates/worth-store/src/bin/physical_store_work_courtroom/bounded_residency/writeback_pressure/append_pressure.rs",
    "workspaces/worth-store/crates/worth-store/src/bin/physical_store_work_courtroom/bounded_residency/writeback_pressure/dispatch_coordination.rs",
    "workspaces/worth-store/crates/worth-store/src/bin/physical_store_work_courtroom/bounded_residency/speculative_pressure/read_pressure/prefetch.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/instance/work_runtime.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/instance/executor/yieldpoint.rs",
    "workspaces/worth-store/tools/store-test-runner/src/courtroom_campaign/bounded_residency_siege/c7_crash_campaign.rs",
    "workspaces/worth-store/tools/store-test-runner/src/courtroom_campaign/bounded_residency_siege/c7_crash_campaign/recovery.rs",
    "workspaces/worth-store/tools/store-test-runner/src/courtroom_campaign/bounded_residency_siege/evidence_projection/c8_recovery.rs",
    "workspaces/worth-store/tools/store-test-runner/src/courtroom_campaign/bounded_residency_siege/oracle/c8_recovery.rs",
    "workspaces/worth-store/tools/store-test-runner/src/mutation_campaign/mod.rs",
    "workspaces/worth-store/tools/store-test-runner/src/mutation_campaign/tests.rs",
    "workspaces/worth-store/tools/store-test-runner/src/mutation_campaign/source_binding_tests.rs",
    "workspaces/worth-store/tools/store-test-runner/src/mutation_campaign/evidence.rs",
    "workspaces/worth-store/tools/store-test-runner/src/mutation_campaign/execution/artifact_evidence.rs",
    "workspaces/worth-store/tools/store-test-runner/src/mutation_campaign/process_execution.rs",
    "workspaces/worth-store/tools/store-test-runner/src/mutation_campaign/source_replacement.rs",
    "workspaces/worth-store/tools/store-test-runner/src/mutation_campaign/workspace_snapshot.rs",
    "workspaces/worth-store/tools/store-test-runner/src/mutation_campaign/c8_retained_record.rs",
    "workspaces/worth-store/tools/store-test-runner/src/mutation_campaign/report.rs",
    "workspaces/worth-store/tools/store-test-runner/src/mutation_campaign/catalog/phase_16.rs",
    "workspaces/worth-store/tools/store-test-runner/src/main.rs",
    "workspaces/worth-store/tools/store-test-runner/src/lib.rs",
    "workspaces/worth-store/tools/store-test-runner/src/arguments.rs",
    "workspaces/worth-store/tools/store-test-runner/src/arguments/parsing.rs",
    "workspaces/worth-store/tools/store-test-runner/src/arguments/tests.rs",
    "workspaces/worth-store/crates/worth-store/tests/physical_record_journeys/physical_work/concurrency.rs",
    "crates/worth-signal/src/data/graph/topology/mutation/cleanup.rs",
    "workspaces/worth-store/crates/worth-store/tests/physical_record_journeys/physical_work/phase_16_lifecycle_maelstrom/mutant_report.rs",
    "workspaces/worth-store/tools/store-test-runner/src/phase_eight_process_suite.rs",
    "workspaces/worth-store/tools/store-test-runner/src/phase_eight_process_suite/child.rs",
];

pub(super) fn validate(root: &Path, closure: &str, expected: &BTreeSet<String>) {
    let mut covered = BTreeSet::new();
    let mut unique = BTreeSet::new();
    assert_eq!(
        closure.lines().next(),
        Some("guarantee,source,relation,source_sha256")
    );
    for line in closure.lines().skip(1).filter(|line| !line.is_empty()) {
        let columns = line.split(',').collect::<Vec<_>>();
        assert_eq!(columns.len(), 4, "invalid Phase 8 closure row {line}");
        assert!(expected.contains(columns[0]), "foreign guarantee {line}");
        assert!(!columns[2].is_empty(), "missing relation {line}");
        reject_derived_evidence_source(columns[1]);
        assert_eq!(
            columns[3],
            normalized_source_sha256(&root.join(columns[1])),
            "source content hash drifted for {}",
            columns[1]
        );
        assert!(unique.insert(line), "duplicate closure row {line}");
        let source = Path::new(columns[1]);
        assert!(!source.is_absolute() && !source.components().any(|part| part.as_os_str() == ".."));
        assert!(root.join(source).is_file(), "missing source {}", columns[1]);
        covered.insert(columns[0].to_owned());
    }
    assert_eq!(
        &covered, expected,
        "every guarantee needs a causal source family"
    );
}

fn reject_derived_evidence_source(source: &str) {
    assert_ne!(
        source, RETAINED_MUTATION_REPORT,
        "retained mutation evidence is derived output, not a causal source"
    );
}

pub(super) fn assert_reverse_complete(root: &Path, closure: &str) {
    let sources = closure_sources(closure);
    for relative_root in REVERSE_CLOSURE_ROOTS {
        let root_path = root.join(relative_root);
        let mut discovered = Vec::new();
        collect_rust_sources(root, &root_path, &mut discovered);
        assert!(
            !discovered.is_empty(),
            "reverse closure root is empty: {relative_root}"
        );
        for source in discovered {
            assert!(
                sources.contains(&source),
                "Phase 8 source closure omitted live Rust source {source}"
            );
        }
    }
    for source in REVERSE_CLOSURE_FILES {
        assert!(
            root.join(source).is_file(),
            "missing reverse closure file {source}"
        );
        assert!(
            sources.contains(source),
            "Phase 8 source closure omitted causal source {source}"
        );
    }
}

pub(super) fn digest(closure: &str) -> String {
    let normalized = closure.replace("\r\n", "\n");
    hex(&Sha256::digest(normalized.as_bytes()).into())
}

fn closure_sources(closure: &str) -> BTreeSet<String> {
    closure
        .lines()
        .skip(1)
        .filter(|line| !line.is_empty())
        .filter_map(|line| line.split(',').nth(1))
        .map(str::to_owned)
        .collect()
}

fn collect_rust_sources(repository_root: &Path, directory: &Path, output: &mut Vec<String>) {
    for entry in std::fs::read_dir(directory).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            collect_rust_sources(repository_root, &path, output);
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            output.push(
                path.strip_prefix(repository_root)
                    .unwrap()
                    .to_string_lossy()
                    .replace('\\', "/"),
            );
        }
    }
}

fn normalized_source_sha256(path: &Path) -> String {
    let bytes = std::fs::read(path).unwrap();
    let normalized = String::from_utf8_lossy(&bytes).replace("\r\n", "\n");
    let canonical = if path
        .to_string_lossy()
        .replace('\\', "/")
        .ends_with("_docs/worth-store/physical-reconstruction-c8-phase-8-closure-ledger.md")
    {
        canonical_phase_eight_ledger(&normalized)
    } else if path
        .to_string_lossy()
        .replace('\\', "/")
        .ends_with("tests/phase_eight_ledger/source_closure.rs")
    {
        canonical_source_closure(&normalized)
    } else if path
        .to_string_lossy()
        .replace('\\', "/")
        .ends_with("tests/phase_eight_ledger/exact_source_map.rs")
    {
        canonical_exact_source_map(&normalized)
    } else {
        normalized
    };
    hex(&Sha256::digest(canonical.as_bytes()).into())
}

fn canonical_source_closure(normalized: &str) -> String {
    canonical_bound_digest(normalized, "pub(super) const SOURCE_CLOSURE_SHA256")
}

fn canonical_exact_source_map(normalized: &str) -> String {
    canonical_bound_digest(normalized, "pub(super) const SOURCE_MAP_SHA256")
}

fn canonical_bound_digest(normalized: &str, declaration: &str) -> String {
    let mut skip_bound_digest = false;
    normalized
        .lines()
        .map(|line| {
            if line.contains(declaration) {
                skip_bound_digest = true;
                "<bound-digest-declaration>"
            } else if skip_bound_digest && line.trim().starts_with('"') {
                skip_bound_digest = false;
                "<bound-digest-value>"
            } else {
                line
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn hex(bytes: &[u8; 32]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn canonical_phase_eight_ledger(normalized: &str) -> String {
    let trailing_newline = normalized.ends_with('\n');
    let mut lines = Vec::new();
    for line in normalized.lines() {
        if line == "## Independent audit history" {
            break;
        }
        lines.push(if line.starts_with("Source closure SHA-256: ") {
            "Source closure SHA-256: <bound-source-closure-digest>"
        } else {
            line
        });
    }
    let mut canonical = lines.join("\n");
    if trailing_newline {
        canonical.push('\n');
    }
    canonical
}

#[cfg(test)]
mod tests {
    use super::{
        canonical_exact_source_map, canonical_phase_eight_ledger, canonical_source_closure,
        reject_derived_evidence_source, RETAINED_MUTATION_REPORT,
    };

    #[test]
    fn retained_mutation_report_cannot_enter_the_causal_source_manifest() {
        assert!(std::panic::catch_unwind(|| reject_derived_evidence_source(
            RETAINED_MUTATION_REPORT
        ))
        .is_err());
        reject_derived_evidence_source(
            "workspaces/worth-store/tools/store-test-runner/src/mutation_campaign/report.rs",
        );
    }

    #[test]
    fn ledger_digest_and_audit_history_are_not_self_binding_inputs() {
        let base = "Source closure SHA-256: one\n| C8-P8-RUNTIME-REPORT-01 | old requirement |\n## Independent audit history\n| old audit |\n";
        let changed = "Source closure SHA-256: two\n| C8-P8-RUNTIME-REPORT-01 | old requirement |\n## Independent audit history\n| new audit |\n";
        let requirement_changed = "Source closure SHA-256: two\n| C8-P8-RUNTIME-REPORT-01 | new requirement |\n## Independent audit history\n| new audit |\n";

        assert_eq!(
            canonical_phase_eight_ledger(base),
            canonical_phase_eight_ledger(changed)
        );
        assert_ne!(
            canonical_phase_eight_ledger(base),
            canonical_phase_eight_ledger(requirement_changed)
        );
    }

    #[test]
    fn source_closure_digest_binding_is_ignored_but_validation_logic_is_not() {
        let base = "pub(super) const SOURCE_CLOSURE_SHA256: &str =\n\"one\";\nfn validate() {}\n";
        let changed_digest =
            "pub(super) const SOURCE_CLOSURE_SHA256: &str =\n\"two\";\nfn validate() {}\n";
        let changed_logic = "pub(super) const SOURCE_CLOSURE_SHA256: &str =\n\"two\";\nfn validate() { panic!(); }\n";

        assert_eq!(
            canonical_source_closure(base),
            canonical_source_closure(changed_digest)
        );
        assert_ne!(
            canonical_source_closure(base),
            canonical_source_closure(changed_logic)
        );
    }

    #[test]
    fn exact_source_map_digest_binding_is_ignored_but_validation_logic_is_not() {
        let base = "pub(super) const SOURCE_MAP_SHA256: &str =\n\"one\";\nfn validate() {}\n";
        let changed_digest =
            "pub(super) const SOURCE_MAP_SHA256: &str =\n\"two\";\nfn validate() {}\n";
        let changed_logic =
            "pub(super) const SOURCE_MAP_SHA256: &str =\n\"two\";\nfn validate() { panic!(); }\n";

        assert_eq!(
            canonical_exact_source_map(base),
            canonical_exact_source_map(changed_digest)
        );
        assert_ne!(
            canonical_exact_source_map(base),
            canonical_exact_source_map(changed_logic)
        );
    }
}
