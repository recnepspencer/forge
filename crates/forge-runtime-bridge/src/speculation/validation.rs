use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::error::{BridgeSpeculationError, BridgeSpeculationErrorKind};

use super::declaration::BridgePreviewSessionDeclaration;
use super::taxonomy::BridgeRequestKind;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidatedBridgePreviewSessionDeclaration {
    declaration: BridgePreviewSessionDeclaration,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl ValidatedBridgePreviewSessionDeclaration {
    pub(crate) fn new(
        declaration: BridgePreviewSessionDeclaration,
    ) -> Result<Self, BridgeSpeculationError> {
        if declaration.request_kind() != BridgeRequestKind::Preview {
            return Err(BridgeSpeculationError::new(
                BridgeSpeculationErrorKind::PreviewRequestKindMismatch,
                format!(
                    "Preview session declaration `{}` must use request kind `Preview`, but received `{:?}`.",
                    declaration.declaration_identity().as_str(),
                    declaration.request_kind(),
                ),
            ));
        }

        if declaration.branch_binding().truth_branch_identity().as_str().is_empty()
            || declaration
                .branch_binding()
                .signal_branch_identity()
                .as_str()
                .is_empty()
        {
            return Err(BridgeSpeculationError::new(
                BridgeSpeculationErrorKind::PreviewBranchBindingMismatch,
                format!(
                    "Preview session declaration `{}` requires non-empty truth and signal branch identities.",
                    declaration.declaration_identity().as_str(),
                ),
            ));
        }

        for (label, digest) in [
            ("truth-view basis", declaration.truth_view_basis_digest()),
            ("source capability", declaration.source_capability_digest()),
            ("request shape", declaration.request_shape_digest()),
            (
                "retained artifact schema",
                declaration.retained_artifact_schema_digest(),
            ),
        ] {
            if digest.is_empty() {
                return Err(BridgeSpeculationError::new(
                    BridgeSpeculationErrorKind::PromotionAdmissibilityMismatch,
                    format!(
                        "Preview session declaration `{}` requires non-empty {} digest.",
                        declaration.declaration_identity().as_str(),
                        label,
                    ),
                ));
            }
        }

        let canonical_basis = Arc::<str>::from(format!(
            "validated-preview-session-declaration|declaration={}",
            declaration.digest()
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());

        Ok(Self {
            declaration,
            canonical_basis,
            digest: Arc::from(format!(
                "validated-preview-session-declaration:sha256:{digest:x}"
            )),
        })
    }

    pub fn declaration(&self) -> &BridgePreviewSessionDeclaration {
        &self.declaration
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
    use super::ValidatedBridgePreviewSessionDeclaration;
    use crate::input::envelope::TruthBranchIdentity;
    use crate::speculation::{
        BridgePreviewSessionDeclaration, BridgePreviewSessionDeclarationIdentity, BridgeRequestKind,
        BridgeSignalBranchIdentity, BridgeSpeculativeBranchBinding,
        BridgeSpeculativeBranchBindingIdentity,
    };

    #[test]
    fn validation_rejects_authoritative_request_kind_for_preview_session() {
        let declaration = BridgePreviewSessionDeclaration::new(
            BridgePreviewSessionDeclarationIdentity::new("preview-declaration"),
            BridgeRequestKind::Authoritative,
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

        let error = ValidatedBridgePreviewSessionDeclaration::new(declaration)
            .expect_err("authoritative request kind should be rejected");
        assert_eq!(
            error.kind(),
            crate::error::BridgeSpeculationErrorKind::PreviewRequestKindMismatch
        );
    }
}
