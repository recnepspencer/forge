use worth_ui::facade::WorthUi;
use worth_ui_harness::facade::{
    HarnessEvidenceFamily, HarnessEvidenceRequirement, HarnessEvidenceValidationDenial,
    HarnessHonestyDenial, HarnessRunDenial, HarnessRunner, HarnessScenario,
    HarnessScenarioOperation, HarnessScenarioStep,
};

#[test]
fn harness_result_requires_runtime_receipts_not_visual_success() {
    let scenario = HarnessScenario::define("harness.honesty.visual-only")
        .expect("valid scenario id")
        .step(
            HarnessScenarioStep::new(
                "observe-frame",
                HarnessScenarioOperation::observe_visible_frame(),
            )
            .requires(HarnessEvidenceRequirement::runtime_receipt()),
        );

    let denial = HarnessRunner::for_app(WorthUi::app().freeze())
        .run(scenario)
        .expect_err("visual-only evidence must not certify runtime success");

    assert_missing_runtime_receipt(denial);
}

#[test]
fn harness_rejects_app_local_shell_state_injection() {
    let scenario = HarnessScenario::define("harness.honesty.injected-state")
        .expect("valid scenario id")
        .step(
            HarnessScenarioStep::new(
                "inject-state",
                HarnessScenarioOperation::attempt_app_local_shell_state_injection(),
            )
            .requires(HarnessEvidenceRequirement::runtime_receipt()),
        );

    let denial = HarnessRunner::for_app(WorthUi::app().freeze())
        .run(scenario)
        .expect_err("app-local shell state injection must fail before completion");

    match denial {
        HarnessRunDenial::Honesty { location, denial } => {
            assert_eq!(location.step_index(), 0);
            assert_eq!(location.step_label(), "inject-state");
            assert_eq!(denial, HarnessHonestyDenial::AppLocalShellStateInjection);
        }
        other => panic!("expected localized honesty denial, got {other:?}"),
    }
}

#[test]
fn runtime_receipt_from_previous_step_cannot_certify_later_visual_step() {
    let app = WorthUi::app().freeze();
    let launch = worth_ui_harness::workbench::minimal_workbench_launch(&app)
        .expect("minimal harness launch should prepare");
    let scenario = HarnessScenario::define("harness.honesty.stale-step-evidence")
        .expect("valid scenario id")
        .step(HarnessScenarioStep::new(
            "launch-runtime",
            HarnessScenarioOperation::launch_runtime(launch),
        ))
        .step(
            HarnessScenarioStep::new(
                "observe-frame",
                HarnessScenarioOperation::observe_visible_frame(),
            )
            .requires(HarnessEvidenceRequirement::runtime_receipt()),
        );

    let denial = HarnessRunner::for_app(app)
        .run(scenario)
        .expect_err("a previous runtime receipt must not certify a later visual-only step");

    assert_missing_runtime_receipt(denial);
}

#[test]
fn empty_scenario_is_denied_instead_of_receipted_as_success() {
    let scenario = HarnessScenario::define("harness.honesty.empty").expect("valid scenario id");

    let denial = HarnessRunner::for_app(WorthUi::app().freeze())
        .run(scenario)
        .expect_err("an empty scenario must not produce a success receipt");

    match denial {
        HarnessRunDenial::EmptyScenario { scenario_id } => {
            assert_eq!(scenario_id.as_str(), "harness.honesty.empty");
        }
        other => panic!("expected empty scenario denial, got {other:?}"),
    }
}

fn assert_missing_runtime_receipt(denial: HarnessRunDenial) {
    match denial {
        HarnessRunDenial::Honesty { location, denial } => match denial {
            HarnessHonestyDenial::EvidenceValidation(
                HarnessEvidenceValidationDenial::MissingRequiredEvidence { family },
            ) => {
                assert_eq!(family, HarnessEvidenceFamily::RuntimeReceipt);
                assert_eq!(
                    location.evidence_family(),
                    Some(HarnessEvidenceFamily::RuntimeReceipt)
                );
            }
            other => panic!("expected missing runtime receipt denial, got {other:?}"),
        },
        other => panic!("expected missing runtime receipt denial, got {other:?}"),
    }
}

