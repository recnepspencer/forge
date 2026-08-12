mod audit_source_manifest;
mod history_contract;
mod source_identity;

use std::collections::BTreeSet;

use super::documents::{read_repository_document, CLOSURE_LEDGER, QA_AUDITS, SPECIFICATION};
use history_contract::{
    finding_applies, validate_audit_history, validate_audit_records, validate_finding_history,
};
use source_identity::{phase_one_source_identity, phase_one_source_paths};

const START: &str = "<!-- c8-ledger:start -->";
const END: &str = "<!-- c8-ledger:end -->";
const REQUIREMENTS_START: &str = "<!-- c8-phase1-requirements:start -->";
const REQUIREMENTS_END: &str = "<!-- c8-phase1-requirements:end -->";
const REQUIRED_GUARANTEES: &[&str] = &[
    "C8-P1-TRUTH-01",
    "C8-P1-API-01",
    "C8-P1-AUTHORITY-01",
    "C8-P1-SESSION-01",
    "C8-P1-EFFECT-01",
    "C8-P1-FRESHNESS-01",
    "C8-P1-PROTOCOL-01",
    "C8-P1-TOPOLOGY-01",
    "C8-P1-DEPENDENCY-01",
    "C8-P1-CUTOVER-01",
    "C8-P1-COMPILE-01",
    "C8-P1-CLEANUP-01",
    "C8-P1-DOCUMENTATION-01",
    "C8-P1-LEDGER-01",
    "C8-P1-LEDGER-02",
    "C8-P1-ENTRY-01",
    "C8-P1-PERSISTED-01",
];
#[test]
fn phase_one_ledger_is_complete_resolved_and_bound_to_current_source() {
    let document = read_repository_document(CLOSURE_LEDGER).expect("read C.8 closure ledger");
    validate_ledger(&document).expect("validate C.8 closure ledger");
}

#[test]
fn per_guarantee_source_closures_are_causal() {
    for guarantee in REQUIRED_GUARANTEES {
        let identity = phase_one_source_identity(guarantee).expect("compute C.8 source identity");
        eprintln!("{guarantee}={identity}");
    }
    let truth = phase_one_source_paths("C8-P1-TRUTH-01").expect("truth sources");
    let topology = phase_one_source_paths("C8-P1-TOPOLOGY-01").expect("topology sources");
    let compile = phase_one_source_paths("C8-P1-COMPILE-01").expect("compile sources");
    let api = phase_one_source_paths("C8-P1-API-01").expect("API sources");
    let entry = phase_one_source_paths("C8-P1-ENTRY-01").expect("entry sources");
    let session = phase_one_source_paths("C8-P1-SESSION-01").expect("session sources");
    let effect = phase_one_source_paths("C8-P1-EFFECT-01").expect("effect sources");
    let freshness = phase_one_source_paths("C8-P1-FRESHNESS-01").expect("freshness sources");
    let protocol = phase_one_source_paths("C8-P1-PROTOCOL-01").expect("protocol sources");
    let ledger_history = phase_one_source_paths("C8-P1-LEDGER-01").expect("ledger history sources");
    assert!(truth.contains("_docs/worth-store/physical-reconstruction-c8-persisted-inputs.csv"));
    assert!(truth.iter().any(|path| path.ends_with(
        "fresh_process_recovery_boundary_gate/persisted_input_contract/syntax_evidence.rs"
    )));
    assert!(!topology.contains("_docs/worth-store/physical-reconstruction-c8-persisted-inputs.csv"));
    assert!(compile
        .iter()
        .any(|path| path.ends_with("c8_recovery_handoff_is_linear.rs")));
    assert!(!api
        .iter()
        .any(|path| path.ends_with("c8_recovery_handoff_is_linear.rs")));
    assert!(api
        .iter()
        .any(|path| path.ends_with("worth-store-recovery-physics/src/lib.rs")));
    assert!(!entry.contains("_docs/worth-store/physical-reconstruction-c8-persisted-inputs.csv"));
    assert!(session.contains("crates/worth-proof/src/linear.rs"));
    assert!(!session.contains("crates/worth-proof/src/effect/performed.rs"));
    assert!(effect.contains("crates/worth-proof/src/effect/performed.rs"));
    assert!(!effect.contains("crates/worth-proof/src/linear.rs"));
    assert!(freshness.contains("crates/worth-proof/src/assumption/freshness.rs"));
    assert!(
        freshness.contains("_docs/worth-store/physical-reconstruction-c8-destination-topology.csv")
    );
    assert!(!freshness.contains("crates/worth-proof/src/effect/performed.rs"));
    assert!(protocol
        .contains("crates/worth-foundational/src/boundary_protocol/compatibility_window.rs"));
    assert!(!protocol.contains("crates/worth-proof/src/linear.rs"));
    assert!(topology.contains("_docs/worth-store/physical-reconstruction-c8-authority-trace.csv"));
    assert!(ledger_history.contains(QA_AUDITS));
    assert!(api
        .iter()
        .any(|path| path
            .ends_with("worth-store-recovery-physics/src/publication/recovery_replay.rs")));
    for phase_seven_contract in [
        "runtime_phase_seven_surface_contract.rs",
        "runtime_phase_seven_surface_contract/backend.rs",
        "runtime_phase_seven_surface_contract/runtime.rs",
        "runtime_phase_seven_surface_contract/store.rs",
        "runtime_phase_seven_surface_contract/wal.rs",
    ] {
        assert!(
            api.iter().any(|path| path.ends_with(phase_seven_contract)),
            "Phase 7 API contract source omitted: {phase_seven_contract}"
        );
    }
}

