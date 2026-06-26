use forge_query::facade::{
    ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily, ForgeQueryDeclarationAspectContract,
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationLegalityContract,
    ForgeQueryDeclarationRouteContract, ForgeQueryDomainEntryMarker,
    ForgeQueryDomainOperatingContext, ForgeQueryRelationalTruthAuthority,
    ForgeQuerySignalNotCompatiblePosture, ForgeQuerySingleOnlyGrouping,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CertifiedSignedArea2DQueryDomain;

impl ForgeQueryDomainEntryMarker for CertifiedSignedArea2DQueryDomain {
    fn domain_key(&self) -> &'static str {
        "worth.spatial.certified_signed_area_2d"
    }

    fn display_name(&self) -> &'static str {
        "WorthSpatialCertifiedSignedArea2DDomain"
    }

    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[
            ForgeQueryCapabilityFamily::QueryComposition,
            ForgeQueryCapabilityFamily::QueryContext,
        ]
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CertifiedSignedArea2DQueryWorld {
    identity: String,
}

impl CertifiedSignedArea2DQueryWorld {
    pub fn new(identity: impl Into<String>) -> Self {
        Self {
            identity: identity.into(),
        }
    }
}

impl ForgeQueryDomainOperatingContext<CertifiedSignedArea2DQueryDomain>
    for CertifiedSignedArea2DQueryWorld
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
        format!("worth.spatial.certified_signed_area_2d.{}", self.identity)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CertifiedSignedArea2DDeclarationFamily;

impl ForgeQueryDeclarationFamilyMarker<CertifiedSignedArea2DQueryDomain>
    for CertifiedSignedArea2DDeclarationFamily
{
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalNotCompatiblePosture;
    type GroupedPosture = ForgeQuerySingleOnlyGrouping;

    fn semantic_family_key() -> &'static str {
        "CertifiedSignedArea2D"
    }

    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        crate::query_aspect_contract::declaration_aspect_contract_from_slices(
            &[
                "geometry.signed_area_2d.primary_loop",
                "geometry.signed_area_2d.planar_neighborhood",
                "geometry.signed_area_2d.frame_identity",
                "geometry.signed_area_2d.movement_rotation",
                "geometry.signed_area_2d.tolerance_policy",
                "geometry.signed_area_2d.winding_fact",
                "geometry.signed_area_2d.precision_fact",
                "geometry.signed_area_2d.degeneracy_policy",
            ],
            &[
                "geometry.signed_area_2d.orientation",
                "geometry.signed_area_2d.degeneracy",
                "geometry.signed_area_2d.signed_area_twice",
                "geometry.signed_area_2d.localized_cause",
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
