use serde::{Deserialize, Serialize};

use super::compiled_product_identity::CompiledProductIdentity;
use super::equivalence_policy_identity::CompiledProductEquivalencePolicyIdentity;
use super::error::{
    CompiledProductSemanticGraphVocabularyError, CompiledProductSemanticGraphVocabularyErrorKind,
};
use super::identity_digest::compiled_product_semantic_graph_identity_digest;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CompiledProductReuseDecisionIdentity {
    compiled_product_identity_digest: String,
    equivalence_policy_identity_digest: String,
    reuse_posture: String,
    identity_digest: String,
}

impl CompiledProductReuseDecisionIdentity {
    pub fn compiled_product_identity_digest(&self) -> &str {
        &self.compiled_product_identity_digest
    }

    pub fn equivalence_policy_identity_digest(&self) -> &str {
        &self.equivalence_policy_identity_digest
    }

    pub fn reuse_posture(&self) -> &str {
        &self.reuse_posture
    }

    pub fn identity_digest(&self) -> &str {
        &self.identity_digest
    }
}

pub fn admit_compiled_product_reuse_decision_identity(
    compiled_product_identity: &CompiledProductIdentity,
    equivalence_policy_identity: &CompiledProductEquivalencePolicyIdentity,
    reuse_posture: impl Into<String>,
) -> Result<CompiledProductReuseDecisionIdentity, CompiledProductSemanticGraphVocabularyError> {
    let reuse_posture = reuse_posture.into();
    if reuse_posture.trim().is_empty() {
        return Err(CompiledProductSemanticGraphVocabularyError::new(
            CompiledProductSemanticGraphVocabularyErrorKind::EmptyReusePosture,
            "compiled-product reuse decision identity requires a non-empty reuse posture",
        ));
    }

    let identity_digest = compiled_product_semantic_graph_identity_digest(
        "worth-schema:compiled-product-reuse-decision-identity:v1",
        &[
            format!("product:{}", compiled_product_identity.identity_digest()),
            format!("policy:{}", equivalence_policy_identity.identity_digest()),
            format!("posture:{reuse_posture}"),
        ],
    );
    Ok(CompiledProductReuseDecisionIdentity {
        compiled_product_identity_digest: compiled_product_identity.identity_digest().to_string(),
        equivalence_policy_identity_digest: equivalence_policy_identity
            .identity_digest()
            .to_string(),
        reuse_posture,
        identity_digest,
    })
}
