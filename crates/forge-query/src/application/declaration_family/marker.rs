use crate::application::{
    ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily,
    ForgeQueryDeclarationGroupedPostureTag, ForgeQueryDeclarationLegalityContract,
    ForgeQueryDeclarationPrimaryAuthorityTag, ForgeQueryDeclarationProgressionContract,
    ForgeQueryDeclarationRouteContract, ForgeQueryDeclarationSignalCompatibilityTag,
    ForgeQueryDomainEntryMarker,
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

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract;

    fn progression_contract(
        _handle_identity_digest: &str,
        _operating_context_identity_digest: &str,
    ) -> ForgeQueryDeclarationProgressionContract {
        ForgeQueryDeclarationProgressionContract::admitted_current()
    }

    fn route_contract() -> ForgeQueryDeclarationRouteContract {
        ForgeQueryDeclarationRouteContract::deferred_auto()
    }
}
