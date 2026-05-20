struct SyntheticReplaySlice {
    _opaque: u64,
}

fn accepts_provenance(_: &forge_foundational::FoundationalBoundaryEvidenceProvenanceArtifact) {}

fn main() {
    let replay_slice = SyntheticReplaySlice { _opaque: 7 };
    accepts_provenance(&replay_slice);
}