#[test]
fn ledger_completeness_and_source_mutants_are_rejected() {
    let document = read_repository_document(CLOSURE_LEDGER).expect("read C.8 closure ledger");
    let rows = parse_rows(&document).expect("parse C.8 closure ledger");
    let first_line = document
        .lines()
        .find(|line| line.starts_with("| C8-P1-"))
        .expect("ledger row");
    let omitted = document.replacen(&format!("{first_line}\n"), "", 1);
    assert!(validate_ledger(&omitted).is_err());
    let duplicated = document.replacen(END, &format!("{first_line}\n{END}"), 1);
    assert!(validate_ledger(&duplicated).is_err());
    let stale = document.replacen(&rows[0].source_identity, "stale-source", 1);
    assert!(validate_ledger(&stale).is_err());
    let unrelated = document.replacen(&rows[0].evidence, "unrelated nonempty evidence", 1);
    assert!(validate_ledger(&unrelated).is_err());
    let pending = document.replacen(
        "focused boundary suite passed",
        "Pending gate verification",
        1,
    );
    assert!(validate_ledger(&pending).is_err());
    let affected = document.replacen(
        "C8-P1-TRUTH-01 C8-P1-PERSISTED-01 C8-P1-LEDGER-02",
        "C8-P1-TRUTH-01 C8-P1-LEDGER-02",
        1,
    );
    assert!(validate_ledger(&affected).is_err());
    let truth_line = document
        .lines()
        .find(|line| line.starts_with("| C8-P1-TRUTH-01 |"))
        .expect("truth guarantee row");
    let mut truth_columns = truth_line.split('|').collect::<Vec<_>>();
    assert_eq!(truth_columns.len(), 11, "closure-ledger column count");
    truth_columns[8] = " C8-P1-F13 ";
    let unrelated_reopening = document.replacen(truth_line, &truth_columns.join("|"), 1);
    assert!(validate_ledger(&unrelated_reopening).is_err());
    let finding_line = document
        .lines()
        .find(|line| line.starts_with("| C8-P1-F28 |"))
        .expect("finding row");
    let duplicate_finding =
        document.replacen(finding_line, &format!("{finding_line}\n{finding_line}"), 1);
    assert!(validate_ledger(&duplicate_finding).is_err());
}

