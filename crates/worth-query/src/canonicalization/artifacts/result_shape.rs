use crate::authoring::ResultShapeFamily;
use crate::identity::{CanonicalEquivalence, CanonicalResultShapeDigest};

use super::entries::CanonicalResultField;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanonicalResultShapeArtifact {
    pub(crate) digest: CanonicalResultShapeDigest,
    pub(crate) family: ResultShapeFamily,
    pub(crate) fields: Vec<CanonicalResultField>,
}

impl CanonicalResultShapeArtifact {
    pub fn digest(&self) -> &CanonicalResultShapeDigest {
        &self.digest
    }

    pub fn result_shape_identity(&self) -> crate::evidence_identity::WorthQueryEvidenceIdentity {
        self.digest.evidence_identity()
    }

    pub fn family(&self) -> &ResultShapeFamily {
        &self.family
    }

    pub fn fields(&self) -> &[CanonicalResultField] {
        &self.fields
    }

    pub fn equivalence_to(&self, other: &Self) -> CanonicalEquivalence {
        if self.family == other.family && self.fields == other.fields && self.digest == other.digest
        {
            CanonicalEquivalence::Equivalent
        } else {
            CanonicalEquivalence::Distinct
        }
    }

    #[cfg(test)]
    pub(crate) fn reverse_fields_for_test(&mut self) {
        self.fields.reverse();
    }

    #[cfg(test)]
    pub(crate) fn corrupt_digest_for_test(&mut self, marker: &str) {
        self.digest = CanonicalResultShapeDigest::from_parts(&[marker.to_string()]);
    }

    #[cfg(test)]
    pub(crate) fn rewrite_first_field_for_test(
        &mut self,
        source_aspect: impl Into<String>,
        source_field: impl Into<String>,
        delivered_name: impl Into<String>,
    ) {
        use crate::authoring::{AspectFieldKey, DeliveredFieldName};

        if let Some(field) = self.fields.first_mut() {
            field.source = AspectFieldKey::from_authoring_parts(source_aspect, source_field)
                .expect("test rewrite must keep non-empty source field key");
            field.delivered_name = DeliveredFieldName::new(delivered_name)
                .expect("test rewrite must keep non-empty delivered name");
        }
    }
}
