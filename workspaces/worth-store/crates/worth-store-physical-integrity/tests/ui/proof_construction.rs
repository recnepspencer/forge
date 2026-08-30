use worth_store_physical_integrity::{
    IntegrityValidatedCurrentRootSelector, IntegrityValidatedPreviousRootSelector,
    IntegrityValidatedPhysicalWorkObligation, IntegrityValidatedRootManifest, PhysicalArtifactScope,
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

fn main() {}
