use std::collections::BTreeSet;

mod semantic_contract;

use super::documents::{
    read_repository_document, split_csv, AUTHORITY_TRACE, DESTINATION_TOPOLOGY,
};
use semantic_contract::{expected_phase, expected_responsibility};

const HEADER: &str = "path,owner,responsibility,dependency_posture,phase,status";
const REQUIRED_DESTINATIONS: &[&str] = &[
    "crates/worth-store-recovery-runtime",
    "crates/worth-store-recovery-runtime/Cargo.toml",
    "crates/worth-store-recovery-runtime/README.md",
    "crates/worth-store-recovery-runtime/src/lib.rs",
    "crates/worth-store-recovery-runtime/src/bin/physical_store_recover.rs",
    "crates/worth-store-recovery-runtime/src/entry/mod.rs",
    "crates/worth-store-recovery-runtime/src/entry/request.rs",
    "crates/worth-store-recovery-runtime/src/entry/admission.rs",
    "crates/worth-store-recovery-runtime/src/entry/authority.rs",
    "crates/worth-store-recovery-runtime/src/entry/authority_binding.rs",
    "crates/worth-store-recovery-runtime/src/entry/session.rs",
    "crates/worth-store-recovery-runtime/src/entry/outcome.rs",
    "crates/worth-store-recovery-runtime/src/progression/mod.rs",
    "crates/worth-store-recovery-runtime/src/progression/admitted.rs",
    "crates/worth-store-recovery-runtime/src/progression/discovered.rs",
    "crates/worth-store-recovery-runtime/src/progression/selected.rs",
    "crates/worth-store-recovery-runtime/src/progression/planned.rs",
    "crates/worth-store-recovery-runtime/src/progression/staged.rs",
    "crates/worth-store-recovery-runtime/src/progression/published.rs",
    "crates/worth-store-recovery-runtime/src/progression/reopened.rs",
    "crates/worth-store-recovery-runtime/src/orchestration/staging/performed_write.rs",
    "crates/worth-store-recovery-runtime/src/orchestration/publication/performed_root_replacement.rs",
    "crates/worth-store-recovery-runtime/src/orchestration/publication/performed_namespace_sync.rs",
    "crates/worth-store-recovery-runtime/src/orchestration/reopen/performed_independent_reopen.rs",
    "crates/worth-store-recovery-runtime/src/cleanup/performed_removal.rs",
    "crates/worth-store-recovery-runtime/src/orchestration/mod.rs",
    "crates/worth-store-recovery-runtime/src/orchestration/discovery.rs",
    "crates/worth-store-recovery-runtime/src/orchestration/planning.rs",
    "crates/worth-store-recovery-runtime/src/orchestration/staging.rs",
    "crates/worth-store-recovery-runtime/src/orchestration/publication.rs",
    "crates/worth-store-recovery-runtime/src/orchestration/reopen.rs",
    "crates/worth-store-recovery-runtime/src/handoff/mod.rs",
    "crates/worth-store-recovery-runtime/src/handoff/operation_fates.rs",
    "crates/worth-store-recovery-runtime/src/handoff/unsupported_scope.rs",
    "crates/worth-store-recovery-runtime/src/handoff/cleanup_posture.rs",
    "crates/worth-store-recovery-runtime/src/cleanup/mod.rs",
    "crates/worth-store-recovery-runtime/src/cleanup/plan.rs",
    "crates/worth-store-recovery-runtime/src/cleanup/eligibility.rs",
    "crates/worth-store-recovery-runtime/src/cleanup/execution.rs",
    "crates/worth-store-recovery-runtime/src/observation/mod.rs",
    "crates/worth-store-recovery-runtime/src/observation/counters.rs",
    "crates/worth-store-recovery-runtime/src/observation/protocol.rs",
    "crates/worth-store-recovery-runtime/src/observation/report.rs",
    "crates/worth-store-recovery-physics/src/lib.rs",
    "crates/worth-store-recovery-physics/src/source_precedence/mod.rs",
    "crates/worth-store-recovery-physics/src/source_precedence/candidate.rs",
    "crates/worth-store-recovery-physics/src/source_precedence/admission.rs",
    "crates/worth-store-recovery-physics/src/source_precedence/current_previous_root.rs",
    "crates/worth-store-recovery-physics/src/source_precedence/checkpoint_base.rs",
    "crates/worth-store-recovery-physics/src/source_precedence/wal_tail.rs",
    "crates/worth-store-recovery-physics/src/source_precedence/compaction_product.rs",
    "crates/worth-store-recovery-physics/src/source_precedence/residue.rs",
    "crates/worth-store-recovery-physics/src/source_precedence/selection.rs",
    "crates/worth-store-recovery-physics/src/wal_prefix/mod.rs",
    "crates/worth-store-recovery-physics/src/wal_prefix/continuity.rs",
    "crates/worth-store-recovery-physics/src/wal_prefix/valid_prefix.rs",
    "crates/worth-store-recovery-physics/src/wal_prefix/torn_tail.rs",
    "crates/worth-store-recovery-physics/src/wal_prefix/denial.rs",
    "crates/worth-store-recovery-physics/src/redo_replay/mod.rs",
    "crates/worth-store-recovery-physics/src/redo_replay/record.rs",
    "crates/worth-store-recovery-physics/src/redo_replay/plan.rs",
    "crates/worth-store-recovery-physics/src/redo_replay/cursor.rs",
    "crates/worth-store-recovery-physics/src/redo_replay/denial.rs",
    "crates/worth-store-recovery-physics/src/page_redo/mod.rs",
    "crates/worth-store-recovery-physics/src/page_redo/page_lsn.rs",
    "crates/worth-store-recovery-physics/src/page_redo/eligibility.rs",
    "crates/worth-store-recovery-physics/src/page_redo/transition.rs",
    "crates/worth-store-recovery-physics/src/page_redo/denial.rs",
    "crates/worth-store-recovery-physics/src/operation_reconciliation/mod.rs",
    "crates/worth-store-recovery-physics/src/operation_reconciliation/identity.rs",
    "crates/worth-store-recovery-physics/src/operation_reconciliation/evidence_join.rs",
    "crates/worth-store-recovery-physics/src/operation_reconciliation/binding_freshness.rs",
    "crates/worth-store-recovery-physics/src/operation_reconciliation/fate.rs",
    "crates/worth-store-recovery-physics/src/operation_reconciliation/denial.rs",
    "crates/worth-store-recovery-physics/src/recovery_budget/mod.rs",
    "crates/worth-store-recovery-physics/src/recovery_budget/limits.rs",
    "crates/worth-store-recovery-physics/src/recovery_budget/plan_cost.rs",
    "crates/worth-store-recovery-physics/src/recovery_budget/counters.rs",
    "crates/worth-store-recovery-physics/src/recovery_budget/denial.rs",
    "crates/worth-store/src/physical_runtime/recovery_freshness/mod.rs",
    "crates/worth-store/src/physical_runtime/recovery_freshness/port.rs",
    "crates/worth-store/src/physical_runtime/recovery_freshness/authority.rs",
    "crates/worth-store/src/physical_runtime/recovery_freshness/binding.rs",
    "crates/worth-store/src/physical_runtime/recovery_freshness/cleanup.rs",
    "crates/worth-store/src/physical_runtime/recovery_construction/mod.rs",
    "crates/worth-store/src/physical_runtime/recovery_construction/port.rs",
    "crates/worth-store/src/physical_runtime/recovery_construction/authority.rs",
    "crates/worth-store/src/physical_runtime/recovery_construction/runtime_identity.rs",
    "crates/worth-store/src/physical_runtime/recovery_construction/handoff.rs",
    "crates/worth-store/src/bin/physical_store_work_courtroom/c8_recovery_writer.rs",
    "crates/worth-store-offline-verifier/src/c8_recovery_observation/mod.rs",
    "crates/worth-store-offline-verifier/src/c8_recovery_observation/artifact_walk.rs",
    "crates/worth-store-offline-verifier/src/c8_recovery_observation/physical_format.rs",
    "crates/worth-store-offline-verifier/src/c8_recovery_observation/conclusion.rs",
    "crates/worth-store-offline-verifier/src/c8_recovery_observation/report_protocol.rs",
    "crates/worth-store-offline-verifier/src/c8_recovery_observation/report.rs",
    "crates/worth-store-physical-certification/src/c8_fresh_process_recovery/mod.rs",
    "crates/worth-store-physical-certification/src/c8_fresh_process_recovery/scenario.rs",
    "crates/worth-store-physical-certification/src/c8_fresh_process_recovery/writer_process.rs",
    "crates/worth-store-physical-certification/src/c8_fresh_process_recovery/recovery_process.rs",
    "crates/worth-store-physical-certification/src/c8_fresh_process_recovery/observer_process.rs",
    "crates/worth-store-physical-certification/src/c8_fresh_process_recovery/crash_matrix.rs",
    "crates/worth-store-physical-certification/src/c8_fresh_process_recovery/oracle.rs",
    "crates/worth-store-physical-certification/src/c8_fresh_process_recovery/schedules/mod.rs",
    "crates/worth-store-physical-certification/src/c8_fresh_process_recovery/schedules/perturbation.rs",
    "crates/worth-store-physical-certification/src/c8_fresh_process_recovery/mutations/mod.rs",
    "crates/worth-store-physical-certification/src/c8_fresh_process_recovery/mutations/corpus.rs",
];

