use crate::orchestration_inventory::{
    ForgeQueryOrchestrationBindingProjection, ForgeQueryOrchestrationInventoryAudit,
};

use super::support::{
    current_row, inventory_with_replaced_row, inventory_without_public_name,
    row_with_binding_projection, row_with_doc_reference,
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
