use serde::{Deserialize, Serialize};

use super::authority_instance_coordinate::CompiledProductAuthorityInstanceCoordinate;
use super::error::{
    CompiledProductSemanticGraphVocabularyError, CompiledProductSemanticGraphVocabularyErrorKind,
};
use super::identity_digest::compiled_product_semantic_graph_identity_digest;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CompiledProductAuthorityTruthIdentity {
    authority_owner: String,
    authority_surface: String,
    authority_digest: String,
    authority_instance_coordinates: Vec<CompiledProductAuthorityInstanceCoordinate>,
    identity_digest: String,
}

impl CompiledProductAuthorityTruthIdentity {
    pub fn authority_owner(&self) -> &str {
        &self.authority_owner
    }

    pub fn authority_surface(&self) -> &str {
        &self.authority_surface
    }

    pub fn authority_digest(&self) -> &str {
        &self.authority_digest
    }

    pub fn authority_instance_coordinates(&self) -> &[CompiledProductAuthorityInstanceCoordinate] {
        &self.authority_instance_coordinates
    }

    pub fn identity_digest(&self) -> &str {
        &self.identity_digest
    }
}

pub fn admit_compiled_product_authority_truth_identity(
    authority_owner: impl Into<String>,
    authority_digest: impl Into<String>,
    authority_surface: impl Into<String>,
) -> Result<CompiledProductAuthorityTruthIdentity, CompiledProductSemanticGraphVocabularyError> {
    admit_compiled_product_authority_truth_identity_with_coordinates(
        authority_owner,
        authority_digest,
        authority_surface,
        std::iter::empty(),
    )
}

pub fn admit_compiled_product_authority_truth_identity_with_coordinates<I>(
    authority_owner: impl Into<String>,
    authority_digest: impl Into<String>,
    authority_surface: impl Into<String>,
    authority_instance_coordinates: I,
) -> Result<CompiledProductAuthorityTruthIdentity, CompiledProductSemanticGraphVocabularyError>
where
    I: IntoIterator<Item = CompiledProductAuthorityInstanceCoordinate>,
{
    let authority_owner = authority_owner.into();
    let authority_digest = authority_digest.into();
    let authority_surface = authority_surface.into();
    let mut authority_instance_coordinates = authority_instance_coordinates
        .into_iter()
        .collect::<Vec<_>>();
    authority_instance_coordinates.sort();
    authority_instance_coordinates.dedup();
    require_non_blank(
        &authority_owner,
        CompiledProductSemanticGraphVocabularyErrorKind::EmptyAuthorityOwner,
        "compiled-product authority truth identity requires a named authority owner",
    )?;
    require_non_blank(
        &authority_surface,
        CompiledProductSemanticGraphVocabularyErrorKind::EmptyAuthoritySurface,
        "compiled-product authority truth identity requires a named authority surface",
    )?;
    require_non_blank(
        &authority_digest,
        CompiledProductSemanticGraphVocabularyErrorKind::EmptyAuthorityDigest,
        "compiled-product authority truth identity requires a non-empty authority digest",
    )?;

    let mut parts = vec![
        format!("owner:{authority_owner}"),
        format!("surface:{authority_surface}"),
        format!("authority:{authority_digest}"),
    ];
    parts.extend(authority_instance_coordinates.iter().map(|coordinate| {
        format!(
            "instance:{}:{}",
            coordinate.coordinate_kind(),
            coordinate.coordinate_value()
        )
    }));
    let identity_digest = compiled_product_semantic_graph_identity_digest(
        "worth-schema:compiled-product-authority-truth-identity:v2",
        &parts,
    );
    Ok(CompiledProductAuthorityTruthIdentity {
        authority_owner,
        authority_surface,
        authority_digest,
        authority_instance_coordinates,
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
