use super::source_identity::source_identity;
use super::{
    parse_ledger, read_repository_document, validate_exact_source_identity, LedgerRow,
    LedgerStatus, LEDGER,
};

const PHASE_EIGHT_GUARANTEES: &[&str] = &[
    "C7-API-02",
    "C7-EVIDENCE-01",
    "C7-LIFECYCLE-01",
    "C7-OBSERVATION-01",
    "C7-SETTLEMENT-01",
    "C7-SETTLEMENT-02",
    "C7-SIGNAL-02",
];

const PHASE_EIGHT_SOURCE_FILES: &[&str] = &[
    "_docs/worth-store/physical-reconstruction-c7-authority-trace.csv",
    "_docs/worth-store/physical-reconstruction-c7-durable-publication-join.md",
    "_docs/worth-store/physical-reconstruction-c7-phase-8-typed-mutation-plan.md",
    "_docs/worth-store/physical-reconstruction-c7-public-api.csv",
    "_docs/worth-store/physical-reconstruction-c7-removal-ledger.csv",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/mod.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/instance/lifecycle.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/record_serving/lifecycle/serving_runtime.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/record_serving/publication/director/certification_submission.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/record_serving/publication/director/lifecycle.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/record_serving/publication/director/managed_mutation.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/record_serving/publication/director/submission.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/record_serving/publication/durable_preparation/prepared.rs",
    "workspaces/worth-store/crates/worth-store/tests/physical_record_journeys/durability_admission/managed_mutation.rs",
    "workspaces/worth-store/crates/worth-store/tests/physical_record_journeys/durability_admission.rs",
    "workspaces/worth-store/crates/worth-store/tests/physical_record_journeys.rs",
    "workspaces/worth-store/crates/worth-store/tests/physical_record_journeys/durability_admission/idempotency_reopen.rs",
    "workspaces/worth-store/crates/worth-store/tests/physical_runtime_authority_ui.rs",
    "workspaces/worth-store/crates/worth-store/tests/physical_runtime_authority/ordinary_mutation_outcomes_are_supported.rs",
    "workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/authority_trace.rs",
    "workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/closure_ledger.rs",
    "workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/closure_ledger/phase_eight.rs",
    "workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/closure_ledger/source_identity.rs",
    "workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/contract/mutation_settlement.rs",
    "workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/inventory/mod.rs",
    "workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/inventory/removal_ledger.rs",
    "workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/inventory/source_discovery.rs",
    "workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/public_api/facade_reachability.rs",
    "workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/public_api/facade_reachability/phase_eight.rs",
    "workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/public_api/locked_surfaces.rs",
    "workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/public_api/mod.rs",
];
const PHASE_EIGHT_SOURCE_TREES: &[&str] = &[
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/durability/evidence_projection",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/durability/lifecycle",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/durability/mutation",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/durability/observation",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/durability/settlement",
    "workspaces/worth-store/crates/worth-store/tests/physical_record_journeys/durability_admission/managed_mutation",
];
const PHASE_EIGHT_COMPILE_ATTACKS: &[&str] = &[
    "mutation_evidence_cannot_reenter_authority",
    "noncompleted_mutation_cannot_acknowledge",
    "ordinary_mutation_phase_driving_is_absent",
];

fn phase_eight_source_identity() -> Result<String, String> {
    source_identity(
        "P8 mutation",
        PHASE_EIGHT_SOURCE_FILES,
        PHASE_EIGHT_SOURCE_TREES,
        PHASE_EIGHT_COMPILE_ATTACKS,
    )
}

#[test]
fn phase_eight_guarantees_are_resolved_before_phase_nine_can_begin() {
    let document = read_repository_document(LEDGER).expect("read C.7 closure ledger");
    let rows = parse_ledger(&document).expect("parse C.7 closure ledger");
    let source_identity = phase_eight_source_identity().expect("hash Phase 8 mutation closure");
    validate_phase_eight_closure(&rows, &source_identity)
        .unwrap_or_else(|denial| panic!("{denial}"));
}

#[test]
fn phase_eight_validator_rejects_omission_stale_proof_and_phase_nine_start() {
    let document = read_repository_document(LEDGER).expect("read C.7 closure ledger");
    let rows = parse_ledger(&document).expect("parse C.7 closure ledger");
    let source_identity = phase_eight_source_identity().expect("hash Phase 8 mutation closure");

    let omitted = rows
        .iter()
        .filter(|row| row.id != "C7-OBSERVATION-01")
        .cloned()
        .collect::<Vec<_>>();
    assert!(validate_phase_eight_closure(&omitted, &source_identity).is_err());

    let mut wrong_phase = rows.clone();
    phase_eight_row_mut(&mut wrong_phase, "C7-SETTLEMENT-02").phase = "9".to_owned();
    assert!(validate_phase_eight_closure(&wrong_phase, &source_identity).is_err());

    let mut stale = rows.clone();
    phase_eight_row_mut(&mut stale, "C7-EVIDENCE-01").source_identity =
        "P8 mutation deadbeefdead".to_owned();
    assert!(validate_phase_eight_closure(&stale, &source_identity).is_err());

    let mut unresolved = rows;
    phase_eight_row_mut(&mut unresolved, "C7-LIFECYCLE-01").status = LedgerStatus::Open;
    let phase_nine = unresolved
        .iter_mut()
        .find(|row| row.phase == "9")
        .expect("ledger retains a Phase 9 guarantee");
    phase_nine.current_evidence = "started early".to_owned();
    assert!(validate_phase_eight_closure(&unresolved, &source_identity).is_err());
}

fn validate_phase_eight_closure(rows: &[LedgerRow], source_identity: &str) -> Result<(), String> {
    let mut unresolved = false;
    for guarantee in PHASE_EIGHT_GUARANTEES {
        let row = rows
            .iter()
            .find(|row| row.id == *guarantee)
            .ok_or_else(|| format!("C.7 ledger omits Phase 8 guarantee `{guarantee}`"))?;
        if row.phase != "8" {
            return Err(format!(
                "Phase 8 guarantee `{guarantee}` is assigned to phase {}",
                row.phase
            ));
        }
        unresolved |= row.status != LedgerStatus::Proved;
        validate_exact_source_identity(rows, guarantee, source_identity)?;
    }
    if unresolved
        && rows.iter().filter(|row| row.phase == "9").any(|row| {
            row.status != LedgerStatus::Open
                || row.current_evidence != "pending"
                || row.source_identity != "pending"
        })
    {
        return Err(
            "Phase 9 evidence began while a Phase 8 guarantee remained unresolved".to_owned(),
        );
    }
    if unresolved {
        return Err("Phase 8 guarantees remain unresolved".to_owned());
    }
    Ok(())
}

fn phase_eight_row_mut<'a>(rows: &'a mut [LedgerRow], identity: &str) -> &'a mut LedgerRow {
    rows.iter_mut()
        .find(|row| row.id == identity)
        .unwrap_or_else(|| panic!("missing controlled Phase 8 guarantee `{identity}`"))
}
