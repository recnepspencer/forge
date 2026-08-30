use worth_store_physical_integrity::{IntegrityValidatedRootManifest, PhysicalArtifactScope};

fn forge(scope: PhysicalArtifactScope) {
    let _forged = IntegrityValidatedRootManifest {
        scope,
        inspected: todo!(),
    };
}

fn main() {}
