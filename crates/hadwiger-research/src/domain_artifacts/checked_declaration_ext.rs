use forge_query::facade::{
    ForgeQueryCanonicalDeclarationArtifact, ForgeQueryDeclaredFamilyChecked,
};

use crate::domain_declarations::HadwigerResearchDeclarationInput;
use crate::query_entry::HadwigerResearchDomainEntry;

pub trait HadwigerDeclaredFamilyCheckedExt<I>
where
    I: HadwigerResearchDeclarationInput,
{
    fn admitted(
        self,
    ) -> Option<ForgeQueryCanonicalDeclarationArtifact<HadwigerResearchDomainEntry, I>>;
}

impl<I> HadwigerDeclaredFamilyCheckedExt<I>
    for ForgeQueryDeclaredFamilyChecked<HadwigerResearchDomainEntry, I>
where
    I: HadwigerResearchDeclarationInput,
{
    fn admitted(
        self,
    ) -> Option<ForgeQueryCanonicalDeclarationArtifact<HadwigerResearchDomainEntry, I>> {
        match self {
            ForgeQueryDeclaredFamilyChecked::Admitted(declaration) => Some(declaration),
            _ => None,
        }
    }
}
