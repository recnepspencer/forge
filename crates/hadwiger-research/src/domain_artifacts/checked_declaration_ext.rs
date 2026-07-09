use worth_query::facade::{
    WORTHQueryCanonicalDeclarationArtifact, WORTHQueryDeclaredFamilyChecked,
};

use crate::domain_declarations::HadwigerResearchDeclarationInput;
use crate::query_entry::HadwigerResearchDomainEntry;

pub trait HadwigerDeclaredFamilyCheckedExt<I>
where
    I: HadwigerResearchDeclarationInput,
{
    fn admitted(
        self,
    ) -> Option<WORTHQueryCanonicalDeclarationArtifact<HadwigerResearchDomainEntry, I>>;
}

impl<I> HadwigerDeclaredFamilyCheckedExt<I>
    for WORTHQueryDeclaredFamilyChecked<HadwigerResearchDomainEntry, I>
where
    I: HadwigerResearchDeclarationInput,
{
    fn admitted(
        self,
    ) -> Option<WORTHQueryCanonicalDeclarationArtifact<HadwigerResearchDomainEntry, I>> {
        match self {
            WORTHQueryDeclaredFamilyChecked::Admitted(declaration) => Some(declaration),
            _ => None,
        }
    }
}
