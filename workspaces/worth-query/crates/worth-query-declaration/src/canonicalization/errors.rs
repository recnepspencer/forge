use crate::authoring::{QueryFamily, ResultShapeFamily};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CanonicalizationFailureClass {
    AuthoringAdmission,
    CompatibilityRejection,
    BindingRejection,
    InternalInvariantBreak,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum QueryCanonicalizationError {
    EmptyRootEntityKey,
    EmptyProjectionSelector,
    EmptyOrderingSelector,
    EmptyProjectionSet,
    EmptyTraversalRelation,
    UnsupportedTraversalDepth {
        relation: String,
        depth: u8,
    },
    EmptyResultFieldSource,
    EmptyDeliveredFieldName,
    EmptyResultShapeFieldSet,
    QueryShapeFamilyMismatch {
        query_family: QueryFamily,
        result_shape_family: ResultShapeFamily,
    },
    UnprojectedShapeField {
        source_aspect: String,
        source_field: String,
        delivered_name: String,
    },
    AmbiguousShapeAliasIdentity {
        delivered_name: String,
        first_source_aspect: String,
        first_source_field: String,
        second_source_aspect: String,
        second_source_field: String,
    },
    DuplicateBindingDescriptorConflict {
        slot: String,
    },
    InvalidCanonicalOrderingBasis {
        artifact: &'static str,
    },
    DigestBasisInconsistency {
        artifact: &'static str,
    },
    BundleInvariantViolation {
        message: &'static str,
    },
}

impl QueryCanonicalizationError {
    pub fn failure_class(&self) -> CanonicalizationFailureClass {
        match self {
            Self::EmptyRootEntityKey
            | Self::EmptyProjectionSelector
            | Self::EmptyOrderingSelector
            | Self::EmptyProjectionSet
            | Self::EmptyTraversalRelation
            | Self::UnsupportedTraversalDepth { .. }
            | Self::EmptyResultFieldSource
            | Self::EmptyDeliveredFieldName
            | Self::EmptyResultShapeFieldSet => CanonicalizationFailureClass::AuthoringAdmission,
            Self::QueryShapeFamilyMismatch { .. }
            | Self::UnprojectedShapeField { .. }
            | Self::AmbiguousShapeAliasIdentity { .. } => {
                CanonicalizationFailureClass::CompatibilityRejection
            }
            Self::DuplicateBindingDescriptorConflict { .. } => {
                CanonicalizationFailureClass::BindingRejection
            }
            Self::InvalidCanonicalOrderingBasis { .. }
            | Self::DigestBasisInconsistency { .. }
            | Self::BundleInvariantViolation { .. } => {
                CanonicalizationFailureClass::InternalInvariantBreak
            }
        }
    }
}
