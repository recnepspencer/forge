use super::{
    workspace_error, DeclarativeLiveQueryRequest, ForgeQueryRuntimeError, QuerySchemaView,
};
use crate::declarative_live::validate_declared_traversal_contract;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryWorkspaceLiveViewDeclaration {
    request: DeclarativeLiveQueryRequest,
    schema_view: QuerySchemaView,
}

impl ForgeQueryWorkspaceLiveViewDeclaration {
    pub fn try_from_request(
        request: DeclarativeLiveQueryRequest,
        schema_view: QuerySchemaView,
    ) -> Result<Self, ForgeQueryRuntimeError> {
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
