use super::operating_context::ForgeQueryDomainOperatingContext;
use crate::application::{
    forge_query_checked_declaration_progression, forge_query_checked_family_declaration,
    forge_query_checked_family_support, forge_query_declaration_foundational_evidence,
    forge_query_declaration_progression_recipe, review_declaration_legality,
    ForgeQueryAdmittedDeclarationProgression, ForgeQueryCanonicalDeclarationArtifact,
    ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily, ForgeQueryDeclarationAdmissionError,
    ForgeQueryDeclarationAdmissionOrLegalityError, ForgeQueryDeclarationCanonicalizationVersion,
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationFamilySupportChecked,
    ForgeQueryDeclarationFamilySupportReport, ForgeQueryDeclarationFoundationalEvidence,
    ForgeQueryDeclarationFoundationalEvidenceChecked,
    ForgeQueryDeclarationFoundationalEvidenceDenial,
    ForgeQueryDeclarationFoundationalEvidenceInput, ForgeQueryDeclarationInput,
    ForgeQueryDeclarationLegalityChecked, ForgeQueryDeclarationLegalityDenial,
    ForgeQueryDeclarationLegalityEvidence, ForgeQueryDeclarationLegalityInput,
    ForgeQueryDeclarationProgressionChecked, ForgeQueryDeclarationProgressionRecipe,
    ForgeQueryDeclarationProgressionTerminalError, ForgeQueryDeclaredFamilyChecked,
    ForgeQueryDomainEntryMarker, ForgeQueryDomainEntrySupportSnapshot,
};
use forge_foundational::facade::FoundationalBoundaryEvidenceMaterializationProfile;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryAdmittedConfiguredDomainHandle<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
> {
    marker: D,
    operating_context: C,
    support_snapshot: ForgeQueryDomainEntrySupportSnapshot,
    required_capability_families: Vec<ForgeQueryCapabilityFamily>,
    required_config_sections: Vec<ForgeQueryConfigSectionFamily>,
    operating_context_identity_digest: String,
    handle_identity_digest: String,
}

