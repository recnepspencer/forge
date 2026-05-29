use forge_query::facade::{
    ForgeQueryCapabilityFamily, ForgeQueryDeclarationCanonicalEntry,
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationInput,
    ForgeQueryDeclarationLegalityContract, ForgeQueryDeclarationSignalCompatibility,
    ForgeQueryDeclarationSignalCompatibilityExplanation, ForgeQueryDomainEntryMarker,
    ForgeQueryNeighborhoodCapableGrouping, ForgeQueryRelationalTruthAuthority,
    ForgeQuerySignalCompatiblePosture,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ExampleDomain;

impl ForgeQueryDomainEntryMarker for ExampleDomain {
    fn domain_key(&self) -> &'static str { "example.domain" }
    fn display_name(&self) -> &'static str { "ExampleDomain" }
    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] { &[] }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ExampleFamily;

impl ForgeQueryDeclarationFamilyMarker<ExampleDomain> for ExampleFamily {
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str { "example-family" }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ExampleInput;

impl ForgeQueryDeclarationInput<ExampleDomain> for ExampleInput {
    type Family = ExampleFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        Vec::new()
    }
}

fn main() {
    let envelope = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
    let explanation: ForgeQueryDeclarationSignalCompatibilityExplanation =
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() };

    let _ = ForgeQueryDeclarationSignalCompatibility::<ExampleDomain, ExampleInput>::new(
        forge_query::facade::ForgeQueryDeclarationPrimaryAuthorityFamily::RelationalTruth,
        forge_query::facade::ForgeQueryDeclarationSignalExecutionFamily::RuntimeDerivedExecution,
        Vec::new(),
        envelope,
        "digest".to_string(),
        explanation,
    );
}
