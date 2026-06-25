use forge_query::facade::{
    ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily, ForgeQueryDeclarationAspectContract,
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationLegalityContract,
    ForgeQueryDeclarationRouteContract, ForgeQueryDomainEntryMarker,
    ForgeQueryDomainOperatingContext, ForgeQueryRelationalTruthAuthority,
    ForgeQuerySignalCompatiblePosture, ForgeQuerySingleOnlyGrouping,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct M6PlanarCloseoutQueryDomain;

impl ForgeQueryDomainEntryMarker for M6PlanarCloseoutQueryDomain {
    fn domain_key(&self) -> &'static str {
        "worth.spatial.planar_m6_closeout"
    }

    fn display_name(&self) -> &'static str {
        "WorthSpatialPlanarM6CloseoutDomain"
    }

    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[ForgeQueryCapabilityFamily::QueryComposition]
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct M6PlanarCloseoutQueryWorld {
    identity: String,
}

impl M6PlanarCloseoutQueryWorld {
    pub fn new(identity: impl Into<String>) -> Self {
        Self {
            identity: identity.into(),
        }
    }
}

impl ForgeQueryDomainOperatingContext<M6PlanarCloseoutQueryDomain> for M6PlanarCloseoutQueryWorld {
    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[
            ForgeQueryCapabilityFamily::QueryComposition,
            ForgeQueryCapabilityFamily::QueryRead,
        ]
    }

    fn required_config_sections(&self) -> &'static [ForgeQueryConfigSectionFamily] {
        &[
            ForgeQueryConfigSectionFamily::Query,
            ForgeQueryConfigSectionFamily::Relational,
        ]
    }

    fn context_identity_digest(&self) -> String {
        format!("worth.spatial.planar_m6_closeout.{}", self.identity)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct M6PlanarCloseoutDeclarationFamily;

impl ForgeQueryDeclarationFamilyMarker<M6PlanarCloseoutQueryDomain>
    for M6PlanarCloseoutDeclarationFamily
{
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQuerySingleOnlyGrouping;

    fn semantic_family_key() -> &'static str {
        "M6PlanarCloseout"
    }

    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        crate::query_aspect_contract::declaration_aspect_contract_from_slices(
            &[
                "geometry.planar_m6_closeout.m7_readiness",
                "geometry.planar_m6_closeout.premetaboss",
                "geometry.planar_m6_closeout.legacy_deletion",
                "geometry.planar_m6_closeout.legacy_fixture_fence",
                "geometry.planar_m6_closeout.query_boundary",
            ],
            &[
                "geometry.planar_m6_closeout.receipt",
                "geometry.planar_m6_closeout.counters",
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
