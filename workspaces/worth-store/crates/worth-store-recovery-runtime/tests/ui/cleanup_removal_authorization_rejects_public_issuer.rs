use worth_store::physical_runtime::AdmittedRecoveryFilesystemMedia;

fn supplied<T>() -> T {
    panic!("compile-only supplied production value")
}

fn bypass(media: &AdmittedRecoveryFilesystemMedia) {
    // Even a caller that possesses every genuine decoded lower-layer value
    // cannot reach a backend unlink boundary. Cleanup effects are owned by the
    // Store coordination continuation, not by a caller-enabled feature.
    let _ = media.remove_recovery_wal_artifact_scheduled(
        supplied(),
        supplied(),
        supplied(),
        supplied(),
        supplied(),
        supplied(),
    );
}

fn main() {
    bypass(supplied());
}
