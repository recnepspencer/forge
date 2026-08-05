use super::taxonomy::WorthQueryDeclarationFamilyTaxonomy;
use crate::application::{
    WorthQueryAsyncDeclarationSupport, WorthQueryCapabilityFamily, WorthQueryConfigSectionFamily,
    WorthQueryDeclarationAspectContract, WorthQueryDeclarationAspectCoverage,
    WorthQueryDeclarationBridgeContinuationContract, WorthQueryDeclarationGroupedPostureTag,
    WorthQueryDeclarationLegalityContract, WorthQueryDeclarationPrimaryAuthorityTag,
    WorthQueryDeclarationProgressionContract, WorthQueryDeclarationRelationalTruthContract,
    WorthQueryDeclarationRouteContract, WorthQueryDeclarationSignalCompatibilityContract,
    WorthQueryDeclarationSignalCompatibilityTag, WorthQueryDomainEntryMarker,
    WorthQueryTemporalDeclarationSupport,
};

pub trait WorthQueryDeclarationFamilyMarker<D: WorthQueryDomainEntryMarker> {
    type PrimaryAuthority: WorthQueryDeclarationPrimaryAuthorityTag;
    type SignalCompatibility: WorthQueryDeclarationSignalCompatibilityTag;
    type GroupedPosture: WorthQueryDeclarationGroupedPostureTag;

    fn semantic_family_key() -> &'static str;

    fn required_capability_families() -> &'static [WorthQueryCapabilityFamily] {
        &[]
    }

    fn required_config_sections() -> &'static [WorthQueryConfigSectionFamily] {
        &[]
    }

    fn aspect_contract() -> WorthQueryDeclarationAspectContract {
        WorthQueryDeclarationAspectContract::empty()
    }

    fn aspect_coverage() -> WorthQueryDeclarationAspectCoverage {
        Self::aspect_contract().default_coverage()
    }

    fn temporal_declaration_support() -> WorthQueryTemporalDeclarationSupport {
        WorthQueryTemporalDeclarationSupport::Unsupported
    }

    fn async_declaration_support() -> WorthQueryAsyncDeclarationSupport {
        WorthQueryAsyncDeclarationSupport::Unsupported
    }

    fn taxonomy() -> WorthQueryDeclarationFamilyTaxonomy {
        WorthQueryDeclarationFamilyTaxonomy::from_type_tags::<
            Self::PrimaryAuthority,
            Self::SignalCompatibility,
            Self::GroupedPosture,
        >()
    }

    fn legality_contract() -> WorthQueryDeclarationLegalityContract;

    fn progression_contract(
        _handle_identity_digest: &str,
        _operating_context_identity_digest: &str,
    ) -> WorthQueryDeclarationProgressionContract {
        WorthQueryDeclarationProgressionContract::admitted_current()
    }

    fn route_contract() -> WorthQueryDeclarationRouteContract {
        WorthQueryDeclarationRouteContract::deferred_auto()
    }

    fn bridge_continuation_contract() -> Option<WorthQueryDeclarationBridgeContinuationContract> {
        match Self::taxonomy().primary_authority_family() {
            crate::application::WorthQueryDeclarationPrimaryAuthorityFamily::BridgeContinuation => {
                Some(WorthQueryDeclarationBridgeContinuationContract::runtime_route_current())
            }
            _ => None,
        }
    }

    fn relational_truth_contract() -> Option<WorthQueryDeclarationRelationalTruthContract> {
        match Self::taxonomy().primary_authority_family() {
            crate::application::WorthQueryDeclarationPrimaryAuthorityFamily::RelationalTruth => {
                Some(WorthQueryDeclarationRelationalTruthContract::authoritative_current_truth())
            }
            _ => None,
        }
    }

    fn signal_compatibility_contract() -> Option<WorthQueryDeclarationSignalCompatibilityContract> {
        None
    }
}
