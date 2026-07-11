use forge_store_layout_indexes::layout_closeout::{
    LegacyAccessPathBypassInventory, LegacySurfaceDisposition,
    LegacySurfaceDispositionAndDedicatedWorkspaceBoundary, LegacySurfaceOwner, LegacySurfaceStage,
};

#[test]
fn phase29_inventory_assigns_explicit_disposition_to_every_named_legacy_surface() {
    let boundary = LegacySurfaceDispositionAndDedicatedWorkspaceBoundary::current();
    let inventory: LegacyAccessPathBypassInventory = boundary.inventory();

    assert_eq!(boundary.dedicated_workspace_crate(), "forge-store");
    assert_eq!(
        boundary.dedicated_workspace_facade(),
        "forge_store::layout_boundary"
    );
    assert!(!boundary.legacy_topology_is_precedent());
    assert_eq!(inventory.rows().len(), 88);

    let rows = inventory.rows();
    assert!(rows.iter().all(|row| !row.surface().is_empty()));
    assert!(rows.iter().any(|row| {
        row.surface() == "ForgeStoreBuilder"
            && row.disposition() == LegacySurfaceDisposition::SupersededAndForbidden
            && row.stage() == LegacySurfaceStage::DeclarationFacade
    }));
    assert!(rows.iter().any(|row| {
        row.surface() == "ForgeStore::plan_aspect_layout_read"
            && row.disposition() == LegacySurfaceDisposition::SupersededAndForbidden
            && row.stage() == LegacySurfaceStage::SelectionFacade
    }));
    assert!(rows.iter().any(|row| {
        row.surface() == "ForgeStore::admit_structural_block_reuse"
            && row.disposition() == LegacySurfaceDisposition::SupersededAndForbidden
            && row.stage() == LegacySurfaceStage::AdmissionFacade
    }));
    assert!(rows.iter().any(|row| {
        row.surface() == "ForgeStore::prepare_milestone_6_layout_support"
            && row.disposition() == LegacySurfaceDisposition::SupersededAndForbidden
            && row.stage() == LegacySurfaceStage::ReadinessFacade
    }));
    assert!(rows.iter().any(|row| {
        row.surface() == "AspectLayoutReadRequest"
            && row.disposition() == LegacySurfaceDisposition::ForbiddenAsAuthority
            && row.owner() == LegacySurfaceOwner::LegacyRootCrate
    }));
    assert!(rows.iter().any(|row| {
        row.surface() == "AspectLayoutReadPlanDecision"
            && row.disposition() == LegacySurfaceDisposition::ForbiddenAsAuthority
            && row.owner() == LegacySurfaceOwner::LegacyRootCrate
    }));
    assert!(rows.iter().any(|row| {
        row.surface() == "AspectLayoutReadExecutionResult"
            && row.disposition() == LegacySurfaceDisposition::ForbiddenAsAuthority
            && row.owner() == LegacySurfaceOwner::LegacyRootCrate
    }));
    assert!(rows.iter().any(|row| {
        row.surface() == "Milestone6ChunkModelExport"
            && row.disposition() == LegacySurfaceDisposition::TerminalOnly
    }));
    assert!(rows.iter().any(|row| {
        row.surface() == "CompatibilityRegistry"
            && row.disposition() == LegacySurfaceDisposition::ConsumedAsInputOnly
    }));
    assert!(rows.iter().any(|row| {
        row.surface() == "CompatibilityRegistrySnapshot"
            && row.disposition() == LegacySurfaceDisposition::ConsumedAsInputOnly
    }));
    assert!(rows.iter().any(|row| {
        row.surface() == "RebuildMaintenanceDeclaration"
            && row.disposition() == LegacySurfaceDisposition::ConsumedAsInputOnly
    }));
    assert!(rows.iter().any(|row| {
        row.surface() == "MaintenanceDeclarationClass"
            && row.disposition() == LegacySurfaceDisposition::ConsumedAsInputOnly
    }));
    assert!(rows.iter().any(|row| {
        row.surface() == "MaintenanceDeclarationId"
            && row.disposition() == LegacySurfaceDisposition::ConsumedAsInputOnly
    }));
    assert!(rows.iter().any(|row| {
        row.surface() == "SubscriptionSupportAccessStructureReport"
            && row.disposition() == LegacySurfaceDisposition::ConsumedAsInputOnly
    }));
    assert!(rows.iter().any(|row| {
        row.surface() == "SupportTrustAccessIndexKind"
            && row.disposition() == LegacySurfaceDisposition::ConsumedAsInputOnly
    }));
    assert!(rows.iter().any(|row| {
        row.surface() == "SupportTrustAccessStructurePlan"
            && row.disposition() == LegacySurfaceDisposition::ConsumedAsInputOnly
    }));
    assert!(rows.iter().any(|row| {
        row.surface() == "Milestone7IndependentReference"
            && row.disposition() == LegacySurfaceDisposition::ForbiddenAsAuthority
    }));
    assert!(rows.iter().any(|row| {
        row.surface() == "BranchDeltaFallbackClass"
            && row.disposition() == LegacySurfaceDisposition::SupersededAndForbidden
    }));
    assert!(rows.iter().any(|row| {
        row.surface() == "SupportTrustAccessPath"
            && row.disposition() == LegacySurfaceDisposition::ForbiddenAsAuthority
    }));
    assert!(rows.iter().any(|row| {
        row.surface() == "Milestone6AccessStructureClaim"
            && row.disposition() == LegacySurfaceDisposition::CertificationOnly
            && row.owner() == LegacySurfaceOwner::CertificationLane
    }));
    assert!(rows.iter().any(|row| {
        row.surface() == "Milestone6LayoutMaterializationReport"
            && row.disposition() == LegacySurfaceDisposition::CertificationOnly
            && row.owner() == LegacySurfaceOwner::CertificationLane
    }));
}

