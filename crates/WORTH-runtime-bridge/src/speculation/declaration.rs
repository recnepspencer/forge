use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::error::BridgeSpeculationError;
use crate::identity::{
    BridgeIdentity, BridgeIdentityEvidence, PreviewSessionDeclarationIdentityTag,
};

use super::binding::BridgeSpeculativeBranchBinding;
use super::declaration_basis::{
    BridgePreviewRequestShapeBasis, BridgePreviewSessionBasis, BridgePreviewStructuralBasis,
};
use super::validation::ValidatedBridgePreviewSessionDeclaration;
use super::BridgeRequestKind;

pub type BridgePreviewSessionDeclarationIdentity =
    BridgeIdentity<PreviewSessionDeclarationIdentityTag>;

impl BridgePreviewSessionDeclarationIdentity {
    pub fn from_bridge_evidence(evidence_identity: &BridgeIdentityEvidence) -> Self {
        Self::admit_bridge_owned(format!(
            "bridge-preview-session-declaration:external-authority-evidence:{}",
            evidence_identity.as_str()
        ))
    }

    pub fn from_stable_name(value: impl Into<Arc<str>>) -> Self {
        Self::admit_bridge_owned(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgePreviewSessionDeclaration {
    declaration_identity: BridgePreviewSessionDeclarationIdentity,
    request_kind: BridgeRequestKind,
    branch_binding: BridgeSpeculativeBranchBinding,
    session_basis: BridgePreviewSessionBasis,
    request_shape_basis: BridgePreviewRequestShapeBasis,
    structural_basis: Option<BridgePreviewStructuralBasis>,
    truth_view_basis_digest: Arc<str>,
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
        session_basis: BridgePreviewSessionBasis,
    ) -> Self {
        Self::from_parts(
            declaration_identity,
            request_kind,
            branch_binding,
            session_basis,
            None,
        )
    }

    pub fn with_structural_basis(self, structural_basis: BridgePreviewStructuralBasis) -> Self {
        Self::from_parts(
            self.declaration_identity,
            self.request_kind,
            self.branch_binding,
            self.session_basis,
            Some(structural_basis),
        )
    }

    fn from_parts(
        declaration_identity: BridgePreviewSessionDeclarationIdentity,
        request_kind: BridgeRequestKind,
        branch_binding: BridgeSpeculativeBranchBinding,
        session_basis: BridgePreviewSessionBasis,
        structural_basis: Option<BridgePreviewStructuralBasis>,
    ) -> Self {
        let request_shape_basis = BridgePreviewRequestShapeBasis::from_request_kind(request_kind);
        let truth_view_basis_digest = Arc::<str>::from(session_basis.truth_view_basis_digest());
        let structural_basis_digest = structural_basis
            .as_ref()
            .map(|basis| Arc::<str>::from(basis.digest()));
        let source_capability_digest = Arc::<str>::from(session_basis.source_capability_digest());
        let request_shape_digest = Arc::<str>::from(request_shape_basis.digest());
        let retained_artifact_schema_digest =
            Arc::<str>::from(session_basis.retained_artifact_schema_digest());
        let canonical_basis = Arc::<str>::from(format!(
            "preview-session-declaration|id={}|request-kind:{request_kind:?}|binding={}|truth-view={}|structural-basis={}|source-capability={}|request-shape={}|request-shape-basis={}|artifact-schema={}",
            declaration_identity.as_str(),
            branch_binding.digest(),
            truth_view_basis_digest.as_ref(),
            structural_basis_digest.as_deref().unwrap_or("none"),
            source_capability_digest.as_ref(),
            request_shape_digest.as_ref(),
            request_shape_basis.canonical_basis(),
            retained_artifact_schema_digest.as_ref(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());

        Self {
            declaration_identity,
            request_kind,
            branch_binding,
            session_basis,
            request_shape_basis,
            structural_basis,
            truth_view_basis_digest,
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

    pub fn structural_basis_digest(&self) -> Option<&str> {
        self.structural_basis_digest.as_deref()
    }

    pub fn session_basis(&self) -> &BridgePreviewSessionBasis {
        &self.session_basis
    }

    pub fn structural_basis(&self) -> Option<&BridgePreviewStructuralBasis> {
        self.structural_basis.as_ref()
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

    use crate::snapshot::BridgeTruthViewSelector;
    use crate::source::{BridgeSourceCapability, BridgeSourceCapabilitySet};
    use crate::speculation::{BridgeSignalBranchIdentity, BridgeSpeculativeBranchBindingIdentity};

    use crate::speculation::{
        BridgePreviewRetainedArtifactSchema, BridgePreviewSessionBasis,
        BridgePreviewSessionDeclaration, BridgePreviewSessionDeclarationIdentity,
        BridgeRequestKind, BridgeSpeculativeBranchBinding,
    };

    fn preview_session_basis() -> BridgePreviewSessionBasis {
        BridgePreviewSessionBasis::new(
            BridgeTruthViewSelector::committed_snapshot(
                crate::truth_identity_fixtures::truth_branch_fixture("truth-branch"),
                crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a"),
            ),
            BridgeSourceCapabilitySet::new(vec![BridgeSourceCapability::SnapshotRead]),
            BridgePreviewRetainedArtifactSchema::PreviewLifecycleArtifactsV1,
        )
    }

    #[test]
    fn declaration_digest_is_stable_for_same_inputs() {
        let left = BridgePreviewSessionDeclaration::new(
            BridgePreviewSessionDeclarationIdentity::admit_bridge_owned("preview-declaration"),
            BridgeRequestKind::Preview,
            BridgeSpeculativeBranchBinding::new(
                BridgeSpeculativeBranchBindingIdentity::admit_bridge_owned("binding"),
                crate::truth_identity_fixtures::truth_branch_fixture("truth-branch"),
                BridgeSignalBranchIdentity::admit_bridge_owned("signal-branch"),
            ),
            preview_session_basis(),
        );
        let right = BridgePreviewSessionDeclaration::new(
            BridgePreviewSessionDeclarationIdentity::admit_bridge_owned("preview-declaration"),
            BridgeRequestKind::Preview,
            BridgeSpeculativeBranchBinding::new(
                BridgeSpeculativeBranchBindingIdentity::admit_bridge_owned("binding"),
                crate::truth_identity_fixtures::truth_branch_fixture("truth-branch"),
                BridgeSignalBranchIdentity::admit_bridge_owned("signal-branch"),
            ),
            preview_session_basis(),
        );

        assert_eq!(left, right);
    }
}
