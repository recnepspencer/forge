use worth_ui::facade::WorthUi;
use worth_ui_harness::facade::{
    HarnessDigestExpectationDenial, HarnessEvidenceFamily, HarnessEvidenceRequirement,
    HarnessEvidenceValidationDenial, HarnessExpectedObservation, HarnessHonestyDenial,
    HarnessReplayDenial, HarnessRunDenial, HarnessRunner, HarnessScenario,
    HarnessScenarioOperation, HarnessScenarioStep,
};

#[test]
fn scenario_replay_produces_equivalent_evidence_bundle() {
    let first_app = WorthUi::app().freeze();
    let first_scenario = replayable_launch_scenario(&first_app, "harness.replay.equivalent");
    let first = HarnessRunner::for_app(first_app)
        .run(first_scenario)
        .expect("initial scenario should complete");

    let replay_app = WorthUi::app().freeze();
    let replay_scenario = replayable_launch_scenario(&replay_app, "harness.replay.equivalent");

    let replay = HarnessRunner::for_app(replay_app)
        .replay(first.replay_record(), replay_scenario)
        .expect("same fixture should replay equivalently");

    assert_eq!(first.evidence(), replay.evidence());
    assert_eq!(first.evidence_ledger(), replay.evidence_ledger());
    assert_eq!(
        first.replay_record().operation_identities(),
        replay.replay_record().operation_identities()
    );
}

#[test]
fn scenario_replay_rejects_operation_identity_drift() {
    let first_app = WorthUi::app().freeze();
    let first_scenario = replayable_launch_scenario(&first_app, "harness.replay.drift");
    let first = HarnessRunner::for_app(first_app)
        .run(first_scenario)
        .expect("initial scenario should complete");
    let drifted = HarnessScenario::define("harness.replay.drift")
        .expect("valid scenario id")
        .step(HarnessScenarioStep::new(
            "observe-frame",
            HarnessScenarioOperation::observe_visible_frame(),
        ));

    let denial = HarnessRunner::for_app(WorthUi::app().freeze())
        .replay(first.replay_record(), drifted)
        .expect_err("replay operation identity drift must fail");

    match denial {
        HarnessRunDenial::ReplayMismatch {
            denial:
                HarnessReplayDenial::OperationIdentityChanged {
                    step_index,
                    expected,
                    provided,
                },
        } => {
            assert_eq!(step_index, 0);
            assert_eq!(expected, "harness.operation.launch_runtime");
            assert_eq!(provided, "harness.operation.observe_visible_frame");
        }
        other => panic!("expected operation identity replay denial, got {other:?}"),
    }
}

#[test]
fn scenario_replay_rejects_extra_completed_operation() {
    let first_app = WorthUi::app().freeze();
    let first_scenario = replayable_launch_scenario(&first_app, "harness.replay.extra-operation");
    let first = HarnessRunner::for_app(first_app)
        .run(first_scenario)
        .expect("initial scenario should complete");

    let replay_app = WorthUi::app().freeze();
    let replay_launch = worth_ui_harness::workbench::minimal_workbench_launch(&replay_app)
        .expect("minimal harness launch should prepare");
    let replay_with_extra_step = HarnessScenario::define("harness.replay.extra-operation")
        .expect("valid scenario id")
        .step(HarnessScenarioStep::new(
            "launch-runtime",
            HarnessScenarioOperation::launch_runtime(replay_launch),
        ))
        .step(HarnessScenarioStep::new(
            "observe-frame",
            HarnessScenarioOperation::observe_visible_frame(),
        ));

    let denial = HarnessRunner::for_app(replay_app)
        .replay(first.replay_record(), replay_with_extra_step)
        .expect_err("replay must reject extra completed operations");

    match denial {
        HarnessRunDenial::ReplayMismatch {
            denial: HarnessReplayDenial::OperationCountChanged { expected, provided },
        } => {
            assert_eq!(expected, 1);
            assert_eq!(provided, 2);
        }
        other => panic!("expected operation count replay denial, got {other:?}"),
    }
}

