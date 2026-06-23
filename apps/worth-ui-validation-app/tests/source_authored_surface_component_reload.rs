mod validation_app_reload_fixture;

use worth_ui::facade::{
    WorthUiPageHostRebindStatus, WorthUiProjectionRebindStatus, WorthUiRuntimeFactId,
    WorthUiSemanticSliceId,
};
use worth_ui_validation_app::reload::{
    ValidationAuthoredReloadEdit, ValidationReloadEvidenceEntry, ValidationReloadStatus,
};

use validation_app_reload_fixture::ValidationAppReloadFixture;

#[test]
fn startup_uses_source_authored_surface_component_selection() {
    let fixture = ValidationAppReloadFixture::new();
    fixture.write_source(&source_with_alt_component());

    let app = fixture.build_app();
    let proof = app.proof_snapshot();

    assert_eq!(
        proof.product_summary().slots()[0].surface_id(),
        "worth.surface.preview.primitive.proof"
    );
    assert_eq!(
        proof.page_slot_interaction().slots()[0].component_id(),
        "worth.component.button"
    );
}

#[test]
fn source_surface_component_edit_reloads_same_surface_id_through_authored_truth() {
    let fixture = ValidationAppReloadFixture::new();
    let mut app = fixture.build_app();

    app.apply_authored_reload_edit(ValidationAuthoredReloadEdit::repoint_surface_component(
        "worth.surface.preview.primitive.proof",
        "worth.component.button",
    ))
    .expect("registered authored component repoint should be expressible semantically");

    let proof = app.proof_snapshot();
    let (latest_rebind, authored_structural, changed_facts) = match proof.latest_evidence() {
        Some(ValidationReloadEvidenceEntry::RuntimeReload {
            status,
            changed_facts,
            authored_structural,
            page_host_rebind,
            ..
        }) => {
            assert_eq!(*status, ValidationReloadStatus::Activated);
            (
                page_host_rebind
                    .as_ref()
                    .expect("source edit should still capture page-host rebind proof"),
                authored_structural
                    .as_ref()
                    .expect("source structural edit should preserve authored structural proof"),
                changed_facts,
            )
        }
        other => panic!("expected runtime reload evidence, got {other:?}"),
    };

    assert_eq!(
        changed_facts,
        &vec![WorthUiRuntimeFactId::authored_mount_component_selection(
            "worth.surface.preview.primitive.proof",
        )]
    );
    assert_eq!(
        proof.product_summary().slots()[0].surface_id(),
        "worth.surface.preview.primitive.proof"
    );
    assert_eq!(
        proof.page_slot_interaction().slots()[0].component_id(),
        "worth.component.button"
    );
    assert_eq!(
        proof.page_slot_interaction().previous_slots()[0].component_id(),
        "worth.component.primitive_proof"
    );
    assert!(authored_structural.rows().iter().any(|row| {
        row.slice_id() == WorthUiSemanticSliceId::AuthoredMountComponentSelection
            && row.subject_label() == "surface:worth.surface.preview.primitive.proof"
    }));
    assert!(proof
        .page_slot_interaction()
        .authored_structural_rows()
        .iter()
        .any(|row| {
            row.slice_id() == WorthUiSemanticSliceId::AuthoredMountComponentSelection
                && row.subject_label() == "surface:worth.surface.preview.primitive.proof"
        }));
    assert!(proof
        .visible_evidence_panel()
        .entries()
        .first()
        .expect("source structural edit should surface visible reload evidence")
        .structural_evidence()
        .is_some());
    let structural_evidence = proof
        .visible_evidence_panel()
        .entries()
        .first()
        .and_then(|entry| entry.structural_evidence())
        .expect("source structural edit should project typed structural visible evidence");
    assert!(structural_evidence
        .header_projection_rows()
        .iter()
        .all(|row| row.status() != WorthUiProjectionRebindStatus::ReboundAfterActivation));
    assert_eq!(structural_evidence.authored_structural_rows().len(), 1);
    assert_eq!(
        structural_evidence.authored_structural_rows()[0].slice_id(),
        WorthUiSemanticSliceId::AuthoredMountComponentSelection
    );
    assert_eq!(
        structural_evidence.authored_structural_rows()[0].subject_label(),
        "surface:worth.surface.preview.primitive.proof"
    );
    assert_eq!(structural_evidence.page_host_projection_rows().len(), 1);
    assert_eq!(
        structural_evidence.page_host_projection_rows()[0].projection_identity(),
        "worth-ui.page-host.HeaderProofPage"
    );
    assert_eq!(latest_rebind.rows().len(), 1);
    assert_eq!(
        latest_rebind.rows()[0].projection_identity(),
        "worth-ui.page-host.HeaderProofPage"
    );
    assert!(matches!(
        latest_rebind.status(),
        WorthUiPageHostRebindStatus::EquivalentAfterActivation
            | WorthUiPageHostRebindStatus::ReboundAfterActivation
    ));
}

#[test]
fn unsupported_surface_component_edit_preserves_prior_valid_truth_with_denial_evidence() {
    let fixture = ValidationAppReloadFixture::new();
    let mut app = fixture.build_app();

    app.apply_authored_reload_edit(ValidationAuthoredReloadEdit::repoint_surface_component(
        "worth.surface.preview.primitive.proof",
        "validation.component.header.unknown",
    ))
    .expect("unsupported structural repoints should still flow through the source boundary");

    let proof = app.proof_snapshot();
    let (status, latest_rebind, authored_structural) = match proof.latest_evidence() {
        Some(ValidationReloadEvidenceEntry::RuntimeReload {
            status,
            page_host_rebind,
            authored_structural,
            ..
        }) => (
            *status,
            page_host_rebind
                .as_ref()
                .expect("denied structural edit should still preserve page-host proof"),
            authored_structural,
        ),
        other => panic!("expected runtime reload evidence, got {other:?}"),
    };

    assert!(matches!(status, ValidationReloadStatus::Denied(_)));
    assert!(authored_structural.is_none());
    assert_eq!(
        proof.page_slot_interaction().slots()[0].component_id(),
        "worth.component.primitive_proof"
    );
    assert_eq!(
        latest_rebind.status(),
        WorthUiPageHostRebindStatus::PreservedDeniedReload
    );
    assert_eq!(latest_rebind.rebuild_attempt_count(), 0);
    let structural_evidence = proof
        .visible_evidence_panel()
        .entries()
        .first()
        .and_then(|entry| entry.structural_evidence())
        .expect("denied structural edit should still project typed structural visible evidence");
    assert!(structural_evidence.authored_structural_rows().is_empty());
    assert_eq!(structural_evidence.page_host_projection_rows().len(), 1);
    assert_eq!(
        structural_evidence.page_host_projection_rows()[0].projection_identity(),
        "worth-ui.page-host.HeaderProofPage"
    );
}

fn source_with_alt_component() -> String {
    worth_ui_validation_app::sample_source::VALIDATION_SAMPLE_SOURCE.replace(
        "surface worth.surface.preview.primitive.proof {\n    component worth.component.primitive_proof",
        "surface worth.surface.preview.primitive.proof {\n    component worth.component.button",
    )
}