#[test]
fn structured_audit_identity_prompt_and_finding_mutants_are_rejected() {
    let ledger = read_repository_document(CLOSURE_LEDGER).expect("read C.8 closure ledger");
    let audits = read_repository_document(QA_AUDITS).expect("read C.8 QA audits");
    let audit_line = audits.lines().nth(1).expect("audit row");
    let mut audit_columns = audit_line.split(',').collect::<Vec<_>>();
    audit_columns[3] = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
    let stale = audits.replacen(audit_line, &audit_columns.join(","), 1);
    assert!(validate_audit_records(&stale, &ledger).is_err());
    let prompt = audits.replacen("Read-only falsify", "Unretained prompt", 1);
    assert!(validate_audit_records(&prompt, &ledger).is_err());
    let findings = audits.replacen("C8-P1-F19;", "C8-P1-F18;", 1);
    assert!(validate_audit_records(&findings, &ledger).is_err());
    let verification = audits.replacen(
        "22-test warnings-denied boundary suite plus four inherited C7 trybuild attacks",
        "22-test warnings-denied boundary suite plus four substituted C7 trybuild attacks",
        1,
    );
    assert!(validate_audit_records(&verification, &ledger).is_err());
    let duplicate_audit = audits.replacen(audit_line, &format!("{audit_line}\n{audit_line}"), 1);
    assert!(validate_audit_records(&duplicate_audit, &ledger).is_err());
    let summary_line = ledger
        .lines()
        .find(|line| line.starts_with("| /root/c8_phase1_critic |"))
        .expect("audit summary");
    let duplicate_summary =
        ledger.replacen(summary_line, &format!("{summary_line}\n{summary_line}"), 1);
    assert!(validate_audit_records(&audits, &duplicate_summary).is_err());
}

fn validate_ledger(document: &str) -> Result<(), String> {
    let rows = parse_rows(document)?;
    let identities = rows
        .iter()
        .map(|row| row.id.as_str())
        .collect::<BTreeSet<_>>();
    let specification = read_repository_document(SPECIFICATION)?;
    let specified = specification_requirement_ids(&specification)?;
    let required = REQUIRED_GUARANTEES.iter().copied().collect::<BTreeSet<_>>();
    if specified != required {
        return Err("C.8 specification and gate requirement sets diverged".into());
    }
    if identities != required || rows.len() != required.len() {
        return Err("C.8 Phase 1 ledger guarantee set is incomplete or duplicated".into());
    }
    let findings = validate_finding_history(document, &required)?;
    validate_audit_history(document)?;
    for row in rows {
        if row.phase != "1" || row.status != "PROVED" {
            return Err(format!("C.8 guarantee {} is not closed", row.id));
        }
        if row.reopened_by != "none" {
            for finding in row.reopened_by.split_whitespace() {
                if !findings.contains(finding) || !finding_applies(finding, &row.id) {
                    return Err(format!(
                        "C.8 guarantee {} names unrelated reopening {}",
                        row.id, finding
                    ));
                }
            }
        }
        let source_identity = phase_one_source_identity(&row.id)?;
        if row.source_identity != source_identity {
            return Err(format!(
                "C.8 guarantee {} has stale source identity; expected {source_identity}",
                row.id,
            ));
        }
        let expected = expected_contract(&row.id)?;
        if (
            row.claim.as_str(),
            row.implementation_owner.as_str(),
            row.evidence.as_str(),
            row.unsupported_scope.as_str(),
        ) != expected
        {
            return Err(format!(
                "C.8 guarantee {} has noncausal or stale evidence",
                row.id
            ));
        }
    }
    Ok(())
}