#[test]
fn scenario_expected_digest_must_be_derived_from_run_inputs() {
    let app = WorthUi::app().freeze();
    let launch = worth_ui_harness::workbench::minimal_workbench_launch(&app)
        .expect("minimal harness launch should prepare");
    let scenario = HarnessScenario::define("harness.digest.fixed")
        .expect("valid scenario id")
        .step(
            HarnessScenarioStep::new(
                "launch-with-fixed-digest",
                HarnessScenarioOperation::launch_runtime(launch),
            )
            .expects(
                HarnessExpectedObservation::active_plan_digest_fixed_for_diagnostics(u64::MAX),
            ),
        );

    let denial = HarnessRunner::for_app(app)
        .run(scenario)
        .expect_err("fixed digest not derived from current run must fail");

    assert_active_plan_fixed_digest_denial(denial, None);
}

#[test]
fn scenario_expected_digest_rejects_even_matching_literal_digest() {
    let first_app = WorthUi::app().freeze();
    let first_scenario = replayable_launch_scenario(&first_app, "harness.digest.literal-source");
    let first = HarnessRunner::for_app(first_app)
        .run(first_scenario)
        .expect("source scenario should complete");
    let literal_digest = first
        .evidence()
        .basis()
        .expect("launch scenario should record a basis")
        .active_plan_digest();

    let app = WorthUi::app().freeze();
    let launch = worth_ui_harness::workbench::minimal_workbench_launch(&app)
        .expect("minimal harness launch should prepare");
    let scenario = HarnessScenario::define("harness.digest.literal-rejected")
        .expect("valid scenario id")
        .step(
            HarnessScenarioStep::new(
                "launch-with-matching-literal-digest",
                HarnessScenarioOperation::launch_runtime(launch),
            )
            .expects(
                HarnessExpectedObservation::active_plan_digest_fixed_for_diagnostics(
                    literal_digest,
                ),
            ),
        );

    let denial = HarnessRunner::for_app(app)
        .run(scenario)
        .expect_err("matching literal digests must still fail derivation honesty");

    assert_active_plan_fixed_digest_denial(denial, Some((literal_digest, literal_digest)));
}

fn replayable_launch_scenario(app: &worth_ui::facade::WorthUiApp, id: &str) -> HarnessScenario {
    let launch = worth_ui_harness::workbench::minimal_workbench_launch(app)
        .expect("minimal harness launch should prepare");
    HarnessScenario::define(id)
        .expect("valid scenario id")
        .step(
            HarnessScenarioStep::new(
                "launch-runtime",
                HarnessScenarioOperation::launch_runtime(launch),
            )
            .expects(HarnessExpectedObservation::runtime_receipt())
            .expects(HarnessExpectedObservation::active_plan_digest_derived_from_run())
            .requires(HarnessEvidenceRequirement::runtime_receipt())
            .requires(HarnessEvidenceRequirement::operation_receipt())
            .requires(HarnessEvidenceRequirement::active_plan_digest()),
        )
}

fn assert_active_plan_fixed_digest_denial(
    denial: HarnessRunDenial,
    expected_values: Option<(u64, u64)>,
) {
    match denial {
        HarnessRunDenial::Honesty { location, denial } => {
            assert_eq!(location.step_index(), 0);
            assert_eq!(
                location.evidence_family(),
                Some(HarnessEvidenceFamily::ActivePlanDigest)
            );
            match denial {
                HarnessHonestyDenial::EvidenceValidation(
                    HarnessEvidenceValidationDenial::DigestExpectation(
                        HarnessDigestExpectationDenial::FixedDigestRejected {
                            family,
                            expected,
                            actual,
                        },
                    ),
                ) => {
                    assert_eq!(family, HarnessEvidenceFamily::ActivePlanDigest);
                    if let Some((expected_literal, actual_digest)) = expected_values {
                        assert_eq!(expected, expected_literal);
                        assert_eq!(actual, actual_digest);
                    }
                }
                other => panic!("expected fixed digest denial, got {other:?}"),
            }
        }
        other => panic!("expected localized digest denial, got {other:?}"),
    }
}
