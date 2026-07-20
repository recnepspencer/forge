use crate::WorthServerProductOperationDenial;

use super::{
    WorthServerDurableProductMutationCompletion, WorthServerDurableProductMutationRecoveryHandle,
};

#[derive(Clone, Debug)]
pub enum WorthServerDurableProductMutationConclusion {
    Committed(WorthServerDurableProductMutationCompletion),
    PreviouslyCommitted(WorthServerDurableProductMutationCompletion),
    StaleBasis { observed_basis_digest: String },
    IdempotencyConflict { bound_request_digest: String },
    Rejected(WorthServerProductOperationDenial),
    InvalidResultArtifact(crate::WorthServerProductResultArtifactError),
    Indeterminate(WorthServerDurableProductMutationRecoveryHandle),
    Failed { reason_key: String, detail: String },
}

impl WorthServerDurableProductMutationConclusion {
    pub fn failed(reason_key: impl Into<String>, detail: impl Into<String>) -> Self {
        Self::Failed {
            reason_key: reason_key.into(),
            detail: detail.into(),
        }
    }
}