#[test]
fn destination_topology_has_one_exact_semantic_home_per_c8_axis() {
    let document =
        read_repository_document(DESTINATION_TOPOLOGY).expect("read C.8 destination topology");
    let rows = parse_topology(&document).expect("parse C.8 destination topology");
    let actual = rows
        .iter()
        .map(|row| row.path.as_str())
        .collect::<BTreeSet<_>>();
    assert_eq!(
        actual,
        REQUIRED_DESTINATIONS.iter().copied().collect(),
        "C.8 destination topology is incomplete or contains a competing home"
    );
}

#[test]
fn planned_dependencies_preserve_reconstruction_only_direction() {
    let document =
        read_repository_document(DESTINATION_TOPOLOGY).expect("read C.8 destination topology");
    let rows = parse_topology(&document).expect("parse C.8 destination topology");
    let runtime = row(&rows, "crates/worth-store-recovery-runtime");
    assert_eq!(
        runtime.dependency_posture,
        "imports-physics-and-store-reconstruction-port"
    );
    let physics = row(
        &rows,
        "crates/worth-store-recovery-physics/src/source_precedence/mod.rs",
    );
    assert_eq!(physics.dependency_posture, "pure-no-store-signal-query");
    let store = row(
        &rows,
        "crates/worth-store/src/physical_runtime/recovery_construction/mod.rs",
    );
    assert_eq!(store.dependency_posture, "no-physics-or-replay-import");
    let observer = row(
        &rows,
        "crates/worth-store-offline-verifier/src/c8_recovery_observation/mod.rs",
    );
    assert_eq!(
        observer.dependency_posture,
        "read-only-no-runtime-decisions"
    );
}

