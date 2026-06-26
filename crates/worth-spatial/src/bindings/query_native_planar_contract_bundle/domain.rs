use forge_query::facade::{
    ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily, ForgeQueryDeclarationAspectContract,
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationLegalityContract,
    ForgeQueryDeclarationRouteContract, ForgeQueryDomainEntryMarker,
    ForgeQueryDomainOperatingContext, ForgeQueryRelationalTruthAuthority,
    ForgeQuerySignalNotCompatiblePosture, ForgeQuerySingleOnlyGrouping,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlanarContractBundleValidationQueryDomain;

impl ForgeQueryDomainEntryMarker for PlanarContractBundleValidationQueryDomain {
    fn domain_key(&self) -> &'static str {
        "worth.spatial.planar_contract_bundle_validation"
    }

    fn display_name(&self) -> &'static str {
        "WorthSpatialPlanarContractBundleValidationDomain"
    }

    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[
            ForgeQueryCapabilityFamily::QueryComposition,
            ForgeQueryCapabilityFamily::QueryContext,
        ]
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarContractBundleValidationQueryWorld {
    identity: String,
}

impl PlanarContractBundleValidationQueryWorld {
    pub fn new(identity: impl Into<String>) -> Self {
        Self {
            identity: identity.into(),
        }
    }
}

impl ForgeQueryDomainOperatingContext<PlanarContractBundleValidationQueryDomain>
    for PlanarContractBundleValidationQueryWorld
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
            "worth.spatial.planar_contract_bundle_validation.{}",
            self.identity
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlanarContractBundleValidationDeclarationFamily;

impl ForgeQueryDeclarationFamilyMarker<PlanarContractBundleValidationQueryDomain>
    for PlanarContractBundleValidationDeclarationFamily
{
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalNotCompatiblePosture;
    type GroupedPosture = ForgeQuerySingleOnlyGrouping;

    fn semantic_family_key() -> &'static str {
        "PlanarContractBundleValidator"
    }

    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        crate::query_aspect_contract::declaration_aspect_contract_from_slices(
            &[
                "geometry.planar_bundle.admission",
                "geometry.planar_bundle.topology_basis",
                "geometry.planar_bundle.movement_rotation",
                "geometry.planar_bundle.diagnostic_scope",
                "geometry.planar_bundle.retained_facts",
                "geometry.planar_bundle.projection_consumption",
                "geometry.planar_bundle.m7_structural_identity",
                "geometry.planar_bundle.m7_motion_posture",
                "geometry.planar_bundle.m7_recovery",
                "geometry.planar_bundle.m7_diagnostics",
                "geometry.planar_bundle.m7_clean_fail_boundary",
                "geometry.planar_bundle.m7_support_posture",
            ],
            &[
                "geometry.planar_bundle.boolean_readiness",
                "geometry.planar_bundle.family_rows",
                "geometry.planar_bundle.m7_closeout_rows",
                "geometry.planar_bundle.counters",
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
