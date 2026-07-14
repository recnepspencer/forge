use crate::FoundationalDiagnosticLocator;

use super::super::lineage::{
    FoundationalBoundaryEvidenceAttestedLineageArtifact,
    FoundationalBoundaryEvidenceBranchLocalLineageArtifact,
    FoundationalBoundaryEvidenceLineageSubject, FoundationalBoundaryEvidencePartialLineageArtifact,
    FoundationalBoundaryEvidencePromotedLineageArtifact,
    FoundationalBoundaryEvidenceReconstructedEquivalenceArtifact,
    FoundationalBoundaryEvidenceReplayDerivedLineageArtifact,
    FoundationalBoundaryEvidenceRestoredLineageArtifact,
};
use super::super::provenance::FoundationalBoundaryEvidenceProvenanceArtifact;
use super::super::receipts::FoundationalBoundaryEvidenceCompletedReceiptArtifact;
use super::continuity::{
    FoundationalBoundaryEvidenceLocatorContinuityAttachment,
    FoundationalBoundaryEvidenceObjectContinuityAttachment,
};
use super::definitions::{
    FoundationalBoundaryEvidenceAttachmentTargetKind,
    FoundationalBoundaryEvidenceContinuityAttachmentScope,
    FoundationalBoundaryEvidenceMaterializationProfile,
};
use super::descriptive::{
    FoundationalBoundaryEvidenceDiagnosticAttachment, FoundationalBoundaryEvidenceSupportAttachment,
};
use super::materialization::FoundationalMaterializedBoundaryEvidenceAttachmentBundle;
use super::target::FoundationalBoundaryEvidenceAttachmentTarget;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalBoundaryEvidenceAttachmentBundle {
    target: FoundationalBoundaryEvidenceAttachmentTarget,
    object_continuity: Option<FoundationalBoundaryEvidenceObjectContinuityAttachment>,
    locator_continuity: Option<FoundationalBoundaryEvidenceLocatorContinuityAttachment>,
    provenance: Option<FoundationalBoundaryEvidenceProvenanceArtifact>,
    receipt: Option<FoundationalBoundaryEvidenceCompletedReceiptArtifact>,
    support: Option<FoundationalBoundaryEvidenceSupportAttachment>,
    diagnostic: Option<FoundationalBoundaryEvidenceDiagnosticAttachment>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalDiagnosticBundleAttachmentBundle {
    inner: FoundationalBoundaryEvidenceAttachmentBundle,
}

impl FoundationalBoundaryEvidenceAttachmentBundle {
    pub(crate) fn new(target: FoundationalBoundaryEvidenceAttachmentTarget) -> Self {
        Self {
            target,
            object_continuity: None,
            locator_continuity: None,
            provenance: None,
            receipt: None,
            support: None,
            diagnostic: None,
        }
    }

    pub const fn target_kind(&self) -> FoundationalBoundaryEvidenceAttachmentTargetKind {
        self.target.target_kind()
    }

    pub fn target(&self) -> &FoundationalBoundaryEvidenceAttachmentTarget {
        &self.target
    }

    pub const fn continuity_scope(
        &self,
    ) -> Option<FoundationalBoundaryEvidenceContinuityAttachmentScope> {
        if self.object_continuity.is_some() {
            Some(FoundationalBoundaryEvidenceContinuityAttachmentScope::ObjectLevel)
        } else if self.locator_continuity.is_some() {
            Some(FoundationalBoundaryEvidenceContinuityAttachmentScope::LocatorLevel)
        } else {
            None
        }
    }

    pub fn object_continuity(
        &self,
    ) -> Option<&FoundationalBoundaryEvidenceObjectContinuityAttachment> {
        self.object_continuity.as_ref()
    }

    pub fn locator_continuity(
        &self,
    ) -> Option<&FoundationalBoundaryEvidenceLocatorContinuityAttachment> {
        self.locator_continuity.as_ref()
    }

    pub fn provenance(&self) -> Option<&FoundationalBoundaryEvidenceProvenanceArtifact> {
        self.provenance.as_ref()
    }

    pub fn receipt(&self) -> Option<&FoundationalBoundaryEvidenceCompletedReceiptArtifact> {
        self.receipt.as_ref()
    }

    pub fn support(&self) -> Option<&FoundationalBoundaryEvidenceSupportAttachment> {
        self.support.as_ref()
    }

    pub fn diagnostic(&self) -> Option<&FoundationalBoundaryEvidenceDiagnosticAttachment> {
        self.diagnostic.as_ref()
    }

    pub(crate) fn attach_object_continuity(
        mut self,
        attachment: FoundationalBoundaryEvidenceObjectContinuityAttachment,
    ) -> Self {
        self.object_continuity = Some(attachment);
        self.locator_continuity = None;
        self
    }

    pub(crate) fn attach_locator_continuity(
        mut self,
        attachment: FoundationalBoundaryEvidenceLocatorContinuityAttachment,
    ) -> Self {
        self.locator_continuity = Some(attachment);
        self.object_continuity = None;
        self
    }

    pub(crate) fn attach_provenance(
        mut self,
        provenance: FoundationalBoundaryEvidenceProvenanceArtifact,
    ) -> Self {
        self.provenance = Some(provenance);
        self
    }

    pub(crate) fn attach_receipt(
        mut self,
        receipt: FoundationalBoundaryEvidenceCompletedReceiptArtifact,
    ) -> Self {
        self.receipt = Some(receipt);
        self
    }

    pub(crate) fn attach_support(
        mut self,
        support: FoundationalBoundaryEvidenceSupportAttachment,
    ) -> Self {
        self.support = Some(support);
        self
    }

    pub(crate) fn attach_diagnostic(
        mut self,
        diagnostic: FoundationalBoundaryEvidenceDiagnosticAttachment,
    ) -> Self {
        self.diagnostic = Some(diagnostic);
        self
    }

    pub fn materialize(
        &self,
        profile: FoundationalBoundaryEvidenceMaterializationProfile,
    ) -> FoundationalMaterializedBoundaryEvidenceAttachmentBundle {
        let support = match profile {
            FoundationalBoundaryEvidenceMaterializationProfile::FullDescriptiveRichness
            | FoundationalBoundaryEvidenceMaterializationProfile::ElideDiagnostics => {
                self.support.clone()
            }
            FoundationalBoundaryEvidenceMaterializationProfile::ElideSupportAndDiagnostics => None,
        };
        let diagnostic = match profile {
            FoundationalBoundaryEvidenceMaterializationProfile::FullDescriptiveRichness => {
                self.diagnostic.clone()
            }
            FoundationalBoundaryEvidenceMaterializationProfile::ElideDiagnostics
            | FoundationalBoundaryEvidenceMaterializationProfile::ElideSupportAndDiagnostics => {
                None
            }
        };

        FoundationalMaterializedBoundaryEvidenceAttachmentBundle::new(
            profile,
            self.target.clone(),
            self.object_continuity.clone(),
            self.locator_continuity.clone(),
            self.provenance.clone(),
            self.receipt.clone(),
            support,
            diagnostic,
        )
    }
}

impl FoundationalDiagnosticBundleAttachmentBundle {
    pub(crate) fn new(target: FoundationalBoundaryEvidenceAttachmentTarget) -> Self {
        Self {
            inner: FoundationalBoundaryEvidenceAttachmentBundle::new(target),
        }
    }

    pub const fn target_kind(&self) -> FoundationalBoundaryEvidenceAttachmentTargetKind {
        self.inner.target_kind()
    }

    pub fn target(&self) -> &FoundationalBoundaryEvidenceAttachmentTarget {
        self.inner.target()
    }

    pub const fn continuity_scope(
        &self,
    ) -> Option<FoundationalBoundaryEvidenceContinuityAttachmentScope> {
        self.inner.continuity_scope()
    }

    pub fn provenance(&self) -> Option<&FoundationalBoundaryEvidenceProvenanceArtifact> {
        self.inner.provenance()
    }

    pub fn object_continuity(
        &self,
    ) -> Option<&FoundationalBoundaryEvidenceObjectContinuityAttachment> {
        self.inner.object_continuity()
    }

    pub fn locator_continuity(
        &self,
    ) -> Option<&FoundationalBoundaryEvidenceLocatorContinuityAttachment> {
        self.inner.locator_continuity()
    }

    pub fn with_attested_continuity(
        mut self,
        attachment: FoundationalBoundaryEvidenceAttestedLineageArtifact,
    ) -> Self {
        self.inner = self.inner.attach_object_continuity(
            FoundationalBoundaryEvidenceObjectContinuityAttachment::Attested(attachment),
        );
        self
    }

    pub fn with_branch_local_continuity(
        mut self,
        attachment: FoundationalBoundaryEvidenceBranchLocalLineageArtifact,
    ) -> Self {
        self.inner = self.inner.attach_object_continuity(
            FoundationalBoundaryEvidenceObjectContinuityAttachment::BranchLocal(attachment),
        );
        self
    }

    pub fn with_promoted_continuity(
        mut self,
        attachment: FoundationalBoundaryEvidencePromotedLineageArtifact,
    ) -> Self {
        self.inner = self.inner.attach_object_continuity(
            FoundationalBoundaryEvidenceObjectContinuityAttachment::Promoted(attachment),
        );
        self
    }

    pub fn with_replay_derived_continuity(
        mut self,
        attachment: FoundationalBoundaryEvidenceReplayDerivedLineageArtifact,
    ) -> Self {
        self.inner = self.inner.attach_object_continuity(
            FoundationalBoundaryEvidenceObjectContinuityAttachment::ReplayDerived(attachment),
        );
        self
    }

    pub fn with_restored_continuity(
        mut self,
        attachment: FoundationalBoundaryEvidenceRestoredLineageArtifact,
    ) -> Self {
        self.inner = self.inner.attach_object_continuity(
            FoundationalBoundaryEvidenceObjectContinuityAttachment::Restored(attachment),
        );
        self
    }

    pub fn with_reconstructed_equivalence(
        mut self,
        attachment: FoundationalBoundaryEvidenceReconstructedEquivalenceArtifact,
    ) -> Self {
        self.inner = self.inner.attach_object_continuity(
            FoundationalBoundaryEvidenceObjectContinuityAttachment::Reconstructed(attachment),
        );
        self
    }

    pub fn with_partial_continuity(
        mut self,
        attachment: FoundationalBoundaryEvidencePartialLineageArtifact,
    ) -> Self {
        self.inner = self.inner.attach_object_continuity(
            FoundationalBoundaryEvidenceObjectContinuityAttachment::Partial(attachment),
        );
        self
    }

    pub fn with_locator_continuity(
        mut self,
        stable_object: FoundationalBoundaryEvidenceLineageSubject,
        from_locator: FoundationalDiagnosticLocator,
        to_locator: FoundationalDiagnosticLocator,
    ) -> Self {
        self.inner = self.inner.attach_locator_continuity(
            FoundationalBoundaryEvidenceLocatorContinuityAttachment::new(
                stable_object,
                from_locator,
                to_locator,
            ),
        );
        self
    }

    pub fn with_provenance_attachment(
        mut self,
        provenance: FoundationalBoundaryEvidenceProvenanceArtifact,
    ) -> Self {
        self.inner = self.inner.attach_provenance(provenance);
        self
    }

    pub fn materialize_under(
        &self,
        profile: FoundationalBoundaryEvidenceMaterializationProfile,
    ) -> FoundationalMaterializedBoundaryEvidenceAttachmentBundle {
        self.inner.materialize(profile)
    }
}
