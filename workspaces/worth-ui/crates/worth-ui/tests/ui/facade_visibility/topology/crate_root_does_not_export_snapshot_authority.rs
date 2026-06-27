use worth_ui::{
    CapabilitySnapshotBuilder, CapabilitySnapshotFreezeInput, CapabilitySnapshotIndexParts,
};

fn main() {
    let _ = core::mem::size_of::<CapabilitySnapshotBuilder>();
    let _ = core::mem::size_of::<CapabilitySnapshotFreezeInput>();
    let _ = core::mem::size_of::<CapabilitySnapshotIndexParts<'static>>();
}
