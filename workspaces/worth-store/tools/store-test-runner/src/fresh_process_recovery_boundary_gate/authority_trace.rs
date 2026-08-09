use std::collections::BTreeSet;

use super::documents::{read_repository_document, split_csv, AUTHORITY_TRACE};

const HEADER: &str = "family,member,owner,substrate,binding_axes,authorizes,phase";
const ENTRY_AXES: &[&str] = &[
    "root-ownership-identity",
    "stable-store-identity",
    "recovery-session-identity",
    "backend-profile-identity",
    "qualified-media-generation",
    "static-configuration-identity",
    "recovery-limit-identity",
];
const ENTRY_INPUTS: &[&str] = &[
    "store-root-locator",
    "static-store-configuration",
    "qualified-backend-profile",
    "physical-recovery-limits",
    "physical-recovery-platform-authority",
];
const FORBIDDEN_ENTRY_INPUTS: &[&str] = &[
    "Store",
    "ServingPhysicalRuntime",
    "PhysicalDurabilityRecoveryHandoff",
    "BufferPoolHandle",
    "SignalGraph",
    "Scheduler",
    "DecodedArtifactCollection",
    "ExpectedRecordModel",
    "PriorRuntimeIdentity",
];
const PHASE_STATES: &[&str] = &[
    "AdmittedPhysicalRecovery",
    "DiscoveredPhysicalRecovery",
    "SelectedPhysicalRecovery",
    "PlannedPhysicalRecovery",
    "StagedPhysicalRecovery",
    "NamespaceDurablePhysicalRecovery",
    "ReopenedPhysicalRecovery",
    "RecoveredPhysicalRuntimeHandoff",
];
const TERMINALS: &[&str] = &[
    "Recovered",
    "Refused",
    "Blocked",
    "PublicationIndeterminate",
];
const PERFORMED_EFFECTS: &[&str] = &[
    "staging-write",
    "recovered-root-replacement",
    "namespace-synchronization",
    "independent-reopen",
    "cleanup-removal",
];
const FRESHNESS_POLICIES: &[&str] = &[
    "idempotency-binding:selected-checkpoint-generation",
    "cleanup-plan:current-published-root-generation",
];
const REPORT_PROTOCOLS: &[&str] = &[
    "store.physical.recovery-report@1[1-1]",
    "store.physical.recovery-observer-report@1[1-1]",
];
const FATES: &[&str] = &[
    "AcknowledgedDurable",
    "DurableUnacknowledged",
    "ProvenNoEffect",
    "Indeterminate",
];
const CONCRETE_AUTHORITIES: &[&str] = &[
    "PhysicalRecoveryPlatformAuthority",
    "PhysicalRecoveryConstructionAuthority",
];

#[test]
fn authority_trace_locks_every_c8_contract_family_exactly() {
    let document = read_repository_document(AUTHORITY_TRACE).expect("read C.8 authority trace");
    let rows = parse_trace(&document).expect("parse C.8 authority trace");
    assert_family(&rows, "entry-axis", ENTRY_AXES);
    assert_family(&rows, "entry-input", ENTRY_INPUTS);
    assert_family(&rows, "entry-forbidden-input", FORBIDDEN_ENTRY_INPUTS);
    assert_family(&rows, "phase-state", PHASE_STATES);
    assert_family(&rows, "session-terminal", TERMINALS);
    assert_family(&rows, "performed-effect", PERFORMED_EFFECTS);
    assert_family(&rows, "freshness-policy", FRESHNESS_POLICIES);
    assert_family(&rows, "report-protocol", REPORT_PROTOCOLS);
    assert_family(&rows, "operation-fate", FATES);
    assert_family(&rows, "concrete-authority", CONCRETE_AUTHORITIES);
    assert_eq!(
        rows.len(),
        ENTRY_AXES.len()
            + ENTRY_INPUTS.len()
            + FORBIDDEN_ENTRY_INPUTS.len()
            + PHASE_STATES.len()
            + TERMINALS.len()
            + PERFORMED_EFFECTS.len()
            + FRESHNESS_POLICIES.len()
            + REPORT_PROTOCOLS.len()
            + FATES.len()
            + CONCRETE_AUTHORITIES.len(),
        "C.8 authority trace contains an unclassified row"
    );
}

#[test]
fn generic_proof_and_report_values_open_no_store_door() {
    let document = read_repository_document(AUTHORITY_TRACE).expect("read C.8 authority trace");
    let rows = parse_trace(&document).expect("parse C.8 authority trace");
    assert!(!document.contains("AuthorityMarker"));
    for row in rows {
        assert!(!matches!(
            row.owner.as_str(),
            "recovery" | "physics" | "support" | "evidence" | "utility"
        ));
        match row.family.as_str() {
            "entry-axis" => {
                assert_eq!(row.substrate, "worth-proof-binding");
                assert_exact_entry_axis(&row);
            }
            "entry-input" => {
                assert_eq!(row.substrate, "store-concrete-input");
                assert_eq!(row.authorizes, "request-declaration-only");
            }
            "entry-forbidden-input" => {
                assert_eq!(row.substrate, "forbidden-live-or-derived-proxy");
                assert_eq!(row.binding_axes, "none");
                assert_eq!(row.authorizes, "nothing");
            }
            "session-terminal" => assert_eq!(row.substrate, "worth-proof-linear-resource"),
            "performed-effect" | "freshness-policy" => {
                validate_exact_semantics(&row).expect("exact C.8 Proof contract")
            }
            "report-protocol" => {
                assert_eq!(row.substrate, "worth-foundational-boundary-protocol");
                assert_eq!(row.authorizes, "descriptive-report-interpretation-only");
            }
            "phase-state" | "operation-fate" | "concrete-authority" => {}
            other => panic!("unknown C.8 authority family `{other}`"),
        }
        assert!(matches!(
            row.phase.as_str(),
            "phase-2" | "phase-3" | "phase-4" | "phase-5" | "phase-6" | "phase-7"
        ));
        assert!(!row.binding_axes.is_empty());
    }
}

