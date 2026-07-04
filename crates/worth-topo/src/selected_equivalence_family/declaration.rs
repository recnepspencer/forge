use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::compiled_product_family::TopologyCompiledProductFamilyIdentity;

use super::comparator_contract::TopologySelectedEquivalenceDimension;
use super::family_identity::TopologySelectedEquivalenceFamilyIdentity;
use super::posture::{
    TopologyCompatibilityPosture, TopologyFreshnessRequirementPosture,
    TopologyOrderingNoisePosture, TopologyRenderedOutputComparisonPosture,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopologySelectedEquivalenceFamilyDeclaration {
    identity: TopologySelectedEquivalenceFamilyIdentity,
    compiled_product_family_identity: TopologyCompiledProductFamilyIdentity,
    equivalence_policy_name: &'static str,
    equivalence_dimensions: &'static [TopologySelectedEquivalenceDimension],
    compatibility_posture: TopologyCompatibilityPosture,
    freshness_requirement_posture: TopologyFreshnessRequirementPosture,
    ordering_noise_posture: TopologyOrderingNoisePosture,
    rendered_output_comparison_posture: TopologyRenderedOutputComparisonPosture,
    family_digest: String,
}

impl TopologySelectedEquivalenceFamilyDeclaration {
    pub(crate) fn derived_topology_semantic_parity() -> Self {
        let identity = TopologySelectedEquivalenceFamilyIdentity::DerivedTopologySemanticParity;
        let compiled_product_family_identity = identity.compiled_product_family_identity();
        let equivalence_policy_name = "derived-topology-semantic-parity";
        let equivalence_dimensions = &[
            TopologySelectedEquivalenceDimension::SelectedEquivalenceBasisIdentity,
            TopologySelectedEquivalenceDimension::SelectedReuseBasisIdentity,
            TopologySelectedEquivalenceDimension::DerivedValidationDigest,
            TopologySelectedEquivalenceDimension::MaterializedTopologyDigest,
            TopologySelectedEquivalenceDimension::InterpretedTopologyDigest,
        ];
        let compatibility_posture = TopologyCompatibilityPosture::DistinctFromEquivalence;
        let freshness_requirement_posture =
            TopologyFreshnessRequirementPosture::SameAdmittedAuthorityAndLocalityRequired;
        let ordering_noise_posture = TopologyOrderingNoisePosture::ExactOrderingRequired;
        let rendered_output_comparison_posture =
            TopologyRenderedOutputComparisonPosture::DerivedOutputDigestsRequired;
        let family_digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "worth-topo:selected-equivalence-family-declaration:v1".to_string(),
                format!("identity:{}", identity.as_str()),
                format!(
                    "compiled-family:{}",
                    compiled_product_family_identity.as_str()
                ),
                format!("equivalence-policy-name:{equivalence_policy_name}"),
                format!(
                    "equivalence-dimensions:{}",
                    equivalence_dimensions
                        .iter()
                        .map(TopologySelectedEquivalenceDimension::as_str)
                        .collect::<Vec<_>>()
                        .join("|")
                ),
                format!("compatibility:{compatibility_posture:?}"),
                format!("freshness:{freshness_requirement_posture:?}"),
                format!("ordering:{ordering_noise_posture:?}"),
                format!("rendered-output:{rendered_output_comparison_posture:?}"),
            ],
        );
        Self {
            identity,
            compiled_product_family_identity,
            equivalence_policy_name,
            equivalence_dimensions,
            compatibility_posture,
            freshness_requirement_posture,
            ordering_noise_posture,
            rendered_output_comparison_posture,
            family_digest,
        }
    }

    pub const fn identity(&self) -> TopologySelectedEquivalenceFamilyIdentity {
        self.identity
    }

    pub const fn compiled_product_family_identity(&self) -> TopologyCompiledProductFamilyIdentity {
        self.compiled_product_family_identity
    }

    pub const fn equivalence_policy_name(&self) -> &'static str {
        self.equivalence_policy_name
    }

    pub const fn equivalence_dimensions(&self) -> &'static [TopologySelectedEquivalenceDimension] {
        self.equivalence_dimensions
    }

    pub const fn compatibility_posture(&self) -> TopologyCompatibilityPosture {
        self.compatibility_posture
    }

    pub const fn freshness_requirement_posture(&self) -> TopologyFreshnessRequirementPosture {
        self.freshness_requirement_posture
    }

    pub const fn ordering_noise_posture(&self) -> TopologyOrderingNoisePosture {
        self.ordering_noise_posture
    }

    pub const fn rendered_output_comparison_posture(
        &self,
    ) -> TopologyRenderedOutputComparisonPosture {
        self.rendered_output_comparison_posture
    }

    pub fn family_digest(&self) -> &str {
        &self.family_digest
    }
}
