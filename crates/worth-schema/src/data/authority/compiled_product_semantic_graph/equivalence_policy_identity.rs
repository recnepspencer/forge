use serde::{Deserialize, Serialize};

use super::error::{
    CompiledProductSemanticGraphVocabularyError, CompiledProductSemanticGraphVocabularyErrorKind,
};
use super::identity_digest::compiled_product_semantic_graph_identity_digest;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CompiledProductEquivalencePolicyIdentity {
    policy_name: String,
    compared_dimensions: Vec<String>,
    identity_digest: String,
}

impl CompiledProductEquivalencePolicyIdentity {
    pub fn policy_name(&self) -> &str {
        &self.policy_name
    }

    pub fn compared_dimensions(&self) -> &[String] {
        &self.compared_dimensions
    }

    pub fn identity_digest(&self) -> &str {
        &self.identity_digest
    }
}

pub fn admit_compiled_product_equivalence_policy_identity<I, S>(
    policy_name: impl Into<String>,
    compared_dimensions: I,
) -> Result<CompiledProductEquivalencePolicyIdentity, CompiledProductSemanticGraphVocabularyError>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let policy_name = policy_name.into();
    if policy_name.trim().is_empty() {
        return Err(CompiledProductSemanticGraphVocabularyError::new(
            CompiledProductSemanticGraphVocabularyErrorKind::EmptyEquivalencePolicyName,
            "compiled-product equivalence policy identity requires a named policy",
        ));
    }

    let mut compared_dimensions = compared_dimensions
        .into_iter()
        .map(Into::into)
        .collect::<Vec<_>>();
    if compared_dimensions
        .iter()
        .any(|dimension| dimension.trim().is_empty())
    {
        return Err(CompiledProductSemanticGraphVocabularyError::new(
            CompiledProductSemanticGraphVocabularyErrorKind::EmptyEquivalenceDimension,
            "compiled-product equivalence policy identity requires non-empty compared dimensions",
        ));
    }
    compared_dimensions.sort();
    compared_dimensions.dedup();

    let mut parts = vec![format!("policy:{policy_name}")];
    parts.extend(
        compared_dimensions
            .iter()
            .map(|dimension| format!("dimension:{dimension}")),
    );
    let identity_digest = compiled_product_semantic_graph_identity_digest(
        "worth-schema:compiled-product-equivalence-policy-identity:v1",
        &parts,
    );
    Ok(CompiledProductEquivalencePolicyIdentity {
        policy_name,
        compared_dimensions,
        identity_digest,
    })
}
