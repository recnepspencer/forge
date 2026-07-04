use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::derived_invalidation_compiled_product_admission::TopologyCompiledProductAdmittedInput;

use super::declaration::TopologySelectedEquivalenceFamilyDeclaration;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopologySelectedEquivalenceBasisIdentity {
    digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopologySelectedCompatibilityBasisIdentity {
    digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopologySelectedReuseBasisIdentity {
    digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopologySelectedFutureProofSeedIdentity {
    digest: String,
}

impl TopologySelectedEquivalenceBasisIdentity {
    pub fn identity_digest(&self) -> &str {
        &self.digest
    }
}

impl TopologySelectedCompatibilityBasisIdentity {
    pub fn identity_digest(&self) -> &str {
        &self.digest
    }

    pub(crate) fn from_identity_digest_for_certification(digest: impl Into<String>) -> Self {
        Self {
            digest: digest.into(),
        }
    }
}

impl TopologySelectedReuseBasisIdentity {
    pub fn identity_digest(&self) -> &str {
        &self.digest
    }

    pub(crate) fn from_identity_digest_for_certification(digest: impl Into<String>) -> Self {
        Self {
            digest: digest.into(),
        }
    }
}

impl TopologySelectedFutureProofSeedIdentity {
    pub fn identity_digest(&self) -> &str {
        &self.digest
    }
}

pub(super) fn admit_topology_selected_basis_identities(
    declaration: &TopologySelectedEquivalenceFamilyDeclaration,
    admitted_input: &TopologyCompiledProductAdmittedInput,
    equivalence_policy_identity_digest: &str,
) -> (
    TopologySelectedEquivalenceBasisIdentity,
    TopologySelectedCompatibilityBasisIdentity,
    TopologySelectedReuseBasisIdentity,
    TopologySelectedFutureProofSeedIdentity,
) {
    let family_input = admitted_input.family_admitted_input();
    let prior_proof_part = match admitted_input.prior_proof_basis() {
        crate::derived_invalidation_compiled_product_admission::TopologyCompiledProductPriorProofBasis::NotRequired => {
            "prior-proof:not-required".to_string()
        }
        crate::derived_invalidation_compiled_product_admission::TopologyCompiledProductPriorProofBasis::SelectedPlan {
            selected_plan_digest,
            touched_closure_digest,
        } => format!("prior-proof:selected-plan:{selected_plan_digest}:{touched_closure_digest}"),
    };
    let equivalence_digest = digest_for_parts(&[
        format!("family:{}", declaration.identity().as_str()),
        format!(
            "compiled-family:{}",
            declaration.compiled_product_family_identity().as_str()
        ),
        format!("equivalence-policy:{equivalence_policy_identity_digest}"),
        format!("authority-truth:{}", family_input.truth_basis_digest_hex()),
        format!("locality:{}", family_input.locality_digest()),
        format!("touched-aspects:{}", family_input.touched_aspect_count()),
        prior_proof_part.clone(),
    ]);
    let compatibility_digest = digest_for_parts(&[
        format!("family:{}", declaration.identity().as_str()),
        "compatibility:distinct-from-equivalence".to_string(),
        format!("equivalence-basis:{equivalence_digest}"),
    ]);
    let reuse_digest = digest_for_parts(&[
        format!("family:{}", declaration.identity().as_str()),
        format!("equivalence-basis:{equivalence_digest}"),
        format!(
            "freshness:{:?}",
            declaration.freshness_requirement_posture()
        ),
        format!("ordering:{:?}", declaration.ordering_noise_posture()),
    ]);
    let future_seed_digest = digest_for_parts(&[
        format!("family:{}", declaration.identity().as_str()),
        format!("reuse-basis:{reuse_digest}"),
        format!(
            "rendered-output:{:?}",
            declaration.rendered_output_comparison_posture()
        ),
        format!("compatibility-basis:{compatibility_digest}"),
    ]);
    (
        TopologySelectedEquivalenceBasisIdentity {
            digest: equivalence_digest,
        },
        TopologySelectedCompatibilityBasisIdentity {
            digest: compatibility_digest,
        },
        TopologySelectedReuseBasisIdentity {
            digest: reuse_digest,
        },
        TopologySelectedFutureProofSeedIdentity {
            digest: future_seed_digest,
        },
    )
}

fn digest_for_parts(parts: &[String]) -> String {
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, parts)
}
