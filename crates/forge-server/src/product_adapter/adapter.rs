use std::sync::Arc;

use super::{
    ForgeServerProductOperationDenial, ForgeServerProductOperationDenialCode,
    ForgeServerProductOperationOutcome, ForgeServerProductOperationPayload,
    ForgeServerProductOperationSuccess, ForgeServerScheduledProductOperation,
};

pub trait ForgeServerProductApplicationAdapter: Send + Sync + 'static {
    fn execute(
        &self,
        operation: &ForgeServerScheduledProductOperation,
    ) -> Result<ForgeServerProductOperationSuccess, ForgeServerProductAdapterExecutionError>;
}

pub trait ForgeServerProductPayloadSchemaValidator: Send + Sync + 'static {
    fn validate(
        &self,
        payload: &ForgeServerProductOperationPayload,
    ) -> Result<(), ForgeServerProductOperationDenial>;
}

pub trait ForgeServerProductOperationErrorMap: Send + Sync + 'static {
    fn map_error(
        &self,
        error: ForgeServerProductAdapterExecutionError,
    ) -> ForgeServerProductOperationOutcome;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeServerProductAdapterExecutionError {
    Denied(ForgeServerProductOperationDenial),
    Failed { reason_key: String, detail: String },
}

impl ForgeServerProductAdapterExecutionError {
    pub fn denied(denial: ForgeServerProductOperationDenial) -> Self {
        Self::Denied(denial)
    }

    pub fn failed(reason_key: impl Into<String>, detail: impl Into<String>) -> Self {
        Self::Failed {
            reason_key: reason_key.into(),
            detail: detail.into(),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub(crate) struct ForgeServerDefaultProductOperationErrorMap;

impl ForgeServerProductOperationErrorMap for ForgeServerDefaultProductOperationErrorMap {
    fn map_error(
        &self,
        error: ForgeServerProductAdapterExecutionError,
    ) -> ForgeServerProductOperationOutcome {
        match error {
            ForgeServerProductAdapterExecutionError::Denied(denial) => {
                ForgeServerProductOperationOutcome::Denied(
                    denial.with_code(ForgeServerProductOperationDenialCode::ProductSemantic),
                )
            }
            ForgeServerProductAdapterExecutionError::Failed { reason_key, detail } => {
                ForgeServerProductOperationOutcome::failed(reason_key, detail)
            }
        }
    }
}

fn default_error_map() -> Arc<dyn ForgeServerProductOperationErrorMap> {
    Arc::new(ForgeServerDefaultProductOperationErrorMap)
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ForgeServerProductOperationErrorMaps;

impl ForgeServerProductOperationErrorMaps {
    pub fn passthrough() -> Arc<dyn ForgeServerProductOperationErrorMap> {
        default_error_map()
    }
}
