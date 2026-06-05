use forge_query::facade::{
    ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily, ForgeQueryDeclarationAspectContract,
    ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationFamilyMarker,
    ForgeQueryDeclarationInput, ForgeQueryDeclarationLegalityContract,
    ForgeQueryDeclarationRouteContract, ForgeQueryDomainEntryMarker,
    ForgeQueryDomainOperatingContext, ForgeQueryNeighborhoodCapableGrouping,
    ForgeQueryRelationalTruthAuthority, ForgeQuerySignalNotCompatiblePosture,
};

use crate::binding::anchoring::PrimitiveAnchorBindingDeclarationEntry;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PrimitiveAnchorBindingQueryDomain;

impl ForgeQueryDomainEntryMarker for PrimitiveAnchorBindingQueryDomain {
    fn domain_key(&self) -> &'static str {
        "worth.kernel.binding.anchor"
    }

    fn display_name(&self) -> &'static str {
        "WorthKernelAnchorBindingDomain"
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
        format!("worth.kernel.binding.anchor.{}", self.identity)
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
        "PrimitiveAnchorBindingDeclarationFamily"
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

impl ForgeQueryDeclarationInput<PrimitiveAnchorBindingQueryDomain>
    for PrimitiveAnchorBindingDeclarationEntry
{
    type Family = PrimitiveAnchorBindingDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        self.canonical_query_entries()
    }
}
