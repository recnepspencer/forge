const DEFAULT_ARTIFACT: &str =
    "_docs/worth-ui/milestone-3.14.1-evidence/p3-predecessor-handoff.json";
const PHASE_FOUR_ARTIFACT: &str =
    "_docs/worth-ui/milestone-3.14.1-evidence/p4-predecessor-handoff.json";
const PHASE_FIVE_ARTIFACT: &str =
    "_docs/worth-ui/milestone-3.14.1-evidence/p5-predecessor-handoff.json";
const PHASE_SIX_ARTIFACT: &str =
    "_docs/worth-ui/milestone-3.14.1-evidence/p6-predecessor-handoff.json";

#[test]
#[ignore = "Phase 3 gate: execute only after the governed predecessor verifier"]
fn phase_three_predecessor_handoff_is_current() {
    let identity = std::env::var("WORTH_UI_PREDECESSOR_ARTIFACT")
        .unwrap_or_else(|_| DEFAULT_ARTIFACT.to_owned());
    let observation = super::predecessor_artifact::validate(&identity)
        .expect("Phase 1-2 predecessor evidence must be current and complete");
    println!(
        "WORTH_UI_LEDGER_COUNTERS={{\"P3-PREDECESSOR-01\":{}}}",
        observation.requirement_count()
    );
}

#[test]
#[ignore = "Phase 4 gate: execute only after the governed predecessor verifier"]
fn phase_four_predecessor_handoff_is_current() {
    let identity = std::env::var("WORTH_UI_PREDECESSOR_ARTIFACT")
        .unwrap_or_else(|_| PHASE_FOUR_ARTIFACT.to_owned());
    let observation = super::predecessor_artifact::validate(&identity)
        .expect("Phase 1-3 predecessor evidence must be current and complete");
    assert_eq!(observation.requirement_count(), 47);
    println!(
        "WORTH_UI_LEDGER_COUNTERS={{\"P4-PREDECESSOR-01\":{}}}",
        observation.requirement_count()
    );
}

#[test]
#[ignore = "Phase 5 gate: execute only after the governed predecessor verifier"]
fn phase_five_predecessor_handoff_is_current() {
    let identity = std::env::var("WORTH_UI_PREDECESSOR_ARTIFACT")
        .unwrap_or_else(|_| PHASE_FIVE_ARTIFACT.to_owned());
    let observation = super::predecessor_artifact::validate(&identity)
        .expect("Phase 1-4 predecessor evidence must be current and complete");
    assert_eq!(observation.requirement_count(), 68);
    println!(
        "WORTH_UI_LEDGER_COUNTERS={{\"P5-PREDECESSOR-01\":{}}}",
        observation.requirement_count()
    );
}

#[test]
#[ignore = "Phase 6 gate: execute only after the governed predecessor verifier"]
fn phase_six_predecessor_handoff_is_current() {
    let identity = std::env::var("WORTH_UI_PREDECESSOR_ARTIFACT")
        .unwrap_or_else(|_| PHASE_SIX_ARTIFACT.to_owned());
    let observation = super::predecessor_artifact::validate(&identity)
        .expect("Phase 1-5 predecessor evidence must be current and complete");
    assert_eq!(observation.requirement_count(), 80);
    println!(
        "WORTH_UI_LEDGER_COUNTERS={{\"P6-PREDECESSOR-01\":{}}}",
        observation.requirement_count()
    );
}
