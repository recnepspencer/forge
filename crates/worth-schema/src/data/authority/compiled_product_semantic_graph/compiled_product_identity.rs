use serde::{Deserialize, Serialize};

use super::authority_truth_identity::CompiledProductAuthorityTruthIdentity;
use super::identity_digest::compiled_product_semantic_graph_identity_digest;
use super::locality_footprint_identity::CompiledProductLocalityFootprintIdentity;
use super::prior_proof_identity::CompiledProductPriorProofIdentity;
use super::stage_identity::CompiledProductStageIdentity;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CompiledProductIdentity {
    authority_truth_identity: CompiledProductAuthorityTruthIdentity,
    locality_footprint_identity: CompiledProductLocalityFootprintIdentity,
    prior_proof_identity: Option<CompiledProductPriorProofIdentity>,
    stage_identity: Option<CompiledProductStageIdentity>,
    identity_digest: String,
}

impl CompiledProductIdentity {
    pub fn authority_truth_identity(&self) -> &CompiledProductAuthorityTruthIdentity {
        &self.authority_truth_identity
    }

    pub fn locality_footprint_identity(&self) -> &CompiledProductLocalityFootprintIdentity {
        &self.locality_footprint_identity
    }

    pub fn prior_proof_identity(&self) -> Option<&CompiledProductPriorProofIdentity> {
        self.prior_proof_identity.as_ref()
    }

    pub fn stage_identity(&self) -> Option<&CompiledProductStageIdentity> {
        self.stage_identity.as_ref()
    }

    pub fn identity_digest(&self) -> &str {
        &self.identity_digest
    }
}

pub fn admit_compiled_product_identity(
    authority_truth_identity: CompiledProductAuthorityTruthIdentity,
    locality_footprint_identity: CompiledProductLocalityFootprintIdentity,
    prior_proof_identity: Option<CompiledProductPriorProofIdentity>,
    stage_identity: Option<CompiledProductStageIdentity>,
) -> CompiledProductIdentity {
    let mut parts = vec![
        format!("authority:{}", authority_truth_identity.identity_digest()),
        format!("locality:{}", locality_footprint_identity.identity_digest()),
    ];
    if let Some(prior_proof_identity) = prior_proof_identity.as_ref() {
        parts.push(format!(
            "prior-proof:{}",
            prior_proof_identity.identity_digest()
        ));
    }
    if let Some(stage_identity) = stage_identity.as_ref() {
        parts.push(format!("stage:{}", stage_identity.identity_digest()));
    }

    let identity_digest = compiled_product_semantic_graph_identity_digest(
        "worth-schema:compiled-product-identity:v1",
        &parts,
    );
    CompiledProductIdentity {
        authority_truth_identity,
        locality_footprint_identity,
        prior_proof_identity,
        stage_identity,
        identity_digest,
    }
}
