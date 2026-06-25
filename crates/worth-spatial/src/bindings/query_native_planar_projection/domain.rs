use forge_query::facade::{
    ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily, ForgeQueryDeclarationAspectContract,
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationLegalityContract,
    ForgeQueryDeclarationRouteContract, ForgeQueryDomainEntryMarker,
    ForgeQueryDomainOperatingContext, ForgeQueryRelationalTruthAuthority,
    ForgeQuerySignalNotCompatiblePosture, ForgeQuerySingleOnlyGrouping,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProjectPointToCertifiedPlane2DQueryDomain;

impl ForgeQueryDomainEntryMarker for ProjectPointToCertifiedPlane2DQueryDomain {
    fn domain_key(&self) -> &'static str {
        "worth.spatial.project_point_to_certified_plane_2d"
    }

    fn display_name(&self) -> &'static str {
        "WorthSpatialProjectPointToCertifiedPlane2DDomain"
    }

    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[
            ForgeQueryCapabilityFamily::QueryComposition,
            ForgeQueryCapabilityFamily::QueryContext,
        ]
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectPointToCertifiedPlane2DQueryWorld {
    identity: String,
}

impl ProjectPointToCertifiedPlane2DQueryWorld {
    pub fn new(identity: impl Into<String>) -> Self {
        Self {
            identity: identity.into(),
        }
    }
}

impl ForgeQueryDomainOperatingContext<ProjectPointToCertifiedPlane2DQueryDomain>
    for ProjectPointToCertifiedPlane2DQueryWorld
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
            "worth.spatial.project_point_to_certified_plane_2d.{}",
            self.identity
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProjectPointToCertifiedPlane2DDeclarationFamily;

impl ForgeQueryDeclarationFamilyMarker<ProjectPointToCertifiedPlane2DQueryDomain>
    for ProjectPointToCertifiedPlane2DDeclarationFamily
{
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalNotCompatiblePosture;
    type GroupedPosture = ForgeQuerySingleOnlyGrouping;

    fn semantic_family_key() -> &'static str {
        "ProjectPointToCertifiedPlane2D"
    }

    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        crate::query_aspect_contract::declaration_aspect_contract_from_slices(
            &[
                "geometry.planar_projection.source_point_identity",
                "geometry.planar_projection.source_point",
                "geometry.planar_projection.source_point_basis",
                "geometry.planar_projection.local_delta",
                "geometry.planar_projection.local_frame_fact",
                "geometry.planar_projection.local_frame_declaration",
                "geometry.planar_projection.local_frame_envelope",
                "geometry.planar_projection.frame_identity",
                "geometry.planar_projection.transform_chain",
                "geometry.planar_projection.movement_rotation",
                "geometry.planar_projection.tolerance_policy",
            ],
            &[
                "geometry.planar_projection.point_2d",
                "geometry.planar_projection.signed_distance",
                "geometry.planar_projection.counters",
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
