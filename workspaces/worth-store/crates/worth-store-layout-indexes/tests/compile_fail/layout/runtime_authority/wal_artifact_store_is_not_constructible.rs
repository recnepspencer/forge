use worth_store_wal::AdmittedWalArtifactStore;

fn worth() -> AdmittedWalArtifactStore {
    AdmittedWalArtifactStore { identity: panic!() }
}

fn main() {
    let _ = worth();
}
