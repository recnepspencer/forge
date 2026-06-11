use forge_query::facade::{
    ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily, ForgeQueryDeclarationAspectContract,
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationLegalityContract,
    ForgeQueryDeclarationRouteContract, ForgeQueryDomainEntryMarker,
    ForgeQueryDomainOperatingContext, ForgeQueryRelationalTruthAuthority,
    ForgeQuerySignalNotCompatiblePosture, ForgeQuerySingleOnlyGrouping,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CertifiedPolygonWinding2DQueryDomain;

impl ForgeQueryDomainEntryMarker for CertifiedPolygonWinding2DQueryDomain {
    fn domain_key(&self) -> &'static str {
        "worth.spatial.certified_polygon_winding_2d"
    }

    fn display_name(&self) -> &'static str {
        "WorthSpatialCertifiedPolygonWinding2DDomain"
    }

    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[
            ForgeQueryCapabilityFamily::QueryComposition,
            ForgeQueryCapabilityFamily::QueryContext,
        ]
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CertifiedPolygonWinding2DQueryWorld {
    identity: String,
}

impl CertifiedPolygonWinding2DQueryWorld {
    pub fn new(identity: impl Into<String>) -> Self {
        Self {
            identity: identity.into(),
        }
    }
}

impl ForgeQueryDomainOperatingContext<CertifiedPolygonWinding2DQueryDomain>
    for CertifiedPolygonWinding2DQueryWorld
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
            "worth.spatial.certified_polygon_winding_2d.{}",
            self.identity
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CertifiedPolygonWinding2DDeclarationFamily;

impl ForgeQueryDeclarationFamilyMarker<CertifiedPolygonWinding2DQueryDomain>
    for CertifiedPolygonWinding2DDeclarationFamily
{
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalNotCompatiblePosture;
    type GroupedPosture = ForgeQuerySingleOnlyGrouping;

    fn semantic_family_key() -> &'static str {
        "CertifiedPolygonWinding2D"
    }

    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        ForgeQueryDeclarationAspectContract::from_slices(
            &[
                "geometry.polygon_winding_2d.primary_loop",
                "geometry.polygon_winding_2d.planar_neighborhood",
                "geometry.polygon_winding_2d.winding_policy",
                "geometry.polygon_winding_2d.local_frame_fact",
                "geometry.polygon_winding_2d.movement_rotation",
                "geometry.polygon_winding_2d.tolerance_policy",
                "geometry.polygon_winding_2d.loop.topology_basis",
                "geometry.polygon_winding_2d.vertex.projection_fact",
            ],
            &[
                "geometry.polygon_winding_2d.primary_winding",
                "geometry.polygon_winding_2d.loop.containment",
                "geometry.polygon_winding_2d.winding_predicate.fact",
                "geometry.polygon_winding_2d.segment_contact.fact",
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
