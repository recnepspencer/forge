use crate::application::ForgeQueryDomainEntryMarker;

use super::taxonomy::ForgeQueryDeclarationFamilyTaxonomy;

pub trait ForgeQueryDeclarationFamilyMarker<D: ForgeQueryDomainEntryMarker> {
    fn semantic_family_key() -> &'static str;

    fn taxonomy() -> ForgeQueryDeclarationFamilyTaxonomy;
}
