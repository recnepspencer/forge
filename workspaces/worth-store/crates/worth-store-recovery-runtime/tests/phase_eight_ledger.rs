use std::path::{Path, PathBuf};

#[path = "phase_eight_ledger/audit_contract.rs"]
mod audit_contract;
#[path = "phase_eight_ledger/compilation_contract.rs"]
mod compilation_contract;
#[path = "phase_eight_ledger/documentation_contract.rs"]
mod documentation_contract;
#[path = "phase_eight_ledger/documented_process.rs"]
mod documented_process;
#[path = "phase_eight_ledger/exact_source_map.rs"]
mod exact_source_map;
#[path = "phase_eight_ledger/finding_contract.rs"]
mod finding_contract;
#[path = "phase_eight_ledger/ledger_contract.rs"]
mod ledger_contract;
#[path = "phase_eight_ledger/source_closure.rs"]
mod source_closure;

#[test]
fn phase_eight_ledger_is_a_requirement_source_and_history_bijection() {
    let root = repository_root();
    let specification = ledger_contract::marked_requirements(
        &read(&root.join(
            "_docs/worth-store/physical-reconstruction-c8-fresh-process-recovery-and-reopen.md",
        )),
        "c8-phase8-requirements",
    );
    let ledger =
        read(&root.join("_docs/worth-store/physical-reconstruction-c8-phase-8-closure-ledger.md"));
    let ledger_rows = ledger_contract::marked_ledger(&ledger, "c8-phase8-ledger");
    let expected = ledger_contract::guarantee_set();
    ledger_contract::validate_bijection(&specification, &ledger_rows);

    let closure =
        read(&root.join("_docs/worth-store/physical-reconstruction-c8-phase-8-source-closure.csv"));
    source_closure::validate(&root, &closure, &expected);
    source_closure::assert_reverse_complete(&root, &closure);
    documentation_contract::validate(&root);
    assert_eq!(
        source_closure::digest(&closure),
        source_closure::SOURCE_CLOSURE_SHA256
    );
    assert!(ledger.contains(&format!(
        "Source closure SHA-256: {}",
        source_closure::SOURCE_CLOSURE_SHA256
    )));
    assert_eq!(
        audit_contract::finding_rows(&ledger),
        finding_contract::FINDINGS.map(str::to_owned).into()
    );
    audit_contract::validate(&ledger);
    audit_contract::validate_external_audits(&root, source_closure::SOURCE_CLOSURE_SHA256);
    audit_contract::validate_correspondence(&root, &ledger);
}

#[test]
fn phase_eight_source_closure_is_a_pre_audit_fixed_point() {
    let root = repository_root();
    let closure =
        read(&root.join("_docs/worth-store/physical-reconstruction-c8-phase-8-source-closure.csv"));
    let expected = ledger_contract::guarantee_set();

    source_closure::validate(&root, &closure, &expected);
    source_closure::assert_reverse_complete(&root, &closure);
    exact_source_map::validate_source_map(&closure);
    assert_eq!(
        source_closure::digest(&closure),
        source_closure::SOURCE_CLOSURE_SHA256
    );
}

#[test]
fn documented_phase_eight_operator_commands_execute() {
    documented_process::execute(&repository_root());
}

#[test]
fn phase_eight_compilation_contract_is_warnings_denied() {
    compilation_contract::execute(&repository_root());
}

fn read(path: &Path) -> String {
    std::fs::read_to_string(path).unwrap()
}

fn repository_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../..")
        .canonicalize()
        .unwrap()
}
