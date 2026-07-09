use crate::WorthServerOperationAuthorityKind;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthServerOperationExecutionStrategy {
    SharedReadExecution,
    DeterministicSubmission,
    ProductAdapterExecution,
    SessionCoordination,
    BinaryTransfer,
    LeaseCoordination,
}

impl WorthServerOperationExecutionStrategy {
    pub(crate) fn from_authority_kind(authority_kind: WorthServerOperationAuthorityKind) -> Self {
        match authority_kind {
            WorthServerOperationAuthorityKind::SharedReadOnly => Self::SharedReadExecution,
            WorthServerOperationAuthorityKind::DeterministicSubmission => {
                Self::DeterministicSubmission
            }
            WorthServerOperationAuthorityKind::ProductDraftMutation => {
                Self::ProductAdapterExecution
            }
            WorthServerOperationAuthorityKind::ProductSessionCoordination => {
                Self::SessionCoordination
            }
            WorthServerOperationAuthorityKind::BinaryStreaming => Self::BinaryTransfer,
            WorthServerOperationAuthorityKind::DiagnosticsOnly => Self::SharedReadExecution,
            WorthServerOperationAuthorityKind::LeaseCoordination => Self::LeaseCoordination,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::SharedReadExecution => "shared-read-execution",
            Self::DeterministicSubmission => "deterministic-submission",
            Self::ProductAdapterExecution => "product-adapter-execution",
            Self::SessionCoordination => "session-coordination",
            Self::BinaryTransfer => "binary-transfer",
            Self::LeaseCoordination => "lease-coordination",
        }
    }
}
