use crate::orchestration_inventory::{
    ForgeQueryOrchestrationAspectPosture, ForgeQueryOrchestrationBindingProjection,
    ForgeQueryOrchestrationContributionCompatibility, ForgeQueryOrchestrationInventoryAudit,
    ForgeQueryOrchestrationStrategyAttachment,
};

use super::row_mutation_support::{
    current_row, inventory_with_replaced_row, inventory_without_public_name,
    row_with_aspect_posture, row_with_binding_projection, row_with_contribution_compatibility,
    row_with_doc_reference, row_with_strategy_attachment,
};

#[test]
fn audit_flags_uninventoried_public_verbs_when_inventory_drops_a_real_surface() {
    let inventory = inventory_without_public_name("orchestrate_signal_compatibility");
    let audit = ForgeQueryOrchestrationInventoryAudit::from_inventory(&inventory);

    assert_eq!(
        audit.uninventoried_public_verbs(),
        &["orchestrate_signal_compatibility".to_string()]
    );
}

#[test]
fn audit_flags_uninventoried_public_recovery_helpers() {
    let inventory = inventory_without_public_name("recover_from_outcome");
    let audit = ForgeQueryOrchestrationInventoryAudit::from_inventory(&inventory);

    assert_eq!(
        audit.uninventoried_public_verbs(),
        &["recover_from_outcome".to_string()]
    );
}

#[test]
fn audit_flags_missing_binding_projection_when_continuation_row_lies_about_shared_binding() {
    let row = current_row("prepare_continuation_from_target");
    let inventory = inventory_with_replaced_row(row_with_binding_projection(
        &row,
        ForgeQueryOrchestrationBindingProjection::None,
    ));
    let audit = ForgeQueryOrchestrationInventoryAudit::from_inventory(&inventory);

    assert_eq!(
        audit.missing_binding_projection_rows(),
        &["prepare_continuation_from_target".to_string()]
    );
}

#[test]
fn audit_flags_undocumented_export_when_doc_reference_does_not_resolve() {
    let row = current_row("orchestrate_declaration_with_contributions");
    let inventory = inventory_with_replaced_row(row_with_doc_reference(
        &row,
        "crates/forge-query/docs/domain-capabilities/does-not-exist.md",
        "missing",
    ));
    let audit = ForgeQueryOrchestrationInventoryAudit::from_inventory(&inventory);

    assert_eq!(
        audit.undocumented_exports(),
        &["orchestrate_declaration_with_contributions".to_string()]
    );
}

#[test]
fn audit_flags_missing_aspect_posture_for_signal_orchestration_rows() {
    let row = current_row("orchestrate_signal_compatibility");
    let inventory = inventory_with_replaced_row(row_with_aspect_posture(
        &row,
        ForgeQueryOrchestrationAspectPosture::None,
    ));
    let audit = ForgeQueryOrchestrationInventoryAudit::from_inventory(&inventory);

    assert_eq!(
        audit.semantic_attachment_gaps(),
        &[
            "orchestrate_signal_compatibility:missing aspect posture".to_string(),
            "prepare_preview_for_active_face_selection:helper semantic drift".to_string(),
            "prepare_runtime_route_for_active_face_selection:helper semantic drift".to_string(),
            "prepare_current_truth_view_for_active_face_selection:helper semantic drift"
                .to_string(),
            "prepare_historical_truth_view_for_active_face_selection:helper semantic drift"
                .to_string(),
        ]
    );
}

#[test]
fn audit_flags_missing_strategy_attachment_for_declaration_entry_rows() {
    let row = current_row("orchestrate_declaration_entry");
    let inventory = inventory_with_replaced_row(row_with_strategy_attachment(
        &row,
        ForgeQueryOrchestrationStrategyAttachment::none(),
    ));
    let audit = ForgeQueryOrchestrationInventoryAudit::from_inventory(&inventory);

    assert_eq!(
        audit.semantic_attachment_gaps(),
        &["orchestrate_declaration_entry:missing strategy attachment".to_string()]
    );
}

#[test]
fn audit_flags_missing_strategy_attachment_for_contribution_composed_rows() {
    let row = current_row("orchestrate_declaration_with_contributions");
    let inventory = inventory_with_replaced_row(row_with_strategy_attachment(
        &row,
        ForgeQueryOrchestrationStrategyAttachment::none(),
    ));
    let audit = ForgeQueryOrchestrationInventoryAudit::from_inventory(&inventory);

    assert_eq!(
        audit.semantic_attachment_gaps(),
        &[
            "orchestrate_declaration_with_contributions:missing strategy attachment".to_string(),
            "orchestrate_material_attachment_for_active_face_selection:helper semantic drift"
                .to_string(),
        ]
    );
}

#[test]
fn audit_flags_missing_contribution_compatibility_for_composed_rows() {
    let row = current_row("orchestrate_declaration_with_contributions");
    let inventory = inventory_with_replaced_row(row_with_contribution_compatibility(
        &row,
        ForgeQueryOrchestrationContributionCompatibility::none(),
    ));
    let audit = ForgeQueryOrchestrationInventoryAudit::from_inventory(&inventory);

    assert_eq!(
        audit.semantic_attachment_gaps(),
        &[
            "orchestrate_declaration_with_contributions:missing contribution compatibility"
                .to_string()
        ]
    );
}

#[test]
fn audit_flags_empty_declaration_scoped_contribution_family_sets() {
    let row = current_row("orchestrate_declaration_with_contributions");
    let inventory = inventory_with_replaced_row(row_with_contribution_compatibility(
        &row,
        ForgeQueryOrchestrationContributionCompatibility::declaration_scoped(Vec::new()),
    ));
    let audit = ForgeQueryOrchestrationInventoryAudit::from_inventory(&inventory);

    assert_eq!(
        audit.semantic_attachment_gaps(),
        &[
            "orchestrate_declaration_with_contributions:missing contribution compatibility"
                .to_string()
        ]
    );
}

#[test]
fn audit_flags_helper_semantic_drift_with_exact_gap() {
    let row = current_row("prepare_preview_for_active_face_selection");
    let inventory = inventory_with_replaced_row(row_with_aspect_posture(
        &row,
        ForgeQueryOrchestrationAspectPosture::RequiredContract,
    ));
    let audit = ForgeQueryOrchestrationInventoryAudit::from_inventory(&inventory);

    assert_eq!(
        audit.semantic_attachment_gaps(),
        &["prepare_preview_for_active_face_selection:helper semantic drift".to_string()]
    );
}
