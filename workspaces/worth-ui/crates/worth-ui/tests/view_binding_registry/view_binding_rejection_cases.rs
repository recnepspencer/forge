use forge_query::facade::ViewShapeDescriptor;
use worth_ui::facade::{
    CapabilityDiagnosticCode, QueryDenialPresentation, QueryResultShapeReference,
    ViewBindingDescriptor, ViewBindingFamily, WorthUi,
};

use super::view_binding_assertions::{assert_diagnostic_codes, assert_registered_view_binding_ids};
use super::view_binding_fixtures::{
    admitted_basis_posture, deferred_basis_view_binding, denied_query_live_compatibility,
    detail_view_binding, pseudo_query_view_binding, query_live_compatibility, table_view_binding,
    unsupported_query_capability_binding, view_binding_id, with_query_support_and_composition,
};

#[test]
fn view_binding_without_query_support_posture_rejected() {
    let report = WorthUi::app()
        .register_view_binding(
            ViewBindingDescriptor::query_owned(
                view_binding_id("workspace.view_binding.missing_query_support"),
                ViewBindingFamily::collection(),
            )
            .with_view_shape(ViewShapeDescriptor::table())
            .with_result_shape(QueryResultShapeReference::from_result_shape_family(
                forge_query::facade::ResultShapeFamily::Collection,
            ))
            .with_live_compatibility(query_live_compatibility())
            .with_denial_presentation(QueryDenialPresentation::structured_status()),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert!(report.accepted_snapshot().view_bindings().is_empty());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[
            CapabilityDiagnosticCode::MissingViewBindingQuerySupportPosture,
            CapabilityDiagnosticCode::MissingViewBindingViewShape,
            CapabilityDiagnosticCode::MissingViewBindingBasisPosture,
        ],
    );
}

#[test]
fn unsupported_query_support_posture_rejected() {
    let report = WorthUi::app()
        .register_view_binding(unsupported_query_capability_binding(
            "workspace.view_binding.unsupported_query",
        ))
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[
            CapabilityDiagnosticCode::UnsupportedViewBindingQuerySupportPosture,
            CapabilityDiagnosticCode::MissingViewBindingViewShape,
            CapabilityDiagnosticCode::MissingViewBindingLiveCompatibility,
        ],
    );
}

#[test]
fn view_binding_without_basis_posture_rejected() {
    let report = WorthUi::app()
        .register_view_binding(with_query_support_and_composition(
            ViewBindingDescriptor::query_owned(
                view_binding_id("workspace.view_binding.missing_basis"),
                ViewBindingFamily::collection(),
            )
            .with_view_shape(ViewShapeDescriptor::table())
            .with_result_shape(QueryResultShapeReference::from_result_shape_family(
                forge_query::facade::ResultShapeFamily::Collection,
            ))
            .with_live_compatibility(query_live_compatibility())
            .with_denial_presentation(QueryDenialPresentation::structured_status()),
        ))
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[CapabilityDiagnosticCode::MissingViewBindingBasisPosture],
    );
}

#[test]
fn unsupported_basis_posture_rejected() {
    let report = WorthUi::app()
        .register_view_binding(deferred_basis_view_binding(
            "workspace.view_binding.deferred_basis",
        ))
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[CapabilityDiagnosticCode::UnsupportedViewBindingBasisPosture],
    );
}

#[test]
fn view_binding_without_result_shape_rejected() {
    let report = WorthUi::app()
        .register_view_binding(with_query_support_and_composition(
            ViewBindingDescriptor::query_owned(
                view_binding_id("workspace.view_binding.no_result_shape"),
                ViewBindingFamily::collection(),
            )
            .with_view_shape(ViewShapeDescriptor::table())
            .with_basis_posture(admitted_basis_posture())
            .with_live_compatibility(query_live_compatibility())
            .with_denial_presentation(QueryDenialPresentation::structured_status()),
        ))
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[CapabilityDiagnosticCode::MissingViewBindingResultShape],
    );
}

#[test]
fn view_binding_without_live_compatibility_rejected() {
    let report = WorthUi::app()
        .register_view_binding(
            with_query_support_and_composition(
                ViewBindingDescriptor::query_owned(
                    view_binding_id("workspace.view_binding.no_live_compatibility"),
                    ViewBindingFamily::collection(),
                )
                .with_view_shape(ViewShapeDescriptor::table())
                .with_result_shape(QueryResultShapeReference::from_result_shape_family(
                    forge_query::facade::ResultShapeFamily::Collection,
                ))
                .with_basis_posture(admitted_basis_posture()),
            )
            .with_denial_presentation(QueryDenialPresentation::structured_status()),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[CapabilityDiagnosticCode::MissingViewBindingLiveCompatibility],
    );
}

#[test]
fn unsupported_live_compatibility_rejected() {
    let report = WorthUi::app()
        .register_view_binding(
            table_view_binding("workspace.view_binding.denied_live_compatibility")
                .with_live_compatibility(denied_query_live_compatibility()),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[CapabilityDiagnosticCode::UnsupportedViewBindingLiveCompatibility],
    );
}

#[test]
fn duplicate_view_binding_id_rejected_before_snapshot_freeze() {
    let report = WorthUi::app()
        .register_view_binding(table_view_binding("workspace.view_binding.duplicate"))
        .register_view_binding(detail_view_binding("workspace.view_binding.duplicate"))
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert!(report.accepted_snapshot().view_bindings().is_empty());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[
            CapabilityDiagnosticCode::DuplicateCapabilityId,
            CapabilityDiagnosticCode::DuplicateCapabilityId,
        ],
    );
}

#[test]
fn local_pseudo_query_binding_rejected() {
    let report = WorthUi::app()
        .register_view_binding(pseudo_query_view_binding(
            "workspace.view_binding.pseudo_query",
        ))
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[
            CapabilityDiagnosticCode::MissingViewBindingQuerySupportPosture,
            CapabilityDiagnosticCode::MissingViewBindingViewShape,
            CapabilityDiagnosticCode::MissingViewBindingBasisPosture,
            CapabilityDiagnosticCode::MissingViewBindingResultShape,
            CapabilityDiagnosticCode::MissingViewBindingLiveCompatibility,
            CapabilityDiagnosticCode::MissingViewBindingDenialPresentation,
            CapabilityDiagnosticCode::LocalPseudoQueryViewBinding,
        ],
    );
}

#[test]
fn rejected_view_binding_does_not_poison_valid_view_binding() {
    let report = WorthUi::app()
        .register_view_binding(pseudo_query_view_binding(
            "workspace.view_binding.pseudo_query",
        ))
        .register_view_binding(table_view_binding("workspace.view_binding.tasks"))
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert_registered_view_binding_ids(
        report.accepted_snapshot().view_bindings(),
        &["workspace.view_binding.tasks"],
    );
}
