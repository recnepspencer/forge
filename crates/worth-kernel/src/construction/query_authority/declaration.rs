use forge_query::facade::runtime::{
    ForgeQueryGraphObligationOperatingWorldSelector, ForgeQueryGraphObligationRegistration,
    ForgeQueryGraphObligationRuleIdentity, ForgeQueryGraphTouchSelector,
};
use forge_query::facade::{
    ForgeQueryBridgeContinuationAuthority, ForgeQueryDeclarationAspectContract,
    ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationFamilyMarker,
    ForgeQueryDeclarationInput, ForgeQueryDeclarationLegalityContract,
    ForgeQueryDeclarationRouteContract, ForgeQueryNeighborhoodCapableGrouping,
    ForgeQuerySignalNotCompatiblePosture,
};

use super::domain::PrimitiveConstructionQueryDomain;
use crate::construction::request::PrimitiveConstructionFamily;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct PrimitiveConstructionQueryDeclarationFamily;

impl ForgeQueryDeclarationFamilyMarker<PrimitiveConstructionQueryDomain>
    for PrimitiveConstructionQueryDeclarationFamily
{
    type PrimaryAuthority = ForgeQueryBridgeContinuationAuthority;
    type SignalCompatibility = ForgeQuerySignalNotCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "PrimitiveConstructionQueryDeclaration"
    }

    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        ForgeQueryDeclarationAspectContract::from_slices(
            &["construction.family"],
            &[],
            &[],
            &[],
            &[],
        )
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> ForgeQueryDeclarationRouteContract {
        ForgeQueryDeclarationRouteContract::bridge_only()
    }

    fn orchestration_graph_touch_collection() -> Option<&'static str> {
        Some("worth-kernel.primitive-construction")
    }

    fn orchestration_graph_obligation_registrations() -> Vec<ForgeQueryGraphObligationRegistration>
    {
        vec![
            ForgeQueryGraphObligationRegistration::operating_context_gate(
                ForgeQueryGraphObligationRuleIdentity::new(
                    "worth.kernel.construction",
                    "primitive-construction-query-authority",
                    "v1",
                )
                .expect("primitive construction query authority rule identity"),
                ForgeQueryGraphTouchSelector::collection("worth-kernel.primitive-construction")
                    .expect("primitive construction touch selector"),
                ForgeQueryGraphObligationOperatingWorldSelector::configured_domain_handle(),
            ),
        ]
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PrimitiveConstructionQueryDeclarationInput {
    family: PrimitiveConstructionFamily,
}

impl PrimitiveConstructionQueryDeclarationInput {
    pub(crate) fn new(family: PrimitiveConstructionFamily) -> Self {
        Self { family }
    }
}

impl ForgeQueryDeclarationInput<PrimitiveConstructionQueryDomain>
    for PrimitiveConstructionQueryDeclarationInput
{
    type Family = PrimitiveConstructionQueryDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![ForgeQueryDeclarationCanonicalEntry::text(
            "construction.family",
            self.family.as_str(),
        )]
    }
}
