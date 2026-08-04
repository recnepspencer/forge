use std::collections::{BTreeMap, BTreeSet};

use super::read_repository_document;

const TRACE_DOCUMENT: &str = "_docs/worth-store/physical-reconstruction-c7-authority-trace.csv";
const HEADER: &str = "lane,sequence,step,path,source_anchor,authority_owner,disposition";

const WAL_INVENTORY_REOPEN_STEPS: &[&str] = &[
    "construction-entry",
    "checkpoint-compaction-reopen",
    "checkpoint-cutoff",
    "wal-inventory-reopen",
    "bounded-enumeration",
    "canonical-identity",
    "metadata-byte-bound",
    "bounded-allocation",
    "bounded-read",
    "active-tail-inspection",
    "interrupted-successor-admission",
    "topology-admission",
    "retained-member-transfer",
    "idempotency-rebuild",
    "wal-runtime-owner",
    "inspection-seal",
    "post-reopen-authority",
];

const C8_RECOVERY_HANDOFF_STEPS: &[&str] = &[
    "checkpoint-drain",
    "mutation-drain",
    "root-and-wal-extraction",
    "operation-fate-reconstruction",
    "checkpoint-basis",
    "residue-classification",
    "source-profile-binding",
    "closeout-progression",
    "successor-extraction",
];

const EXPECTED_LANES: &[(&str, &[&str])] = &[
    (
        "ordinary-publication",
        &[
            "managed-worker-entry",
            "group-projection",
            "current-root-snapshot",
            "transition-admission",
            "candidate-planning",
            "candidate-write",
            "candidate-synchronization",
            "atomic-replacement",
            "namespace-durability",
            "current-root-advance",
            "retained-root",
        ],
    ),
    (
        "ordinary-mutation-preparation",
        &[
            "submission-entry",
            "source-preflight",
            "canonical-fingerprint",
            "idempotency-admission",
            "operation-identity-reservation",
            "prepared-authority",
        ],
    ),
    (
        "ordinary-pre-seal-cancellation",
        &[
            "handle-request",
            "cancellation-director",
            "atomic-registry-transition",
            "terminal-fact",
        ],
    ),
    (
        "ordinary-wal-append",
        &[
            "managed-worker-entry",
            "group-admission",
            "idempotency-seal",
            "wal-reservation",
            "wal-planning",
            "signal-readiness",
            "scheduler-admission",
            "executor-dispatch",
            "backend-effect",
            "settlement",
            "member-promotion",
            "group-sealing",
        ],
    ),
    (
        "ordinary-wal-segment-create",
        &[
            "whole-group-rotation",
            "canonical-artifact-identity",
            "frame-declaration",
            "typed-command-lowering",
            "signal-readiness",
            "scheduler-admission",
            "executor-dispatch",
            "create-new-effect",
            "create-settlement",
            "member-promotion",
        ],
    ),
    ("ordinary-wal-inventory-reopen", WAL_INVENTORY_REOPEN_STEPS),
    (
        "ordinary-wal-barrier",
        &[
            "managed-worker-entry",
            "declaration-binding",
            "signal-readiness",
            "policy-admission",
            "scheduler-admission",
            "executor-dispatch",
            "backend-effect",
            "group-settlement",
            "all-member-derivation",
            "member-promotion",
        ],
    ),
    (
        "ordinary-data-dispatch",
        &[
            "managed-worker-entry",
            "durable-admission",
            "candidate-residency",
            "new-artifact-effect",
            "existing-artifact-writeback",
            "effect-settlement",
            "frame-set-completion",
            "dispatch-promotion",
            "exact-join",
            "settled-promotion",
        ],
    ),
    (
        "recovery-wal-observation-basis",
        &[
            "observation-scope",
            "certification-receipt",
            "profile-validation",
            "append-validation",
            "barrier-validation",
            "durability-observation",
            "crash-transfer",
        ],
    ),
    (
        "recovery-checkpoint-cutover",
        &[
            "checkpoint-validation",
            "observation-derived-durability",
            "publication-plan",
            "cutover-receipt",
            "wal-retention",
        ],
    ),
    (
        "recovery-page-redo",
        &["page-lsn-meaning", "redo-eligibility", "redo-application"],
    ),
    (
        "ordinary-mutation-lifecycle",
        &[
            "prepared-start",
            "attempt-registration",
            "worker-execution",
            "terminal-persistence",
            "observation-finalization",
            "waiter-notification",
            "close-drain",
        ],
    ),
    (
        "ordinary-mutation-settlement",
        &[
            "completed-fact",
            "completion-only-acknowledgment",
            "proven-no-effect-fate",
            "indeterminate-fate",
            "executed-evidence-projection",
            "diagnostic-evidence-projection",
        ],
    ),
    ("c8-recovery-handoff", C8_RECOVERY_HANDOFF_STEPS),
];

