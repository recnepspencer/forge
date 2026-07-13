use forge_store_wal::AdmittedWalArtifactStore;

fn forge() -> AdmittedWalArtifactStore {
    AdmittedWalArtifactStore { identity: panic!() }
}

fn main() {
    let _ = forge();
}
