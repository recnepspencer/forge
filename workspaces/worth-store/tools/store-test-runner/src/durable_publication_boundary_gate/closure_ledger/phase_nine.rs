use super::source_identity::source_identity;
use super::{
    parse_ledger, read_repository_document, validate_exact_source_identity, LedgerRow,
    LedgerStatus, LEDGER,
};

const PHASE_NINE_GRAPH_GUARANTEE: &str = "C7-GRAPH-02";
const PHASE_NINE_CLOSEOUT_GUARANTEES: &[&str] = &["C7-CLEANUP-02", "C7-DOCUMENTATION-01"];
const PHASE_NINE_GRAPH_SOURCE_FILES: &[&str] = &[
    "_docs/worth-store/physical-reconstruction-c7-durable-publication-join.md",
    "workspaces/worth-store/Cargo.toml",
    "workspaces/worth-store/crates/worth-store-branch-deltas/Cargo.toml",
    "workspaces/worth-store/crates/worth-store-layout-indexes/Cargo.toml",
    "workspaces/worth-store/crates/worth-store-layout-indexes/tests/compile_fail/support.rs",
    "workspaces/worth-store/crates/worth-store-physical-isolation/Cargo.toml",
    "workspaces/worth-store/crates/worth-store-recovery-physics/Cargo.toml",
    "workspaces/worth-store/crates/worth-store-snapshots/Cargo.toml",
    "workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/closure_ledger.rs",
    "workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/closure_ledger/phase_nine.rs",
    "workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/cutover/feature_graph.rs",
    "workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/cutover/mod.rs",
    "workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/mod.rs",
];
const PHASE_NINE_CLOSEOUT_SOURCE_FILES: &[&str] = &[
    "_docs/worth-store/physical-durability-and-checkpoints.md",
    "_docs/worth-store/physical-foundation-reconstruction-roadmap.md",
    "_docs/worth-store/physical-reality-audit.csv",
    "_docs/worth-store/physical-reconstruction-c7-authority-trace.csv",
    "_docs/worth-store/physical-reconstruction-c7-durable-publication-join.md",
    "_docs/worth-store/physical-reconstruction-c7-public-api.csv",
    "_docs/worth-store/physical-reconstruction-c7-removal-ledger.csv",
    "_docs/worth-store/storage-foundation-aspect-native-gate.md",
    "_docs/worth-store/storage-foundation-s9.md",
    "workspaces/worth-store/crates/worth-store/README.md",
    "workspaces/worth-store/crates/worth-store-physical-backend/README.md",
    "workspaces/worth-store/crates/worth-store-recovery-physics/README.md",
    "workspaces/worth-store/crates/worth-store-wal/README.md",
    "workspaces/worth-store/crates/worth-store/tests/physical_record_journeys/durability_admission.rs",
    "workspaces/worth-store/crates/worth-store/tests/physical_record_journeys/durability_admission/durability_documentation.rs",
    "workspaces/worth-store/crates/worth-store/tests/physical_record_journeys/durability_admission/wal_ownership_shape.rs",
    "workspaces/worth-store/crates/worth-store/tests/physical_record_journeys/physical_work/lifecycle.rs",
    "workspaces/worth-store/crates/worth-store/tests/physical_record_journeys/physical_work/readiness.rs",
    "workspaces/worth-store/crates/worth-store/tests/physical_runtime_authority_ui.rs",
    "workspaces/worth-store/crates/worth-store/tests/physical_runtime_authority/physical_durability_guide_examples.rs",
    "workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/closure_ledger.rs",
    "workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/closure_ledger/phase_nine.rs",
    "workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/contract/durability_aspect_contracts.rs",
    "workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/contract/mod.rs",
    "workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/cutover/declaration_topology.rs",
    "workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/cutover/mod.rs",
    "workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/inventory/mod.rs",
    "workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/inventory/removal_ledger.rs",
    "workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/inventory/source_discovery.rs",
    "workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/public_api/locked_surfaces.rs",
    "workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/public_api/mod.rs",
];
const PHASE_NINE_CLOSEOUT_SOURCE_TREES: &[&str] = &[
    "workspaces/worth-store/crates/worth-store-wal/src/publication_declaration",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/record_serving/work_semantics/durability",
];

fn phase_nine_graph_source_identity() -> Result<String, String> {
    source_identity("P9 graph", PHASE_NINE_GRAPH_SOURCE_FILES, &[], &[])
}

fn phase_nine_closeout_source_identity() -> Result<String, String> {
    source_identity(
        "P9 closeout",
        PHASE_NINE_CLOSEOUT_SOURCE_FILES,
        PHASE_NINE_CLOSEOUT_SOURCE_TREES,
        &[],
    )
}

#[test]
fn proved_phase_nine_feature_graph_tracks_its_exact_source_closure() {
    let document = read_repository_document(LEDGER).expect("read C.7 closure ledger");
    let rows = parse_ledger(&document).expect("parse C.7 closure ledger");
    let source_identity =
        phase_nine_graph_source_identity().expect("hash Phase 9 feature graph closure");
    validate_phase_nine_graph_closure(&rows, &source_identity)
        .unwrap_or_else(|denial| panic!("{denial}"));
}