pub(super) struct AuthorityTraceAccounting {
    pub(super) lanes: usize,
    pub(super) c8_recovery_handoff_steps: usize,
    pub(super) wal_inventory_reopen_steps: usize,
}

pub(super) fn current_accounting() -> AuthorityTraceAccounting {
    AuthorityTraceAccounting {
        lanes: EXPECTED_LANES.len(),
        c8_recovery_handoff_steps: C8_RECOVERY_HANDOFF_STEPS.len(),
        wal_inventory_reopen_steps: WAL_INVENTORY_REOPEN_STEPS.len(),
    }
}

#[test]
fn every_current_authority_lane_resolves_to_ordered_production_sources() {
    let document = read_repository_document(TRACE_DOCUMENT).expect("read C.7 authority trace");
    let rows = parse_trace(&document).expect("parse C.7 authority trace");
    validate_shape(&rows).unwrap_or_else(|denial| panic!("{denial}"));
    for row in rows {
        let source = read_repository_document(&format!("workspaces/worth-store/{}", row.path))
            .unwrap_or_else(|denial| panic!("{denial}"));
        assert!(
            source.contains(&row.anchor),
            "{} step {} lost anchor `{}` in {}",
            row.lane,
            row.step,
            row.anchor,
            row.path
        );
        assert!(!row.owner.is_empty());
        assert!(matches!(
            row.disposition.as_str(),
            "preserve" | "narrow" | "move" | "replace" | "delete"
        ));
    }
}

fn parse_trace(document: &str) -> Result<Vec<TraceRow>, String> {
    let mut lines = document.lines();
    if lines.next() != Some(HEADER) {
        return Err("C.7 authority trace has an invalid schema header".to_owned());
    }
    let mut rows = Vec::new();
    for (index, line) in lines.enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let columns = line.split(',').map(str::trim).collect::<Vec<_>>();
        if columns.len() != 7 || columns.iter().any(|column| column.is_empty()) {
            return Err(format!("invalid C.7 authority trace row {}", index + 2));
        }
        rows.push(TraceRow {
            lane: columns[0].to_owned(),
            sequence: columns[1]
                .parse()
                .map_err(|_| format!("invalid trace sequence at row {}", index + 2))?,
            step: columns[2].to_owned(),
            path: columns[3].to_owned(),
            anchor: columns[4].to_owned(),
            owner: columns[5].to_owned(),
            disposition: columns[6].to_owned(),
        });
    }
    Ok(rows)
}

fn validate_shape(rows: &[TraceRow]) -> Result<(), String> {
    let mut lanes = BTreeMap::<String, Vec<(u8, String)>>::new();
    for row in rows {
        lanes
            .entry(row.lane.clone())
            .or_default()
            .push((row.sequence, row.step.clone()));
    }
    let expected_names = EXPECTED_LANES
        .iter()
        .map(|(lane, _)| *lane)
        .collect::<BTreeSet<_>>();
    let actual_names = lanes.keys().map(String::as_str).collect::<BTreeSet<_>>();
    if actual_names != expected_names {
        return Err(format!(
            "C.7 authority lanes differ; expected {expected_names:?}, actual {actual_names:?}"
        ));
    }
    for (lane, expected) in EXPECTED_LANES {
        let actual = lanes.get_mut(*lane).expect("lane set was reconciled");
        actual.sort();
        if actual
            .iter()
            .enumerate()
            .any(|(index, (sequence, _))| *sequence as usize != index + 1)
        {
            return Err(format!("C.7 authority lane `{lane}` is not contiguous"));
        }
        let actual_steps = actual
            .iter()
            .map(|(_, step)| step.as_str())
            .collect::<Vec<_>>();
        if actual_steps != *expected {
            let predicate = if *lane == "c8-recovery-handoff" {
                "MUTANT_PREDICATE:c8-handoff-authority-lane-omitted: "
            } else {
                ""
            };
            return Err(format!(
                "{predicate}C.7 authority lane `{lane}` changed; expected {expected:?}, actual {actual_steps:?}"
            ));
        }
    }
    Ok(())
}

#[test]
fn trace_shape_rejects_omitted_reordered_and_unclassified_lanes() {
    let document = read_repository_document(TRACE_DOCUMENT).expect("read C.7 authority trace");
    let rows = parse_trace(&document).expect("parse C.7 authority trace");
    assert!(validate_shape(&rows[1..]).is_err());

    let mut reordered = rows;
    reordered[0].sequence = 2;
    assert!(validate_shape(&reordered).is_err());
}

struct TraceRow {
    lane: String,
    sequence: u8,
    step: String,
    path: String,
    anchor: String,
    owner: String,
    disposition: String,
}
