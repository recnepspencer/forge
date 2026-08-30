use worth_store_physical_integrity::{
    IntegrityValidatedCurrentRootSelector, IntegrityValidatedPreviousRootSelector,
    IntegrityValidatedExtentChunkFrame, IntegrityValidatedExtentManifest,
    IntegrityValidatedPageFrame, IntegrityValidatedPhysicalWorkObligation,
    IntegrityValidatedRootManifest, IntegrityValidatedWalFrame, PhysicalArtifactScope,
};

fn forge_current(scope: PhysicalArtifactScope) {
    let _forged = IntegrityValidatedCurrentRootSelector {
        scope,
        inspected: todo!(),
    };
}

fn forge_previous(scope: PhysicalArtifactScope) {
    let _forged = IntegrityValidatedPreviousRootSelector {
        scope,
        inspected: todo!(),
    };
}

fn forge_manifest(scope: PhysicalArtifactScope) {
    let _forged = IntegrityValidatedRootManifest {
        scope,
        inspected: todo!(),
    };
}

fn forge_physical_work(scope: PhysicalArtifactScope) {
    let _forged = IntegrityValidatedPhysicalWorkObligation {
        scope,
        inspected: todo!(),
    };
}

fn forge_extent_manifest(scope: PhysicalArtifactScope) {
    let _forged = IntegrityValidatedExtentManifest {
        scope,
        inspected: todo!(),
    };
}

fn forge_page(scope: PhysicalArtifactScope) {
    let _forged = IntegrityValidatedPageFrame {
        scope,
        inspected: todo!(),
    };
}

fn forge_wal(scope: PhysicalArtifactScope) {
    let _forged = IntegrityValidatedWalFrame {
        scope,
        inspected: todo!(),
    };
}

fn forge_extent_chunk(scope: PhysicalArtifactScope) {
    let _forged = IntegrityValidatedExtentChunkFrame {
        scope,
        inspected: todo!(),
    };
}

fn main() {}
