use worth_ui::facade::{
    declaration::{MosaicStateSlotDescriptor, MosaicStateSlotId, MosaicStateSlotKind},
};

fn main() {
    let _descriptor = MosaicStateSlotDescriptor {
        id: MosaicStateSlotId::new("workspace.state.sidebar_width").unwrap(),
        kind: MosaicStateSlotKind::splitter_position(),
        owner_identity: None,
        persistence_policy: None,
        replacement_rule: None,
        truth_posture: None,
        label: None,
    };
}
