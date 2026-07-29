use worth_ui::facade::{
    app::WorthUi,
    declaration::{
        MosaicStateOwnerIdentity, MosaicStatePersistencePolicy, MosaicStateReplacementRule,
        MosaicStateTruthPosture,
    },
    diagnostics::CapabilityDiagnosticCode,
};

use super::state_assertions::assert_diagnostic_codes;
use super::state_fixtures::{complete_state_slot, splitter_position_slot};

#[test]
fn state_slot_without_owner_identity_rejected() {
    let report = WorthUi::app()
        .with_change_profile(worth_ui_runtime::facade::rebind::UiChangeProfile::platform_pulse())
        .register_mosaic_state_slot(
            splitter_position_slot("workspace.state.no_owner")
                .with_owner_identity(MosaicStateOwnerIdentity::missing_for_diagnostics()),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert!(report.accepted_snapshot().mosaic_state_slots().is_empty());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[CapabilityDiagnosticCode::MissingMosaicStateSlotOwnerIdentity],
    );
}

#[test]
fn state_slot_without_persistence_posture_rejected() {
    let report = WorthUi::app()
        .with_change_profile(worth_ui_runtime::facade::rebind::UiChangeProfile::platform_pulse())
        .register_mosaic_state_slot(
            splitter_position_slot("workspace.state.no_persistence")
                .with_persistence_policy(MosaicStatePersistencePolicy::missing_for_diagnostics()),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert!(report.accepted_snapshot().mosaic_state_slots().is_empty());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[CapabilityDiagnosticCode::MissingMosaicStateSlotPersistencePolicy],
    );
}

#[test]
fn state_slot_without_replacement_rules_rejected() {
    let report = WorthUi::app()
        .with_change_profile(worth_ui_runtime::facade::rebind::UiChangeProfile::platform_pulse())
        .register_mosaic_state_slot(
            splitter_position_slot("workspace.state.no_replacement")
                .with_replacement_rule(MosaicStateReplacementRule::missing_for_diagnostics()),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert!(report.accepted_snapshot().mosaic_state_slots().is_empty());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[CapabilityDiagnosticCode::MissingMosaicStateSlotReplacementRule],
    );
}

#[test]
fn ui_state_slot_cannot_claim_authoritative_truth() {
    assert_authoritative_truth_rejected(
        MosaicStateTruthPosture::authoritative_query_truth_for_diagnostics(),
    );
    assert_authoritative_truth_rejected(
        MosaicStateTruthPosture::authoritative_relational_truth_for_diagnostics(),
    );
}

#[test]
fn derived_runtime_state_is_admitted_without_authority_claim() {
    let app = WorthUi::app()
        .with_change_profile(worth_ui_runtime::facade::rebind::UiChangeProfile::platform_pulse())
        .register_mosaic_state_slot(
            complete_state_slot(
                "workspace.state.derived_focus",
                worth_ui::facade::declaration::MosaicStateSlotKind::focused_region(),
            )
            .with_truth_posture(
                MosaicStateTruthPosture::derived_from_authoritative_runtime_truth(),
            ),
        )
        .freeze()
        .expect("application preparation should succeed");

    assert_eq!(app.capabilities().mosaic_state_slots().len(), 1);
}

#[test]
fn state_slot_reports_every_missing_required_posture() {
    let report = WorthUi::app()
        .with_change_profile(worth_ui_runtime::facade::rebind::UiChangeProfile::platform_pulse())
        .register_mosaic_state_slot(
            splitter_position_slot("workspace.state.missing")
                .with_owner_identity(MosaicStateOwnerIdentity::missing_for_diagnostics())
                .with_persistence_policy(MosaicStatePersistencePolicy::missing_for_diagnostics())
                .with_replacement_rule(MosaicStateReplacementRule::missing_for_diagnostics())
                .with_truth_posture(MosaicStateTruthPosture::missing_for_diagnostics()),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert!(report.accepted_snapshot().mosaic_state_slots().is_empty());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[
            CapabilityDiagnosticCode::MissingMosaicStateSlotOwnerIdentity,
            CapabilityDiagnosticCode::MissingMosaicStateSlotPersistencePolicy,
            CapabilityDiagnosticCode::MissingMosaicStateSlotReplacementRule,
            CapabilityDiagnosticCode::MissingMosaicStateSlotTruthPosture,
        ],
    );
}

#[test]
fn rejected_state_slot_does_not_poison_valid_state_slot() {
    let report = WorthUi::app()
        .with_change_profile(worth_ui_runtime::facade::rebind::UiChangeProfile::platform_pulse())
        .register_mosaic_state_slot(
            splitter_position_slot("workspace.state.invalid").with_truth_posture(
                MosaicStateTruthPosture::authoritative_query_truth_for_diagnostics(),
            ),
        )
        .register_mosaic_state_slot(splitter_position_slot("workspace.state.valid"))
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert_eq!(report.accepted_snapshot().mosaic_state_slots().len(), 1);
    assert!(report
        .accepted_snapshot()
        .mosaic_state_slots()
        .get(&super::state_fixtures::state_slot_id(
            "workspace.state.valid"
        ))
        .is_some());
}

fn assert_authoritative_truth_rejected(posture: MosaicStateTruthPosture) {
    let report = WorthUi::app()
        .with_change_profile(worth_ui_runtime::facade::rebind::UiChangeProfile::platform_pulse())
        .register_mosaic_state_slot(
            complete_state_slot(
                "workspace.state.authoritative_truth",
                worth_ui::facade::declaration::MosaicStateSlotKind::selection_token(),
            )
            .with_truth_posture(posture),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert!(report.accepted_snapshot().mosaic_state_slots().is_empty());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[CapabilityDiagnosticCode::AuthoritativeTruthMosaicStateSlot],
    );
}
