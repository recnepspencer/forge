use crate::application::{
    ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily,
    ForgeQueryDeclarationBridgeContinuationContract, ForgeQueryDeclarationGroupedPostureTag,
    ForgeQueryDeclarationLegalityContract, ForgeQueryDeclarationPrimaryAuthorityTag,
    ForgeQueryDeclarationProgressionContract, ForgeQueryDeclarationRelationalTruthContract,
    ForgeQueryDeclarationRouteContract, ForgeQueryDeclarationSignalCompatibilityContract,
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

    fn bridge_continuation_contract() -> Option<ForgeQueryDeclarationBridgeContinuationContract> {
        match Self::taxonomy().primary_authority_family() {
            crate::application::ForgeQueryDeclarationPrimaryAuthorityFamily::BridgeContinuation => {
                Some(ForgeQueryDeclarationBridgeContinuationContract::runtime_route_current())
            }
            _ => None,
        }
    }

    fn relational_truth_contract() -> Option<ForgeQueryDeclarationRelationalTruthContract> {
        match Self::taxonomy().primary_authority_family() {
            crate::application::ForgeQueryDeclarationPrimaryAuthorityFamily::RelationalTruth => {
                Some(ForgeQueryDeclarationRelationalTruthContract::authoritative_current_truth())
            }
            _ => None,
        }
    }

    fn signal_compatibility_contract() -> Option<ForgeQueryDeclarationSignalCompatibilityContract> {
        None
    }
}
