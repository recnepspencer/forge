use worth_query::facade::foundation::{
    WorthQueryCanonicalDeclarationArtifact, WorthQueryDeclaredFamilyChecked,
};

use crate::domain_declarations::HadwigerResearchDeclarationInput;
use crate::query_entry::HadwigerResearchDomainEntry;

pub trait HadwigerDeclaredFamilyCheckedExt<I>
where
    I: HadwigerResearchDeclarationInput,
{
    fn admitted(
        self,
    ) -> Option<WorthQueryCanonicalDeclarationArtifact<HadwigerResearchDomainEntry, I>>;
}

impl<I> HadwigerDeclaredFamilyCheckedExt<I>
    for WorthQueryDeclaredFamilyChecked<HadwigerResearchDomainEntry, I>
where
    I: HadwigerResearchDeclarationInput,
{
    fn admitted(
        self,
    ) -> Option<WorthQueryCanonicalDeclarationArtifact<HadwigerResearchDomainEntry, I>> {
        match self {
            WorthQueryDeclaredFamilyChecked::Admitted(declaration) => Some(declaration),
            _ => None,
        }
    }
}
