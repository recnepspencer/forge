use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::spatial_compiled_product_family::SpatialCompiledProductFamilyIdentity;

use super::family_identity::SpatialSelectedEquivalenceFamilyIdentity;
use super::posture::{
    SpatialCompatibilityPosture, SpatialFreshnessRequirementPosture, SpatialOrderingNoisePosture,
    SpatialRenderedOutputComparisonPosture,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpatialSelectedEquivalenceFamilyDeclaration {
    identity: SpatialSelectedEquivalenceFamilyIdentity,
    compiled_product_family_identity: SpatialCompiledProductFamilyIdentity,
    equivalence_policy_name: &'static str,
    equivalence_dimensions: &'static [&'static str],
    compatibility_posture: SpatialCompatibilityPosture,
    freshness_requirement_posture: SpatialFreshnessRequirementPosture,
    ordering_noise_posture: SpatialOrderingNoisePosture,
    rendered_output_comparison_posture: SpatialRenderedOutputComparisonPosture,
    family_digest: String,
}

impl SpatialSelectedEquivalenceFamilyDeclaration {
    pub(crate) fn evidence_lookup_semantic_parity() -> Self {
        Self::new(
            SpatialSelectedEquivalenceFamilyIdentity::EvidenceLookupSemanticParity,
            "evidence-lookup-index-semantic-parity",
            &[
                "compiled-product-identity",
                "authority-truth-identity",
                "locality-footprint-identity",
                "prior-proof-identity",
            ],
            SpatialOrderingNoisePosture::DeclaredBenignOrderingNoiseAllowed,
        )
    }

    pub(crate) fn retained_cancellation_semantic_parity() -> Self {
        Self::new(
            SpatialSelectedEquivalenceFamilyIdentity::RetainedCancellationSemanticParity,
            "retained-cancellation-semantic-parity",
            &[
                "compiled-product-identity",
                "authority-truth-identity",
                "locality-footprint-identity",
                "prior-proof-identity",
            ],
            SpatialOrderingNoisePosture::ExactOrderingRequired,
        )
    }

    pub(crate) fn retained_replay_semantic_parity() -> Self {
        Self::new(
            SpatialSelectedEquivalenceFamilyIdentity::RetainedReplaySemanticParity,
            "retained-replay-semantic-parity",
            &[
                "compiled-product-identity",
                "authority-truth-identity",
                "locality-footprint-identity",
            ],
            SpatialOrderingNoisePosture::ExactOrderingRequired,
        )
    }

    fn new(
        identity: SpatialSelectedEquivalenceFamilyIdentity,
        equivalence_policy_name: &'static str,
        equivalence_dimensions: &'static [&'static str],
        ordering_noise_posture: SpatialOrderingNoisePosture,
    ) -> Self {
        let compiled_product_family_identity = identity.compiled_product_family_identity();
        let compatibility_posture = SpatialCompatibilityPosture::DistinctFromEquivalence;
        let freshness_requirement_posture =
            SpatialFreshnessRequirementPosture::SameAdmittedAuthorityAndLocalityRequired;
        let rendered_output_comparison_posture =
            SpatialRenderedOutputComparisonPosture::NotPartOfBasis;
        let family_digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "worth-spatial:selected-equivalence-family-declaration:v1".to_string(),
                format!("identity:{}", identity.as_str()),
                format!(
                    "compiled-family:{}",
                    compiled_product_family_identity.as_str()
                ),
                format!("equivalence-policy-name:{equivalence_policy_name}"),
                format!(
                    "equivalence-dimensions:{}",
                    equivalence_dimensions.join("|")
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

    pub const fn identity(&self) -> SpatialSelectedEquivalenceFamilyIdentity {
        self.identity
    }

    pub const fn compiled_product_family_identity(&self) -> SpatialCompiledProductFamilyIdentity {
        self.compiled_product_family_identity
    }

    pub const fn equivalence_policy_name(&self) -> &'static str {
        self.equivalence_policy_name
    }

    pub const fn equivalence_dimensions(&self) -> &'static [&'static str] {
        self.equivalence_dimensions
    }

    pub const fn compatibility_posture(&self) -> SpatialCompatibilityPosture {
        self.compatibility_posture
    }

    pub const fn freshness_requirement_posture(&self) -> SpatialFreshnessRequirementPosture {
        self.freshness_requirement_posture
    }

    pub const fn ordering_noise_posture(&self) -> SpatialOrderingNoisePosture {
        self.ordering_noise_posture
    }

    pub const fn rendered_output_comparison_posture(
        &self,
    ) -> SpatialRenderedOutputComparisonPosture {
        self.rendered_output_comparison_posture
    }

    pub fn family_digest(&self) -> &str {
        &self.family_digest
    }
}
