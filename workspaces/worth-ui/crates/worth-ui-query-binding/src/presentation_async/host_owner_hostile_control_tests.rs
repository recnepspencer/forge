use super::authority_tests::installed_owner_parts;
use super::tests::{basis, installed_owner};
use super::*;

const CASES: [&str; 10] = [
    "completion-without-query-admission",
    "physical-completion-outside-signal",
    "foreign-or-stale-basis",
    "duplicate-or-out-of-order-current",
    "indeterminate-flattened-to-completed",
    "signal-as-effect-authority",
    "cross-graph-authority",
    "serialized-recovery-authority",
    "reporting-material-as-authority",
    "terminal-query-resource-leak",
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Verdict {
    CompilerRejected,
    AdmissionDenied,
    CompletionDenied,
    Unresolved,
    TerminalZero,
}

#[derive(Debug)]
struct CaseOutcome {
    case: &'static str,
    verdict: Verdict,
    trace: &'static str,
}

#[test]
fn typed_async_hostile_family_matches_the_independent_transition_adjudicator() {
    let (mut left, mut left_issuer) = installed_owner_parts();
    let (mut right, mut right_issuer) = installed_owner_parts();
    let left_graph = left
        .workspace
        .owned_async_runtime_topology()
        .unwrap()
        .signal_graph_instance();
    let right_graph = right
        .workspace
        .owned_async_runtime_topology()
        .unwrap()
        .signal_graph_instance();
    assert_ne!(left_graph, right_graph);
    let mut outcomes = Vec::with_capacity(CASES.len());

    let foreign_pending = right
        .admit_pending(right_issuer.issue(basis(101)).unwrap())
        .unwrap();
    let no_admission_completion = left_issuer.certify_presented(&foreign_pending, 64);
    let no_admission = left.admit_presented(&foreign_pending, no_admission_completion);
    outcomes.push(outcome(
        CASES[0],
        denial_verdict(no_admission),
        "foreign pending has no Query admission in the settling owner",
    ));
    outcomes.push(compile_outcome(
        CASES[1],
        "certification-phase5-async-authority",
    ));

    let foreign_request = right_issuer.issue(basis(102)).unwrap();
    let foreign_basis = left.admit_pending(foreign_request);
    outcomes.push(outcome(
        CASES[2],
        if matches!(
            foreign_basis,
            Err(WorthUiPresentationPendingAdmissionDenial::ForeignCorrespondenceAuthority)
        ) {
            Verdict::AdmissionDenied
        } else {
            panic!("foreign correspondence authority entered Query admission")
        },
        "owner rejected foreign correspondence authority before Query admission",
    ));

    let pending = left
        .admit_pending(left_issuer.issue(basis(103)).unwrap())
        .unwrap();
    let completion = left_issuer.certify_presented(&pending, 64);
    left.admit_presented(&pending, completion).unwrap();
    let duplicate = left_issuer.certify_presented(&pending, 64);
    outcomes.push(outcome(
        CASES[3],
        denial_verdict(left.admit_presented(&pending, duplicate)),
        "consumed pending receipt cannot settle a second completion",
    ));

    let mut indeterminate = installed_owner();
    let pending = indeterminate.admit_pending(basis(104)).unwrap();
    let unresolved = indeterminate.admit_effects_indeterminate(&pending).unwrap();
    outcomes.push(outcome(
        CASES[4],
        if unresolved.observation().posture() == WorthUiPresentationAsyncPosture::Unresolved {
            Verdict::Unresolved
        } else {
            panic!("effects-indeterminate completion was flattened")
        },
        "owner retained Unresolved rather than fabricating Current",
    ));
    assert_cancellation_postures();
    outcomes.push(compile_outcome(
        CASES[5],
        "certification-phase5-signal-effect-authority",
    ));

    let cross_graph_completion = left_issuer.certify_presented(&foreign_pending, 64);
    outcomes.push(outcome(
        CASES[6],
        denial_verdict(left.admit_presented(&foreign_pending, cross_graph_completion)),
        "foreign graph receipt cannot settle through local graph authority",
    ));
    outcomes.push(compile_outcome(
        CASES[7],
        "certification-phase5-serialized-recovery-authority",
    ));
    outcomes.push(compile_outcome(
        CASES[8],
        "certification-phase5-reporting-authority",
    ));

    left.close_terminal_resources().unwrap();
    indeterminate.close_terminal_resources().unwrap();
    right.reject_before_effects(&foreign_pending).unwrap();
    let terminal = right.close_terminal_resources().unwrap();
    outcomes.push(outcome(
        CASES[9],
        if terminal.closed_query_resources() == 0 {
            Verdict::TerminalZero
        } else {
            panic!("terminal owner retained Query resources")
        },
        "terminal close retained zero Query resources",
    ));

    assert_eq!(outcomes.len(), CASES.len());
    for (ordinal, observed) in outcomes.iter().enumerate() {
        assert_eq!(observed.case, CASES[ordinal]);
        assert!(!observed.trace.is_empty());
        assert_eq!(observed.verdict, independently_predicted(observed.case));
    }
    println!(
        "WORTH_UI_LEDGER_MUTATION_CONTROLS={{\"P5-TEXT-ASYNC-PRESENTATION-01\":\"bypass-query-or-stale-presentation-completion\"}}"
    );
    println!(
        "WORTH_UI_LEDGER_MUTATION_CASES={{\"P5-TEXT-ASYNC-PRESENTATION-01\":[\"completion-without-query-admission\",\"physical-completion-outside-signal\",\"foreign-or-stale-basis\",\"duplicate-or-out-of-order-current\",\"indeterminate-flattened-to-completed\",\"signal-as-effect-authority\",\"cross-graph-authority\",\"serialized-recovery-authority\",\"reporting-material-as-authority\",\"terminal-query-resource-leak\"]}}"
    );
}

fn assert_cancellation_postures() {
    let mut before_effects = installed_owner();
    let pending = before_effects.admit_pending(basis(105)).unwrap();
    before_effects.cancel_before_effects(&pending).unwrap();
    let close = before_effects.close_terminal_resources().unwrap();
    assert!(close
        .transitions()
        .iter()
        .any(|transition| transition.kind() == WorthUiPresentationTransitionKind::Cancelled));

    let mut partial_effects = installed_owner();
    let pending = partial_effects.admit_pending(basis(106)).unwrap();
    let unresolved = partial_effects
        .cancel_after_effects_may_have_begun(&pending)
        .unwrap();
    assert_eq!(
        unresolved.observation().posture(),
        WorthUiPresentationAsyncPosture::Unresolved
    );
    partial_effects.close_terminal_resources().unwrap();
}

fn denial_verdict(
    result: Result<WorthUiPresentationPresentedReceipt, WorthUiPresentationSettlementDenial>,
) -> Verdict {
    match result {
        Err(
            WorthUiPresentationSettlementDenial::ForeignPendingReceiptAuthority
            | WorthUiPresentationSettlementDenial::ForeignCompletionAuthority
            | WorthUiPresentationSettlementDenial::CompletionReceiptMismatch
            | WorthUiPresentationSettlementDenial::InvalidPendingReceipt,
        ) => Verdict::CompletionDenied,
        _ => panic!("hostile completion was not causally denied"),
    }
}

fn compile_outcome(case: &'static str, target: &'static str) -> CaseOutcome {
    let expected = match case {
        "physical-completion-outside-signal" => "certification-phase5-async-authority",
        "signal-as-effect-authority" => "certification-phase5-signal-effect-authority",
        "serialized-recovery-authority" => "certification-phase5-serialized-recovery-authority",
        "reporting-material-as-authority" => "certification-phase5-reporting-authority",
        _ => panic!("compile outcome requested for a runtime-hostile case"),
    };
    assert_eq!(target, expected);
    assert_governed_compile_receipt(target);
    outcome(
        case,
        Verdict::CompilerRejected,
        "governed compile artifact owns exact fail/pass authority pair",
    )
}

fn assert_governed_compile_receipt(target: &str) {
    use sha2::{Digest, Sha256};

    let repository = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../..");
    let artifact_path =
        repository.join("_docs/worth-ui/milestone-3.14.1-evidence/compile-contracts.json");
    let artifact: serde_json::Value = serde_json::from_slice(
        &std::fs::read(&artifact_path).expect("governed compile artifact is readable"),
    )
    .expect("governed compile artifact is valid JSON");
    let matches = artifact["cases"]
        .as_array()
        .expect("compile artifact carries cases")
        .iter()
        .filter(|row| row["owner"] == "certification" && row["kind"] == "fail")
        .filter(|row| row["target"] == target)
        .collect::<Vec<_>>();
    let [row] = matches.as_slice() else {
        panic!("compile target {target} is not represented exactly once");
    };
    for (path_field, digest_field) in [("source", "source_sha256"), ("snapshot", "snapshot_sha256")]
    {
        let path = row[path_field]
            .as_str()
            .expect("compile receipt retains its governed path");
        let expected = row[digest_field]
            .as_str()
            .expect("compile receipt retains its governed digest");
        let actual = Sha256::digest(
            std::fs::read(repository.join(path)).expect("compile receipt source is readable"),
        );
        let actual = actual
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect::<String>();
        assert_eq!(actual, expected, "compile receipt drifted for {target}");
    }
}

const fn outcome(case: &'static str, verdict: Verdict, trace: &'static str) -> CaseOutcome {
    CaseOutcome {
        case,
        verdict,
        trace,
    }
}

fn independently_predicted(case: &str) -> Verdict {
    match case {
        "completion-without-query-admission"
        | "duplicate-or-out-of-order-current"
        | "cross-graph-authority" => Verdict::CompletionDenied,
        "foreign-or-stale-basis" => Verdict::AdmissionDenied,
        "physical-completion-outside-signal"
        | "signal-as-effect-authority"
        | "serialized-recovery-authority"
        | "reporting-material-as-authority" => Verdict::CompilerRejected,
        "indeterminate-flattened-to-completed" => Verdict::Unresolved,
        "terminal-query-resource-leak" => Verdict::TerminalZero,
        _ => panic!("independent adjudicator received an unknown fault"),
    }
}
