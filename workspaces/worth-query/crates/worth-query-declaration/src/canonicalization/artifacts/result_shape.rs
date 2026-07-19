use crate::authoring::ResultShapeFamily;
use crate::identity::{CanonicalEquivalence, CanonicalResultShapeDigest};

use super::entries::CanonicalResultField;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanonicalResultShapeArtifact {
    pub digest: CanonicalResultShapeDigest,
    pub family: ResultShapeFamily,
    pub fields: Vec<CanonicalResultField>,
}

impl CanonicalResultShapeArtifact {
    pub fn digest(&self) -> &CanonicalResultShapeDigest {
        &self.digest
    }

    pub fn result_shape_identity(&self) -> &CanonicalResultShapeDigest {
        &self.digest
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
}
