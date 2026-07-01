use schema::facade::platform::authority::compiled_product_semantic_graph::CompiledProductEquivalencePolicyIdentity;

use super::basis_identity::{
    SpatialSelectedCompatibilityBasisIdentity, SpatialSelectedEquivalenceBasisIdentity,
    SpatialSelectedFutureProofSeedIdentity, SpatialSelectedReuseBasisIdentity,
};
use super::declaration::SpatialSelectedEquivalenceFamilyDeclaration;
use super::family_identity::SpatialSelectedEquivalenceFamilyIdentity;
use super::posture::{
    SpatialCompatibilityPosture, SpatialFreshnessRequirementPosture, SpatialOrderingNoisePosture,
    SpatialRenderedOutputComparisonPosture,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectedSpatialEquivalenceFamily {
    declaration: SpatialSelectedEquivalenceFamilyDeclaration,
    equivalence_policy_identity: CompiledProductEquivalencePolicyIdentity,
    equivalence_basis_identity: SpatialSelectedEquivalenceBasisIdentity,
    compatibility_basis_identity: SpatialSelectedCompatibilityBasisIdentity,
    reuse_basis_identity: SpatialSelectedReuseBasisIdentity,
    future_public_proof_seed_identity: SpatialSelectedFutureProofSeedIdentity,
}

impl SelectedSpatialEquivalenceFamily {
    pub(crate) fn new(
        declaration: SpatialSelectedEquivalenceFamilyDeclaration,
        equivalence_policy_identity: CompiledProductEquivalencePolicyIdentity,
        equivalence_basis_identity: SpatialSelectedEquivalenceBasisIdentity,
        compatibility_basis_identity: SpatialSelectedCompatibilityBasisIdentity,
        reuse_basis_identity: SpatialSelectedReuseBasisIdentity,
        future_public_proof_seed_identity: SpatialSelectedFutureProofSeedIdentity,
    ) -> Self {
        Self {
            declaration,
            equivalence_policy_identity,
            equivalence_basis_identity,
            compatibility_basis_identity,
            reuse_basis_identity,
            future_public_proof_seed_identity,
        }
    }

    pub const fn family_identity(&self) -> SpatialSelectedEquivalenceFamilyIdentity {
        self.declaration.identity()
    }

    pub fn equivalence_policy_identity(&self) -> &CompiledProductEquivalencePolicyIdentity {
        &self.equivalence_policy_identity
    }

    pub fn equivalence_basis_identity(&self) -> &SpatialSelectedEquivalenceBasisIdentity {
        &self.equivalence_basis_identity
    }

    pub fn compatibility_basis_identity(&self) -> &SpatialSelectedCompatibilityBasisIdentity {
        &self.compatibility_basis_identity
    }

    pub fn reuse_basis_identity(&self) -> &SpatialSelectedReuseBasisIdentity {
        &self.reuse_basis_identity
    }

    pub fn future_public_proof_seed_identity(&self) -> &SpatialSelectedFutureProofSeedIdentity {
        &self.future_public_proof_seed_identity
    }

    pub const fn compatibility_posture(&self) -> SpatialCompatibilityPosture {
        self.declaration.compatibility_posture()
    }

    pub const fn freshness_requirement_posture(&self) -> SpatialFreshnessRequirementPosture {
        self.declaration.freshness_requirement_posture()
    }

    pub const fn ordering_noise_posture(&self) -> SpatialOrderingNoisePosture {
        self.declaration.ordering_noise_posture()
    }

    pub const fn rendered_output_comparison_posture(
        &self,
    ) -> SpatialRenderedOutputComparisonPosture {
        self.declaration.rendered_output_comparison_posture()
    }
}
