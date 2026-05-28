use crate::application::ForgeQueryDeclarationEntryOrchestrationVerbInventory;
use crate::orchestration_inventory::{
    ForgeQueryOrchestrationInventoryAudit, ForgeQueryOrchestrationSurfaceFamily,
    ForgeQueryOrchestrationSurfaceInventory, ForgeQueryOrchestrationSurfaceVisibility,
};

#[test]
fn declaration_entry_inventory_projects_from_canonical_orchestration_inventory() {
    let declaration_inventory = ForgeQueryDeclarationEntryOrchestrationVerbInventory::current();
    let projected = ForgeQueryOrchestrationSurfaceInventory::current()
        .rows()
        .iter()
        .filter(|row| {
            matches!(
                row.family(),
                ForgeQueryOrchestrationSurfaceFamily::DeclarationEntry
                    | ForgeQueryOrchestrationSurfaceFamily::RouteFromProgressed
                    | ForgeQueryOrchestrationSurfaceFamily::ReceiptFromProgressed
                    | ForgeQueryOrchestrationSurfaceFamily::EnvelopeFromProgressed
            ) && row.visibility() != ForgeQueryOrchestrationSurfaceVisibility::OrdinaryOutcome
        })
        .map(|row| row.public_name())
        .collect::<Vec<_>>();
    let verb_names = declaration_inventory
        .verbs()
        .iter()
        .map(|verb| verb.public_name())
        .collect::<Vec<_>>();

    assert_eq!(projected, verb_names);
}

#[test]
fn current_audit_is_clean() {
    let audit = ForgeQueryOrchestrationInventoryAudit::current();

    assert!(audit.duplicate_public_names().is_empty());
    assert!(audit.uninventoried_public_verbs().is_empty());
    assert!(audit.undocumented_exports().is_empty());
    assert!(audit.missing_doc_rows().is_empty());
    assert!(audit.missing_transcript_rows().is_empty());
    assert!(audit.missing_certification_rows().is_empty());
    assert!(audit.missing_support_rows().is_empty());
    assert!(audit.missing_binding_projection_rows().is_empty());
    assert!(audit.ordinary_projection_mismatches().is_empty());
    assert!(audit.family_visibility_gaps().is_empty());
    assert!(audit.semantic_attachment_gaps().is_empty());
}
