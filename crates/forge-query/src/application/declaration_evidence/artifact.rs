use forge_foundational::facade::{
    CanonicalDerivedDigest, FoundationalBoundaryEvidenceCompletedReceiptArtifact,
    FoundationalBoundaryEvidenceMaterializationProfile,
    FoundationalBoundaryEvidencePlanningReceiptArtifact,
    FoundationalBoundaryEvidenceProvenanceArtifact, FoundationalBoundaryEvidenceSupportAttachment,
    FoundationalMaterializedBoundaryEvidenceAttachmentBundle,
};

use crate::application::declaration_publication::foundational_publication_for_profile;
use crate::application::{
    ForgeQueryDeclarationAspectContract, ForgeQueryDeclarationAspectCoverage,
    ForgeQueryDeclarationAspectCoverageBasis, ForgeQueryDeclarationAspectPublication,
    ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker,
};

use super::{
    bundle::build_bundle, class::ForgeQueryDeclarationFoundationalEvidenceClass,
    denial::ForgeQueryDeclarationFoundationalEvidenceDenial, provenance::build_provenance,
    receipt::build_primary_receipt, subject::ForgeQueryDeclarationFoundationalEvidenceInput,
    support::build_support_attachment,
};

pub struct ForgeQueryDeclarationFoundationalEvidence<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    subject: ForgeQueryDeclarationFoundationalEvidenceInput<D, I>,
    class: ForgeQueryDeclarationFoundationalEvidenceClass,
    handle_identity_digest: String,
    operating_context_identity_digest: String,
    declaration_family_key: &'static str,
    declaration_digest: String,
    support_digest: String,
    legality_digest: Option<String>,
    progression_digest: Option<String>,
    provenance: FoundationalBoundaryEvidenceProvenanceArtifact,
    planning_receipt: Option<FoundationalBoundaryEvidencePlanningReceiptArtifact>,
    receipt: Option<FoundationalBoundaryEvidenceCompletedReceiptArtifact>,
    support_attachment: Option<FoundationalBoundaryEvidenceSupportAttachment>,
    attachment_bundle: FoundationalMaterializedBoundaryEvidenceAttachmentBundle,
    attachment_bundle_digest: CanonicalDerivedDigest,
    materialization_profile: FoundationalBoundaryEvidenceMaterializationProfile,
    aspect_contract: ForgeQueryDeclarationAspectContract,
    aspect_coverage: ForgeQueryDeclarationAspectCoverage,
    aspect_coverage_basis: ForgeQueryDeclarationAspectCoverageBasis,
    aspect_publication: ForgeQueryDeclarationAspectPublication,
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryDeclarationFoundationalEvidence<D, I>
{
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        subject: ForgeQueryDeclarationFoundationalEvidenceInput<D, I>,
        handle_identity_digest: String,
        operating_context_identity_digest: String,
        provenance: FoundationalBoundaryEvidenceProvenanceArtifact,
        planning_receipt: Option<FoundationalBoundaryEvidencePlanningReceiptArtifact>,
        receipt: Option<FoundationalBoundaryEvidenceCompletedReceiptArtifact>,
        support_attachment: FoundationalBoundaryEvidenceSupportAttachment,
        attachment_bundle: FoundationalMaterializedBoundaryEvidenceAttachmentBundle,
        attachment_bundle_digest: CanonicalDerivedDigest,
        materialization_profile: FoundationalBoundaryEvidenceMaterializationProfile,
        aspect_contract: ForgeQueryDeclarationAspectContract,
        aspect_coverage: ForgeQueryDeclarationAspectCoverage,
        aspect_coverage_basis: ForgeQueryDeclarationAspectCoverageBasis,
        aspect_publication: ForgeQueryDeclarationAspectPublication,
    ) -> Self {
        let class = subject.class();
        let declaration_family_key = subject.declaration_family_key();
        let declaration_digest = subject.declaration_digest_string();
        let support_digest = subject.support_digest().to_string();
        let legality_digest = subject.legality_digest().map(ToOwned::to_owned);
        let progression_digest = subject.progression_digest().map(ToOwned::to_owned);

        Self {
            subject,
            class,
            handle_identity_digest,
            operating_context_identity_digest,
            declaration_family_key,
            declaration_digest,
            support_digest,
            legality_digest,
            progression_digest,
            provenance,
            planning_receipt,
            receipt,
            support_attachment: Some(support_attachment),
            attachment_bundle,
            attachment_bundle_digest,
            materialization_profile,
            aspect_contract,
            aspect_coverage,
            aspect_coverage_basis,
            aspect_publication,
        }
    }

    pub fn class(&self) -> ForgeQueryDeclarationFoundationalEvidenceClass {
        self.class
    }

    pub fn handle_identity_digest(&self) -> &str {
        &self.handle_identity_digest
    }

    pub fn operating_context_identity_digest(&self) -> &str {
        &self.operating_context_identity_digest
    }

    pub fn declaration_family_key(&self) -> &'static str {
        self.declaration_family_key
    }

    pub fn declaration_digest(&self) -> &str {
        &self.declaration_digest
    }

    pub fn support_digest(&self) -> &str {
        &self.support_digest
    }

    pub fn legality_digest(&self) -> Option<&str> {
        self.legality_digest.as_deref()
    }

    pub fn progression_digest(&self) -> Option<&str> {
        self.progression_digest.as_deref()
    }

    pub fn provenance(&self) -> &FoundationalBoundaryEvidenceProvenanceArtifact {
        &self.provenance
    }

    pub fn planning_receipt(&self) -> Option<&FoundationalBoundaryEvidencePlanningReceiptArtifact> {
        self.planning_receipt.as_ref()
    }

    pub fn receipt(&self) -> Option<&FoundationalBoundaryEvidenceCompletedReceiptArtifact> {
        self.receipt.as_ref()
    }

    pub fn support_attachment(&self) -> Option<&FoundationalBoundaryEvidenceSupportAttachment> {
        self.support_attachment.as_ref()
    }

    pub fn legality_contract(&self) -> crate::application::ForgeQueryDeclarationLegalityContract {
        self.subject.legality_contract()
    }

    pub fn progression_contract(
        &self,
    ) -> Option<crate::application::ForgeQueryDeclarationProgressionContract> {
        self.subject.progression_contract()
    }

    pub fn attachment_bundle(&self) -> &FoundationalMaterializedBoundaryEvidenceAttachmentBundle {
        &self.attachment_bundle
    }

    pub fn attachment_bundle_digest(&self) -> &CanonicalDerivedDigest {
        &self.attachment_bundle_digest
    }

    pub fn materialization_profile(&self) -> FoundationalBoundaryEvidenceMaterializationProfile {
        self.materialization_profile
    }

    pub fn aspect_contract(&self) -> &ForgeQueryDeclarationAspectContract {
        &self.aspect_contract
    }

    pub fn aspect_coverage(&self) -> &ForgeQueryDeclarationAspectCoverage {
        &self.aspect_coverage
    }

    pub fn aspect_coverage_basis(&self) -> ForgeQueryDeclarationAspectCoverageBasis {
        self.aspect_coverage_basis
    }

    pub fn aspect_publication(&self) -> &ForgeQueryDeclarationAspectPublication {
        &self.aspect_publication
    }

    pub fn subject(&self) -> &ForgeQueryDeclarationFoundationalEvidenceInput<D, I> {
        &self.subject
    }
}

