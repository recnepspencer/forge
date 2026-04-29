use forge_store::{FoundationEvidenceWitness, Roadmap2SequenceId, S0StableDigest};

fn main() {
    let _witness = FoundationEvidenceWitness::new(
        Roadmap2SequenceId::new("S1").unwrap(),
        S0StableDigest::new("evidence:s1").unwrap(),
    );
}
