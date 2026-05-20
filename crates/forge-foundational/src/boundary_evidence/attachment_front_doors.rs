use crate::diagnostics::FoundationalDiagnosticLocator;
use crate::locators::BoundaryArtifactLocator;

use super::attachments::{
    FoundationalBoundaryEvidenceAttachmentBundle, FoundationalBoundaryEvidenceAttachmentTarget,
    FoundationalBoundaryEvidenceDiagnosticAttachment,
    FoundationalBoundaryEvidenceLocatorContinuityAttachment,
    FoundationalBoundaryEvidenceMaterializationProfile,
    FoundationalBoundaryEvidenceObjectContinuityAttachment,
    FoundationalBoundaryEvidenceSupportAttachment, FoundationalDiagnosticBundleAttachmentBundle,
    FoundationalMaterializedBoundaryEvidenceAttachmentBundle,
};
use super::lineage::{
    FoundationalBoundaryEvidenceAttestedLineageArtifact,
    FoundationalBoundaryEvidenceBranchLocalLineageArtifact,
    FoundationalBoundaryEvidenceLineageSubject, FoundationalBoundaryEvidencePartialLineageArtifact,
    FoundationalBoundaryEvidencePromotedLineageArtifact,
    FoundationalBoundaryEvidenceReconstructedEquivalenceArtifact,
    FoundationalBoundaryEvidenceReplayDerivedLineageArtifact,
    FoundationalBoundaryEvidenceRestoredLineageArtifact,
};
use super::provenance::FoundationalBoundaryEvidenceProvenanceArtifact;
use super::receipts::FoundationalBoundaryEvidenceCompletedReceiptArtifact;
use super::support::{
    FoundationalBoundaryEvidencePublishedSupportArtifact,
    FoundationalBoundaryEvidenceSupportCloseoutArtifact,
    FoundationalBoundaryEvidenceTransientLifecycleSupportArtifact,
};
use crate::FoundationalTransitionLocator;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct FoundationalBoundaryEvidenceAttachmentFrontDoor;

impl FoundationalBoundaryEvidenceAttachmentFrontDoor {
    pub fn for_boundary_artifact(
        self,
        locator: BoundaryArtifactLocator,
    ) -> FoundationalBoundaryEvidenceAttachmentBundle {
        FoundationalBoundaryEvidenceAttachmentBundle::new(
            FoundationalBoundaryEvidenceAttachmentTarget::BoundaryArtifact(locator),
        )
    }

    pub fn for_transition(
        self,
        locator: FoundationalTransitionLocator,
    ) -> FoundationalBoundaryEvidenceAttachmentBundle {
        FoundationalBoundaryEvidenceAttachmentBundle::new(
            FoundationalBoundaryEvidenceAttachmentTarget::Transition(locator),
        )
    }

    pub fn for_diagnostic_bundle(
        self,
        locator: FoundationalDiagnosticLocator,
    ) -> FoundationalDiagnosticBundleAttachmentBundle {
        FoundationalDiagnosticBundleAttachmentBundle::new(
            FoundationalBoundaryEvidenceAttachmentTarget::DiagnosticBundle(locator),
        )
    }
}

impl FoundationalBoundaryEvidenceAttachmentBundle {
    pub fn with_attested_continuity(
        self,
        artifact: FoundationalBoundaryEvidenceAttestedLineageArtifact,
    ) -> Self {
        self.attach_object_continuity(
            FoundationalBoundaryEvidenceObjectContinuityAttachment::Attested(artifact),
        )
    }

    pub fn with_branch_local_continuity(
        self,
        artifact: FoundationalBoundaryEvidenceBranchLocalLineageArtifact,
    ) -> Self {
        self.attach_object_continuity(
            FoundationalBoundaryEvidenceObjectContinuityAttachment::BranchLocal(artifact),
        )
    }

