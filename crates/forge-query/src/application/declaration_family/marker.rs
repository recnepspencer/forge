use crate::application::{
    ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily,
    ForgeQueryDeclarationGroupedPostureTag, ForgeQueryDeclarationPrimaryAuthorityTag,
    ForgeQueryDeclarationSignalCompatibilityTag, ForgeQueryDomainEntryMarker,
};

use super::taxonomy::ForgeQueryDeclarationFamilyTaxonomy;

pub trait ForgeQueryDeclarationFamilyMarker<D: ForgeQueryDomainEntryMarker> {
    type PrimaryAuthority: ForgeQueryDeclarationPrimaryAuthorityTag;
    type SignalCompatibility: ForgeQueryDeclarationSignalCompatibilityTag;
    type GroupedPosture: ForgeQueryDeclarationGroupedPostureTag;

    fn semantic_family_key() -> &'static str;

    fn required_capability_families() -> &'static [ForgeQueryCapabilityFamily] {
        &[]
    }

    fn required_config_sections() -> &'static [ForgeQueryConfigSectionFamily] {
        &[]
    }

    fn taxonomy() -> ForgeQueryDeclarationFamilyTaxonomy {
        ForgeQueryDeclarationFamilyTaxonomy::from_type_tags::<
            Self::PrimaryAuthority,
            Self::SignalCompatibility,
            Self::GroupedPosture,
        >()
    }
}
