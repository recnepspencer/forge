use schema::facade::platform::authority::{
    CanonicalTopologyMutationBatch, CertifiedTopologyInterpretation, DerivedTopologyReadBasis,
    DerivedTruthBasisIdentity, PersistedTopologyTruthBatch, TopologyReadArtifact,
};

fn main() {
    let _ = std::mem::size_of::<CanonicalTopologyMutationBatch>();
    let _ = std::mem::size_of::<PersistedTopologyTruthBatch>();
    let _ = std::mem::size_of::<DerivedTopologyReadBasis>();
    let _ = std::mem::size_of::<DerivedTruthBasisIdentity>();
    let _ = std::mem::size_of::<TopologyReadArtifact>();
    let _ = std::mem::size_of::<CertifiedTopologyInterpretation>();
}
