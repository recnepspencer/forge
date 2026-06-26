use forge_query::facade::{
    ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily, ForgeQueryDeclarationAspectContract,
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationLegalityContract,
    ForgeQueryDeclarationRouteContract, ForgeQueryDomainEntryMarker,
    ForgeQueryDomainOperatingContext, ForgeQueryRelationalTruthAuthority,
    ForgeQuerySignalCompatiblePosture, ForgeQuerySingleOnlyGrouping,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlanarRecoveryPostureQueryDomain;

impl ForgeQueryDomainEntryMarker for PlanarRecoveryPostureQueryDomain {
    fn domain_key(&self) -> &'static str {
        "worth.spatial.planar_recovery_posture"
    }

    fn display_name(&self) -> &'static str {
        "WorthSpatialPlanarRecoveryPostureDomain"
    }

    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[
            ForgeQueryCapabilityFamily::QueryComposition,
            ForgeQueryCapabilityFamily::QueryContext,
        ]
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarRecoveryPostureQueryWorld {
    identity: String,
}

impl PlanarRecoveryPostureQueryWorld {
    pub fn new(identity: impl Into<String>) -> Self {
        Self {
            identity: identity.into(),
        }
    }
}

impl ForgeQueryDomainOperatingContext<PlanarRecoveryPostureQueryDomain>
    for PlanarRecoveryPostureQueryWorld
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
        format!("worth.spatial.planar_recovery_posture.{}", self.identity)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlanarRecoveryPostureDeclarationFamily;

impl ForgeQueryDeclarationFamilyMarker<PlanarRecoveryPostureQueryDomain>
    for PlanarRecoveryPostureDeclarationFamily
{
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQuerySingleOnlyGrouping;

    fn semantic_family_key() -> &'static str {
        "PlanarRecoveryPosture"
    }

    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        crate::query_aspect_contract::declaration_aspect_contract_from_slices(
            &[
                "geometry.planar_recovery.source",
                "geometry.planar_recovery.blocker",
                "geometry.planar_recovery.truth_effect",
                "geometry.planar_recovery.basis_receipts",
            ],
            &[
                "geometry.planar_recovery.action",
                "geometry.planar_recovery.receipt",
                "geometry.planar_recovery.counters",
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
