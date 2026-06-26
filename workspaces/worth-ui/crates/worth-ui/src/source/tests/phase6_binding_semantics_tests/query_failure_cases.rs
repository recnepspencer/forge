use super::binding_app_fixture::admitted_app;
use super::binding_phase_fixture::legally_structured_with_binding_entry;
use super::binding_query_fixture::{
    frozen_view_binding_entry_for_descriptor,
    query_owned_view_binding_without_live_compatibility_descriptor,
};
use crate::capability::{ViewBindingDescriptor, ViewBindingFamily, ViewBindingId};
use crate::source::{WorthUiBindingDiagnosticCode, WorthUiBindingSemanticsLowerer};

#[test]
fn local_pseudo_query_claim_rejected_for_binding_node_query_semantics() {
    let app = admitted_app();
    let snapshot = app.capabilities();
    let legally_structured = legally_structured_with_binding_entry(
        snapshot,
        frozen_view_binding_entry_for_descriptor(
            ViewBindingDescriptor::local_pseudo_query_for_diagnostics(
                ViewBindingId::new("workspace.view_binding.selection").unwrap(),
                ViewBindingFamily::collection(),
                "ui_local_runtime_plan",
            ),
        ),
    );

    let report = WorthUiBindingSemanticsLowerer::lower(&legally_structured, snapshot).unwrap_err();
    let diagnostic_codes = report
        .diagnostics()
        .iter()
        .map(|diagnostic| diagnostic.code())
        .collect::<Vec<_>>();

    assert_eq!(
        diagnostic_codes,
        vec![WorthUiBindingDiagnosticCode::LocalPseudoQueryClaimRejected]
    );
    assert_eq!(report.metrics().direct_lookup_count(), 6);
    assert_eq!(report.metrics().families_scanned(), 0);
    assert_eq!(report.metrics().query_owned_semantic_checks(), 2);
}

#[test]
fn missing_query_live_compatibility_localizes_with_exact_code() {
    let app = admitted_app();
    let snapshot = app.capabilities();
    let legally_structured = legally_structured_with_binding_entry(
        snapshot,
        frozen_view_binding_entry_for_descriptor(
            query_owned_view_binding_without_live_compatibility_descriptor(),
        ),
    );

    let report = WorthUiBindingSemanticsLowerer::lower(&legally_structured, snapshot).unwrap_err();
    let diagnostic_codes = report
        .diagnostics()
        .iter()
        .map(|diagnostic| diagnostic.code())
        .collect::<Vec<_>>();

    assert_eq!(
        diagnostic_codes,
        vec![WorthUiBindingDiagnosticCode::MissingQueryLiveCompatibility]
    );
    assert_eq!(report.metrics().direct_lookup_count(), 6);
    assert_eq!(report.metrics().families_scanned(), 0);
    assert_eq!(report.metrics().query_owned_semantic_checks(), 2);
}
