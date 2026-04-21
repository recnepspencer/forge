use forge_store::QuarantinedDecodedArtifact;

fn main() {
    let _ = QuarantinedDecodedArtifact::new(
        unreachable!(),
        unreachable!(),
        unreachable!(),
        unreachable!(),
        "digest",
        "diagnostic",
    );
}
