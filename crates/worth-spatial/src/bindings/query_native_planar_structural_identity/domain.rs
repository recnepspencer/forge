use forge_query::facade::{
    ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily, ForgeQueryDeclarationAspectContract,
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationLegalityContract,
    ForgeQueryDeclarationRouteContract, ForgeQueryDomainEntryMarker,
    ForgeQueryDomainOperatingContext, ForgeQueryRelationalTruthAuthority,
    ForgeQuerySignalNotCompatiblePosture, ForgeQuerySingleOnlyGrouping,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlanarStructuralIdentityQueryDomain;

impl ForgeQueryDomainEntryMarker for PlanarStructuralIdentityQueryDomain {
    fn domain_key(&self) -> &'static str {
        "worth.spatial.planar_structural_identity"
    }

    fn display_name(&self) -> &'static str {
        "WorthSpatialPlanarStructuralIdentityDomain"
    }

    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[
            ForgeQueryCapabilityFamily::QueryComposition,
            ForgeQueryCapabilityFamily::QueryContext,
        ]
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarStructuralIdentityQueryWorld {
    identity: String,
}

impl PlanarStructuralIdentityQueryWorld {
    pub fn new(identity: impl Into<String>) -> Self {
        Self {
            identity: identity.into(),
        }
    }
}

impl ForgeQueryDomainOperatingContext<PlanarStructuralIdentityQueryDomain>
    for PlanarStructuralIdentityQueryWorld
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
        format!("worth.spatial.planar_structural_identity.{}", self.identity)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlanarStructuralIdentityDeclarationFamily;

impl ForgeQueryDeclarationFamilyMarker<PlanarStructuralIdentityQueryDomain>
    for PlanarStructuralIdentityDeclarationFamily
{
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalNotCompatiblePosture;
    type GroupedPosture = ForgeQuerySingleOnlyGrouping;

    fn semantic_family_key() -> &'static str {
        "PlanarStructuralIdentity"
    }

    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        crate::query_aspect_contract::declaration_aspect_contract_from_slices(
            &[
                "geometry.planar_structural_identity.boolean_readiness",
                "geometry.planar_structural_identity.canonical_transform_basis",
                "geometry.planar_structural_identity.contrast_identities",
            ],
            &[
                "geometry.planar_structural_identity.structural_digest",
                "geometry.planar_structural_identity.transform_digest",
                "geometry.planar_structural_identity.inspection_rows",
                "geometry.planar_structural_identity.counters",
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