#[test]
fn authority_session_effect_and_handoff_definition_homes_are_exact() {
    let document = read_repository_document(DESTINATION_TOPOLOGY).expect("read C.8 topology");
    let rows = parse_topology(&document).expect("parse C.8 topology");
    let exact = [
        (
            "crates/worth-store-recovery-runtime/src/entry/authority_binding.rs",
            "recovery-runtime/entry/authority_binding",
        ),
        (
            "crates/worth-store-recovery-runtime/src/entry/session.rs",
            "recovery-runtime/entry/session",
        ),
        (
            "crates/worth-store/src/physical_runtime/recovery_construction/handoff.rs",
            "worth-store/recovery-construction/handoff",
        ),
    ];
    for (path, owner) in exact {
        assert_eq!(row(&rows, path).owner, owner);
    }
    let effect_homes = [
        ("orchestration/staging/performed_write.rs", "phase-5"),
        (
            "orchestration/publication/performed_root_replacement.rs",
            "phase-6",
        ),
        (
            "orchestration/publication/performed_namespace_sync.rs",
            "phase-6",
        ),
        (
            "orchestration/reopen/performed_independent_reopen.rs",
            "phase-6",
        ),
        ("cleanup/performed_removal.rs", "phase-7"),
    ];
    for (suffix, phase) in effect_homes {
        let effect = rows
            .iter()
            .find(|row| row.path.ends_with(suffix))
            .unwrap_or_else(|| panic!("missing performed-effect home {suffix}"));
        assert_eq!(effect.phase, phase);
    }
    let trace = read_repository_document(AUTHORITY_TRACE).expect("read C.8 authority trace");
    let trace_freshness_owners = trace
        .lines()
        .skip(1)
        .map(|line| split_csv(line, 7).expect("parse authority trace row"))
        .filter(|columns| columns[0] == "freshness-policy")
        .map(|columns| columns[2].to_owned())
        .collect::<BTreeSet<_>>();
    let topology_freshness_owners = rows
        .iter()
        .filter(|row| {
            row.path
                .ends_with("/physical_runtime/recovery_freshness/binding.rs")
                || row
                    .path
                    .ends_with("/physical_runtime/recovery_freshness/cleanup.rs")
        })
        .map(|row| row.owner.clone())
        .collect::<BTreeSet<_>>();
    assert_eq!(trace_freshness_owners, topology_freshness_owners);
    for (path, phase) in [
        (
            "crates/worth-store/src/physical_runtime/recovery_freshness/port.rs",
            "phase-2",
        ),
        (
            "crates/worth-store/src/physical_runtime/recovery_freshness/authority.rs",
            "phase-2",
        ),
    ] {
        assert_eq!(row(&rows, path).phase, phase);
    }
    assert!(!rows.iter().any(|row| row
        .path
        .ends_with("recovery-runtime/src/handoff/recovered_physical_runtime.rs")));
}

