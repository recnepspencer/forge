use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::error::BridgeSpeculationError;
use crate::identity::{BridgeIdentity, PreviewSessionDeclarationIdentityTag};

use super::binding::{BridgeSpeculativeBranchBinding, BridgeSpeculativeBranchBindingIdentity};
use super::validation::ValidatedBridgePreviewSessionDeclaration;
use super::{BridgeRequestKind, BridgeSignalBranchIdentity};

pub type BridgePreviewSessionDeclarationIdentity =
    BridgeIdentity<PreviewSessionDeclarationIdentityTag>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgePreviewSessionDeclaration {
    declaration_identity: BridgePreviewSessionDeclarationIdentity,
    request_kind: BridgeRequestKind,
    branch_binding: BridgeSpeculativeBranchBinding,
    truth_view_basis_digest: Arc<str>,
    merge_history_basis_digest: Option<Arc<str>>,
    structural_basis_digest: Option<Arc<str>>,
    source_capability_digest: Arc<str>,
    request_shape_digest: Arc<str>,
    retained_artifact_schema_digest: Arc<str>,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgePreviewSessionDeclaration {
    pub fn new(
        declaration_identity: BridgePreviewSessionDeclarationIdentity,
        request_kind: BridgeRequestKind,
        branch_binding: BridgeSpeculativeBranchBinding,
        truth_view_basis_digest: impl Into<Arc<str>>,
        source_capability_digest: impl Into<Arc<str>>,
        request_shape_digest: impl Into<Arc<str>>,
        retained_artifact_schema_digest: impl Into<Arc<str>>,
    ) -> Self {
        Self::from_parts(
            declaration_identity,
            request_kind,
            branch_binding,
            truth_view_basis_digest.into(),
            None,
            None,
            source_capability_digest.into(),
            request_shape_digest.into(),
            retained_artifact_schema_digest.into(),
        )
    }

    pub fn with_merge_history_basis_digest(
        self,
        merge_history_basis_digest: impl Into<Arc<str>>,
    ) -> Self {
        Self::from_parts(
            self.declaration_identity,
            self.request_kind,
            self.branch_binding,
            self.truth_view_basis_digest,
            Some(merge_history_basis_digest.into()),
            self.structural_basis_digest,
            self.source_capability_digest,
            self.request_shape_digest,
            self.retained_artifact_schema_digest,
        )
    }

    pub fn with_structural_basis_digest(
        self,
        structural_basis_digest: impl Into<Arc<str>>,
    ) -> Self {
        Self::from_parts(
            self.declaration_identity,
            self.request_kind,
            self.branch_binding,
            self.truth_view_basis_digest,
            self.merge_history_basis_digest,
            Some(structural_basis_digest.into()),
            self.source_capability_digest,
            self.request_shape_digest,
            self.retained_artifact_schema_digest,
        )
    }

    fn from_parts(
        declaration_identity: BridgePreviewSessionDeclarationIdentity,
        request_kind: BridgeRequestKind,
        branch_binding: BridgeSpeculativeBranchBinding,
        truth_view_basis_digest: Arc<str>,
        merge_history_basis_digest: Option<Arc<str>>,
        structural_basis_digest: Option<Arc<str>>,
        source_capability_digest: Arc<str>,
        request_shape_digest: Arc<str>,
        retained_artifact_schema_digest: Arc<str>,
    ) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "preview-session-declaration|id={}|request-kind:{request_kind:?}|binding={}|truth-view={}|merge-basis={}|structural-basis={}|source-capability={}|request-shape={}|artifact-schema={}",
            declaration_identity.as_str(),
            branch_binding.digest(),
            truth_view_basis_digest.as_ref(),
            merge_history_basis_digest.as_deref().unwrap_or("none"),
            structural_basis_digest.as_deref().unwrap_or("none"),
            source_capability_digest.as_ref(),
            request_shape_digest.as_ref(),
            retained_artifact_schema_digest.as_ref(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());

        Self {
            declaration_identity,
            request_kind,
            branch_binding,
            truth_view_basis_digest,
            merge_history_basis_digest,
            structural_basis_digest,
            source_capability_digest,
            request_shape_digest,
            retained_artifact_schema_digest,
            canonical_basis,
            digest: Arc::from(format!("preview-session-declaration:sha256:{digest:x}")),
        }
    }

    pub fn validate(
        self,
    ) -> Result<ValidatedBridgePreviewSessionDeclaration, BridgeSpeculationError> {
        ValidatedBridgePreviewSessionDeclaration::new(self)
    }

    pub fn declaration_identity(&self) -> &BridgePreviewSessionDeclarationIdentity {
        &self.declaration_identity
    }

    pub fn request_kind(&self) -> BridgeRequestKind {
        self.request_kind
    }

    pub fn branch_binding(&self) -> &BridgeSpeculativeBranchBinding {
        &self.branch_binding
    }

    pub fn truth_view_basis_digest(&self) -> &str {
        self.truth_view_basis_digest.as_ref()
    }

    pub fn merge_history_basis_digest(&self) -> Option<&str> {
        self.merge_history_basis_digest.as_deref()
    }

    pub fn structural_basis_digest(&self) -> Option<&str> {
        self.structural_basis_digest.as_deref()
    }

    pub fn source_capability_digest(&self) -> &str {
        self.source_capability_digest.as_ref()
    }

    pub fn request_shape_digest(&self) -> &str {
        self.request_shape_digest.as_ref()
    }

    pub fn retained_artifact_schema_digest(&self) -> &str {
        self.retained_artifact_schema_digest.as_ref()
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use crate::input::envelope::TruthBranchIdentity;

    use super::{
        BridgePreviewSessionDeclaration, BridgePreviewSessionDeclarationIdentity, BridgeRequestKind,
        BridgeSignalBranchIdentity, BridgeSpeculativeBranchBinding,
        BridgeSpeculativeBranchBindingIdentity,
    };

    #[test]
    fn declaration_digest_is_stable_for_same_inputs() {
        let left = BridgePreviewSessionDeclaration::new(
            BridgePreviewSessionDeclarationIdentity::new("preview-declaration"),
            BridgeRequestKind::Preview,
            BridgeSpeculativeBranchBinding::new(
                BridgeSpeculativeBranchBindingIdentity::new("binding"),
                TruthBranchIdentity::new("truth-branch"),
                BridgeSignalBranchIdentity::new("signal-branch"),
            ),
            "truth-view-digest",
            "source-capability-digest",
            "request-shape-digest",
            "artifact-schema-digest",
        );
        let right = BridgePreviewSessionDeclaration::new(
            BridgePreviewSessionDeclarationIdentity::new("preview-declaration"),
            BridgeRequestKind::Preview,
            BridgeSpeculativeBranchBinding::new(
                BridgeSpeculativeBranchBindingIdentity::new("binding"),
                TruthBranchIdentity::new("truth-branch"),
                BridgeSignalBranchIdentity::new("signal-branch"),
            ),
            "truth-view-digest",
            "source-capability-digest",
            "request-shape-digest",
            "artifact-schema-digest",
        );

        assert_eq!(left, right);
    }
}
