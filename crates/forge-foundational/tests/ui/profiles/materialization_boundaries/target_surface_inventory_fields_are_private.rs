use std::marker::PhantomData;

use forge_foundational::{
    FoundationalDescriptiveSurface, FoundationalProfileAttachmentTargetKind,
    FoundationalTargetSurfaceInventory, SupportArtifactTarget,
};

fn main() {
    let _ = FoundationalTargetSurfaceInventory::<SupportArtifactTarget> {
        target_kind: FoundationalProfileAttachmentTargetKind::SupportArtifact,
        surfaces: &[FoundationalDescriptiveSurface::Lineage],
        marker: PhantomData,
    };
}
