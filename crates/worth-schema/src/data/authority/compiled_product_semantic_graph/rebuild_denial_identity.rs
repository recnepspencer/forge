use serde::{Deserialize, Serialize};

use super::compiled_product_identity::CompiledProductIdentity;
use super::error::{
    CompiledProductSemanticGraphVocabularyError, CompiledProductSemanticGraphVocabularyErrorKind,
};
use super::identity_digest::compiled_product_semantic_graph_identity_digest;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CompiledProductRebuildDenialIdentity {
    compiled_product_identity_digest: String,
    denial_reason: String,
    identity_digest: String,
}

impl CompiledProductRebuildDenialIdentity {
    pub fn compiled_product_identity_digest(&self) -> &str {
        &self.compiled_product_identity_digest
    }

    pub fn denial_reason(&self) -> &str {
        &self.denial_reason
    }

    pub fn identity_digest(&self) -> &str {
        &self.identity_digest
    }
}

pub fn admit_compiled_product_rebuild_denial_identity(
    compiled_product_identity: &CompiledProductIdentity,
    denial_reason: impl Into<String>,
) -> Result<CompiledProductRebuildDenialIdentity, CompiledProductSemanticGraphVocabularyError> {
    let denial_reason = denial_reason.into();
    if denial_reason.trim().is_empty() {
        return Err(CompiledProductSemanticGraphVocabularyError::new(
            CompiledProductSemanticGraphVocabularyErrorKind::EmptyRebuildReason,
            "compiled-product rebuild denial identity requires a non-empty denial reason",
        ));
    }

    let identity_digest = compiled_product_semantic_graph_identity_digest(
        "worth-schema:compiled-product-rebuild-denial-identity:v1",
        &[
            format!("product:{}", compiled_product_identity.identity_digest()),
            format!("reason:{denial_reason}"),
        ],
    );
    Ok(CompiledProductRebuildDenialIdentity {
        compiled_product_identity_digest: compiled_product_identity.identity_digest().to_string(),
        denial_reason,
        identity_digest,
    })
}
