use worth_ui::facade::{
    declaration::{MosaicPlacementAction, MosaicPlacementPolicyDescriptor, MosaicPlacementPolicyId},
};

fn main() {
    let _descriptor = MosaicPlacementPolicyDescriptor {
        id: MosaicPlacementPolicyId::new("workspace.placement.primary")
            .expect("valid mosaic placement policy id"),
        action: MosaicPlacementAction::dock(),
        source: None,
        target: None,
        persistence: None,
        stable_identity_behavior: None,
        conflict_behavior: None,
        reload_reconciliation: None,
        support: None,
        label: None,
    };
}
