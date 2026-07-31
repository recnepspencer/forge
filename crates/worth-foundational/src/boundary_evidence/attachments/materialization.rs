use crate::canonicalization::{
    admit_canonical_sequence_digest_derivation_with_budget, derive_canonical_digest,
    prepare_canonical_basis_sequence, CanonicalBasisConstructionDenial, CanonicalBasisDomain,
    CanonicalBasisEntry, CanonicalBasisEntryKind, CanonicalBasisLocus, CanonicalBasisReadyArtifact,
    CanonicalBasisValue, CanonicalDerivedDigest, CanonicalDigestAlgorithmId,
    CanonicalDigestDerivationDenial, CanonicalDigestWorkBudget, CanonicalizationRuleVersion,
};
use crate::values::InternedString;

use super::super::provenance::FoundationalBoundaryEvidenceProvenanceArtifact;
use super::super::receipts::FoundationalBoundaryEvidenceCompletedReceiptArtifact;
use super::definitions::{
    FoundationalBoundaryEvidenceContinuityAttachmentScope,
    FoundationalBoundaryEvidenceMaterializationProfile,
};
use super::{
    canonical_fragment_for_provenance_attachment, canonical_fragment_for_receipt_attachment,
    FoundationalBoundaryEvidenceAttachmentTarget, FoundationalBoundaryEvidenceDiagnosticAttachment,
    FoundationalBoundaryEvidenceLocatorContinuityAttachment,
    FoundationalBoundaryEvidenceObjectContinuityAttachment,
    FoundationalBoundaryEvidenceSupportAttachment,
};
use worth_proof::TransitionOutcome;

