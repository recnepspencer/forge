use schema::facade::platform::authority::{
    CertifiedTopologyInterpretation, DerivedTopologyReadBasis, DerivedTruthBasisIdentity,
    PersistedTopologyTruth, TopologyCommittedMutationSet, TopologyReadArtifact,
};

fn main() {
    let _ = std::mem::size_of::<TopologyCommittedMutationSet>();
    let _ = std::mem::size_of::<PersistedTopologyTruth>();
    let _ = std::mem::size_of::<DerivedTopologyReadBasis>();
    let _ = std::mem::size_of::<DerivedTruthBasisIdentity>();
    let _ = std::mem::size_of::<TopologyReadArtifact>();
    let _ = std::mem::size_of::<CertifiedTopologyInterpretation>();
}