    pub fn with_promoted_continuity(
        self,
        artifact: FoundationalBoundaryEvidencePromotedLineageArtifact,
    ) -> Self {
        self.attach_object_continuity(
            FoundationalBoundaryEvidenceObjectContinuityAttachment::Promoted(artifact),
        )
    }

    pub fn with_replay_derived_continuity(
        self,
        artifact: FoundationalBoundaryEvidenceReplayDerivedLineageArtifact,
    ) -> Self {
        self.attach_object_continuity(
            FoundationalBoundaryEvidenceObjectContinuityAttachment::ReplayDerived(artifact),
        )
    }

    pub fn with_restored_continuity(
        self,
        artifact: FoundationalBoundaryEvidenceRestoredLineageArtifact,
    ) -> Self {
        self.attach_object_continuity(
            FoundationalBoundaryEvidenceObjectContinuityAttachment::Restored(artifact),
        )
    }

    pub fn with_reconstructed_equivalence(
        self,
        artifact: FoundationalBoundaryEvidenceReconstructedEquivalenceArtifact,
    ) -> Self {
        self.attach_object_continuity(
            FoundationalBoundaryEvidenceObjectContinuityAttachment::Reconstructed(artifact),
        )
    }

    pub fn with_partial_continuity(
        self,
        artifact: FoundationalBoundaryEvidencePartialLineageArtifact,
    ) -> Self {
        self.attach_object_continuity(
            FoundationalBoundaryEvidenceObjectContinuityAttachment::Partial(artifact),
        )
    }

    pub fn with_locator_continuity(
        self,
        stable_object: FoundationalBoundaryEvidenceLineageSubject,
        from_locator: FoundationalDiagnosticLocator,
        to_locator: FoundationalDiagnosticLocator,
    ) -> Self {
        self.attach_locator_continuity(
            FoundationalBoundaryEvidenceLocatorContinuityAttachment::new(
                stable_object,
                from_locator,
                to_locator,
            ),
        )
    }

    pub fn with_provenance_attachment(
        self,
        provenance: FoundationalBoundaryEvidenceProvenanceArtifact,
    ) -> Self {
        self.attach_provenance(provenance)
    }

    pub fn with_receipt_attachment(
        self,
        receipt: FoundationalBoundaryEvidenceCompletedReceiptArtifact,
    ) -> Self {
        self.attach_receipt(receipt)
    }

    pub fn with_published_support(
        self,
        artifact: FoundationalBoundaryEvidencePublishedSupportArtifact,
    ) -> Self {
        self.attach_support(FoundationalBoundaryEvidenceSupportAttachment::Published(
            artifact,
        ))
    }

    pub fn with_support_closeout(
        self,
        artifact: FoundationalBoundaryEvidenceSupportCloseoutArtifact,
    ) -> Self {
        self.attach_support(FoundationalBoundaryEvidenceSupportAttachment::Closeout(
            artifact,
        ))
    }

    pub fn with_transient_lifecycle_support(
        self,
        artifact: FoundationalBoundaryEvidenceTransientLifecycleSupportArtifact,
    ) -> Self {
        self.attach_support(
            FoundationalBoundaryEvidenceSupportAttachment::TransientLifecycle(artifact),
        )
    }

    pub fn with_support_report_locator(self, locator: FoundationalDiagnosticLocator) -> Self {
        self.attach_diagnostic(
            FoundationalBoundaryEvidenceDiagnosticAttachment::SupportReport(locator),
        )
    }

    pub fn with_explanation_bundle_locator(self, locator: FoundationalDiagnosticLocator) -> Self {
        self.attach_diagnostic(
            FoundationalBoundaryEvidenceDiagnosticAttachment::ExplanationBundle(locator),
        )
    }

    pub fn materialize_under(
        &self,
        profile: FoundationalBoundaryEvidenceMaterializationProfile,
    ) -> FoundationalMaterializedBoundaryEvidenceAttachmentBundle {
        self.materialize(profile)
    }
}
