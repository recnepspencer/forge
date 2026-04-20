use crate::authoring::{QueryFamily, ResultShapeFamily};

use super::family::ViewShapeFamily;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ViewShapeCompatibilityMatrixArtifact {
    query_family: QueryFamily,
    result_shape_family: ResultShapeFamily,
    requested_family: ViewShapeFamily,
    admitted: bool,
}

impl ViewShapeCompatibilityMatrixArtifact {
    pub(crate) fn pending(
        query_family: QueryFamily,
        result_shape_family: ResultShapeFamily,
        requested_family: ViewShapeFamily,
    ) -> Self {
        Self {
            query_family,
            result_shape_family,
            requested_family,
            admitted: false,
        }
    }

    pub(crate) fn mark_admitted(self) -> Self {
        Self {
            admitted: true,
            ..self
        }
    }

    pub fn query_family(&self) -> &QueryFamily {
        &self.query_family
    }

    pub fn result_shape_family(&self) -> &ResultShapeFamily {
        &self.result_shape_family
    }

    pub fn requested_family(&self) -> ViewShapeFamily {
        self.requested_family
    }

    pub fn admitted(&self) -> bool {
        self.admitted
    }
}
