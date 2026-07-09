use crate::authoring::{QueryFamily, ResultShapeFamily};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AuthoredBundleError {
    QueryShapeFamilyMismatch {
        query_family: QueryFamily,
        result_shape_family: ResultShapeFamily,
    },
    UnprojectedShapeField {
        source_aspect: String,
        source_field: String,
        delivered_name: String,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AuthoredBundleFailureClass {
    FamilyMismatch,
    ProjectionShapeMismatch,
}

impl AuthoredBundleError {
    pub fn failure_class(&self) -> AuthoredBundleFailureClass {
        match self {
            Self::QueryShapeFamilyMismatch { .. } => AuthoredBundleFailureClass::FamilyMismatch,
            Self::UnprojectedShapeField { .. } => {
                AuthoredBundleFailureClass::ProjectionShapeMismatch
            }
        }
    }
}
