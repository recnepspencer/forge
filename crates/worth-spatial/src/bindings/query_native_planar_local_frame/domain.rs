use forge_query::facade::{
    ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily, ForgeQueryDeclarationAspectContract,
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationLegalityContract,
    ForgeQueryDeclarationRouteContract, ForgeQueryDomainEntryMarker,
    ForgeQueryDomainOperatingContext, ForgeQueryRelationalTruthAuthority,
    ForgeQuerySignalNotCompatiblePosture, ForgeQuerySingleOnlyGrouping,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlanarLocalFrameCertificateQueryDomain;

impl ForgeQueryDomainEntryMarker for PlanarLocalFrameCertificateQueryDomain {
    fn domain_key(&self) -> &'static str {
        "worth.spatial.planar_local_frame_certificate"
    }

    fn display_name(&self) -> &'static str {
        "WorthSpatialPlanarLocalFrameCertificateDomain"
    }

    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[
            ForgeQueryCapabilityFamily::QueryComposition,
            ForgeQueryCapabilityFamily::QueryContext,
        ]
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarLocalFrameCertificateQueryWorld {
    identity: String,
}

impl PlanarLocalFrameCertificateQueryWorld {
    pub fn new(identity: impl Into<String>) -> Self {
        Self {
            identity: identity.into(),
        }
    }
}

impl ForgeQueryDomainOperatingContext<PlanarLocalFrameCertificateQueryDomain>
    for PlanarLocalFrameCertificateQueryWorld
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
            "worth.spatial.planar_local_frame_certificate.{}",
            self.identity
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlanarLocalFrameCertificateDeclarationFamily;

impl ForgeQueryDeclarationFamilyMarker<PlanarLocalFrameCertificateQueryDomain>
    for PlanarLocalFrameCertificateDeclarationFamily
{
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalNotCompatiblePosture;
    type GroupedPosture = ForgeQuerySingleOnlyGrouping;

    fn semantic_family_key() -> &'static str {
        "PlanarLocalFrameCertificate"
    }

    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        ForgeQueryDeclarationAspectContract::from_slices(
            &[
                "geometry.planar_local_frame.frame_identity",
                "geometry.planar_local_frame.origin",
                "geometry.planar_local_frame.normal",
                "geometry.planar_local_frame.local_feature_scale",
                "geometry.planar_local_frame.world_magnitude",
                "geometry.planar_local_frame.normalization_scale",
                "geometry.planar_local_frame.transform_chain",
                "geometry.planar_local_frame.movement_rotation",
                "geometry.planar_local_frame.tolerance_policy",
                "geometry.planar_local_frame.precision_fact",
                "geometry.planar_local_frame.precision_declaration",
                "geometry.planar_local_frame.precision_envelope",
            ],
            &[
                "geometry.planar_local_frame.axes",
                "geometry.planar_local_frame.scale_separation",
                "geometry.planar_local_frame.counters",
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
