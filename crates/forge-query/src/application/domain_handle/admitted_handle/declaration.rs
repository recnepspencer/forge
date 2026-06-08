use crate::application::{
    forge_query_checked_family_declaration, forge_query_checked_family_support,
    review_declaration_legality, ForgeQueryCanonicalDeclarationArtifact,
    ForgeQueryDeclarationAdmissionError, ForgeQueryDeclarationAdmissionOrLegalityError,
    ForgeQueryDeclarationCanonicalizationVersion, ForgeQueryDeclarationFamilyMarker,
    ForgeQueryDeclarationFamilySupportChecked, ForgeQueryDeclarationFamilySupportReport,
    ForgeQueryDeclarationInput, ForgeQueryDeclarationLegalityChecked,
    ForgeQueryDeclarationLegalityDenial, ForgeQueryDeclarationLegalityEvidence,
    ForgeQueryDeclarationLegalityInput, ForgeQueryDeclaredFamilyChecked,
    ForgeQueryDomainEntryMarker,
};

use super::ForgeQueryAdmittedConfiguredDomainHandle;
use crate::application::ForgeQueryDomainOperatingContext;
use crate::runtime::ForgeQueryRuntimeFacadeFamily;

impl<D: ForgeQueryDomainEntryMarker, C: ForgeQueryDomainOperatingContext<D>>
    ForgeQueryAdmittedConfiguredDomainHandle<D, C>
{
    pub fn family_support<F>(&self) -> ForgeQueryDeclarationFamilySupportReport<D, F>
    where
        F: ForgeQueryDeclarationFamilyMarker<D>,
    {
        match self.family_support_checked::<F>() {
            ForgeQueryDeclarationFamilySupportChecked::Admitted(report)
            | ForgeQueryDeclarationFamilySupportChecked::Deferred(report)
            | ForgeQueryDeclarationFamilySupportChecked::Unsupported(report)
            | ForgeQueryDeclarationFamilySupportChecked::InvalidContext(report) => report,
        }
    }

    pub fn family_support_checked<F>(&self) -> ForgeQueryDeclarationFamilySupportChecked<D, F>
    where
        F: ForgeQueryDeclarationFamilyMarker<D>,
    {
        forge_query_checked_family_support::<D, C, F>(self)
    }

    pub fn declare<I>(
        &self,
        input: I,
    ) -> Result<
        ForgeQueryCanonicalDeclarationArtifact<D, I>,
        ForgeQueryDeclarationAdmissionError<D, I>,
    >
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        match self.declare_checked(input) {
            ForgeQueryDeclaredFamilyChecked::Admitted(artifact) => Ok(artifact),
            ForgeQueryDeclaredFamilyChecked::Deferred(denial) => {
                Err(ForgeQueryDeclarationAdmissionError::Deferred(denial))
            }
            ForgeQueryDeclaredFamilyChecked::AsyncDeferred(denial) => {
                Err(ForgeQueryDeclarationAdmissionError::AsyncDeferred(denial))
            }
            ForgeQueryDeclaredFamilyChecked::TemporalDeferred(denial) => Err(
                ForgeQueryDeclarationAdmissionError::TemporalDeferred(denial),
            ),
            ForgeQueryDeclaredFamilyChecked::Unsupported(denial) => {
                Err(ForgeQueryDeclarationAdmissionError::Unsupported(denial))
            }
            ForgeQueryDeclaredFamilyChecked::AsyncUnsupported(denial) => Err(
                ForgeQueryDeclarationAdmissionError::AsyncUnsupported(denial),
            ),
            ForgeQueryDeclaredFamilyChecked::TemporalUnsupported(denial) => Err(
                ForgeQueryDeclarationAdmissionError::TemporalUnsupported(denial),
            ),
            ForgeQueryDeclaredFamilyChecked::InvalidContext(denial) => {
                Err(ForgeQueryDeclarationAdmissionError::InvalidContext(denial))
            }
            ForgeQueryDeclaredFamilyChecked::Canonicalization(error) => {
                Err(ForgeQueryDeclarationAdmissionError::Canonicalization(error))
            }
        }
    }

    pub fn declare_checked<I>(&self, input: I) -> ForgeQueryDeclaredFamilyChecked<D, I>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        forge_query_checked_family_declaration(
            self,
            input,
            ForgeQueryDeclarationCanonicalizationVersion::default(),
        )
    }

    pub fn declare_with_version<I>(
        &self,
        input: I,
        version: ForgeQueryDeclarationCanonicalizationVersion,
    ) -> Result<
        ForgeQueryCanonicalDeclarationArtifact<D, I>,
        ForgeQueryDeclarationAdmissionError<D, I>,
    >
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        match forge_query_checked_family_declaration(self, input, version) {
            ForgeQueryDeclaredFamilyChecked::Admitted(artifact) => Ok(artifact),
            ForgeQueryDeclaredFamilyChecked::Deferred(denial) => {
                Err(ForgeQueryDeclarationAdmissionError::Deferred(denial))
            }
            ForgeQueryDeclaredFamilyChecked::AsyncDeferred(denial) => {
                Err(ForgeQueryDeclarationAdmissionError::AsyncDeferred(denial))
            }
            ForgeQueryDeclaredFamilyChecked::TemporalDeferred(denial) => Err(
                ForgeQueryDeclarationAdmissionError::TemporalDeferred(denial),
            ),
            ForgeQueryDeclaredFamilyChecked::Unsupported(denial) => {
                Err(ForgeQueryDeclarationAdmissionError::Unsupported(denial))
            }
            ForgeQueryDeclaredFamilyChecked::AsyncUnsupported(denial) => Err(
                ForgeQueryDeclarationAdmissionError::AsyncUnsupported(denial),
            ),
            ForgeQueryDeclaredFamilyChecked::TemporalUnsupported(denial) => Err(
                ForgeQueryDeclarationAdmissionError::TemporalUnsupported(denial),
            ),
            ForgeQueryDeclaredFamilyChecked::InvalidContext(denial) => {
                Err(ForgeQueryDeclarationAdmissionError::InvalidContext(denial))
            }
            ForgeQueryDeclaredFamilyChecked::Canonicalization(error) => {
                Err(ForgeQueryDeclarationAdmissionError::Canonicalization(error))
            }
        }
    }

    pub fn review_legality<I>(
        &self,
        declaration: ForgeQueryCanonicalDeclarationArtifact<D, I>,
    ) -> Result<
        ForgeQueryDeclarationLegalityEvidence<D, I>,
        ForgeQueryDeclarationLegalityDenial<D, I>,
    >
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        match self.review_legality_checked(declaration) {
            ForgeQueryDeclarationLegalityChecked::Legal(evidence) => Ok(evidence),
            ForgeQueryDeclarationLegalityChecked::Illegal(denial) => Err(denial),
        }
    }

    pub fn review_legality_checked<I>(
        &self,
        declaration: ForgeQueryCanonicalDeclarationArtifact<D, I>,
    ) -> ForgeQueryDeclarationLegalityChecked<D, I>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        let support_report = self.family_support::<I::Family>();
        let legality_contract = I::Family::legality_contract();
        let temporal_runtime_support_status = self
            .support_snapshot()
            .runtime_support_matrix()
            .row_for_family(ForgeQueryRuntimeFacadeFamily::Temporal)
            .map(|row| row.status());
        let async_runtime_support_status = self
            .support_snapshot()
            .runtime_support_matrix()
            .row_for_family(ForgeQueryRuntimeFacadeFamily::AsyncResource)
            .map(|row| row.status());
        let input = ForgeQueryDeclarationLegalityInput::new(
            declaration,
            support_report,
            legality_contract,
            self.retained_world_basis(),
            temporal_runtime_support_status,
            async_runtime_support_status,
        );
        review_declaration_legality(self.handle_identity_digest(), input)
    }

    pub fn declare_and_review<I>(
        &self,
        input: I,
    ) -> Result<
        ForgeQueryDeclarationLegalityEvidence<D, I>,
        ForgeQueryDeclarationAdmissionOrLegalityError<D, I>,
    >
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        let declaration = self
            .declare(input)
            .map_err(ForgeQueryDeclarationAdmissionOrLegalityError::Admission)?;
        self.review_legality(declaration)
            .map_err(ForgeQueryDeclarationAdmissionOrLegalityError::Legality)
    }
}
