use forge_query::facade::{
    ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily, ForgeQueryDeclarationAspectContract,
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationLegalityContract,
    ForgeQueryDeclarationRouteContract, ForgeQueryDomainEntryMarker,
    ForgeQueryDomainOperatingContext, ForgeQueryRelationalTruthAuthority,
    ForgeQuerySignalCompatiblePosture, ForgeQuerySingleOnlyGrouping,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProjectionConsumedPlanarFactsQueryDomain;

impl ForgeQueryDomainEntryMarker for ProjectionConsumedPlanarFactsQueryDomain {
    fn domain_key(&self) -> &'static str {
        "worth.spatial.projection_consumed_planar_facts"
    }

    fn display_name(&self) -> &'static str {
        "WorthSpatialProjectionConsumedPlanarFactsDomain"
    }

    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[
            ForgeQueryCapabilityFamily::QueryComposition,
            ForgeQueryCapabilityFamily::QueryContext,
        ]
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectionConsumedPlanarFactsQueryWorld {
    identity: String,
}

impl ProjectionConsumedPlanarFactsQueryWorld {
    pub fn new(identity: impl Into<String>) -> Self {
        Self {
            identity: identity.into(),
        }
    }
}

impl ForgeQueryDomainOperatingContext<ProjectionConsumedPlanarFactsQueryDomain>
    for ProjectionConsumedPlanarFactsQueryWorld
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
            "worth.spatial.projection_consumed_planar_facts.{}",
            self.identity
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProjectionConsumedPlanarFactsDeclarationFamily;

impl ForgeQueryDeclarationFamilyMarker<ProjectionConsumedPlanarFactsQueryDomain>
    for ProjectionConsumedPlanarFactsDeclarationFamily
{
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQuerySingleOnlyGrouping;

    fn semantic_family_key() -> &'static str {
        "ProjectionConsumedPlanarFacts"
    }

    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        crate::query_aspect_contract::declaration_aspect_contract_from_slices(
            &[
                "geometry.planar_projection_consumption.retained_source",
                "geometry.planar_projection_consumption.structural_identity",
                "geometry.planar_projection_consumption.motion_posture",
                "geometry.planar_projection_consumption.topology_contract",
                "geometry.planar_projection_consumption.projection_receipts",
                "geometry.planar_projection_consumption.materialization_basis",
            ],
            &[
                "geometry.planar_projection_consumption.projected_fact",
                "geometry.planar_projection_consumption.receipt",
                "geometry.planar_projection_consumption.counters",
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
