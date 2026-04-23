use crate::identity::hash_parts;

use super::declaration::QuerySubscriptionDeclarationArtifact;
use super::posture::QuerySubscriptionBasisPosture;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum QuerySubscriptionBasisBindingRequestKind {
    CurrentHead,
    BranchHead,
    RuntimeSnapshot,
    PreviewScoped,
}

impl QuerySubscriptionBasisBindingRequestKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::CurrentHead => "current_head",
            Self::BranchHead => "branch_head",
            Self::RuntimeSnapshot => "runtime_snapshot",
            Self::PreviewScoped => "preview_scoped",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuerySubscriptionBasisBindingRequest {
    request_kind: QuerySubscriptionBasisBindingRequestKind,
    source_declaration_digest: String,
    digest: String,
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
                QuerySubscriptionBasisBindingRequestKind::PreviewScoped
            }
        };
        let source_declaration_digest = declaration.declaration_digest().as_str().to_string();
        let digest = hash_parts(&[
            "query_subscription_basis_binding_request_v1".to_string(),
            request_kind.as_str().to_string(),
            format!("source_declaration:{source_declaration_digest}"),
            format!("source_equivalence:{}", declaration.equivalence_digest()),
        ]);
        Self {
            request_kind,
            source_declaration_digest,
            digest,
        }
    }

    pub fn request_kind(&self) -> &QuerySubscriptionBasisBindingRequestKind {
        &self.request_kind
    }

    pub fn source_declaration_digest(&self) -> &str {
        &self.source_declaration_digest
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }
}
