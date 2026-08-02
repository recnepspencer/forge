use super::source_identity::source_identity;
use super::{
    parse_ledger, read_repository_document, validate_exact_source_identity, LedgerRow,
    LedgerStatus, LEDGER,
};

const PHASE_SEVEN_GUARANTEES: &[&str] = &[
    "C7-ROOT-01",
    "C7-ROOT-02",
    "C7-ROOT-03",
    "C7-ROOT-04",
    "C7-ROOT-05",
    "C7-ROOT-06",
    "C7-ROOT-07",
    "C7-ROOT-08",
];

const PHASE_SEVEN_ROOT_SOURCE_FILES: &[&str] = &[
    "_docs/worth-store/physical-reconstruction-c7-authority-trace.csv",
    "_docs/worth-store/physical-reconstruction-c7-phase-7-root-publication-plan.md",
    "_docs/worth-store/physical-reconstruction-c7-public-api.csv",
    "_docs/worth-store/physical-reconstruction-c7-removal-ledger.csv",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/mod.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/durability/grouping/root_publication.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/durability/mutation/progression/root_publication_member.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/instance/scheduler_admission.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/instance/scheduler_admission/root_publication.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/record_serving/mod.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/record_serving/planning/prepared_root_projection.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/record_serving/publication/director/mod.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/record_serving/publication/director/root_candidate_execution.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/record_serving/publication/director/root_preparation.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/record_serving/publication/director/root_progression.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/record_serving/publication/director/submission.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/record_serving/publication/mod.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/record_serving/publication/root_candidate_writes.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/record_serving/work_semantics/durability/root_publication_basis.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/work/declaration/root_publication_scope.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/work/execution/command/root_publication.rs",
    "workspaces/worth-store/crates/worth-store/tests/physical_record_journeys.rs",
    "workspaces/worth-store/crates/worth-store/tests/physical_runtime_authority_ui.rs",
    "workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/authority_trace.rs",
    "workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/closure_ledger/phase_seven.rs",
    "workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/closure_ledger/source_identity.rs",
    "workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/contract/mutation_preparation/manifest_capacity_transition.rs",
    "workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/contract/root_planning_observation.rs",
    "workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/inventory/mod.rs",
    "workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/inventory/removal_ledger.rs",
    "workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/inventory/source_discovery.rs",
    "workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/public_api/facade_reachability.rs",
    "workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/public_api/facade_reachability/phase_seven.rs",
    "workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/public_api/locked_surfaces.rs",
    "workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/public_api/mod.rs",
];
const PHASE_SEVEN_ROOT_SOURCE_TREES: &[&str] = &[
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/durability/publication",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/record_serving/planning/rebased_root",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/record_serving/publication/root_candidate",
    "workspaces/worth-store/crates/worth-store/tests/physical_record_journeys/durability_admission/data_durability",
    "workspaces/worth-store/crates/worth-store/tests/physical_record_journeys/durable_publication",
    "workspaces/worth-store/crates/worth-store/tests/physical_record_journeys/manifest_scale",
];
const PHASE_SEVEN_ROOT_COMPILE_ATTACKS: &[&str] = &["root_publication_plans_are_linear"];

fn phase_seven_root_source_identity() -> Result<String, String> {
    source_identity(
        "P7 root",
        PHASE_SEVEN_ROOT_SOURCE_FILES,
        PHASE_SEVEN_ROOT_SOURCE_TREES,
        PHASE_SEVEN_ROOT_COMPILE_ATTACKS,
    )
}

#[test]
fn phase_seven_guarantees_are_resolved_before_phase_eight_can_begin() {
    let document = read_repository_document(LEDGER).expect("read C.7 closure ledger");
    let rows = parse_ledger(&document).expect("parse C.7 closure ledger");
    let source_identity = phase_seven_root_source_identity().expect("hash Phase 7 root closure");
    validate_phase_seven_closure(&rows, &source_identity)
        .unwrap_or_else(|denial| panic!("{denial}"));
}

#[test]
fn phase_seven_validator_rejects_unresolved_and_stale_closure() {
    let document = read_repository_document(LEDGER).expect("read C.7 closure ledger");
    let rows = parse_ledger(&document).expect("parse C.7 closure ledger");
    let source_identity = phase_seven_root_source_identity().expect("hash Phase 7 root closure");

    let mut unresolved = rows.clone();
    phase_seven_row_mut(&mut unresolved, "C7-ROOT-08").status = LedgerStatus::Open;
    assert!(validate_phase_seven_closure(&unresolved, &source_identity).is_err());

    let mut stale = rows.clone();
    phase_seven_row_mut(&mut stale, "C7-ROOT-01").source_identity =
        "P7 root deadbeefdead".to_owned();
    assert!(validate_phase_seven_closure(&stale, &source_identity).is_err());

    let omitted = rows
        .iter()
        .filter(|row| row.id != "C7-ROOT-04")
        .cloned()
        .collect::<Vec<_>>();
    assert!(validate_phase_seven_closure(&omitted, &source_identity).is_err());

    let mut wrong_phase = rows;
    phase_seven_row_mut(&mut wrong_phase, "C7-ROOT-06").phase = "8".to_owned();
    assert!(validate_phase_seven_closure(&wrong_phase, &source_identity).is_err());
}

fn validate_phase_seven_closure(rows: &[LedgerRow], source_identity: &str) -> Result<(), String> {
    for guarantee in PHASE_SEVEN_GUARANTEES {
        let row = rows
            .iter()
            .find(|row| row.id == *guarantee)
            .ok_or_else(|| format!("C.7 ledger omits Phase 7 guarantee `{guarantee}`"))?;
        if row.phase != "7" {
            return Err(format!(
                "Phase 7 guarantee `{guarantee}` is assigned to phase {}",
                row.phase
            ));
        }
        if row.status != LedgerStatus::Proved {
            return Err(format!(
                "Phase 7 guarantee `{guarantee}` remains unresolved"
            ));
        }
        validate_exact_source_identity(rows, guarantee, source_identity)?;
    }
    Ok(())
}

fn phase_seven_row_mut<'a>(rows: &'a mut [LedgerRow], identity: &str) -> &'a mut LedgerRow {
    rows.iter_mut()
        .find(|row| row.id == identity)
        .unwrap_or_else(|| panic!("missing controlled Phase 7 guarantee `{identity}`"))
}
