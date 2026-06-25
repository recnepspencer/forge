use forge_query::facade::{
    ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily, ForgeQueryDeclarationAspectContract,
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationLegalityContract,
    ForgeQueryDeclarationRouteContract, ForgeQueryDomainEntryMarker,
    ForgeQueryDomainOperatingContext, ForgeQueryRelationalTruthAuthority,
    ForgeQuerySignalNotCompatiblePosture, ForgeQuerySingleOnlyGrouping,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CoplanarOverlapContractQueryDomain;

impl ForgeQueryDomainEntryMarker for CoplanarOverlapContractQueryDomain {
    fn domain_key(&self) -> &'static str {
        "worth.spatial.coplanar_overlap_contract"
    }

    fn display_name(&self) -> &'static str {
        "WorthSpatialCoplanarOverlapContractDomain"
    }

    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[
            ForgeQueryCapabilityFamily::QueryComposition,
            ForgeQueryCapabilityFamily::QueryContext,
        ]
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CoplanarOverlapContractQueryWorld {
    identity: String,
}

impl CoplanarOverlapContractQueryWorld {
    pub fn new(identity: impl Into<String>) -> Self {
        Self {
            identity: identity.into(),
        }
    }
}

impl ForgeQueryDomainOperatingContext<CoplanarOverlapContractQueryDomain>
    for CoplanarOverlapContractQueryWorld
{
    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[
            ForgeQueryCapabilityFamily::HistoricalEvaluation,
            ForgeQueryCapabilityFamily::QueryComposition,
            ForgeQueryCapabilityFamily::QueryContext,
        ]
    }

    fn required_config_sections(&self) -> &'static [ForgeQueryConfigSectionFamily] {
        &[
            ForgeQueryConfigSectionFamily::Query,
            ForgeQueryConfigSectionFamily::Relational,
        ]
    }

    fn context_identity_digest(&self) -> String {
        format!("worth.spatial.coplanar_overlap_contract.{}", self.identity)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CoplanarOverlapContractDeclarationFamily;

impl ForgeQueryDeclarationFamilyMarker<CoplanarOverlapContractQueryDomain>
    for CoplanarOverlapContractDeclarationFamily
{
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalNotCompatiblePosture;
    type GroupedPosture = ForgeQuerySingleOnlyGrouping;

    fn semantic_family_key() -> &'static str {
        "CoplanarOverlapContractExtractor"
    }

    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        crate::query_aspect_contract::declaration_aspect_contract_from_slices(
            &[
                "geometry.coplanar_overlap.pair",
                "geometry.coplanar_overlap.planar_neighborhood",
                "geometry.coplanar_overlap.policy",
                "geometry.coplanar_overlap.first_area_fact",
                "geometry.coplanar_overlap.second_area_fact",
            ],
            &[
                "geometry.coplanar_overlap.shared_interval",
                "geometry.coplanar_overlap.overlap_island",
                "geometry.coplanar_overlap.containment",
                "geometry.coplanar_overlap.ambiguous_contact",
                "geometry.coplanar_overlap.policy_exit",
            ],
            &[],
            &[],
            &[],
        )
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> ForgeQueryDeclarationRouteContract {
        ForgeQueryDeclarationRouteContract::relational_only()
    }
}
