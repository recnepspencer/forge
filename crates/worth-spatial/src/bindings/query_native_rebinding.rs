use forge_query::facade::{
    ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily, ForgeQueryDeclarationAspectContract,
    ForgeQueryDeclarationBridgeContinuationContract, ForgeQueryDeclarationFamilyMarker,
    ForgeQueryDeclarationLegalityContract, ForgeQueryDeclarationRouteContract,
    ForgeQueryDeclarationSignalCompatibilityContract, ForgeQueryDomainEntryMarker,
    ForgeQueryDomainOperatingContext, ForgeQueryNeighborhoodCapableGrouping,
    ForgeQueryRelationalTruthAuthority, ForgeQuerySignalCompatiblePosture,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PrimitiveRebindingQueryDomain;

impl ForgeQueryDomainEntryMarker for PrimitiveRebindingQueryDomain {
    fn domain_key(&self) -> &'static str {
        "worth.spatial.rebinding"
    }

    fn display_name(&self) -> &'static str {
        "WorthSpatialRebindingDomain"
    }

    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[ForgeQueryCapabilityFamily::QueryComposition]
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveRebindingQueryWorld {
    identity: String,
}

impl PrimitiveRebindingQueryWorld {
    pub fn new(identity: impl Into<String>) -> Self {
        Self {
            identity: identity.into(),
        }
    }
}

impl ForgeQueryDomainOperatingContext<PrimitiveRebindingQueryDomain>
    for PrimitiveRebindingQueryWorld
{
    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[
            ForgeQueryCapabilityFamily::HistoricalEvaluation,
            ForgeQueryCapabilityFamily::PreviewSession,
            ForgeQueryCapabilityFamily::QueryComposition,
            ForgeQueryCapabilityFamily::QueryRead,
            ForgeQueryCapabilityFamily::WorkflowOrchestration,
        ]
    }

    fn required_config_sections(&self) -> &'static [ForgeQueryConfigSectionFamily] {
        &[
            ForgeQueryConfigSectionFamily::Query,
            ForgeQueryConfigSectionFamily::Relational,
            ForgeQueryConfigSectionFamily::RuntimeBridge,
            ForgeQueryConfigSectionFamily::Signal,
        ]
    }

    fn context_identity_digest(&self) -> String {
        format!("worth.spatial.rebinding.{}", self.identity)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PrimitiveRebindingDeclarationFamily;

impl ForgeQueryDeclarationFamilyMarker<PrimitiveRebindingQueryDomain>
    for PrimitiveRebindingDeclarationFamily
{
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "PrimitiveRebinding"
    }

    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        ForgeQueryDeclarationAspectContract::from_slices(
            &[
                "rebinding.kind",
                "rebinding.prior",
                "rebinding.neighborhood",
            ],
            &["rebinding.candidates"],
            &[],
            &[],
            &[],
        )
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> ForgeQueryDeclarationRouteContract {
        ForgeQueryDeclarationRouteContract::relational_and_bridge()
    }

    fn bridge_continuation_contract() -> Option<ForgeQueryDeclarationBridgeContinuationContract> {
        Some(ForgeQueryDeclarationBridgeContinuationContract::runtime_route_current())
    }

    fn signal_compatibility_contract() -> Option<ForgeQueryDeclarationSignalCompatibilityContract> {
        Some(ForgeQueryDeclarationSignalCompatibilityContract::runtime_derived_execution())
    }
}
