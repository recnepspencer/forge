use worth_foundational::facade::{
    CanonicalBasisConstructionDenial, FoundationalBoundaryEvidenceAttachmentDigestDerivationDenial,
    FoundationalBoundaryEvidenceProvenanceConstructionDenial,
    FoundationalBoundaryEvidenceSupportConstructionDenial,
};

use crate::application::{WorthQueryDeclarationInput, WorthQueryDomainEntryMarker};

use super::class::WorthQueryDeclarationFoundationalEvidenceClass;

#[derive(Debug, Eq, PartialEq)]
pub enum WorthQueryDeclarationFoundationalEvidenceDenial<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
> {
    WrongAdmittedWorld {
        class: WorthQueryDeclarationFoundationalEvidenceClass,
        expected_handle_identity_digest: String,
        actual_handle_identity_digest: String,
        expected_operating_context_identity_digest: String,
        actual_operating_context_identity_digest: Option<String>,
        _marker: std::marker::PhantomData<(D, I)>,
    },
    Provenance {
        class: WorthQueryDeclarationFoundationalEvidenceClass,
        denial: FoundationalBoundaryEvidenceProvenanceConstructionDenial,
        _marker: std::marker::PhantomData<(D, I)>,
    },
    Support {
        class: WorthQueryDeclarationFoundationalEvidenceClass,
        denial: FoundationalBoundaryEvidenceSupportConstructionDenial,
        _marker: std::marker::PhantomData<(D, I)>,
    },
    AttachmentCanonicalBasis {
        class: WorthQueryDeclarationFoundationalEvidenceClass,
        denial: CanonicalBasisConstructionDenial,
        _marker: std::marker::PhantomData<(D, I)>,
    },
    AttachmentDigest {
        class: WorthQueryDeclarationFoundationalEvidenceClass,
        denial: FoundationalBoundaryEvidenceAttachmentDigestDerivationDenial,
        _marker: std::marker::PhantomData<(D, I)>,
    },
}

impl<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>
    WorthQueryDeclarationFoundationalEvidenceDenial<D, I>
{
    pub(crate) fn wrong_world(
        class: WorthQueryDeclarationFoundationalEvidenceClass,
        expected_handle_identity_digest: String,
        actual_handle_identity_digest: String,
        expected_operating_context_identity_digest: String,
        actual_operating_context_identity_digest: Option<String>,
    ) -> Self {
        Self::WrongAdmittedWorld {
            class,
            expected_handle_identity_digest,
            actual_handle_identity_digest,
            expected_operating_context_identity_digest,
            actual_operating_context_identity_digest,
            _marker: std::marker::PhantomData,
        }
    }

    pub(crate) fn provenance(
        class: WorthQueryDeclarationFoundationalEvidenceClass,
        denial: FoundationalBoundaryEvidenceProvenanceConstructionDenial,
    ) -> Self {
        Self::Provenance {
            class,
            denial,
            _marker: std::marker::PhantomData,
        }
    }

    pub(crate) fn support(
        class: WorthQueryDeclarationFoundationalEvidenceClass,
        denial: FoundationalBoundaryEvidenceSupportConstructionDenial,
    ) -> Self {
        Self::Support {
            class,
            denial,
            _marker: std::marker::PhantomData,
        }
    }

    pub(crate) fn attachment_canonical_basis(
        class: WorthQueryDeclarationFoundationalEvidenceClass,
        denial: CanonicalBasisConstructionDenial,
    ) -> Self {
        Self::AttachmentCanonicalBasis {
            class,
            denial,
            _marker: std::marker::PhantomData,
        }
    }

    pub(crate) fn attachment_digest(
        class: WorthQueryDeclarationFoundationalEvidenceClass,
        denial: FoundationalBoundaryEvidenceAttachmentDigestDerivationDenial,
    ) -> Self {
        Self::AttachmentDigest {
            class,
            denial,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn class(&self) -> WorthQueryDeclarationFoundationalEvidenceClass {
        match self {
            Self::WrongAdmittedWorld { class, .. }
            | Self::Provenance { class, .. }
            | Self::Support { class, .. }
            | Self::AttachmentCanonicalBasis { class, .. }
            | Self::AttachmentDigest { class, .. } => *class,
        }
    }

    pub fn reason(&self) -> &'static str {
        match self {
            Self::WrongAdmittedWorld { .. } => "foundational evidence must stay inside one admitted world",
            Self::Provenance { .. } => "foundational provenance construction denied the requested locality and freshness pairing",
            Self::Support { .. } => "foundational support attachment construction denied the requested support posture",
            Self::AttachmentCanonicalBasis { .. } => "foundational attachment bundle canonical basis construction failed",
            Self::AttachmentDigest { .. } => "foundational attachment bundle digest derivation failed",
        }
    }
}
