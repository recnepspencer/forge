use worth_ui::facade::{
    app::WorthUi,
    registry::{
        MosaicPlacementAction, MosaicPlacementSource, MosaicPlacementTarget, MosaicRegionRole,
    },
};

use super::mosaic_placement_registry_assertions::assert_registered_mosaic_placement_ids;
use super::mosaic_placement_registry_fixtures::{
    auxiliary_dock_policy, complete_policy, primary_dock_policy,
};

#[test]
fn equivalent_mosaic_placement_policies_produce_equivalent_legality_tables() {
    let first = WorthUi::app()
        .register_mosaic_placement_policy(primary_dock_policy("workspace.placement.primary"))
        .register_mosaic_placement_policy(auxiliary_dock_policy("workspace.placement.side"))
        .freeze()
        .expect("application preparation should succeed");
    let second = WorthUi::app()
        .register_mosaic_placement_policy(auxiliary_dock_policy("workspace.placement.side"))
        .register_mosaic_placement_policy(primary_dock_policy("workspace.placement.primary"))
        .freeze()
        .expect("application preparation should succeed");

    assert_eq!(
        first.capabilities().mosaic_placement_policies(),
        second.capabilities().mosaic_placement_policies()
    );
    assert_eq!(
        first.capabilities().digest(),
        second.capabilities().digest()
    );
    assert_registered_mosaic_placement_ids(
        first.capabilities().mosaic_placement_policies(),
        &["workspace.placement.primary", "workspace.placement.side"],
    );
}

#[test]
fn different_mosaic_placement_meaning_changes_snapshot_digest() {
    let dock = WorthUi::app()
        .register_mosaic_placement_policy(primary_dock_policy("workspace.placement.primary"))
        .freeze()
        .expect("application preparation should succeed");
    let tab = WorthUi::app()
        .register_mosaic_placement_policy(
            complete_policy("workspace.placement.primary", MosaicPlacementAction::tab())
                .with_source(MosaicPlacementSource::region_role(
                    MosaicRegionRole::primary(),
                ))
                .with_target(MosaicPlacementTarget::region_stack(
                    MosaicRegionRole::stack(),
                )),
        )
        .freeze()
        .expect("application preparation should succeed");

    assert_ne!(
        dock.capabilities().mosaic_placement_policies(),
        tab.capabilities().mosaic_placement_policies()
    );
    assert_ne!(dock.capabilities().digest(), tab.capabilities().digest());
}
