use topology::facade::compiled_product_family::{
    TopologyAuthorityBasisPosture, TopologyCompiledProductConsumer,
    TopologyCompiledProductFamilyDeclaration, TopologyCompiledProductFamilyIdentity,
    TopologyEquivalencePolicyPosture, TopologyLocalityFootprintPosture,
    TopologyPriorProofPosture, TopologyStageIdentityPosture,
    TopologyValidatorEvidenceRolePosture,
};

fn main() {
    let _ = TopologyCompiledProductFamilyDeclaration::new(
        TopologyCompiledProductFamilyIdentity::DerivedTopologyEquivalenceContract,
        vec![TopologyCompiledProductConsumer::DerivedEquivalenceContractProjection],
        TopologyAuthorityBasisPosture::DerivedTopologyTruthBasis,
        TopologyLocalityFootprintPosture::InvalidationClosure,
        TopologyPriorProofPosture::NotRequired,
        TopologyStageIdentityPosture::NotRequired,
        TopologyValidatorEvidenceRolePosture::DerivedValidationDigestEquivalenceDimension,
        TopologyEquivalencePolicyPosture::DerivedTopologySemanticParity,
        "forged-family",
        &["compiled-product-identity"],
    );
}
