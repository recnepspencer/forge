use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::identity::{
    BridgeIdentity, PreviewReuseEquivalenceIdentityTag, PromotionAdmissibilityProofIdentityTag,
};
use crate::input::envelope::TruthBranchIdentity;

use super::binding::BridgeSignalBranchIdentity;
use super::session::{
    BridgePreviewSession, BridgePreviewSessionIdentity, PreviewExecutionRecordIdentity,
};
use super::taxonomy::{PreviewActive, PreviewAdmitted};

pub type BridgePromotionAdmissibilityProofIdentity =
    BridgeIdentity<PromotionAdmissibilityProofIdentityTag>;
pub type BridgePreviewReuseEquivalenceIdentity = BridgeIdentity<PreviewReuseEquivalenceIdentityTag>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgePromotionAdmissibilityProof {
    proof_identity: BridgePromotionAdmissibilityProofIdentity,
    preview_session_identity: BridgePreviewSessionIdentity,
    preview_execution_record_identity: PreviewExecutionRecordIdentity,
    truth_branch_identity: TruthBranchIdentity,
    signal_branch_identity: BridgeSignalBranchIdentity,
    truth_view_basis_digest: Arc<str>,
    structural_basis_digest: Option<Arc<str>>,
    source_capability_digest: Arc<str>,
    request_shape_digest: Arc<str>,
    retained_artifact_schema_digest: Arc<str>,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgePromotionAdmissibilityProof {
    pub fn from_active_session(session: &BridgePreviewSession<PreviewActive>) -> Self {
        let declaration = session.declaration().declaration();
        let execution_record_identity = session
            .execution_record_identity()
            .expect("active preview sessions must carry execution record identity")
            .clone();
        let canonical_basis = Arc::<str>::from(format!(
            "promotion-admissibility-proof|session={}|execution-record={}|truth-branch={}|signal-branch={}|truth-view={}|structural-basis={}|source-capability={}|request-shape={}|artifact-schema={}",
            session.session_identity().as_str(),
            execution_record_identity.as_str(),
            declaration.branch_binding().truth_branch_identity().as_str(),
            declaration.branch_binding().signal_branch_identity().as_str(),
            declaration.truth_view_basis_digest(),
            declaration.structural_basis_digest().unwrap_or("none"),
            declaration.source_capability_digest(),
            declaration.request_shape_digest(),
            declaration.retained_artifact_schema_digest(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            proof_identity: BridgePromotionAdmissibilityProofIdentity::new(format!(
                "promotion-admissibility-proof:sha256:{digest:x}"
            )),
            preview_session_identity: session.session_identity().clone(),
            preview_execution_record_identity: execution_record_identity,
            truth_branch_identity: declaration.branch_binding().truth_branch_identity().clone(),
            signal_branch_identity: declaration
                .branch_binding()
                .signal_branch_identity()
                .clone(),
            truth_view_basis_digest: Arc::from(declaration.truth_view_basis_digest()),
            structural_basis_digest: declaration.structural_basis_digest().map(Arc::from),
            source_capability_digest: Arc::from(declaration.source_capability_digest()),
            request_shape_digest: Arc::from(declaration.request_shape_digest()),
            retained_artifact_schema_digest: Arc::from(
                declaration.retained_artifact_schema_digest(),
            ),
            canonical_basis,
            digest: Arc::from(format!("promotion-admissibility-proof:sha256:{digest:x}")),
        }
    }

    pub fn matches_active_session(&self, session: &BridgePreviewSession<PreviewActive>) -> bool {
        let declaration = session.declaration().declaration();
        session.session_identity() == &self.preview_session_identity
            && session
                .execution_record_identity()
                .map(PreviewExecutionRecordIdentity::as_str)
                == Some(self.preview_execution_record_identity.as_str())
            && declaration.branch_binding().truth_branch_identity() == &self.truth_branch_identity
            && declaration.branch_binding().signal_branch_identity() == &self.signal_branch_identity
            && declaration.truth_view_basis_digest() == self.truth_view_basis_digest.as_ref()
            && declaration.structural_basis_digest() == self.structural_basis_digest.as_deref()
            && declaration.source_capability_digest() == self.source_capability_digest.as_ref()
            && declaration.request_shape_digest() == self.request_shape_digest.as_ref()
            && declaration.retained_artifact_schema_digest()
                == self.retained_artifact_schema_digest.as_ref()
    }

    pub fn proof_identity(&self) -> &BridgePromotionAdmissibilityProofIdentity {
        &self.proof_identity
    }

    pub fn truth_branch_identity(&self) -> &TruthBranchIdentity {
        &self.truth_branch_identity
    }

    pub fn signal_branch_identity(&self) -> &BridgeSignalBranchIdentity {
        &self.signal_branch_identity
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgePreviewReuseEquivalence {
    equivalence_identity: BridgePreviewReuseEquivalenceIdentity,
    source_preview_session_identity: BridgePreviewSessionIdentity,
    source_preview_execution_record_identity: PreviewExecutionRecordIdentity,
    target_preview_session_identity: BridgePreviewSessionIdentity,
    truth_branch_identity: TruthBranchIdentity,
    signal_branch_identity: BridgeSignalBranchIdentity,
    truth_view_basis_digest: Arc<str>,
    structural_basis_digest: Option<Arc<str>>,
    source_capability_digest: Arc<str>,
    request_shape_digest: Arc<str>,
    retained_artifact_schema_digest: Arc<str>,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgePreviewReuseEquivalence {
    pub fn between_sessions(
        source: &BridgePreviewSession<PreviewActive>,
        target: &BridgePreviewSession<PreviewAdmitted>,
    ) -> Self {
        let source_declaration = source.declaration().declaration();
        let source_execution_record_identity = source
            .execution_record_identity()
            .expect("active preview sessions must carry execution record identity")
            .clone();
        let canonical_basis = Arc::<str>::from(format!(
            "preview-reuse-equivalence|source-session={}|source-execution-record={}|target-session={}|source-declaration={}|target-declaration={}|truth-branch={}|signal-branch={}|truth-view={}|structural-basis={}|source-capability={}|request-shape={}|artifact-schema={}",
            source.session_identity().as_str(),
            source_execution_record_identity.as_str(),
            target.session_identity().as_str(),
            source.declaration().digest(),
            target.declaration().digest(),
            source_declaration.branch_binding().truth_branch_identity().as_str(),
            source_declaration.branch_binding().signal_branch_identity().as_str(),
            source_declaration.truth_view_basis_digest(),
            source_declaration.structural_basis_digest().unwrap_or("none"),
            source_declaration.source_capability_digest(),
            source_declaration.request_shape_digest(),
            source_declaration.retained_artifact_schema_digest(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            equivalence_identity: BridgePreviewReuseEquivalenceIdentity::new(format!(
                "preview-reuse-equivalence:sha256:{digest:x}"
            )),
            source_preview_session_identity: source.session_identity().clone(),
            source_preview_execution_record_identity: source_execution_record_identity,
            target_preview_session_identity: target.session_identity().clone(),
            truth_branch_identity: source_declaration
                .branch_binding()
                .truth_branch_identity()
                .clone(),
            signal_branch_identity: source_declaration
                .branch_binding()
                .signal_branch_identity()
                .clone(),
            truth_view_basis_digest: Arc::from(source_declaration.truth_view_basis_digest()),
            structural_basis_digest: source_declaration.structural_basis_digest().map(Arc::from),
            source_capability_digest: Arc::from(source_declaration.source_capability_digest()),
            request_shape_digest: Arc::from(source_declaration.request_shape_digest()),
            retained_artifact_schema_digest: Arc::from(
                source_declaration.retained_artifact_schema_digest(),
            ),
            canonical_basis,
            digest: Arc::from(format!("preview-reuse-equivalence:sha256:{digest:x}")),
        }
    }

    pub fn matches_sessions(
        &self,
        source: &BridgePreviewSession<PreviewActive>,
        target: &BridgePreviewSession<PreviewAdmitted>,
    ) -> bool {
        let source_declaration = source.declaration().declaration();
        let target_declaration = target.declaration().declaration();
        source.session_identity() == &self.source_preview_session_identity
            && target.session_identity() == &self.target_preview_session_identity
            && source
                .execution_record_identity()
                .map(PreviewExecutionRecordIdentity::as_str)
                == Some(self.source_preview_execution_record_identity.as_str())
            && source_declaration.branch_binding().truth_branch_identity()
                == &self.truth_branch_identity
            && source_declaration.branch_binding().signal_branch_identity()
                == &self.signal_branch_identity
            && target_declaration.branch_binding().truth_branch_identity()
                == &self.truth_branch_identity
            && target_declaration.branch_binding().signal_branch_identity()
                == &self.signal_branch_identity
            && source_declaration.truth_view_basis_digest() == self.truth_view_basis_digest.as_ref()
            && target_declaration.truth_view_basis_digest() == self.truth_view_basis_digest.as_ref()
            && source_declaration.structural_basis_digest()
                == self.structural_basis_digest.as_deref()
            && target_declaration.structural_basis_digest()
                == self.structural_basis_digest.as_deref()
            && source_declaration.source_capability_digest()
                == self.source_capability_digest.as_ref()
            && target_declaration.source_capability_digest()
                == self.source_capability_digest.as_ref()
            && source_declaration.request_shape_digest() == self.request_shape_digest.as_ref()
            && target_declaration.request_shape_digest() == self.request_shape_digest.as_ref()
            && source_declaration.retained_artifact_schema_digest()
                == self.retained_artifact_schema_digest.as_ref()
            && target_declaration.retained_artifact_schema_digest()
                == self.retained_artifact_schema_digest.as_ref()
    }

    pub fn equivalence_identity(&self) -> &BridgePreviewReuseEquivalenceIdentity {
        &self.equivalence_identity
    }

    pub fn source_preview_session_identity(&self) -> &BridgePreviewSessionIdentity {
        &self.source_preview_session_identity
    }

    pub fn target_preview_session_identity(&self) -> &BridgePreviewSessionIdentity {
        &self.target_preview_session_identity
    }

    pub fn source_preview_execution_record_identity(&self) -> &PreviewExecutionRecordIdentity {
        &self.source_preview_execution_record_identity
    }

    pub fn truth_branch_identity(&self) -> &TruthBranchIdentity {
        &self.truth_branch_identity
    }

    pub fn signal_branch_identity(&self) -> &BridgeSignalBranchIdentity {
        &self.signal_branch_identity
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
    use crate::snapshot::BridgeTruthViewSelector;
    use crate::source::{BridgeSourceCapability, BridgeSourceCapabilitySet};
    use crate::speculation::{
        BridgePreviewRetainedArtifactSchema, BridgePreviewSessionBasis,
        BridgePreviewSessionDeclaration, BridgePreviewSessionDeclarationIdentity,
        BridgeRequestKind, BridgeSpeculativeBranchBinding, BridgeSpeculativeBranchBindingIdentity,
        PreviewSessionActivation,
    };

    use super::*;

    fn declaration(
        declaration_identity: &str,
        truth_branch_identity: TruthBranchIdentity,
        signal_branch_identity: BridgeSignalBranchIdentity,
    ) -> crate::speculation::ValidatedBridgePreviewSessionDeclaration {
        BridgePreviewSessionDeclaration::new(
            BridgePreviewSessionDeclarationIdentity::new(declaration_identity),
            BridgeRequestKind::Preview,
            BridgeSpeculativeBranchBinding::new(
                BridgeSpeculativeBranchBindingIdentity::new(format!(
                    "binding:{declaration_identity}"
                )),
                truth_branch_identity.clone(),
                signal_branch_identity,
            ),
            BridgePreviewSessionBasis::new(
                BridgeTruthViewSelector::committed_snapshot(
                    truth_branch_identity,
                    crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a"),
                ),
                BridgeSourceCapabilitySet::new(vec![BridgeSourceCapability::SnapshotRead]),
                BridgePreviewRetainedArtifactSchema::PreviewLifecycleArtifactsV1,
            ),
        )
        .validate()
        .expect("native preview declaration should validate")
    }

    fn active_session(
        session_identity: &str,
    ) -> BridgePreviewSession<crate::speculation::PreviewActive> {
        let declared = BridgePreviewSession::<crate::speculation::PreviewDeclared>::declare(
            BridgePreviewSessionIdentity::new(session_identity),
            declaration(
                &format!("declaration:{session_identity}"),
                crate::truth_identity_fixtures::truth_branch_fixture("truth-branch"),
                BridgeSignalBranchIdentity::new("signal-branch"),
            ),
        );
        let admitted = declared.admit();
        admitted.activate(PreviewSessionActivation::new(
            PreviewExecutionRecordIdentity::new(format!("execution:{session_identity}")),
        ))
    }

    fn admitted_session(
        session_identity: &str,
    ) -> BridgePreviewSession<crate::speculation::PreviewAdmitted> {
        BridgePreviewSession::<crate::speculation::PreviewDeclared>::declare(
            BridgePreviewSessionIdentity::new(session_identity),
            declaration(
                &format!("declaration:{session_identity}"),
                crate::truth_identity_fixtures::truth_branch_fixture("truth-branch"),
                BridgeSignalBranchIdentity::new("signal-branch"),
            ),
        )
        .admit()
    }

    #[test]
    fn promotion_admissibility_proof_retains_typed_branch_binding_identities() {
        let active = active_session("source-session");
        let proof = BridgePromotionAdmissibilityProof::from_active_session(&active);

        assert_eq!(
            proof.truth_branch_identity(),
            &crate::truth_identity_fixtures::truth_branch_fixture("truth-branch")
        );
        assert_eq!(
            proof.signal_branch_identity(),
            &BridgeSignalBranchIdentity::new("signal-branch")
        );
        assert!(proof.matches_active_session(&active));
    }

    #[test]
    fn preview_reuse_equivalence_retains_typed_branch_binding_identities() {
        let source = active_session("source-session");
        let target = admitted_session("target-session");
        let equivalence = BridgePreviewReuseEquivalence::between_sessions(&source, &target);

        assert_eq!(
            equivalence.truth_branch_identity(),
            &crate::truth_identity_fixtures::truth_branch_fixture("truth-branch")
        );
        assert_eq!(
            equivalence.signal_branch_identity(),
            &BridgeSignalBranchIdentity::new("signal-branch")
        );
        assert!(equivalence.matches_sessions(&source, &target));
    }
}
