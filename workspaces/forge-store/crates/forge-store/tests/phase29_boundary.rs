use forge_store::layout_boundary::{
    LegacyAccessPathBypassInventory, LegacySurfaceDisposition,
    LegacySurfaceDispositionAndDedicatedWorkspaceBoundary,
};

#[test]
fn workspace_facade_exposes_named_layout_boundary_instead_of_legacy_read_surfaces() {
    let boundary = LegacySurfaceDispositionAndDedicatedWorkspaceBoundary::current();
    let inventory: LegacyAccessPathBypassInventory = boundary.inventory();

    assert_eq!(boundary.dedicated_workspace_crate(), "forge-store");
    assert_eq!(
        inventory.disposition_for("ForgeStore"),
        LegacySurfaceDisposition::SupersededAndForbidden
    );
    assert_eq!(
        inventory.disposition_for("AspectLayoutReadRequest"),
        LegacySurfaceDisposition::ForbiddenAsAuthority
    );
    assert_eq!(
        inventory.disposition_for("AspectLayoutReadExecutionResult"),
        LegacySurfaceDisposition::ForbiddenAsAuthority
    );
}
