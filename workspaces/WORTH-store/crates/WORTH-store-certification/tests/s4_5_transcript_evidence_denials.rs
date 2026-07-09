#[path = "s4_5_counter_strength/support.rs"]
mod counter_support;

use worth_store_physical_certification::{
    CounterContractOracle, ExecutedTranscriptParts, FixtureCapabilityDeclaration,
    FixtureMutationBoundary, LargeStoreFixtureProfile, ObservedPhysicalTrace,
    PhysicalFixtureBuilder, PhysicalInterleavingSchedule, PhysicalSimulationPlan,
    PhysicalSimulationProfile, ProductionBackedPhysicalFixture, ReusablePhysicalOracleFamily,
    StateSpaceBudget, TerminalProjectionOnlyEvidenceDenied, TranscriptReplayDenial,
    TranscriptReplayOracle,
};
use worth_store_physical_certification::{OracleFamilyKind, PhysicalProofOracleVerdict};
use worth_store_test_support::{
    developer_smoke_replay_seed, production_backed_physical_fixture_materialization,
};

#[test]
fn transcript_and_evidence_deny_missing_or_wrong_verdicts() {
    let plan = counter_support::lower_s5_plan();
    let trace = counter_support::observed_trace(&plan);
    let counter_receipt = counter_support::counter_receipt(&plan, trace.clone());
    let parts = ExecutedTranscriptParts::new(
        &plan,
        schedule(&plan),
        &production_fixture(),
        trace.clone(),
        counter_receipt,
    )
    .unwrap();

    assert_eq!(
        worth_store_physical_certification::PhysicalSimulationTranscript::from_executed_parts(
            parts
        )
        .unwrap_err(),
        TranscriptReplayDenial::MissingOracleVerdict
    );

    let wrong_verdict = ReusablePhysicalOracleFamily::s5_readiness_shape()
        .oracle(CounterContractOracle)
        .judge(&plan, &trace)
        .unwrap();
    let wrong_parts = executed_parts(&plan).with_oracle_verdict(wrong_verdict);

    assert_eq!(
        worth_store_physical_certification::PhysicalSimulationTranscript::from_executed_parts(
            wrong_parts,
        )
        .unwrap_err(),
        TranscriptReplayDenial::MissingTranscriptReplayOracleVerdict
    );
}

#[test]
fn transcript_denies_replay_only_verdict_when_plan_requires_another_oracle_family() {
    let plan = counter_support::lower_s5_plan();
    let trace = counter_support::observed_trace(&plan);
    let counter_receipt = counter_support::counter_receipt(&plan, trace.clone());
    let replay_only_parts = ExecutedTranscriptParts::new(
        &plan,
        schedule(&plan),
        &production_fixture(),
        trace,
        counter_receipt,
    )
    .unwrap()
    .with_transcript_replay_verdict()
    .unwrap();

    let denial =
        worth_store_physical_certification::PhysicalSimulationTranscript::from_executed_parts(
            replay_only_parts,
        )
        .unwrap_err();

    assert_eq!(
        denial,
        TranscriptReplayDenial::RequiredOracleFamilyMissing(OracleFamilyKind::S5ReadinessShape)
    );
}

#[test]
fn transcript_denies_oracle_verdicts_from_a_different_plan() {
    let plan = counter_support::lower_s5_plan();
    let alternate_plan =
        counter_support::lower_s5_plan_for_profile(PhysicalSimulationProfile::CiCertification);
    let alternate_trace = counter_support::observed_trace(&alternate_plan);
    let copied_verdict = ReusablePhysicalOracleFamily::transcript_replay_evidence()
        .oracle(TranscriptReplayOracle)
        .judge(&alternate_plan, &alternate_trace)
        .unwrap();

    let denial =
        worth_store_physical_certification::PhysicalSimulationTranscript::from_executed_parts(
            executed_parts(&plan).with_oracle_verdict(copied_verdict),
        )
        .unwrap_err();

    assert_eq!(denial, TranscriptReplayDenial::OracleVerdictPlanMismatch);
}

#[test]
fn proof_light_transcript_replay_verdict_denies_without_replay_basis() {
    let plan = counter_support::lower_s5_plan();
    let trace = counter_support::observed_trace(&plan);
    let copied_shape_only_verdict = ReusablePhysicalOracleFamily::transcript_replay_evidence()
        .oracle(TranscriptReplayOracle)
        .judge(&plan, &trace)
        .unwrap();

    let denial =
        worth_store_physical_certification::PhysicalSimulationTranscript::from_executed_parts(
            executed_parts(&plan).with_oracle_verdict(copied_shape_only_verdict),
        )
        .unwrap_err();

    assert_eq!(
        denial,
        TranscriptReplayDenial::TranscriptReplayVerdictMissingReplayEvidence
    );
}

