use schema::facade::platform::authority::compiled_product_semantic_graph::CompiledProductEquivalencePolicyIdentity;

use super::basis_identity::{
    TopologySelectedCompatibilityBasisIdentity, TopologySelectedEquivalenceBasisIdentity,
    TopologySelectedFutureProofSeedIdentity, TopologySelectedReuseBasisIdentity,
};
use super::comparator_contract::{
    TopologySelectedEquivalenceComparatorContract, TopologySelectedEquivalenceDimension,
};
use super::declaration::TopologySelectedEquivalenceFamilyDeclaration;
use super::family_identity::TopologySelectedEquivalenceFamilyIdentity;
use super::posture::{
    TopologyCompatibilityPosture, TopologyFreshnessRequirementPosture,
    TopologyOrderingNoisePosture, TopologyRenderedOutputComparisonPosture,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectedTopologyEquivalenceFamily {
    declaration: TopologySelectedEquivalenceFamilyDeclaration,
    equivalence_policy_identity: CompiledProductEquivalencePolicyIdentity,
    equivalence_basis_identity: TopologySelectedEquivalenceBasisIdentity,
    compatibility_basis_identity: TopologySelectedCompatibilityBasisIdentity,
    reuse_basis_identity: TopologySelectedReuseBasisIdentity,
    future_public_proof_seed_identity: TopologySelectedFutureProofSeedIdentity,
}

impl SelectedTopologyEquivalenceFamily {
    pub(crate) fn new(
        declaration: TopologySelectedEquivalenceFamilyDeclaration,
        equivalence_policy_identity: CompiledProductEquivalencePolicyIdentity,
        equivalence_basis_identity: TopologySelectedEquivalenceBasisIdentity,
        compatibility_basis_identity: TopologySelectedCompatibilityBasisIdentity,
        reuse_basis_identity: TopologySelectedReuseBasisIdentity,
        future_public_proof_seed_identity: TopologySelectedFutureProofSeedIdentity,
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

    pub const fn family_identity(&self) -> TopologySelectedEquivalenceFamilyIdentity {
        self.declaration.identity()
    }

    pub fn equivalence_policy_identity(&self) -> &CompiledProductEquivalencePolicyIdentity {
        &self.equivalence_policy_identity
    }

    pub fn equivalence_basis_identity(&self) -> &TopologySelectedEquivalenceBasisIdentity {
        &self.equivalence_basis_identity
    }

    pub fn compatibility_basis_identity(&self) -> &TopologySelectedCompatibilityBasisIdentity {
        &self.compatibility_basis_identity
    }

    pub fn reuse_basis_identity(&self) -> &TopologySelectedReuseBasisIdentity {
        &self.reuse_basis_identity
    }

    pub fn future_public_proof_seed_identity(&self) -> &TopologySelectedFutureProofSeedIdentity {
        &self.future_public_proof_seed_identity
    }

    pub(crate) fn with_hostile_selected_basis_overrides(
        mut self,
        compatibility_basis_identity_digest: Option<&str>,
        reuse_basis_identity_digest: Option<&str>,
    ) -> Self {
        if let Some(identity_digest) = compatibility_basis_identity_digest {
            self.compatibility_basis_identity =
                TopologySelectedCompatibilityBasisIdentity::from_identity_digest_for_certification(
                    identity_digest,
                );
        }
        if let Some(identity_digest) = reuse_basis_identity_digest {
            self.reuse_basis_identity =
                TopologySelectedReuseBasisIdentity::from_identity_digest_for_certification(
                    identity_digest,
                );
        }
        self
    }

    pub fn comparator_contract(&self) -> TopologySelectedEquivalenceComparatorContract {
        TopologySelectedEquivalenceComparatorContract::new(
            self.family_identity(),
            self.equivalence_policy_identity
                .identity_digest()
                .to_string(),
            self.equivalence_dimensions().to_vec(),
            self.compatibility_posture(),
            self.freshness_requirement_posture(),
            self.ordering_noise_posture(),
            self.rendered_output_comparison_posture(),
        )
    }

    pub const fn equivalence_dimensions(&self) -> &'static [TopologySelectedEquivalenceDimension] {
        self.declaration.equivalence_dimensions()
    }

    pub const fn compatibility_posture(&self) -> TopologyCompatibilityPosture {
        self.declaration.compatibility_posture()
    }

    pub const fn freshness_requirement_posture(&self) -> TopologyFreshnessRequirementPosture {
        self.declaration.freshness_requirement_posture()
    }

    pub const fn ordering_noise_posture(&self) -> TopologyOrderingNoisePosture {
        self.declaration.ordering_noise_posture()
    }

    pub const fn rendered_output_comparison_posture(
        &self,
    ) -> TopologyRenderedOutputComparisonPosture {
        self.declaration.rendered_output_comparison_posture()
    }
}