const ATTACHMENT_DIGEST_BUDGET: CanonicalDigestWorkBudget =
    match CanonicalDigestWorkBudget::new(512, 128 * 1_024) {
        Some(budget) => budget,
        None => panic!("attachment digest budget is nonzero"),
    };

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalMaterializedBoundaryEvidenceAttachmentBundle {
    profile: FoundationalBoundaryEvidenceMaterializationProfile,
    target: FoundationalBoundaryEvidenceAttachmentTarget,
    object_continuity: Option<FoundationalBoundaryEvidenceObjectContinuityAttachment>,
    locator_continuity: Option<FoundationalBoundaryEvidenceLocatorContinuityAttachment>,
    provenance: Option<FoundationalBoundaryEvidenceProvenanceArtifact>,
    receipt: Option<FoundationalBoundaryEvidenceCompletedReceiptArtifact>,
    support: Option<FoundationalBoundaryEvidenceSupportAttachment>,
    diagnostic: Option<FoundationalBoundaryEvidenceDiagnosticAttachment>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FoundationalBoundaryEvidenceAttachmentDigestDerivationDenial {
    CanonicalBasis(CanonicalBasisConstructionDenial),
    Digest(CanonicalDigestDerivationDenial),
}

impl FoundationalMaterializedBoundaryEvidenceAttachmentBundle {
    pub(crate) fn new(
        profile: FoundationalBoundaryEvidenceMaterializationProfile,
        target: FoundationalBoundaryEvidenceAttachmentTarget,
        object_continuity: Option<FoundationalBoundaryEvidenceObjectContinuityAttachment>,
        locator_continuity: Option<FoundationalBoundaryEvidenceLocatorContinuityAttachment>,
        provenance: Option<FoundationalBoundaryEvidenceProvenanceArtifact>,
        receipt: Option<FoundationalBoundaryEvidenceCompletedReceiptArtifact>,
        support: Option<FoundationalBoundaryEvidenceSupportAttachment>,
        diagnostic: Option<FoundationalBoundaryEvidenceDiagnosticAttachment>,
    ) -> Self {
        Self {
            profile,
            target,
            object_continuity,
            locator_continuity,
            provenance,
            receipt,
            support,
            diagnostic,
        }
    }

    pub const fn materialization_profile(
        &self,
    ) -> FoundationalBoundaryEvidenceMaterializationProfile {
        self.profile
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

    pub fn support(&self) -> Option<&FoundationalBoundaryEvidenceSupportAttachment> {
        self.support.as_ref()
    }

    pub fn diagnostic(&self) -> Option<&FoundationalBoundaryEvidenceDiagnosticAttachment> {
        self.diagnostic.as_ref()
    }

    pub fn target(&self) -> &FoundationalBoundaryEvidenceAttachmentTarget {
        &self.target
    }
}

pub fn prepare_boundary_evidence_attachment_bundle_for_canonical_basis(
    version: CanonicalizationRuleVersion,
    bundle: &FoundationalMaterializedBoundaryEvidenceAttachmentBundle,
) -> TransitionOutcome<CanonicalBasisReadyArtifact, CanonicalBasisConstructionDenial> {
    let mut entries = vec![attachment_entry(
        "target",
        bundle.target.canonical_fragment(),
    )];
    if let Some(scope) = bundle.continuity_scope() {
        entries.push(attachment_entry("continuity_scope", format!("{scope:?}")));
    }
    if let Some(continuity) = &bundle.object_continuity {
        entries.push(attachment_entry(
            "object_continuity",
            continuity.canonical_fragment(),
        ));
    }
    if let Some(continuity) = &bundle.locator_continuity {
        entries.push(attachment_entry(
            "locator_continuity",
            continuity.canonical_fragment(),
        ));
    }
    if let Some(provenance) = &bundle.provenance {
        entries.push(attachment_entry(
            "provenance",
            canonical_fragment_for_provenance_attachment(provenance),
        ));
    }
    if let Some(receipt) = &bundle.receipt {
        entries.push(attachment_entry(
            "receipt",
            canonical_fragment_for_receipt_attachment(receipt),
        ));
    }
    if let Some(support) = &bundle.support {
        entries.push(attachment_entry("support", support.canonical_fragment()));
    }
    if let Some(diagnostic) = &bundle.diagnostic {
        entries.push(attachment_entry(
            "diagnostic",
            diagnostic.canonical_fragment(),
        ));
    }
    entries.push(attachment_entry(
        "materialization_profile",
        format!("{:?}", bundle.profile),
    ));

    prepare_canonical_basis_sequence(version, CanonicalBasisDomain::BoundaryArtifact, entries)
}

pub fn derive_boundary_evidence_attachment_bundle_digest(
    version: CanonicalizationRuleVersion,
    bundle: &FoundationalMaterializedBoundaryEvidenceAttachmentBundle,
    algorithm_id: CanonicalDigestAlgorithmId,
) -> TransitionOutcome<
    CanonicalDerivedDigest,
    FoundationalBoundaryEvidenceAttachmentDigestDerivationDenial,
> {
    let basis =
        match prepare_boundary_evidence_attachment_bundle_for_canonical_basis(version, bundle) {
            TransitionOutcome::Success(basis) => basis,
            TransitionOutcome::Denied(denial) => {
                return TransitionOutcome::Denied(
                    FoundationalBoundaryEvidenceAttachmentDigestDerivationDenial::CanonicalBasis(
                        denial,
                    ),
                )
            }
        };
    let slot = crate::canonicalization::CanonicalSingleSequenceDigestAlgorithmSlot::single_sequence(
        algorithm_id,
        basis.payload().domain(),
        basis.payload().version().clone(),
    );

    match admit_canonical_sequence_digest_derivation_with_budget(
        basis,
        slot,
        ATTACHMENT_DIGEST_BUDGET,
    ) {
        TransitionOutcome::Success(ready) => {
            TransitionOutcome::Success(derive_canonical_digest(ready))
        }
        TransitionOutcome::Denied(denial) => TransitionOutcome::Denied(
            FoundationalBoundaryEvidenceAttachmentDigestDerivationDenial::Digest(denial),
        ),
    }
}

fn attachment_entry(name: &str, value: String) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        CanonicalBasisDomain::BoundaryArtifact,
        CanonicalBasisLocus::Named(InternedString::from(name)),
        CanonicalBasisEntryKind::BoundaryAttachment,
        CanonicalBasisValue::ExactText(InternedString::from(value)),
    )
}