#[test]
fn performed_and_freshness_axis_substitution_mutants_are_rejected() {
    let document = read_repository_document(AUTHORITY_TRACE).expect("read C.8 authority trace");
    let mut rows = parse_trace(&document).expect("parse C.8 authority trace");
    let performed = rows
        .iter_mut()
        .find(|row| row.member == "recovered-root-replacement")
        .expect("performed row");
    performed.binding_axes = "session-root-and-generation".into();
    assert!(validate_exact_semantics(performed).is_err());

    let freshness = rows
        .iter_mut()
        .find(|row| row.member == "idempotency-binding:selected-checkpoint-generation")
        .expect("freshness row");
    freshness.owner = "caller-supplied-generation".into();
    assert!(validate_exact_semantics(freshness).is_err());
}

fn validate_exact_semantics(row: &TraceRow) -> Result<(), String> {
    let expected = match row.member.as_str() {
        "staging-write" => ("recovery-runtime/orchestration-staging", "worth-proof-performed", "session-subject-generation-action-kind-outcome-and-effect-occurrence", "next-staged-page-transition"),
        "recovered-root-replacement" => ("recovery-runtime/orchestration-publication", "worth-proof-performed", "session-root-staging-generation-action-kind-outcome-and-effect-occurrence", "next-namespace-transition"),
        "namespace-synchronization" => ("recovery-runtime/orchestration-publication", "worth-proof-performed", "session-root-published-generation-action-kind-outcome-and-effect-occurrence", "namespace-durable-transition"),
        "independent-reopen" => ("recovery-runtime/orchestration-reopen", "worth-proof-performed", "session-root-published-generation-action-kind-outcome-and-effect-occurrence", "reopened-transition"),
        "cleanup-removal" => ("recovery-runtime/cleanup-execution", "worth-proof-performed", "session-cleanup-plan-artifact-action-kind-outcome-and-effect-occurrence", "one-artifact-cleanup-settlement"),
        "idempotency-binding:selected-checkpoint-generation" => ("worth-store/recovery-freshness/binding", "worth-proof-freshness", "stable-store-selected-checkpoint-binding-owner-sampled-generation-sealed-c7-basis-and-policy-identity", "matching-fate-branch-only"),
        "cleanup-plan:current-published-root-generation" => ("worth-store/recovery-freshness/cleanup", "worth-proof-freshness", "stable-store-current-published-root-cleanup-plan-artifact-owner-sampled-generation-sealed-publication-basis-and-policy-identity", "matching-cleanup-effect-only"),
        other => return Err(format!("unknown exact C.8 Proof member `{other}`")),
    };
    let actual = (
        row.owner.as_str(),
        row.substrate.as_str(),
        row.binding_axes.as_str(),
        row.authorizes.as_str(),
    );
    (actual == expected)
        .then_some(())
        .ok_or_else(|| format!("C.8 Proof semantics drifted for {}", row.member))
}

fn assert_exact_entry_axis(row: &TraceRow) {
    let expected = match row.member.as_str() {
        "stable-store-identity" => ("admitted-world-only", "admitted-world-construction-only"),
        "recovery-session-identity" => ("entry-and-admitted-world", "one-session-progression-only"),
        "backend-profile-identity" => ("entry-and-admitted-world", "profile-bound-entry-only"),
        "qualified-media-generation" => (
            "entry-and-admitted-world",
            "media-generation-bound-entry-only",
        ),
        "static-configuration-identity" => {
            ("entry-and-admitted-world", "configuration-bound-entry-only")
        }
        "recovery-limit-identity" => ("entry-and-admitted-world", "limit-bound-entry-only"),
        "root-ownership-identity" => ("entry-and-admitted-world", "entry-binding-comparison-only"),
        other => panic!("unknown C.8 entry binding axis `{other}`"),
    };
    assert_eq!(
        (row.binding_axes.as_str(), row.authorizes.as_str()),
        expected
    );
}

fn assert_family(rows: &[TraceRow], family: &str, expected: &[&str]) {
    let actual = rows
        .iter()
        .filter(|row| row.family == family)
        .map(|row| row.member.as_str())
        .collect::<BTreeSet<_>>();
    assert_eq!(
        actual,
        expected.iter().copied().collect(),
        "C.8 authority family `{family}` is incomplete"
    );
}

fn parse_trace(document: &str) -> Result<Vec<TraceRow>, String> {
    let mut lines = document.lines();
    if lines.next() != Some(HEADER) {
        return Err("C.8 authority trace has an invalid schema".into());
    }
    lines
        .filter(|line| !line.trim().is_empty())
        .enumerate()
        .map(|(index, line)| {
            let columns = split_csv(line, 7)
                .map_err(|error| format!("C.8 authority row {}: {error}", index + 2))?;
            Ok(TraceRow {
                family: columns[0].to_owned(),
                member: columns[1].to_owned(),
                owner: columns[2].to_owned(),
                substrate: columns[3].to_owned(),
                binding_axes: columns[4].to_owned(),
                authorizes: columns[5].to_owned(),
                phase: columns[6].to_owned(),
            })
        })
        .collect()
}

#[derive(Clone)]
struct TraceRow {
    family: String,
    member: String,
    owner: String,
    substrate: String,
    binding_axes: String,
    authorizes: String,
    phase: String,
}
