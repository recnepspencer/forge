use crate::application::{
    worth_query_declaration_foundational_evidence, WorthQueryDeclarationFoundationalEvidence,
    WorthQueryDeclarationFoundationalEvidenceChecked,
    WorthQueryDeclarationFoundationalEvidenceDenial,
    WorthQueryDeclarationFoundationalEvidenceInput, WorthQueryDeclarationInput,
    WorthQueryDomainEntryMarker,
};
use worth_foundational::facade::FoundationalBoundaryEvidenceMaterializationProfile;

use super::WorthQueryAdmittedConfiguredDomainHandle;
use crate::application::WorthQueryDomainOperatingContext;

impl<D: WorthQueryDomainEntryMarker, C: WorthQueryDomainOperatingContext<D>>
    WorthQueryAdmittedConfiguredDomainHandle<D, C>
{
    pub fn describe_foundational<I>(
        &self,
        subject: WorthQueryDeclarationFoundationalEvidenceInput<D, I>,
    ) -> Result<
        WorthQueryDeclarationFoundationalEvidence<D, I>,
        WorthQueryDeclarationFoundationalEvidenceDenial<D, I>,
    >
    where
        I: WorthQueryDeclarationInput<D>,
    {
        self.describe_foundational_with_profile(
            subject,
            FoundationalBoundaryEvidenceMaterializationProfile::FullDescriptiveRichness,
        )
    }

    pub fn describe_foundational_checked<I>(
        &self,
        subject: WorthQueryDeclarationFoundationalEvidenceInput<D, I>,
    ) -> WorthQueryDeclarationFoundationalEvidenceChecked<D, I>
    where
        I: WorthQueryDeclarationInput<D>,
    {
        match self.describe_foundational(subject) {
            Ok(evidence) => WorthQueryDeclarationFoundationalEvidenceChecked::Described(evidence),
            Err(denial) => {
                WorthQueryDeclarationFoundationalEvidenceChecked::ConstructionDenied(denial)
            }
        }
    }

    pub fn describe_foundational_with_profile<I>(
        &self,
        subject: WorthQueryDeclarationFoundationalEvidenceInput<D, I>,
        profile: FoundationalBoundaryEvidenceMaterializationProfile,
    ) -> Result<
        WorthQueryDeclarationFoundationalEvidence<D, I>,
        WorthQueryDeclarationFoundationalEvidenceDenial<D, I>,
    >
    where
        I: WorthQueryDeclarationInput<D>,
    {
        let world_basis = self.retained_world_basis();
        worth_query_declaration_foundational_evidence(&world_basis, subject, profile)
    }
}
