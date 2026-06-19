use super::ForgeServerOperationScope;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeServerOperationAuthorityKind {
    SharedReadOnly,
    DeterministicSubmission,
    ProductDraftMutation,
    ProductSessionCoordination,
    BinaryStreaming,
    DiagnosticsOnly,
    LeaseCoordination,
}

impl ForgeServerOperationAuthorityKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SharedReadOnly => "shared-read-only",
            Self::DeterministicSubmission => "deterministic-submission",
            Self::ProductDraftMutation => "product-draft-mutation",
            Self::ProductSessionCoordination => "product-session-coordination",
            Self::BinaryStreaming => "binary-streaming",
            Self::DiagnosticsOnly => "diagnostics-only",
            Self::LeaseCoordination => "lease-coordination",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerOperationAuthorityFootprint {
    authority_kind: ForgeServerOperationAuthorityKind,
    scope: ForgeServerOperationScope,
    descriptor_digest: String,
    canonical_digest: String,
}

impl ForgeServerOperationAuthorityFootprint {
    pub(crate) fn new(
        authority_kind: ForgeServerOperationAuthorityKind,
        scope: ForgeServerOperationScope,
        descriptor_digest: impl Into<String>,
    ) -> Self {
        let descriptor_digest = descriptor_digest.into();
        let scope_digest = scope.canonical_digest();
        let canonical_digest = format!(
            "forge-server-operation-authority-footprint-v1|kind={}|scope={scope_digest}|descriptor={descriptor_digest}",
            authority_kind.as_str(),
        );
        Self {
            authority_kind,
            scope,
            descriptor_digest,
            canonical_digest,
        }
    }

    pub fn authority_kind(&self) -> ForgeServerOperationAuthorityKind {
        self.authority_kind
    }

    pub fn scope(&self) -> &ForgeServerOperationScope {
        &self.scope
    }

    pub fn descriptor_digest(&self) -> &str {
        &self.descriptor_digest
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}
