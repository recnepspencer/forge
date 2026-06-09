use forge_query::facade::{
    ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily, ForgeQueryDeclarationAspectContract,
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationLegalityContract,
    ForgeQueryDeclarationRouteContract, ForgeQueryDomainEntryMarker,
    ForgeQueryDomainOperatingContext, ForgeQueryNeighborhoodCapableGrouping,
    ForgeQueryRelationalTruthAuthority, ForgeQuerySignalNotCompatiblePosture,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PrimitiveBindingQueryDomain;

impl ForgeQueryDomainEntryMarker for PrimitiveBindingQueryDomain {
    fn domain_key(&self) -> &'static str {
        "worth.spatial.binding"
    }

    fn display_name(&self) -> &'static str {
        "WorthSpatialBindingDomain"
    }

    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[ForgeQueryCapabilityFamily::QueryComposition]
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveBindingQueryWorld {
    identity: String,
}

impl PrimitiveBindingQueryWorld {
    pub fn new(identity: impl Into<String>) -> Self {
        Self {
            identity: identity.into(),
        }
    }
}

impl ForgeQueryDomainOperatingContext<PrimitiveBindingQueryDomain> for PrimitiveBindingQueryWorld {
    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[
            ForgeQueryCapabilityFamily::HistoricalEvaluation,
            ForgeQueryCapabilityFamily::QueryComposition,
        ]
    }

    fn required_config_sections(&self) -> &'static [ForgeQueryConfigSectionFamily] {
        &[
            ForgeQueryConfigSectionFamily::Query,
            ForgeQueryConfigSectionFamily::Relational,
        ]
    }

    fn context_identity_digest(&self) -> String {
        format!("worth.spatial.binding.{}", self.identity)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PrimitiveBindingDeclarationFamily;

impl ForgeQueryDeclarationFamilyMarker<PrimitiveBindingQueryDomain>
    for PrimitiveBindingDeclarationFamily
{
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalNotCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "PrimitiveBinding"
    }

    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        ForgeQueryDeclarationAspectContract::from_slices(
            &["binding.kind", "binding.site", "binding.geometry"],
            &["binding.topology_contract"],
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PrimitiveAnchorBindingQueryDomain;

impl ForgeQueryDomainEntryMarker for PrimitiveAnchorBindingQueryDomain {
    fn domain_key(&self) -> &'static str {
        "worth.spatial.binding.anchor"
    }

    fn display_name(&self) -> &'static str {
        "WorthSpatialAnchorBindingDomain"
    }

    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[ForgeQueryCapabilityFamily::QueryComposition]
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveAnchorBindingQueryWorld {
    identity: String,
}

impl PrimitiveAnchorBindingQueryWorld {
    pub fn new(identity: impl Into<String>) -> Self {
        Self {
            identity: identity.into(),
        }
    }
}

impl ForgeQueryDomainOperatingContext<PrimitiveAnchorBindingQueryDomain>
    for PrimitiveAnchorBindingQueryWorld
{
    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[
            ForgeQueryCapabilityFamily::HistoricalEvaluation,
            ForgeQueryCapabilityFamily::QueryComposition,
        ]
    }

    fn required_config_sections(&self) -> &'static [ForgeQueryConfigSectionFamily] {
        &[
            ForgeQueryConfigSectionFamily::Query,
            ForgeQueryConfigSectionFamily::Relational,
        ]
    }

    fn context_identity_digest(&self) -> String {
        format!("worth.spatial.binding.anchor.{}", self.identity)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PrimitiveAnchorBindingDeclarationFamily;

impl ForgeQueryDeclarationFamilyMarker<PrimitiveAnchorBindingQueryDomain>
    for PrimitiveAnchorBindingDeclarationFamily
{
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalNotCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "PrimitiveAnchorBinding"
    }

    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        ForgeQueryDeclarationAspectContract::from_slices(
            &["binding.kind", "binding.site", "binding.geometry"],
            &["binding.topology_contract"],
            &[
                "binding.anchor.carrier",
                "binding.anchor.parameter",
                "binding.anchor.role",
            ],
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
