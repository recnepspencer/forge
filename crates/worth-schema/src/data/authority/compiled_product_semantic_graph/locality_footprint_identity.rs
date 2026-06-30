use serde::{Deserialize, Serialize};

use super::error::{
    CompiledProductSemanticGraphVocabularyError, CompiledProductSemanticGraphVocabularyErrorKind,
};
use super::identity_digest::compiled_product_semantic_graph_identity_digest;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CompiledProductLocalityFootprintIdentity {
    locality_kind: String,
    locality_digest: String,
    identity_digest: String,
}

impl CompiledProductLocalityFootprintIdentity {
    pub fn touched_closure(
        locality_digest: impl Into<String>,
    ) -> Result<Self, CompiledProductSemanticGraphVocabularyError> {
        admit_locality_footprint_identity("touched-closure", locality_digest)
    }

    pub fn invalidation_closure(
        locality_digest: impl Into<String>,
    ) -> Result<Self, CompiledProductSemanticGraphVocabularyError> {
        admit_locality_footprint_identity("invalidation-closure", locality_digest)
    }

    pub fn evidence_neighborhood(
        locality_digest: impl Into<String>,
    ) -> Result<Self, CompiledProductSemanticGraphVocabularyError> {
        admit_locality_footprint_identity("evidence-neighborhood", locality_digest)
    }

    pub fn grouped_batch_footprint(
        locality_digest: impl Into<String>,
    ) -> Result<Self, CompiledProductSemanticGraphVocabularyError> {
        admit_locality_footprint_identity("grouped-batch-footprint", locality_digest)
    }

    pub fn materialization_target_footprint(
        locality_digest: impl Into<String>,
    ) -> Result<Self, CompiledProductSemanticGraphVocabularyError> {
        admit_locality_footprint_identity("materialization-target-footprint", locality_digest)
    }

    pub fn locality_kind(&self) -> &str {
        &self.locality_kind
    }

    pub fn locality_digest(&self) -> &str {
        &self.locality_digest
    }

    pub fn identity_digest(&self) -> &str {
        &self.identity_digest
    }
}

pub fn admit_locality_footprint_identity(
    locality_kind: impl Into<String>,
    locality_digest: impl Into<String>,
) -> Result<CompiledProductLocalityFootprintIdentity, CompiledProductSemanticGraphVocabularyError> {
    let locality_kind = locality_kind.into();
    let locality_digest = locality_digest.into();
    require_non_blank(
        &locality_kind,
        CompiledProductSemanticGraphVocabularyErrorKind::EmptyLocalityKind,
        "compiled-product locality identity requires a named locality kind",
    )?;
    require_non_blank(
        &locality_digest,
        CompiledProductSemanticGraphVocabularyErrorKind::EmptyLocalityDigest,
        "compiled-product locality identity requires a non-empty locality digest",
    )?;

    let identity_digest = compiled_product_semantic_graph_identity_digest(
        "worth-schema:compiled-product-locality-footprint-identity:v1",
        &[
            format!("kind:{locality_kind}"),
            format!("digest:{locality_digest}"),
        ],
    );
    Ok(CompiledProductLocalityFootprintIdentity {
        locality_kind,
        locality_digest,
        identity_digest,
    })
}

fn require_non_blank(
    value: &str,
    kind: CompiledProductSemanticGraphVocabularyErrorKind,
    detail: &'static str,
) -> Result<(), CompiledProductSemanticGraphVocabularyError> {
    if value.trim().is_empty() {
        Err(CompiledProductSemanticGraphVocabularyError::new(
            kind, detail,
        ))
    } else {
        Ok(())
    }
}
