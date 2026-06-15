use worth_ui::facade::WorthUi;
use worth_ui_harness::facade::{
    HarnessEvidenceFamily, HarnessEvidenceRequirement, HarnessEvidenceValidationDenial,
    HarnessHonestyDenial, HarnessRunDenial, HarnessScenario, HarnessScenarioOperation,
    HarnessScenarioStep,
};
use worth_ui_validation_app::honesty::{
    ValidationAppEvidenceGateDenial, ValidationAppHonestyBoundary,
};

#[test]
fn validation_app_rejects_app_local_shell_state_injection() {
    let scenario = HarnessScenario::define("validation.honesty.injected-state")
        .expect("valid scenario id")
        .step(
            HarnessScenarioStep::new(
                "inject-state",
                HarnessScenarioOperation::attempt_app_local_shell_state_injection(),
            )
            .requires(HarnessEvidenceRequirement::runtime_receipt()),
        );

    let denial = ValidationAppHonestyBoundary::runner_for_public_app(WorthUi::app().freeze())
        .run(scenario)
        .expect_err("app-local shell state injection must fail before scenario execution");

    match denial {
        HarnessRunDenial::Honesty { location, denial } => {
            assert_eq!(location.step_index(), 0);
            assert_eq!(location.step_label(), "inject-state");
            assert_eq!(denial, HarnessHonestyDenial::AppLocalShellStateInjection);
        }
        other => panic!("expected app-local shell state honesty denial, got {other:?}"),
    }
}

#[test]
fn validation_app_result_requires_runtime_receipts_not_visual_success() {
    let scenario = HarnessScenario::define("validation.honesty.visual-only")
        .expect("valid scenario id")
        .step(
            HarnessScenarioStep::new(
                "observe-frame",
                HarnessScenarioOperation::observe_visible_frame(),
            )
            .requires(HarnessEvidenceRequirement::runtime_receipt()),
        );

    let denial = ValidationAppHonestyBoundary::runner_for_public_app(WorthUi::app().freeze())
        .run(scenario)
        .expect_err("visual-only evidence must not certify validation app success");

    assert_missing_required_family(
        denial,
        0,
        "observe-frame",
        HarnessEvidenceFamily::RuntimeReceipt,
    );
}

#[test]
fn validation_app_previous_step_receipt_cannot_certify_later_visual_step() {
    let app = WorthUi::app().freeze();
    let launch = ValidationAppHonestyBoundary::prepare_public_facade_launch(&app)
        .expect("validation app launch should prepare through the public facade");
    let scenario = HarnessScenario::define("validation.honesty.stale-step-receipt")
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

    let denial = ValidationAppHonestyBoundary::runner_for_public_app(app)
        .run(scenario)
        .expect_err("earlier runtime receipt must not certify a later visual step");

    assert_missing_required_family(
        denial,
        1,
        "observe-frame",
        HarnessEvidenceFamily::RuntimeReceipt,
    );
}

#[test]
fn validation_app_evidence_gate_rejects_visual_only_receipts() {
    let scenario = HarnessScenario::define("validation.honesty.visual-receipt")
        .expect("valid scenario id")
        .step(
            HarnessScenarioStep::new(
                "observe-frame",
                HarnessScenarioOperation::observe_visible_frame(),
            )
            .requires(HarnessEvidenceRequirement::visible_frame_observation()),
        );

    let receipt = ValidationAppHonestyBoundary::runner_for_public_app(WorthUi::app().freeze())
        .run(scenario)
        .expect("visual observation can produce a harness receipt");
    let denial = ValidationAppHonestyBoundary::require_runtime_backed_receipt(&receipt)
        .expect_err("validation app completion requires runtime-backed evidence");

    assert_eq!(
        denial,
        ValidationAppEvidenceGateDenial::MissingRuntimeReceipt
    );
}

#[test]
fn validation_app_evidence_gate_rejects_stale_runtime_setup_evidence() {
    let app = WorthUi::app().freeze();
    let launch = ValidationAppHonestyBoundary::prepare_public_facade_launch(&app)
        .expect("validation app launch should prepare through the public facade");
    let scenario = HarnessScenario::define("validation.honesty.stale-aggregate-evidence")
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
            .requires(HarnessEvidenceRequirement::visible_frame_observation()),
        );

    let receipt = ValidationAppHonestyBoundary::runner_for_public_app(app)
        .run(scenario)
        .expect("aggregate receipt can contain runtime evidence from setup");
    assert!(receipt
        .evidence()
        .contains(HarnessEvidenceFamily::RuntimeReceipt));
    assert!(receipt
        .evidence()
        .contains(HarnessEvidenceFamily::VisibleFrameObservation));

    let denial = ValidationAppHonestyBoundary::require_runtime_backed_receipt(&receipt)
        .expect_err("validation app completion must not borrow stale setup evidence");

    assert_eq!(
        denial,
        ValidationAppEvidenceGateDenial::MissingRuntimeReceipt
    );
}

fn assert_missing_required_family(
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
                other => panic!("expected missing required evidence, got {other:?}"),
            }
        }
        other => panic!("expected localized evidence denial, got {other:?}"),
    }
}
