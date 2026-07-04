use serde::{Deserialize, Serialize};

use super::error::{
    CompiledProductSemanticGraphVocabularyError, CompiledProductSemanticGraphVocabularyErrorKind,
};
use super::identity_digest::compiled_product_semantic_graph_identity_digest;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CompiledProductStageIdentity {
    stage_digest: String,
    identity_digest: String,
}

impl CompiledProductStageIdentity {
    pub fn stage_digest(&self) -> &str {
        &self.stage_digest
    }

    pub fn identity_digest(&self) -> &str {
        &self.identity_digest
    }
}

pub fn admit_compiled_product_stage_identity(
    stage_digest: impl Into<String>,
) -> Result<CompiledProductStageIdentity, CompiledProductSemanticGraphVocabularyError> {
    let stage_digest = stage_digest.into();
    if stage_digest.trim().is_empty() {
        return Err(CompiledProductSemanticGraphVocabularyError::new(
            CompiledProductSemanticGraphVocabularyErrorKind::EmptyStageDigest,
            "compiled-product stage identity requires a non-empty stage digest",
        ));
    }

    let identity_digest = compiled_product_semantic_graph_identity_digest(
        "worth-schema:compiled-product-stage-identity:v1",
        &[format!("stage:{stage_digest}")],
    );
    Ok(CompiledProductStageIdentity {
        stage_digest,
        identity_digest,
    })
}