#[test]
fn topology_rows_have_specific_owners_and_phase_honest_status() {
    let document =
        read_repository_document(DESTINATION_TOPOLOGY).expect("read C.8 destination topology");
    let rows = parse_topology(&document).expect("parse C.8 destination topology");
    for row in rows {
        assert!(!matches!(
            row.owner.as_str(),
            "recovery" | "physics" | "support" | "evidence" | "utility"
        ));
        assert!(!row.responsibility.contains(" and "));
        assert_eq!(row.responsibility, expected_responsibility(&row.path));
        assert_eq!(row.phase, expected_phase(&row.path));
        assert!(matches!(
            row.phase.as_str(),
            "phase-2"
                | "phase-3"
                | "phase-4"
                | "phase-5"
                | "phase-6"
                | "phase-7"
                | "phase-8"
                | "phase-9"
        ));
        assert!(matches!(
            row.status.as_str(),
            "create" | "preserve" | "narrow" | "replace"
        ));
    }
}

#[test]
fn same_stem_and_page_redo_phase_substitutions_are_rejected() {
    assert_ne!(
        expected_responsibility("crates/worth-store-recovery-physics/src/redo_replay/plan.rs"),
        expected_responsibility("crates/worth-store-recovery-runtime/src/cleanup/plan.rs")
    );
    assert_ne!(
        expected_responsibility(
            "crates/worth-store-recovery-physics/src/source_precedence/admission.rs"
        ),
        expected_responsibility("crates/worth-store-recovery-runtime/src/entry/admission.rs")
    );
    assert_eq!(
        expected_phase("crates/worth-store-recovery-physics/src/page_redo/eligibility.rs"),
        "phase-4"
    );
    assert_eq!(
        expected_phase("crates/worth-store-recovery-physics/src/page_redo/transition.rs"),
        "phase-5"
    );
}

fn row<'a>(rows: &'a [TopologyRow], path: &str) -> &'a TopologyRow {
    rows.iter()
        .find(|row| row.path == path)
        .unwrap_or_else(|| panic!("missing C.8 destination `{path}`"))
}

fn parse_topology(document: &str) -> Result<Vec<TopologyRow>, String> {
    let mut lines = document.lines();
    if lines.next() != Some(HEADER) {
        return Err("C.8 destination topology has an invalid schema".into());
    }
    lines
        .filter(|line| !line.trim().is_empty())
        .enumerate()
        .map(|(index, line)| {
            let columns = split_csv(line, 6)
                .map_err(|error| format!("C.8 topology row {}: {error}", index + 2))?;
            Ok(TopologyRow {
                path: columns[0].to_owned(),
                owner: columns[1].to_owned(),
                responsibility: columns[2].to_owned(),
                dependency_posture: columns[3].to_owned(),
                phase: columns[4].to_owned(),
                status: columns[5].to_owned(),
            })
        })
        .collect()
}

struct TopologyRow {
    path: String,
    owner: String,
    responsibility: String,
    dependency_posture: String,
    phase: String,
    status: String,
}