#[test]
fn phase29_boundary_forbids_displaced_authority_and_leaves_only_the_named_workspace_lane() {
    let boundary = LegacySurfaceDispositionAndDedicatedWorkspaceBoundary::current();
    let inventory = boundary.inventory();

    assert!(boundary.forbids_legacy_authority("ForgeStore"));
    assert!(boundary.forbids_legacy_authority("ForgeStore::execute_aspect_layout_read"));
    assert!(boundary.forbids_legacy_authority(
        "ForgeStore::rebuild_milestone_6_derived_artifacts_from_authority"
    ));
    assert!(boundary.forbids_legacy_authority("AspectLayoutReadRequest"));
    assert!(boundary.forbids_legacy_authority("ExplicitBroadFallbackPlan"));
    assert!(boundary.forbids_legacy_authority("BranchDeltaFallbackClass"));
    assert!(boundary.forbids_legacy_authority("Milestone7IndependentReference"));
    assert!(boundary.forbids_legacy_authority("AspectLayoutReadExecutionResult"));
    assert!(boundary.forbids_legacy_authority("SupportTrustAccessPath"));
    assert!(boundary.forbids_legacy_authority("Milestone6LayoutMaterializationReport"));
    assert_eq!(
        inventory.disposition_for("MaintenanceDeclarationClass"),
        LegacySurfaceDisposition::ConsumedAsInputOnly
    );
    assert_eq!(
        inventory.disposition_for("MaintenanceDeclarationId"),
        LegacySurfaceDisposition::ConsumedAsInputOnly
    );
    assert_eq!(
        inventory.disposition_for("RebuildMaintenanceDeclaration"),
        LegacySurfaceDisposition::ConsumedAsInputOnly
    );
    assert_eq!(
        inventory.disposition_for("SubscriptionSupportAccessStructureReport"),
        LegacySurfaceDisposition::ConsumedAsInputOnly
    );
    assert_eq!(
        inventory.disposition_for("SupportTrustAccessIndexKind"),
        LegacySurfaceDisposition::ConsumedAsInputOnly
    );
}
