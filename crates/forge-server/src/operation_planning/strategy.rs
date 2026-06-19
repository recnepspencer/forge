use crate::ForgeServerOperationAuthorityKind;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeServerOperationExecutionStrategy {
    SharedReadExecution,
    DeterministicSubmission,
    ProductAdapterExecution,
    SessionCoordination,
    BinaryTransfer,
    LeaseCoordination,
}

impl ForgeServerOperationExecutionStrategy {
    pub(crate) fn from_authority_kind(authority_kind: ForgeServerOperationAuthorityKind) -> Self {
        match authority_kind {
            ForgeServerOperationAuthorityKind::SharedReadOnly => Self::SharedReadExecution,
            ForgeServerOperationAuthorityKind::DeterministicSubmission => {
                Self::DeterministicSubmission
            }
            ForgeServerOperationAuthorityKind::ProductDraftMutation => {
                Self::ProductAdapterExecution
            }
            ForgeServerOperationAuthorityKind::ProductSessionCoordination => {
                Self::SessionCoordination
            }
            ForgeServerOperationAuthorityKind::BinaryStreaming => Self::BinaryTransfer,
            ForgeServerOperationAuthorityKind::DiagnosticsOnly => Self::SharedReadExecution,
            ForgeServerOperationAuthorityKind::LeaseCoordination => Self::LeaseCoordination,
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
