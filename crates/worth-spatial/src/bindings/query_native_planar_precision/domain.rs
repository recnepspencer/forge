use forge_query::facade::{
    ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily, ForgeQueryDeclarationAspectContract,
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationLegalityContract,
    ForgeQueryDeclarationRouteContract, ForgeQueryDomainEntryMarker,
    ForgeQueryDomainOperatingContext, ForgeQueryRelationalTruthAuthority,
    ForgeQuerySignalNotCompatiblePosture, ForgeQuerySingleOnlyGrouping,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlanarPrecisionCertificationQueryDomain;

impl ForgeQueryDomainEntryMarker for PlanarPrecisionCertificationQueryDomain {
    fn domain_key(&self) -> &'static str {
        "worth.spatial.planar_precision_certification"
    }

    fn display_name(&self) -> &'static str {
        "WorthSpatialPlanarPrecisionCertificationDomain"
    }

    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[
            ForgeQueryCapabilityFamily::QueryComposition,
            ForgeQueryCapabilityFamily::QueryContext,
        ]
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarPrecisionCertificationQueryWorld {
    identity: String,
}

impl PlanarPrecisionCertificationQueryWorld {
    pub fn new(identity: impl Into<String>) -> Self {
        Self {
            identity: identity.into(),
        }
    }
}

impl ForgeQueryDomainOperatingContext<PlanarPrecisionCertificationQueryDomain>
    for PlanarPrecisionCertificationQueryWorld
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
            "worth.spatial.planar_precision_certification.{}",
            self.identity
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlanarPrecisionCertificationDeclarationFamily;

impl ForgeQueryDeclarationFamilyMarker<PlanarPrecisionCertificationQueryDomain>
    for PlanarPrecisionCertificationDeclarationFamily
{
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalNotCompatiblePosture;
    type GroupedPosture = ForgeQuerySingleOnlyGrouping;

    fn semantic_family_key() -> &'static str {
        "PlanarPrecisionCertification"
    }

    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        crate::query_aspect_contract::declaration_aspect_contract_from_slices(
            &[
                "geometry.planar_precision.predicate_fact",
                "geometry.planar_precision.local_frame",
                "geometry.planar_precision.topology_basis",
                "geometry.planar_precision.movement_rotation",
                "geometry.planar_precision.tolerance_policy",
                "geometry.planar_precision.local_feature_scale",
                "geometry.planar_precision.world_magnitude",
                "geometry.planar_precision.normalization_scale",
            ],
            &[
                "geometry.planar_precision.precision_escalation",
                "geometry.planar_precision.scale_separation",
                "geometry.planar_precision.counters",
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