#[test]
fn phase_nine_graph_validator_rejects_omission_wrong_phase_and_stale_identity() {
    let document = read_repository_document(LEDGER).expect("read C.7 closure ledger");
    let rows = parse_ledger(&document).expect("parse C.7 closure ledger");
    let source_identity =
        phase_nine_graph_source_identity().expect("hash Phase 9 feature graph closure");

    let omitted = rows
        .iter()
        .filter(|row| row.id != PHASE_NINE_GRAPH_GUARANTEE)
        .cloned()
        .collect::<Vec<_>>();
    assert!(validate_phase_nine_graph_closure(&omitted, &source_identity).is_err());

    let mut wrong_phase = rows.clone();
    phase_nine_graph_row_mut(&mut wrong_phase).phase = "8".to_owned();
    assert!(validate_phase_nine_graph_closure(&wrong_phase, &source_identity).is_err());

    let mut stale = rows;
    let row = phase_nine_graph_row_mut(&mut stale);
    row.status = LedgerStatus::Proved;
    row.source_identity = "P9 graph deadbeefdead".to_owned();
    assert!(validate_phase_nine_graph_closure(&stale, &source_identity).is_err());
}

fn validate_phase_nine_graph_closure(
    rows: &[LedgerRow],
    source_identity: &str,
) -> Result<(), String> {
    let row = rows
        .iter()
        .find(|row| row.id == PHASE_NINE_GRAPH_GUARANTEE)
        .ok_or_else(|| {
            format!("C.7 ledger omits Phase 9 guarantee `{PHASE_NINE_GRAPH_GUARANTEE}`")
        })?;
    if row.phase != "9" {
        return Err(format!(
            "Phase 9 guarantee `{PHASE_NINE_GRAPH_GUARANTEE}` is assigned to phase {}",
            row.phase
        ));
    }
    if row.status != LedgerStatus::Proved {
        return Err(format!(
            "Phase 9 guarantee `{PHASE_NINE_GRAPH_GUARANTEE}` remains unresolved"
        ));
    }
    validate_exact_source_identity(rows, PHASE_NINE_GRAPH_GUARANTEE, source_identity)
}

fn phase_nine_graph_row_mut(rows: &mut [LedgerRow]) -> &mut LedgerRow {
    rows.iter_mut()
        .find(|row| row.id == PHASE_NINE_GRAPH_GUARANTEE)
        .unwrap_or_else(|| panic!("missing controlled Phase 9 graph guarantee"))
}

#[test]
fn phase_nine_closeout_guarantees_track_their_exact_source_closure() {
    let document = read_repository_document(LEDGER).expect("read C.7 closure ledger");
    let rows = parse_ledger(&document).expect("parse C.7 closure ledger");
    let source_identity =
        phase_nine_closeout_source_identity().expect("hash Phase 9 closeout closure");
    validate_phase_nine_closeout(&rows, &source_identity)
        .unwrap_or_else(|denial| panic!("{denial}"));
}

#[test]
fn phase_nine_closeout_rejects_omission_wrong_phase_stale_proof_and_phase_ten_start() {
    let document = read_repository_document(LEDGER).expect("read C.7 closure ledger");
    let rows = parse_ledger(&document).expect("parse C.7 closure ledger");
    let source_identity =
        phase_nine_closeout_source_identity().expect("hash Phase 9 closeout closure");

    let omitted = rows
        .iter()
        .filter(|row| row.id != "C7-DOCUMENTATION-01")
        .cloned()
        .collect::<Vec<_>>();
    assert!(validate_phase_nine_closeout(&omitted, &source_identity).is_err());

    let mut wrong_phase = rows.clone();
    phase_nine_closeout_row_mut(&mut wrong_phase, "C7-CLEANUP-02").phase = "8".to_owned();
    assert!(validate_phase_nine_closeout(&wrong_phase, &source_identity).is_err());

    let mut stale = rows.clone();
    let stale_row = phase_nine_closeout_row_mut(&mut stale, "C7-DOCUMENTATION-01");
    stale_row.status = LedgerStatus::Proved;
    stale_row.source_identity = "P9 closeout deadbeefdead".to_owned();
    assert!(validate_phase_nine_closeout(&stale, &source_identity).is_err());

    let mut premature = rows;
    phase_nine_closeout_row_mut(&mut premature, "C7-CLEANUP-02").status = LedgerStatus::Open;
    let phase_ten = premature
        .iter_mut()
        .find(|row| row.phase == "10")
        .expect("ledger retains a Phase 10 guarantee");
    phase_ten.current_evidence = "started early".to_owned();
    assert!(validate_phase_nine_closeout(&premature, &source_identity).is_err());
}

fn validate_phase_nine_closeout(rows: &[LedgerRow], source_identity: &str) -> Result<(), String> {
    let mut unresolved = false;
    for guarantee in PHASE_NINE_CLOSEOUT_GUARANTEES {
        let row = rows
            .iter()
            .find(|row| row.id == *guarantee)
            .ok_or_else(|| format!("C.7 ledger omits Phase 9 guarantee `{guarantee}`"))?;
        if row.phase != "9" {
            return Err(format!(
                "Phase 9 guarantee `{guarantee}` is assigned to phase {}",
                row.phase
            ));
        }
        unresolved |= row.status != LedgerStatus::Proved;
        validate_exact_source_identity(rows, guarantee, source_identity)?;
    }
    if unresolved
        && rows.iter().filter(|row| row.phase == "10").any(|row| {
            row.status != LedgerStatus::Open
                || row.current_evidence != "pending"
                || row.source_identity != "pending"
        })
    {
        return Err(
            "Phase 10 evidence began while a Phase 9 closeout guarantee remained unresolved"
                .to_owned(),
        );
    }
    if unresolved {
        return Err("Phase 9 closeout guarantees remain unresolved".to_owned());
    }
    Ok(())
}

fn phase_nine_closeout_row_mut<'a>(rows: &'a mut [LedgerRow], identity: &str) -> &'a mut LedgerRow {
    rows.iter_mut()
        .find(|row| row.id == identity)
        .unwrap_or_else(|| panic!("missing controlled Phase 9 guarantee `{identity}`"))
}
