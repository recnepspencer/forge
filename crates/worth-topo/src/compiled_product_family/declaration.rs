use serde::Serialize;
use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::consumer::TopologyCompiledProductConsumer;
use super::family_identity::TopologyCompiledProductFamilyIdentity;
use super::posture::{
    TopologyAuthorityBasisPosture, TopologyEquivalencePolicyPosture,
    TopologyLocalityFootprintPosture, TopologyPriorProofPosture, TopologyStageIdentityPosture,
    TopologyValidatorEvidenceRolePosture,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TopologyCompiledProductFamilyDeclaration {
    identity: TopologyCompiledProductFamilyIdentity,
    supported_consumers: Vec<TopologyCompiledProductConsumer>,
    authority_basis: TopologyAuthorityBasisPosture,
    locality_footprint: TopologyLocalityFootprintPosture,
    prior_proof: TopologyPriorProofPosture,
    stage_identity: TopologyStageIdentityPosture,
    validator_evidence_role: TopologyValidatorEvidenceRolePosture,
    equivalence_policy: TopologyEquivalencePolicyPosture,
    equivalence_policy_name: &'static str,
    equivalence_dimensions: &'static [&'static str],
    family_digest: String,
}

impl TopologyCompiledProductFamilyDeclaration {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        identity: TopologyCompiledProductFamilyIdentity,
        mut supported_consumers: Vec<TopologyCompiledProductConsumer>,
        authority_basis: TopologyAuthorityBasisPosture,
        locality_footprint: TopologyLocalityFootprintPosture,
        prior_proof: TopologyPriorProofPosture,
        stage_identity: TopologyStageIdentityPosture,
        validator_evidence_role: TopologyValidatorEvidenceRolePosture,
        equivalence_policy: TopologyEquivalencePolicyPosture,
        equivalence_policy_name: &'static str,
        equivalence_dimensions: &'static [&'static str],
    ) -> Self {
        supported_consumers.sort_by_key(|consumer| consumer.as_str());
        let family_digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "worth-topo:compiled-product-family-declaration:v1".to_string(),
                format!("identity:{}", identity.as_str()),
                format!(
                    "consumers:{}",
                    supported_consumers
                        .iter()
                        .map(|consumer| consumer.as_str())
                        .collect::<Vec<_>>()
                        .join("|")
                ),
                format!("authority:{authority_basis:?}"),
                format!("locality:{locality_footprint:?}"),
                format!("prior-proof:{prior_proof:?}"),
                format!("stage:{stage_identity:?}"),
                format!("validator-evidence:{validator_evidence_role:?}"),
                format!("equivalence-posture:{equivalence_policy:?}"),
                format!("equivalence-policy-name:{equivalence_policy_name}"),
                format!(
                    "equivalence-dimensions:{}",
                    equivalence_dimensions.join("|")
                ),
            ],
        );
        Self {
            identity,
            supported_consumers,
            authority_basis,
            locality_footprint,
            prior_proof,
            stage_identity,
            validator_evidence_role,
            equivalence_policy,
            equivalence_policy_name,
            equivalence_dimensions,
            family_digest,
        }
    }

    pub const fn identity(&self) -> TopologyCompiledProductFamilyIdentity {
        self.identity
    }

    pub fn supported_consumers(&self) -> &[TopologyCompiledProductConsumer] {
        &self.supported_consumers
    }

    pub const fn authority_basis(&self) -> TopologyAuthorityBasisPosture {
        self.authority_basis
    }

    pub const fn locality_footprint(&self) -> TopologyLocalityFootprintPosture {
        self.locality_footprint
    }

    pub const fn prior_proof(&self) -> TopologyPriorProofPosture {
        self.prior_proof
    }

    pub const fn stage_identity(&self) -> TopologyStageIdentityPosture {
        self.stage_identity
    }

    pub const fn validator_evidence_role(&self) -> TopologyValidatorEvidenceRolePosture {
        self.validator_evidence_role
    }

    pub const fn equivalence_policy(&self) -> TopologyEquivalencePolicyPosture {
        self.equivalence_policy
    }

    pub const fn equivalence_policy_name(&self) -> &'static str {
        self.equivalence_policy_name
    }

    pub const fn equivalence_dimensions(&self) -> &'static [&'static str] {
        self.equivalence_dimensions
    }

    pub fn supports(&self, consumer: TopologyCompiledProductConsumer) -> bool {
        self.supported_consumers.contains(&consumer)
    }

    pub fn family_digest(&self) -> &str {
        &self.family_digest
    }
}
