use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::workload_platform::compiled_product_admission::SpatialCompiledProductAdmittedInput;

use super::declaration::SpatialSelectedEquivalenceFamilyDeclaration;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpatialSelectedEquivalenceBasisIdentity {
    digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpatialSelectedCompatibilityBasisIdentity {
    digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpatialSelectedReuseBasisIdentity {
    digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpatialSelectedFutureProofSeedIdentity {
    digest: String,
}

impl SpatialSelectedEquivalenceBasisIdentity {
    pub fn identity_digest(&self) -> &str {
        &self.digest
    }
}

impl SpatialSelectedCompatibilityBasisIdentity {
    pub fn identity_digest(&self) -> &str {
        &self.digest
    }
}

impl SpatialSelectedReuseBasisIdentity {
    pub fn identity_digest(&self) -> &str {
        &self.digest
    }
}

impl SpatialSelectedFutureProofSeedIdentity {
    #[cfg(test)]
    pub fn identity_digest(&self) -> &str {
        &self.digest
    }
}

pub(super) fn admit_spatial_selected_basis_identities(
    declaration: &SpatialSelectedEquivalenceFamilyDeclaration,
    admitted_input: &SpatialCompiledProductAdmittedInput,
    equivalence_policy_identity_digest: &str,
) -> (
    SpatialSelectedEquivalenceBasisIdentity,
    SpatialSelectedCompatibilityBasisIdentity,
    SpatialSelectedReuseBasisIdentity,
    SpatialSelectedFutureProofSeedIdentity,
) {
    let family_input = admitted_input.family_admitted_input();
    let equivalence_digest = digest_for_parts(&[
        format!("family:{}", declaration.identity().as_str()),
        format!(
            "compiled-family:{}",
            declaration.compiled_product_family_identity().as_str()
        ),
        format!("equivalence-policy:{equivalence_policy_identity_digest}"),
        format!("authority:{}", family_input.source_authority_digest()),
        format!("locality:{}", family_input.locality_footprint_digest()),
        format!(
            "evidence-support:{}",
            family_input.evidence_support_digest()
        ),
        format!(
            "prior-proof:{}",
            family_input.prior_proof_digest().unwrap_or("not-required")
        ),
        format!(
            "stage:{}",
            family_input
                .stage_receipt_digest()
                .unwrap_or("not-required")
        ),
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
        format!("compatibility-basis:{compatibility_digest}"),
        format!(
            "rendered-output:{:?}",
            declaration.rendered_output_comparison_posture()
        ),
    ]);
    (
        SpatialSelectedEquivalenceBasisIdentity {
            digest: equivalence_digest,
        },
        SpatialSelectedCompatibilityBasisIdentity {
            digest: compatibility_digest,
        },
        SpatialSelectedReuseBasisIdentity {
            digest: reuse_digest,
        },
        SpatialSelectedFutureProofSeedIdentity {
            digest: future_seed_digest,
        },
    )
}

fn digest_for_parts(parts: &[String]) -> String {
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, parts)
}
