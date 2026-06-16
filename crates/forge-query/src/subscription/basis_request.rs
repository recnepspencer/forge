use crate::evidence_identity::ForgeQueryEvidenceIdentity;

use super::declaration::QuerySubscriptionDeclarationArtifact;
use super::evidence_identities::basis_binding_request_identity;
use super::posture::QuerySubscriptionBasisPosture;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum QuerySubscriptionBasisBindingRequestKind {
    CurrentHead,
    BranchHead,
    RuntimeSnapshot,
    PreviewScoped,
    DeniedUnsupportedBasis,
}

impl QuerySubscriptionBasisBindingRequestKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::CurrentHead => "current_head",
            Self::BranchHead => "branch_head",
            Self::RuntimeSnapshot => "runtime_snapshot",
            Self::PreviewScoped => "preview_scoped",
            Self::DeniedUnsupportedBasis => "denied_unsupported_basis",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuerySubscriptionBasisBindingRequest {
    request_kind: QuerySubscriptionBasisBindingRequestKind,
    source_declaration_identity: ForgeQueryEvidenceIdentity,
    evidence_identity: ForgeQueryEvidenceIdentity,
}

impl QuerySubscriptionBasisBindingRequest {
    pub(super) fn from_declaration(declaration: &QuerySubscriptionDeclarationArtifact) -> Self {
        let request_kind = match declaration.basis_posture() {
            QuerySubscriptionBasisPosture::CurrentHead => {
                QuerySubscriptionBasisBindingRequestKind::CurrentHead
            }
            QuerySubscriptionBasisPosture::BranchHead => {
                QuerySubscriptionBasisBindingRequestKind::BranchHead
            }
            QuerySubscriptionBasisPosture::RuntimeHistoricalSnapshot => {
                QuerySubscriptionBasisBindingRequestKind::RuntimeSnapshot
            }
            QuerySubscriptionBasisPosture::PreviewScoped => {
                QuerySubscriptionBasisBindingRequestKind::PreviewScoped
            }
            QuerySubscriptionBasisPosture::DeniedUnsupportedBasis => {
                QuerySubscriptionBasisBindingRequestKind::DeniedUnsupportedBasis
            }
        };
        let source_declaration_identity = declaration.declaration_identity().clone();
        let evidence_identity = basis_binding_request_identity(
            &request_kind,
            &source_declaration_identity,
            declaration.equivalence_identity(),
        );
        Self {
            request_kind,
            source_declaration_identity,
            evidence_identity,
        }
    }

    pub fn request_kind(&self) -> &QuerySubscriptionBasisBindingRequestKind {
        &self.request_kind
    }

    pub fn source_declaration_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.source_declaration_identity
    }

    pub fn evidence_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.evidence_identity
    }
}
