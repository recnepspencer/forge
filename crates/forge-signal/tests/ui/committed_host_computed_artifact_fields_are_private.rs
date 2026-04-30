use forge_signal::facade::CommittedHostComputedArtifact;

fn main() {
    let _ = CommittedHostComputedArtifact { staged: loop {} };
}