pub(crate) fn forge_query_declaration_foundational_evidence<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    expected_handle_identity_digest: &str,
    expected_operating_context_identity_digest: &str,
    subject: ForgeQueryDeclarationFoundationalEvidenceInput<D, I>,
    profile: FoundationalBoundaryEvidenceMaterializationProfile,
) -> Result<
    ForgeQueryDeclarationFoundationalEvidence<D, I>,
    ForgeQueryDeclarationFoundationalEvidenceDenial<D, I>,
> {
    if subject.handle_identity_digest() != expected_handle_identity_digest {
        return Err(
            ForgeQueryDeclarationFoundationalEvidenceDenial::wrong_world(
                subject.class(),
                expected_handle_identity_digest.to_string(),
                subject.handle_identity_digest().to_string(),
                expected_operating_context_identity_digest.to_string(),
                Some(subject.operating_context_identity_digest().to_string()),
            ),
        );
    }
    if subject.operating_context_identity_digest() != expected_operating_context_identity_digest {
        return Err(
            ForgeQueryDeclarationFoundationalEvidenceDenial::wrong_world(
                subject.class(),
                expected_handle_identity_digest.to_string(),
                subject.handle_identity_digest().to_string(),
                expected_operating_context_identity_digest.to_string(),
                Some(subject.operating_context_identity_digest().to_string()),
            ),
        );
    }

    let provenance = build_provenance(&subject)?;
    let primary_receipt = build_primary_receipt(&subject, provenance.clone());
    let support_attachment =
        build_support_attachment(&subject, provenance.clone(), primary_receipt.completed())?;
    let planning_receipt = primary_receipt.planning().cloned();
    let receipt = primary_receipt.completed().cloned();
    let (attachment_bundle, attachment_bundle_digest) = build_bundle(
        &subject,
        provenance.clone(),
        planning_receipt.clone(),
        receipt.clone(),
        support_attachment.clone(),
        profile,
    )?;
    let retained_operating_context_identity_digest =
        subject.operating_context_identity_digest().to_string();
    let aspect_contract = subject.aspect_contract().clone();
    let aspect_coverage = subject.aspect_coverage();
    let aspect_coverage_basis = subject.aspect_coverage_basis();
    let aspect_publication =
        foundational_publication_for_profile(&aspect_contract, &aspect_coverage, profile);

    Ok(ForgeQueryDeclarationFoundationalEvidence::new(
        subject,
        expected_handle_identity_digest.to_string(),
        retained_operating_context_identity_digest,
        provenance,
        planning_receipt,
        receipt,
        support_attachment,
        attachment_bundle,
        attachment_bundle_digest,
        profile,
        aspect_contract,
        aspect_coverage,
        aspect_coverage_basis,
        aspect_publication,
    ))
}
