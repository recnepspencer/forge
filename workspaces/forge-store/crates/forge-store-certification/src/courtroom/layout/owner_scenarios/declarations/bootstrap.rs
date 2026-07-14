use forge_store_layout_indexes::{bootstrap_catalog, BootstrapOnlyAccessPath, ObserveOwnerCase};
use forge_store_physical_format::physical_bootstrap_catalog;

use super::super::LayoutOwnerObservationLedger;

pub(super) fn execute(ledger: &mut LayoutOwnerObservationLedger) {
    let mut physical = forge_store_test_support::open_layout_physical_facade();
    let first = physical.publish_physical_root().unwrap();
    let first_open = first.admit_bootstrap_open_witness().unwrap();
    let catalog = physical_bootstrap_catalog()
        .discover_catalog(&first_open)
        .unwrap();
    let current_root = physical_bootstrap_catalog()
        .discover_catalog(&first_open)
        .unwrap()
        .current_root();
    let admitted = bootstrap_catalog().read_catalog(
        BootstrapOnlyAccessPath::fixed_bootstrap_access_path(),
        catalog,
        current_root,
    );
    ledger.record_bootstrap_catalog_read(admitted.owner_case_observation());

    let second = physical.publish_physical_root().unwrap();
    let second_open = second.admit_bootstrap_open_witness().unwrap();
    let stale_catalog = physical_bootstrap_catalog()
        .discover_catalog(&first_open)
        .unwrap();
    let advanced_root = physical_bootstrap_catalog()
        .discover_catalog(&second_open)
        .unwrap()
        .current_root();
    let stale = bootstrap_catalog().read_catalog(
        BootstrapOnlyAccessPath::fixed_bootstrap_access_path(),
        stale_catalog,
        advanced_root,
    );
    ledger.record_bootstrap_catalog_read(stale.owner_case_observation());
}
