use forge_query::facade::{
    ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily, ForgeQueryDeclarationAspectContract,
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationLegalityContract,
    ForgeQueryDeclarationRouteContract, ForgeQueryDomainEntryMarker,
    ForgeQueryDomainOperatingContext, ForgeQueryRelationalTruthAuthority,
    ForgeQuerySignalCompatiblePosture, ForgeQuerySingleOnlyGrouping,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlanarCleanFailBoundaryQueryDomain;

impl ForgeQueryDomainEntryMarker for PlanarCleanFailBoundaryQueryDomain {
    fn domain_key(&self) -> &'static str {
        "worth.spatial.planar_clean_fail_boundary"
    }

    fn display_name(&self) -> &'static str {
        "WorthSpatialPlanarCleanFailBoundaryDomain"
    }

    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[
            ForgeQueryCapabilityFamily::QueryComposition,
            ForgeQueryCapabilityFamily::QueryContext,
        ]
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarCleanFailBoundaryQueryWorld {
    identity: String,
}

impl PlanarCleanFailBoundaryQueryWorld {
    pub fn new(identity: impl Into<String>) -> Self {
        Self {
            identity: identity.into(),
        }
    }
}

impl ForgeQueryDomainOperatingContext<PlanarCleanFailBoundaryQueryDomain>
    for PlanarCleanFailBoundaryQueryWorld
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
        format!("worth.spatial.planar_clean_fail_boundary.{}", self.identity)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlanarCleanFailBoundaryDeclarationFamily;

impl ForgeQueryDeclarationFamilyMarker<PlanarCleanFailBoundaryQueryDomain>
    for PlanarCleanFailBoundaryDeclarationFamily
{
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQuerySingleOnlyGrouping;

    fn semantic_family_key() -> &'static str {
        "PlanarCleanFailBoundary"
    }

    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        crate::query_aspect_contract::declaration_aspect_contract_from_slices(
            &[
                "geometry.clean_fail.class",
                "geometry.clean_fail.action",
                "geometry.clean_fail.source",
                "geometry.clean_fail.source_detail",
                "geometry.clean_fail.admission_row",
                "geometry.clean_fail.transform_posture",
                "geometry.clean_fail.recovery",
                "geometry.clean_fail.diagnostics",
                "geometry.clean_fail.no_repair",
                "geometry.clean_fail.no_bounded_conversion",
                "geometry.clean_fail.truth_effect",
            ],
            &[
                "geometry.clean_fail.receipt",
                "geometry.clean_fail.inspection_rows",
                "geometry.clean_fail.counters",
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
