use crate::canonicalization::CanonicalResultShapeArtifact;
use crate::identity::{CanonicalResultShapeDigest, SchemaBasisDigest, ValidatedResultShapeDigest};

use super::ValidatedResultShapeBinding;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidatedResultShapeArtifact {
    digest: ValidatedResultShapeDigest,
    canonical_result_shape_digest: CanonicalResultShapeDigest,
    schema_basis: SchemaBasisDigest,
    bindings: Vec<ValidatedResultShapeBinding>,
}

impl ValidatedResultShapeArtifact {
    pub fn digest(&self) -> &ValidatedResultShapeDigest {
        &self.digest
    }

    pub fn canonical_result_shape_digest(&self) -> &CanonicalResultShapeDigest {
        &self.canonical_result_shape_digest
    }

    pub fn schema_basis(&self) -> &SchemaBasisDigest {
        &self.schema_basis
    }

    pub fn bindings(&self) -> &[ValidatedResultShapeBinding] {
        &self.bindings
    }

    pub fn validated_result_shape_identity(&self) -> &ValidatedResultShapeDigest {
        &self.digest
    }
}

pub fn build_validated_result_shape_artifact(
    canonical_result_shape: &CanonicalResultShapeArtifact,
    schema_basis: &SchemaBasisDigest,
    bindings: Vec<ValidatedResultShapeBinding>,
) -> ValidatedResultShapeArtifact {
    let mut parts = vec![
        format!(
            "canonical_result_shape:{}",
            canonical_result_shape.digest().as_str()
        ),
        format!("schema_basis:{}", schema_basis.as_str()),
    ];
    parts.extend(
        bindings
            .iter()
            .map(ValidatedResultShapeBinding::digest_part),
    );

    ValidatedResultShapeArtifact {
        digest: ValidatedResultShapeDigest::from_parts(&parts),
        canonical_result_shape_digest: canonical_result_shape.digest().clone(),
        schema_basis: schema_basis.clone(),
        bindings,
    }
}
