use worth_ui::facade::{
    app::WorthUi,
    diagnostics::CapabilityDiagnosticCode,
    registry::{
        RuntimeOutcomeDenialPosture, RuntimeOutcomeFamily, RuntimeOutcomeProjectionDescriptor,
        RuntimeOutcomeRecoveryPosture,
    },
};

use super::runtime_outcome_projection_assertions::{
    assert_diagnostic_codes, assert_registered_runtime_outcome_projection_ids,
};
use super::runtime_outcome_projection_fixtures::{
    denied_projection, denied_source_reference, failed_projection, projection_id,
};

#[test]
fn unknown_runtime_outcome_family_rejected() {
    let report = WorthUi::app()
        .register_runtime_outcome_projection(RuntimeOutcomeProjectionDescriptor::new(
            projection_id("workspace.outcome.unknown"),
            RuntimeOutcomeFamily::unknown_for_diagnostics("spinner"),
            denied_source_reference(),
        ))
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert!(report
        .accepted_snapshot()
        .runtime_outcome_projections()
        .is_empty());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[CapabilityDiagnosticCode::UnknownRuntimeOutcomeFamily],
    );
}

#[test]
fn local_status_enum_projection_rejected() {
    let report = WorthUi::app()
        .register_runtime_outcome_projection(
            RuntimeOutcomeProjectionDescriptor::local_status_enum_for_diagnostics(
                projection_id("workspace.outcome.local_status"),
                "Loading | Success | Error",
            ),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[
            CapabilityDiagnosticCode::UnknownRuntimeOutcomeFamily,
            CapabilityDiagnosticCode::MissingRuntimeOutcomeSource,
            CapabilityDiagnosticCode::LocalStatusEnumRuntimeOutcomeProjection,
        ],
    );
}

#[test]
fn outcome_projection_missing_denial_posture_rejected() {
    let report = WorthUi::app()
        .register_runtime_outcome_projection(RuntimeOutcomeProjectionDescriptor::new(
            projection_id("workspace.outcome.denied_without_posture"),
            RuntimeOutcomeFamily::denied(),
            denied_source_reference(),
        ))
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[CapabilityDiagnosticCode::MissingRuntimeOutcomeDenialPosture],
    );
}

#[test]
fn outcome_projection_missing_recovery_posture_rejected() {
    let report = WorthUi::app()
        .register_runtime_outcome_projection(RuntimeOutcomeProjectionDescriptor::new(
            projection_id("workspace.outcome.failed_without_recovery"),
            RuntimeOutcomeFamily::failed(),
            super::runtime_outcome_projection_fixtures::failed_source_reference(),
        ))
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[CapabilityDiagnosticCode::MissingRuntimeOutcomeRecoveryPosture],
    );
}

#[test]
fn duplicate_runtime_outcome_projection_id_rejected_before_snapshot_freeze() {
    let report = WorthUi::app()
        .register_runtime_outcome_projection(denied_projection("workspace.outcome.duplicate"))
        .register_runtime_outcome_projection(failed_projection("workspace.outcome.duplicate"))
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert!(report
        .accepted_snapshot()
        .runtime_outcome_projections()
        .is_empty());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[
            CapabilityDiagnosticCode::DuplicateCapabilityId,
            CapabilityDiagnosticCode::DuplicateCapabilityId,
        ],
    );
}

#[test]
fn runtime_outcome_family_cannot_relabel_query_source_meaning() {
    let report = WorthUi::app()
        .register_runtime_outcome_projection(RuntimeOutcomeProjectionDescriptor::new(
            projection_id("workspace.outcome.denied_as_ready"),
            RuntimeOutcomeFamily::ready(),
            denied_source_reference(),
        ))
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert!(report
        .accepted_snapshot()
        .runtime_outcome_projections()
        .is_empty());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[CapabilityDiagnosticCode::RuntimeOutcomeFamilySourceMismatch],
    );
}

#[test]
fn unexpected_denial_posture_rejected_for_non_denial_family() {
    let report = WorthUi::app()
        .register_runtime_outcome_projection(
            super::runtime_outcome_projection_fixtures::ready_projection(
                "workspace.outcome.ready_with_denial",
            )
            .with_denial_posture(RuntimeOutcomeDenialPosture::structured_status()),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[CapabilityDiagnosticCode::UnexpectedRuntimeOutcomeDenialPosture],
    );
}

#[test]
fn unexpected_recovery_posture_rejected_for_non_recovery_family() {
    let report = WorthUi::app()
        .register_runtime_outcome_projection(
            denied_projection("workspace.outcome.denied_with_recovery")
                .with_recovery_posture(RuntimeOutcomeRecoveryPosture::retry_hint()),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[CapabilityDiagnosticCode::UnexpectedRuntimeOutcomeRecoveryPosture],
    );
}

#[test]
fn mismatched_ui_outcome_source_cannot_be_projected_as_failed() {
    let report = WorthUi::app()
        .register_runtime_outcome_projection(
            RuntimeOutcomeProjectionDescriptor::new(
                projection_id("workspace.outcome.unsupported_as_failed"),
                RuntimeOutcomeFamily::failed(),
                worth_ui::facade::registry::RuntimeOutcomeSourceReference::new(
                    RuntimeOutcomeFamily::recoverable(),
                ),
            )
            .with_recovery_posture(RuntimeOutcomeRecoveryPosture::retry_hint()),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[CapabilityDiagnosticCode::RuntimeOutcomeFamilySourceMismatch],
    );
}

#[test]
fn rejected_runtime_outcome_projection_does_not_poison_valid_projection() {
    let report = WorthUi::app()
        .register_runtime_outcome_projection(
            RuntimeOutcomeProjectionDescriptor::local_status_enum_for_diagnostics(
                projection_id("workspace.outcome.local_status"),
                "Loading | Success | Error",
            ),
        )
        .register_runtime_outcome_projection(denied_projection("workspace.outcome.denied"))
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert_registered_runtime_outcome_projection_ids(
        report.accepted_snapshot().runtime_outcome_projections(),
        &["workspace.outcome.denied"],
    );
}