fn expected_contract(
    id: &str,
) -> Result<(&'static str, &'static str, &'static str, &'static str), String> {
    match id {
        "C8-P1-TRUTH-01" => Ok(("Fresh recovery distinguishes real persisted Store producers from explicit producer gaps and admits no live or derived proxy", "C8 governing specification", "persisted_input_roles_bind_real_producers_or_explicit_gaps and omitted_foreign_and_derived_proxy_mutants_are_rejected", "none")),
        "C8-P1-API-01" => Ok(("Every reachable recovery-physics item plus every production-reachable delivered Phase 2 through Phase 7 facade surface and every planned facade has one exact disposition", "C8 public API inventory", "live current and delivered facade derivation exact inventory equality supporting-baseline drift checks and dispositions_name_one_real_destination_owner", "none")),
        "C8-P1-AUTHORITY-01" => Ok(("Concrete Store authority retains every entry and admitted-world binding axis beneath private Worth Proof substrate", "C8 authority trace", "authority_trace_locks_every_c8_contract_family_exactly plus performed_and_freshness_axis_substitution_mutants_are_rejected", "none")),
        "C8-P1-SESSION-01" => Ok(("One owner-issued recovery session reaches exactly one of four Store terminals", "C8 authority trace", "authority_trace_locks_every_c8_contract_family_exactly and exact linear-resource substrate", "none")),
        "C8-P1-EFFECT-01" => Ok(("Performed evidence exists only for five exact owner-recorded C4 action kinds with outcome and occurrence bindings", "C8 authority trace", "performed_and_freshness_axis_substitution_mutants_are_rejected", "none")),
        "C8-P1-FRESHNESS-01" => Ok(("Store owners sample checkpoint and published-root generation freshness from sealed bases under two exact policies", "C8 authority trace", "performed_and_freshness_axis_substitution_mutants_are_rejected", "none")),
        "C8-P1-PROTOCOL-01" => Ok(("Recovery and observer reports use distinct version-one Foundational families with one-version windows and no Store authority", "C8 authority trace", "generic_proof_and_report_values_open_no_store_door", "none")),
        "C8-P1-TOPOLOGY-01" => Ok(("Every C8 semantic leaf has one exact destination responsibility and delivery phase while future insertion preserves owner direction", "C8 destination topology", "destination_topology_has_one_exact_semantic_home_per_c8_axis plus topology_rows_have_specific_owners_and_phase_honest_status", "none")),
        "C8-P1-DEPENDENCY-01" => Ok(("Current recovery dependency edges are frozen while Signal Query replay and ordinary reverse imports remain forbidden", "C8 Cargo graph", "checked_in_recovery_dependency_cut_matches_cargo_metadata plus delivered_phase_dependency_direction_is_honest", "none")),
        "C8-P1-CUTOVER-01" => Ok(("Every recovery-physics owner syntax-reachable direct consumer nested observer route C7 lineage input and authoritative document has one reconciled disposition", "C8 cutover inventory", "scoped_cutover_inventory_matches_current_source_and_consumer_closure plus syntax_references_reject_globs_comments_and_alias_bypasses", "none")),
        "C8-P1-COMPILE-01" => Ok(("C7 operation facts reports clones and public constructors cannot mint or duplicate the inherited C7 closeout handoff", "worth-store physical runtime authority tests", "phase_ten_c8_recovery_handoff_is_compile_sealed four C7 trybuild attacks", "actual C8 entry and handoff compile sealing belongs to phases 2 and 6")),
        "C8-P1-CLEANUP-01" => Ok(("Parallel cutover names old entry verifier evidence workflow and reverse-dependency deletion gates without creating Phase 2 stubs", "C8 API Cargo and cutover inventories", "exact semantic dispositions plus worth-store-recovery-runtime absence gate", "none")),
        "C8-P1-DOCUMENTATION-01" => Ok(("C8 specification and reconstruction roadmap describe and link the same Proof and Foundational contracts", "worth-store documentation owners", "authoritative_roadmap_c8_specification_link_exists plus roadmap_and_specification_share_exact_fates_and_entry_inputs", "none")),
        "C8-P1-LEDGER-01" => Ok(("The living ledger contains every Phase 1 semantic guarantee exactly once with causal evidence and closed finding history", "C8 closure-ledger gate", "phase_one_ledger_is_complete_resolved_and_bound_to_current_source plus causal ledger mutants", "historical review capture tokens are retained as non-reproducible non-proof; the current closure review is anchored to the exact reviewed commit")),
        "C8-P1-LEDGER-02" => Ok(("Every proved Phase 1 row is bound to its own causally relevant current source closure", "C8 closure-ledger source identity", "per_guarantee_source_closures_are_causal over role-specific artifacts producers consumers observer children trybuild and gates", "historical review capture tokens are not source proof; committed closure evidence is revision anchored")),
        "C8-P1-ENTRY-01" => Ok(("The planned fresh-process request admits exactly five owner-governed inputs and rejects every live runtime handoff decoded-artifact and prior-identity proxy", "C8 authority trace", "exact entry-input and forbidden-input families plus omitted_and_foreign_live_input_mutants_are_rejected", "none")),
        "C8-P1-PERSISTED-01" => Ok(("Every authoritative persisted recovery role names one real producer and admission pair or one exact owner-assigned producer gap", "C8 persisted-input inventory", "parsed owner-qualified functions exact call paths receiver chains local callbacks and named eager callback contracts under warnings-denied Store type checking plus transitive configured literal-boolean-dead shadowed lazy-callback and wrong-callee mutants", "runtime execution proof belongs to implementation phases 2 through 9")),
        other => Err(format!("unknown C.8 guarantee contract `{other}`")),
    }
}

