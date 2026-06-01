use forge_query::facade::{
    ForgeQueryBridgeContinuationAuthority, ForgeQueryCapabilityFamily,
    ForgeQueryDeclarationBridgeBinding, ForgeQueryDeclarationBridgeContinuationFamily,
    ForgeQueryDeclarationBridgeContinuationMode, ForgeQueryDeclarationBridgeContinuationRequest,
    ForgeQueryDeclarationBridgeRouting, ForgeQueryDeclarationBridgeRoutingClass,
    ForgeQueryDeclarationBridgeRoutingExplanation, ForgeQueryDeclarationCanonicalEntry,
    ForgeQueryDeclarationEnvelope, ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationInput,
    ForgeQueryDeclarationLegalityContract, ForgeQueryDomainEntryMarker,
    ForgeQueryNeighborhoodCapableGrouping, ForgeQuerySignalCompatiblePosture,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ExampleDomain;

impl ForgeQueryDomainEntryMarker for ExampleDomain {
    fn domain_key(&self) -> &'static str { "example.domain" }
    fn display_name(&self) -> &'static str { "ExampleDomain" }
    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] { &[] }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ExampleInput;

impl ForgeQueryDeclarationInput<ExampleDomain> for ExampleInput {
    type Family = ExampleFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        Vec::new()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ExampleFamily;

impl ForgeQueryDeclarationFamilyMarker<ExampleDomain> for ExampleFamily {
    type PrimaryAuthority = ForgeQueryBridgeContinuationAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str { "example-family" }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }
}

fn main() {
    let envelope: ForgeQueryDeclarationEnvelope<ExampleDomain, ExampleInput> =
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
    let binding: ForgeQueryDeclarationBridgeBinding =
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
    let explanation: ForgeQueryDeclarationBridgeRoutingExplanation =
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() };

    let _ = ForgeQueryDeclarationBridgeRouting::new(
        ForgeQueryDeclarationBridgeRoutingClass::ExclusiveBridgeContinuation,
        ForgeQueryDeclarationBridgeContinuationRequest::new(
            ForgeQueryDeclarationBridgeContinuationMode::RuntimeRoute,
            forge_query::facade::ForgeQueryDeclarationBridgeTruthContext::Current,
        ),
        ForgeQueryDeclarationBridgeContinuationFamily::RuntimeRoute,
        binding,
        envelope,
        "digest".to_string(),
        explanation,
    );
}
