#[allow(dead_code)]
mod phase_three_support;

use phase_three_support::*;
use worth_store_physical_format::{
    BootstrapCatalog, CurrentRootCatalogEntry, CurrentRootCatalogGeneration, DurableRootSelector,
    PhysicalRecordFormatDeclaration, RootSelectorIdentity, RootSelectorRole, ROOT_SELECTOR_BYTES,
};
use worth_store_recovery_physics::{
    PhysicalRootCandidateDenial, PhysicalRootSelectionDenial, SelectedPhysicalRootRole,
};
use worth_store_recovery_runtime::PhysicalRecoverySourceDenial;

#[test]
fn torn_current_fallback_requires_the_exact_completed_publication_anchor() {
    let exact_parent = tempfile::tempdir().unwrap();
    let exact_root = exact_parent.path().join("exact-anchor");
    let exact_store = initialize_store(&exact_root);
    publish_synthetic_genesis(&exact_root, exact_store);
    publish_previous_and_anchor(&exact_root, exact_store, 2);
    let selected = admitted_recovery(&exact_root)
        .discover()
        .unwrap()
        .select()
        .unwrap();
    assert_eq!(
        selected.root_role(),
        SelectedPhysicalRootRole::PreviousFallback
    );
    assert_eq!(selected.root_generation(), 1);
    let _ = selected.cancel_before_reconstruction();

    let stale_parent = tempfile::tempdir().unwrap();
    let stale_root = stale_parent.path().join("stale-anchor");
    let stale_store = initialize_store(&stale_root);
    publish_synthetic_genesis(&stale_root, stale_store);
    publish_previous_and_anchor(&stale_root, stale_store, 3);
    let blocked = expect_blocked(
        admitted_recovery(&stale_root)
            .discover()
            .unwrap()
            .select()
            .err()
            .expect("a stale publication anchor must block previous fallback"),
    );
    assert!(matches!(
        blocked.evidence().source_denials.as_slice(),
        [
            PhysicalRecoverySourceDenial::RootSlot {
                slot: RootSelectorRole::Current,
                denial: PhysicalRootCandidateDenial::SelectorFormat(_),
                ..
            },
            PhysicalRecoverySourceDenial::RootSelection(
                PhysicalRootSelectionDenial::PreviousFallbackAnchorGenerationMismatch
            )
        ]
    ));
}

fn publish_previous_and_anchor(
    root: &std::path::Path,
    store: worth_store_physical_format::store_namespace::StableStoreIdentity,
    anchor_generation: u64,
) {
    let format = PhysicalRecordFormatDeclaration::builder().admit().unwrap();
    let previous = DurableRootSelector::new(
        store,
        format,
        RootSelectorIdentity::new(1).unwrap(),
        RootSelectorRole::Previous,
        1,
        RootSelectorIdentity::new(2),
        Some(2),
    )
    .unwrap();
    let records = root.join("families").join("records");
    std::fs::write(records.join("root-previous.selector"), previous.encode()).unwrap();
    let catalog = BootstrapCatalog::new(
        store,
        format,
        CurrentRootCatalogEntry::new(CurrentRootCatalogGeneration::new(anchor_generation).unwrap()),
    );
    std::fs::write(records.join("bootstrap.catalog"), catalog.encode()).unwrap();
    std::fs::write(
        records.join("root-current.selector"),
        [0_u8; ROOT_SELECTOR_BYTES],
    )
    .unwrap();
}
