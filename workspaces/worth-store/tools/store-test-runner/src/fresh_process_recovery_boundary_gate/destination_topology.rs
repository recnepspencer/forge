use std::collections::BTreeSet;

mod required_destinations;
mod semantic_contract;
mod semantic_tests;

use super::documents::{
    read_repository_document, split_csv, AUTHORITY_TRACE, DESTINATION_TOPOLOGY,
};
use semantic_contract::{expected_phase, expected_responsibility};

const HEADER: &str = "path,owner,responsibility,dependency_posture,phase,status";
use required_destinations::REQUIRED_DESTINATIONS;

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
    assert_core_definition_homes(&rows);
    assert_effect_homes(&rows);
    assert_freshness_homes(&rows);
    assert!(!rows.iter().any(|row| row
        .path
        .ends_with("recovery-runtime/src/handoff/recovered_physical_runtime.rs")));
}

fn assert_core_definition_homes(rows: &[TopologyRow]) {
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
        assert_eq!(row(rows, path).owner, owner);
    }
}

fn assert_effect_homes(rows: &[TopologyRow]) {
    let effect_homes = [
        (
            "physical_runtime/recovery_coordination/effect.rs",
            "phase-5",
        ),
        (
            "physical_runtime/recovery_coordination/effect/reopen.rs",
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
}

fn assert_freshness_homes(rows: &[TopologyRow]) {
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
        assert_eq!(row(rows, path).phase, phase);
    }
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
        assert_eq!(
            row.phase,
            expected_phase(&row.path),
            "phase mismatch for {}",
            row.path
        );
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
