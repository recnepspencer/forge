use forge_query::facade::{
    ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily, ForgeQueryDeclarationAspectContract,
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationLegalityContract,
    ForgeQueryDeclarationRouteContract, ForgeQueryDomainEntryMarker,
    ForgeQueryDomainOperatingContext, ForgeQueryRelationalTruthAuthority,
    ForgeQuerySignalCompatiblePosture, ForgeQuerySingleOnlyGrouping,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlanarTopologyContractCompletenessQueryDomain;

impl ForgeQueryDomainEntryMarker for PlanarTopologyContractCompletenessQueryDomain {
    fn domain_key(&self) -> &'static str {
        "worth.spatial.planar_topology_contract_completeness"
    }

    fn display_name(&self) -> &'static str {
        "WorthSpatialPlanarTopologyContractCompletenessDomain"
    }

    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[
            ForgeQueryCapabilityFamily::QueryComposition,
            ForgeQueryCapabilityFamily::QueryContext,
        ]
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarTopologyContractCompletenessQueryWorld {
    identity: String,
}

impl PlanarTopologyContractCompletenessQueryWorld {
    pub fn new(identity: impl Into<String>) -> Self {
        Self {
            identity: identity.into(),
        }
    }
}

impl ForgeQueryDomainOperatingContext<PlanarTopologyContractCompletenessQueryDomain>
    for PlanarTopologyContractCompletenessQueryWorld
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
        format!(
            "worth.spatial.planar_topology_contract_completeness.{}",
            self.identity
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlanarTopologyContractCompletenessDeclarationFamily;

impl ForgeQueryDeclarationFamilyMarker<PlanarTopologyContractCompletenessQueryDomain>
    for PlanarTopologyContractCompletenessDeclarationFamily
{
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQuerySingleOnlyGrouping;

    fn semantic_family_key() -> &'static str {
        "PlanarTopologyContractCompleteness"
    }

    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        ForgeQueryDeclarationAspectContract::from_slices(
            &[
                "geometry.planar_topology_contract.topology_receipt",
                "geometry.planar_topology_contract.declared_query_surface",
                "geometry.planar_topology_contract.planar_neighborhood",
            ],
            &[
                "geometry.planar_topology_contract.completeness_fact_digest",
                "geometry.planar_topology_contract.inspection_rows",
                "geometry.planar_topology_contract.projection_consumption",
                "geometry.planar_topology_contract.counters",
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
