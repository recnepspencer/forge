use std::sync::Arc;

use super::{
    WorthServerProductOperationDenial, WorthServerProductOperationDenialCode,
    WorthServerProductOperationOutcome, WorthServerProductOperationPayload,
    WorthServerProductOperationSuccess, WorthServerScheduledProductOperation,
};

pub trait WorthServerProductApplicationAdapter: Send + Sync + 'static {
    fn execute(
        &self,
        operation: &WorthServerScheduledProductOperation,
    ) -> Result<WorthServerProductOperationSuccess, WorthServerProductAdapterExecutionError>;
}

pub trait WorthServerProductPayloadSchemaValidator: Send + Sync + 'static {
    fn validate(
        &self,
        payload: &WorthServerProductOperationPayload,
    ) -> Result<(), WorthServerProductOperationDenial>;
}

pub trait WorthServerProductOperationErrorMap: Send + Sync + 'static {
    fn map_error(
        &self,
        error: WorthServerProductAdapterExecutionError,
    ) -> WorthServerProductOperationOutcome;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthServerProductAdapterExecutionError {
    Denied(WorthServerProductOperationDenial),
    InvalidResultArtifact(crate::WorthServerProductResultArtifactError),
    Failed { reason_key: String, detail: String },
}

impl WorthServerProductAdapterExecutionError {
    pub fn denied(denial: WorthServerProductOperationDenial) -> Self {
        Self::Denied(denial)
    }

    pub fn failed(reason_key: impl Into<String>, detail: impl Into<String>) -> Self {
        Self::Failed {
            reason_key: reason_key.into(),
            detail: detail.into(),
        }
    }

    pub fn invalid_result_artifact(error: crate::WorthServerProductResultArtifactError) -> Self {
        Self::InvalidResultArtifact(error)
    }
}

#[derive(Clone, Debug, Default)]
pub(crate) struct WorthServerDefaultProductOperationErrorMap;

impl WorthServerProductOperationErrorMap for WorthServerDefaultProductOperationErrorMap {
    fn map_error(
        &self,
        error: WorthServerProductAdapterExecutionError,
    ) -> WorthServerProductOperationOutcome {
        match error {
            WorthServerProductAdapterExecutionError::Denied(denial) => {
                WorthServerProductOperationOutcome::Denied(
                    denial.with_code(WorthServerProductOperationDenialCode::ProductSemantic),
                )
            }
            WorthServerProductAdapterExecutionError::InvalidResultArtifact(error) => {
                WorthServerProductOperationOutcome::failed(
                    "invalid_result_artifact",
                    error.detail(),
                )
            }
            WorthServerProductAdapterExecutionError::Failed { reason_key, detail } => {
                WorthServerProductOperationOutcome::failed(reason_key, detail)
            }
        }
    }
}

fn default_error_map() -> Arc<dyn WorthServerProductOperationErrorMap> {
    Arc::new(WorthServerDefaultProductOperationErrorMap)
}

#[derive(Clone, Copy, Debug, Default)]
pub struct WorthServerProductOperationErrorMaps;

impl WorthServerProductOperationErrorMaps {
    pub fn passthrough() -> Arc<dyn WorthServerProductOperationErrorMap> {
        default_error_map()
    }
}
