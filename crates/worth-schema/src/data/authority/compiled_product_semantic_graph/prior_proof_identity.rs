use serde::{Deserialize, Serialize};

use super::error::{
    CompiledProductSemanticGraphVocabularyError, CompiledProductSemanticGraphVocabularyErrorKind,
};
use super::identity_digest::compiled_product_semantic_graph_identity_digest;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum CompiledProductPriorProofRole {
    ValidityPreconditionOnly,
    ProductShapingBasis,
    EquivalenceDimension,
    ReuseDenialWitnessOnly,
}

impl CompiledProductPriorProofRole {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ValidityPreconditionOnly => "validity-precondition-only",
            Self::ProductShapingBasis => "product-shaping-basis",
            Self::EquivalenceDimension => "equivalence-dimension",
            Self::ReuseDenialWitnessOnly => "reuse-denial-witness-only",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CompiledProductPriorProofIdentity {
    proof_digest: String,
    role: CompiledProductPriorProofRole,
    identity_digest: String,
}

impl CompiledProductPriorProofIdentity {
    pub fn proof_digest(&self) -> &str {
        &self.proof_digest
    }

    pub const fn role(&self) -> CompiledProductPriorProofRole {
        self.role
    }

    pub fn identity_digest(&self) -> &str {
        &self.identity_digest
    }
}

pub fn admit_compiled_product_prior_proof_identity(
    proof_digest: impl Into<String>,
    role: CompiledProductPriorProofRole,
) -> Result<CompiledProductPriorProofIdentity, CompiledProductSemanticGraphVocabularyError> {
    let proof_digest = proof_digest.into();
    if proof_digest.trim().is_empty() {
        return Err(CompiledProductSemanticGraphVocabularyError::new(
            CompiledProductSemanticGraphVocabularyErrorKind::EmptyPriorProofDigest,
            "compiled-product prior-proof identity requires a non-empty proof digest",
        ));
    }

    let identity_digest = compiled_product_semantic_graph_identity_digest(
        "worth-schema:compiled-product-prior-proof-identity:v1",
        &[
            format!("role:{}", role.as_str()),
            format!("proof:{proof_digest}"),
        ],
    );
    Ok(CompiledProductPriorProofIdentity {
        proof_digest,
        role,
        identity_digest,
    })
}
