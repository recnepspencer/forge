use super::{
    workspace_error, DeclarativeLiveQueryRequest, QuerySchemaView, WorthQueryRuntimeError,
};
use crate::declarative_live::validate_declared_traversal_contract;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryWorkspaceLiveViewDeclaration {
    request: DeclarativeLiveQueryRequest,
    schema_view: QuerySchemaView,
}

impl WorthQueryWorkspaceLiveViewDeclaration {
    pub fn try_from_request(
        request: DeclarativeLiveQueryRequest,
        schema_view: QuerySchemaView,
    ) -> Result<Self, WorthQueryRuntimeError> {
        validate_declared_traversal_contract(&request, &schema_view)
            .map_err(|error| workspace_error(format!("{error:?}")))?;
        Ok(Self {
            request,
            schema_view,
        })
    }

    pub fn request(&self) -> &DeclarativeLiveQueryRequest {
        &self.request
    }

    pub fn schema_view(&self) -> &QuerySchemaView {
        &self.schema_view
    }

    pub(in crate::runtime) fn into_parts(self) -> (DeclarativeLiveQueryRequest, QuerySchemaView) {
        (self.request, self.schema_view)
    }

    pub(in crate::runtime) fn from_request(
        request: DeclarativeLiveQueryRequest,
        schema_view: QuerySchemaView,
    ) -> Self {
        Self {
            request,
            schema_view,
        }
    }
}
