struct SyntheticEventHistoryRecord {
    _opaque: u64,
}

fn accepts_lineage(_: &forge_foundational::FoundationalBoundaryEvidenceAttestedLineageArtifact) {}

fn main() {
    let history = SyntheticEventHistoryRecord { _opaque: 13 };
    accepts_lineage(&history);
}