fn specification_requirement_ids(document: &str) -> Result<BTreeSet<&str>, String> {
    let body = document
        .split_once(REQUIREMENTS_START)
        .and_then(|(_, tail)| tail.split_once(REQUIREMENTS_END).map(|(body, _)| body))
        .ok_or_else(|| "C.8 Phase 1 requirement markers are missing".to_owned())?;
    let mut lines = body.lines().filter(|line| !line.trim().is_empty());
    let header = lines.next().unwrap_or_default();
    let divider = lines.next().unwrap_or_default();
    if !header.contains("Requirement ID") || !divider.contains("---") {
        return Err("C.8 Phase 1 requirement table has an invalid header".into());
    }
    lines
        .map(|line| {
            let columns = line
                .trim()
                .trim_matches('|')
                .split('|')
                .map(str::trim)
                .collect::<Vec<_>>();
            if columns.len() != 2 || columns[1].is_empty() {
                return Err("C.8 Phase 1 requirement row must have two columns".to_owned());
            }
            Ok(columns[0])
        })
        .collect()
}

fn parse_rows(document: &str) -> Result<Vec<LedgerRow>, String> {
    let body = document
        .split_once(START)
        .and_then(|(_, tail)| tail.split_once(END).map(|(body, _)| body))
        .ok_or_else(|| "C.8 closure ledger markers are missing".to_owned())?;
    let mut lines = body.lines().filter(|line| !line.trim().is_empty());
    let header = lines
        .next()
        .ok_or_else(|| "C.8 ledger header is missing".to_owned())?;
    let divider = lines
        .next()
        .ok_or_else(|| "C.8 ledger divider is missing".to_owned())?;
    if !header.contains("Guarantee") || !divider.contains("---") {
        return Err("C.8 ledger table has an invalid header".into());
    }
    lines
        .enumerate()
        .map(|(index, line)| parse_row(line).map_err(|error| format!("row {}: {error}", index + 3)))
        .collect()
}

fn parse_row(line: &str) -> Result<LedgerRow, String> {
    let columns = line
        .trim()
        .trim_matches('|')
        .split('|')
        .map(str::trim)
        .collect::<Vec<_>>();
    if columns.len() != 9 || columns.iter().any(|column| column.is_empty()) {
        return Err("expected nine nonempty ledger columns".into());
    }
    Ok(LedgerRow {
        id: columns[0].to_owned(),
        phase: columns[1].to_owned(),
        claim: columns[2].to_owned(),
        implementation_owner: columns[3].to_owned(),
        evidence: columns[4].to_owned(),
        source_identity: columns[5].to_owned(),
        status: columns[6].to_owned(),
        reopened_by: columns[7].to_owned(),
        unsupported_scope: columns[8].to_owned(),
    })
}

#[derive(Clone)]
struct LedgerRow {
    id: String,
    phase: String,
    claim: String,
    implementation_owner: String,
    evidence: String,
    source_identity: String,
    status: String,
    reopened_by: String,
    unsupported_scope: String,
}