impl<D: ForgeQueryDomainEntryMarker, C: ForgeQueryDomainOperatingContext<D>>
    ForgeQueryAdmittedConfiguredDomainHandle<D, C>
{
    pub(crate) fn new(
        marker: D,
        operating_context: C,
        support_snapshot: ForgeQueryDomainEntrySupportSnapshot,
        required_capability_families: Vec<ForgeQueryCapabilityFamily>,
        required_config_sections: Vec<ForgeQueryConfigSectionFamily>,
        operating_context_identity_digest: String,
        handle_identity_digest: String,
    ) -> Self {
        Self {
            marker,
            operating_context,
            support_snapshot,
            required_capability_families,
            required_config_sections,
            operating_context_identity_digest,
            handle_identity_digest,
        }
    }

    pub fn domain_key(&self) -> &'static str {
        self.marker.domain_key()
    }

    pub fn display_name(&self) -> &'static str {
        self.marker.display_name()
    }

    pub fn operating_context(&self) -> &C {
        &self.operating_context
    }

    pub fn support_snapshot(&self) -> &ForgeQueryDomainEntrySupportSnapshot {
        &self.support_snapshot
    }

    pub fn required_capability_families(&self) -> &[ForgeQueryCapabilityFamily] {
        &self.required_capability_families
    }

    pub fn required_config_sections(&self) -> &[ForgeQueryConfigSectionFamily] {
        &self.required_config_sections
    }

    pub fn handle_identity_digest(&self) -> &str {
        &self.handle_identity_digest
    }

    pub fn operating_context_identity_digest(&self) -> &str {
        &self.operating_context_identity_digest
    }

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
            ForgeQueryDeclaredFamilyChecked::Unsupported(denial) => {
                Err(ForgeQueryDeclarationAdmissionError::Unsupported(denial))
            }
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
            ForgeQueryDeclaredFamilyChecked::Unsupported(denial) => {
                Err(ForgeQueryDeclarationAdmissionError::Unsupported(denial))
            }
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
        let input = ForgeQueryDeclarationLegalityInput::new(
            declaration,
            support_report,
            legality_contract,
            self.operating_context_identity_digest().to_string(),
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

    pub fn declaration_progression_recipe<I>(
        &self,
        legal: ForgeQueryDeclarationLegalityEvidence<D, I>,
    ) -> ForgeQueryDeclarationProgressionRecipe<D, I>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        forge_query_declaration_progression_recipe(
            legal,
            self.operating_context_identity_digest().to_string(),
        )
    }

    pub fn progress_declaration<I>(
        &self,
        legal: ForgeQueryDeclarationLegalityEvidence<D, I>,
    ) -> Result<
        ForgeQueryAdmittedDeclarationProgression<D, I>,
        ForgeQueryDeclarationProgressionTerminalError<D, I>,
    >
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        match self.progress_declaration_checked(legal) {
            ForgeQueryDeclarationProgressionChecked::Admitted(admitted) => Ok(admitted),
            ForgeQueryDeclarationProgressionChecked::Deferred(progress) => Err(
                ForgeQueryDeclarationProgressionTerminalError::Deferred(progress),
            ),
            ForgeQueryDeclarationProgressionChecked::Denied(progress) => Err(
                ForgeQueryDeclarationProgressionTerminalError::Denied(progress),
            ),
            ForgeQueryDeclarationProgressionChecked::Stale(progress) => Err(
                ForgeQueryDeclarationProgressionTerminalError::Stale(progress),
            ),
            ForgeQueryDeclarationProgressionChecked::RebindRequired(progress) => {
                Err(ForgeQueryDeclarationProgressionTerminalError::RebindRequired(progress))
            }
            ForgeQueryDeclarationProgressionChecked::Failed(progress) => Err(
                ForgeQueryDeclarationProgressionTerminalError::Failed(progress),
            ),
        }
    }

    pub fn progress_declaration_recipe<I>(
        &self,
        recipe: ForgeQueryDeclarationProgressionRecipe<D, I>,
    ) -> Result<
        ForgeQueryAdmittedDeclarationProgression<D, I>,
        ForgeQueryDeclarationProgressionTerminalError<D, I>,
    >
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        match forge_query_checked_declaration_progression(recipe) {
            ForgeQueryDeclarationProgressionChecked::Admitted(admitted) => Ok(admitted),
            ForgeQueryDeclarationProgressionChecked::Deferred(progress) => Err(
                ForgeQueryDeclarationProgressionTerminalError::Deferred(progress),
            ),
            ForgeQueryDeclarationProgressionChecked::Denied(progress) => Err(
                ForgeQueryDeclarationProgressionTerminalError::Denied(progress),
            ),
            ForgeQueryDeclarationProgressionChecked::Stale(progress) => Err(
                ForgeQueryDeclarationProgressionTerminalError::Stale(progress),
            ),
            ForgeQueryDeclarationProgressionChecked::RebindRequired(progress) => {
                Err(ForgeQueryDeclarationProgressionTerminalError::RebindRequired(progress))
            }
            ForgeQueryDeclarationProgressionChecked::Failed(progress) => Err(
                ForgeQueryDeclarationProgressionTerminalError::Failed(progress),
            ),
        }
    }

    pub fn progress_declaration_checked<I>(
        &self,
        legal: ForgeQueryDeclarationLegalityEvidence<D, I>,
    ) -> ForgeQueryDeclarationProgressionChecked<D, I>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        forge_query_checked_declaration_progression(self.declaration_progression_recipe(legal))
    }

    pub fn progress_declaration_recipe_checked<I>(
        &self,
        recipe: ForgeQueryDeclarationProgressionRecipe<D, I>,
    ) -> ForgeQueryDeclarationProgressionChecked<D, I>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        forge_query_checked_declaration_progression(recipe)
    }

    pub fn declare_review_and_progress<I>(
        &self,
        input: I,
    ) -> Result<
        ForgeQueryAdmittedDeclarationProgression<D, I>,
        ForgeQueryDeclarationEntryProgressionError<D, I>,
    >
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        let legal = self
            .declare_and_review(input)
            .map_err(ForgeQueryDeclarationEntryProgressionError::Entry)?;
        self.progress_declaration(legal)
            .map_err(ForgeQueryDeclarationEntryProgressionError::Progression)
    }

    pub fn describe_foundational<I>(
        &self,
        subject: ForgeQueryDeclarationFoundationalEvidenceInput<D, I>,
    ) -> Result<
        ForgeQueryDeclarationFoundationalEvidence<D, I>,
        ForgeQueryDeclarationFoundationalEvidenceDenial<D, I>,
    >
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        self.describe_foundational_with_profile(
            subject,
            FoundationalBoundaryEvidenceMaterializationProfile::FullDescriptiveRichness,
        )
    }

    pub fn describe_foundational_checked<I>(
        &self,
        subject: ForgeQueryDeclarationFoundationalEvidenceInput<D, I>,
    ) -> ForgeQueryDeclarationFoundationalEvidenceChecked<D, I>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        match self.describe_foundational(subject) {
            Ok(evidence) => ForgeQueryDeclarationFoundationalEvidenceChecked::Described(evidence),
            Err(denial) => {
                ForgeQueryDeclarationFoundationalEvidenceChecked::ConstructionDenied(denial)
            }
        }
    }

    pub fn describe_foundational_with_profile<I>(
        &self,
        subject: ForgeQueryDeclarationFoundationalEvidenceInput<D, I>,
        profile: FoundationalBoundaryEvidenceMaterializationProfile,
    ) -> Result<
        ForgeQueryDeclarationFoundationalEvidence<D, I>,
        ForgeQueryDeclarationFoundationalEvidenceDenial<D, I>,
    >
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        forge_query_declaration_foundational_evidence(
            self.handle_identity_digest(),
            self.operating_context_identity_digest(),
            subject,
            profile,
        )
    }
}

pub enum ForgeQueryDeclarationEntryProgressionError<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    Entry(ForgeQueryDeclarationAdmissionOrLegalityError<D, I>),
    Progression(ForgeQueryDeclarationProgressionTerminalError<D, I>),
}