#[test]
fn transcript_denies_counter_receipts_admitted_for_a_different_plan() {
    let plan = counter_support::lower_s5_plan();
    let trace = counter_support::observed_trace(&plan);
    let alternate_plan =
        counter_support::lower_s5_plan_for_profile(PhysicalSimulationProfile::CiCertification);
    let alternate_receipt = counter_support::counter_receipt(
        &alternate_plan,
        counter_support::observed_trace(&alternate_plan),
    );

    let denial = ExecutedTranscriptParts::new(
        &plan,
        schedule(&plan),
        &production_fixture(),
        trace,
        alternate_receipt,
    )
    .unwrap_err();

    assert_eq!(denial, TranscriptReplayDenial::CounterReceiptPlanMismatch);
}

#[test]
fn transcript_denies_missing_seed_before_replay_bundle_exists() {
    let plan = counter_support::lower_s5_plan();
    let trace = counter_support::observed_trace(&plan);
    let fixture = production_fixture();
    let counter_receipt = counter_support::counter_receipt(&plan, trace.clone());

    let denial = ExecutedTranscriptParts::from_optional_schedule(
        &plan,
        None,
        &fixture,
        trace,
        counter_receipt,
    )
    .unwrap_err();

    assert_eq!(denial, TranscriptReplayDenial::MissingSeed);
}

#[test]
fn loose_logs_terminal_json_and_same_run_comparisons_are_denials() {
    assert_eq!(
        worth_store_physical_certification::reject_loose_log_transcript_attempt().unwrap_err(),
        TranscriptReplayDenial::LooseLogDenied
    );
    assert_eq!(
        worth_store_physical_certification::reject_terminal_json_transcript_attempt().unwrap_err(),
        TranscriptReplayDenial::TerminalJsonDenied
    );
    assert_eq!(
        worth_store_physical_certification::reject_same_run_self_comparison_transcript_attempt()
            .unwrap_err(),
        TranscriptReplayDenial::SameRunSelfComparisonDenied
    );
    assert_eq!(
        worth_store_physical_certification::reject_loose_log_evidence_attempt().unwrap_err(),
        worth_store_physical_certification::PhysicalEvidenceBundleDenial::LooseLogDenied
    );
    assert_eq!(
        worth_store_physical_certification::reject_terminal_json_evidence_attempt().unwrap_err(),
        TerminalProjectionOnlyEvidenceDenied::TerminalJsonProjection
    );
}

fn executed_parts(plan: &PhysicalSimulationPlan) -> ExecutedTranscriptParts {
    let trace = counter_support::observed_trace(plan);
    let counter_receipt = counter_support::counter_receipt(plan, trace.clone());
    let readiness_verdict = s5_readiness_verdict(plan, &trace);
    ExecutedTranscriptParts::new(
        plan,
        schedule(plan),
        &production_fixture(),
        trace,
        counter_receipt,
    )
    .unwrap()
    .with_oracle_verdict(readiness_verdict)
}

fn s5_readiness_verdict(
    plan: &PhysicalSimulationPlan,
    trace: &ObservedPhysicalTrace,
) -> PhysicalProofOracleVerdict {
    ReusablePhysicalOracleFamily::s5_readiness_shape()
        .oracle(CounterContractOracle)
        .judge(plan, trace)
        .unwrap()
}

fn schedule(plan: &PhysicalSimulationPlan) -> PhysicalInterleavingSchedule {
    PhysicalInterleavingSchedule::from_lowered_plan(
        plan,
        developer_smoke_replay_seed(),
        StateSpaceBudget::bounded_steps(8).unwrap(),
    )
    .unwrap()
}

fn production_fixture() -> ProductionBackedPhysicalFixture {
    PhysicalFixtureBuilder::production_backed("phase10-transcript-denial-fixture")
        .materialize_with(
            production_backed_physical_fixture_materialization(
                LargeStoreFixtureProfile::StoreLargerThanMemory,
                10,
            )
            .unwrap(),
        )
        .capability(FixtureCapabilityDeclaration::for_mutation_boundary(
            FixtureMutationBoundary::Manifest,
        ))
        .and_reopen_through_physical_authority()
        .unwrap()
}