#[test]
fn scenario_result_rejects_missing_required_counter_family() {
    let scenario = HarnessScenario::define("harness.honesty.missing-counter")
        .expect("valid scenario id")
        .step(
            HarnessScenarioStep::new(
                "observe-frame",
                HarnessScenarioOperation::observe_visible_frame(),
            )
            .requires(HarnessEvidenceRequirement::counter_family()),
        );

    let denial = HarnessRunner::for_app(WorthUi::app().freeze())
        .run(scenario)
        .expect_err("visual observation cannot satisfy counter evidence");

    assert_localized_missing_family(
        denial,
        0,
        "observe-frame",
        HarnessEvidenceFamily::CounterFamily,
    );
}

#[test]
fn scenario_result_rejects_missing_required_command_identity() {
    let scenario = HarnessScenario::define("harness.honesty.missing-command-identity")
        .expect("valid scenario id")
        .step(
            HarnessScenarioStep::new(
                "observe-frame",
                HarnessScenarioOperation::observe_visible_frame(),
            )
            .requires(HarnessEvidenceRequirement::command_identity()),
        );

    let denial = HarnessRunner::for_app(WorthUi::app().freeze())
        .run(scenario)
        .expect_err("visual observation cannot satisfy command identity evidence");

    assert_localized_missing_family(
        denial,
        0,
        "observe-frame",
        HarnessEvidenceFamily::CommandIdentity,
    );
}

#[test]
fn scenario_runtime_launch_does_not_claim_missing_state_receipt_source() {
    let app = WorthUi::app().freeze();
    let launch = worth_ui_harness::workbench::minimal_workbench_launch(&app)
        .expect("minimal harness launch should prepare");
    let scenario = HarnessScenario::define("harness.honesty.launch-missing-state")
        .expect("valid scenario id")
        .step(
            HarnessScenarioStep::new(
                "launch-runtime",
                HarnessScenarioOperation::launch_runtime(launch),
            )
            .requires(HarnessEvidenceRequirement::state_receipt()),
        );

    let denial = HarnessRunner::for_app(app)
        .run(scenario)
        .expect_err("runtime launch must not claim state receipts before a source exists");

    assert_localized_missing_family(
        denial,
        0,
        "launch-runtime",
        HarnessEvidenceFamily::StateReceipt,
    );
}

#[test]
fn scenario_completed_operation_receipt_is_recorded_for_visual_steps() {
    let scenario = HarnessScenario::define("harness.honesty.visual-operation-receipt")
        .expect("valid scenario id")
        .step(
            HarnessScenarioStep::new(
                "observe-frame",
                HarnessScenarioOperation::observe_visible_frame(),
            )
            .requires(HarnessEvidenceRequirement::operation_receipt())
            .requires(HarnessEvidenceRequirement::visible_frame_observation()),
        );

    let receipt = HarnessRunner::for_app(WorthUi::app().freeze())
        .run(scenario)
        .expect("completed visual operation should carry an operation receipt");

    assert!(receipt
        .evidence_ledger()
        .contains_family_at_step(0, HarnessEvidenceFamily::OperationReceipt));
}

#[test]
fn scenario_failure_localizes_to_operation_and_evidence_family() {
    let scenario = HarnessScenario::define("harness.failure.localized")
        .expect("valid scenario id")
        .step(HarnessScenarioStep::new(
            "visual-only",
            HarnessScenarioOperation::observe_visible_frame(),
        ))
        .step(
            HarnessScenarioStep::new(
                "missing-state",
                HarnessScenarioOperation::observe_visible_frame(),
            )
            .requires(HarnessEvidenceRequirement::state_receipt()),
        );

    let denial = HarnessRunner::for_app(WorthUi::app().freeze())
        .run(scenario)
        .expect_err("missing state receipt should localize to the failing step");

    assert_localized_missing_family(
        denial,
        1,
        "missing-state",
        HarnessEvidenceFamily::StateReceipt,
    );
}

fn assert_localized_missing_family(
    denial: HarnessRunDenial,
    expected_step_index: usize,
    expected_step_label: &str,
    expected_family: HarnessEvidenceFamily,
) {
    match denial {
        HarnessRunDenial::Honesty { location, denial } => {
            assert_eq!(location.step_index(), expected_step_index);
            assert_eq!(location.step_label(), expected_step_label);
            assert_eq!(location.evidence_family(), Some(expected_family));
            match denial {
                HarnessHonestyDenial::EvidenceValidation(
                    HarnessEvidenceValidationDenial::MissingRequiredEvidence { family },
                ) => assert_eq!(family, expected_family),
                other => panic!("expected missing family denial, got {other:?}"),
            }
        }
        other => panic!("expected localized missing family denial, got {other:?}"),
    }
}
