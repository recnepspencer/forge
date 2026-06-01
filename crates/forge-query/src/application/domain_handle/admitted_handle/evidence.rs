use crate::application::{
    forge_query_declaration_foundational_evidence, ForgeQueryDeclarationFoundationalEvidence,
    ForgeQueryDeclarationFoundationalEvidenceChecked,
    ForgeQueryDeclarationFoundationalEvidenceDenial,
    ForgeQueryDeclarationFoundationalEvidenceInput, ForgeQueryDeclarationInput,
    ForgeQueryDomainEntryMarker,
};
use forge_foundational::facade::FoundationalBoundaryEvidenceMaterializationProfile;

use super::ForgeQueryAdmittedConfiguredDomainHandle;
use crate::application::ForgeQueryDomainOperatingContext;

impl<D: ForgeQueryDomainEntryMarker, C: ForgeQueryDomainOperatingContext<D>>
    ForgeQueryAdmittedConfiguredDomainHandle<D, C>
{
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
