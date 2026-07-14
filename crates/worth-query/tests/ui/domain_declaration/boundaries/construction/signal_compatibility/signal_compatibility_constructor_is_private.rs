use worth_query::facade::foundation::{WorthQueryCapabilityFamily, WorthQueryDeclarationCanonicalEntry, WorthQueryDeclarationFamilyMarker, WorthQueryDeclarationInput, WorthQueryDeclarationLegalityContract, WorthQueryDeclarationSignalCompatibility, WorthQueryDeclarationSignalCompatibilityExplanation, WorthQueryDomainEntryMarker, WorthQueryNeighborhoodCapableGrouping, WorthQueryRelationalTruthAuthority, WorthQuerySignalCompatiblePosture};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ExampleDomain;

impl WorthQueryDomainEntryMarker for ExampleDomain {
    fn domain_key(&self) -> &'static str { "example.domain" }
    fn display_name(&self) -> &'static str { "ExampleDomain" }
    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] { &[] }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ExampleFamily;

impl WorthQueryDeclarationFamilyMarker<ExampleDomain> for ExampleFamily {
    type PrimaryAuthority = WorthQueryRelationalTruthAuthority;
    type SignalCompatibility = WorthQuerySignalCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str { "example-family" }

    fn legality_contract() -> WorthQueryDeclarationLegalityContract {
        WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ExampleInput;

impl WorthQueryDeclarationInput<ExampleDomain> for ExampleInput {
    type Family = ExampleFamily;

    fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
        Vec::new()
    }
}

fn main() {
    let envelope = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
    let explanation: WorthQueryDeclarationSignalCompatibilityExplanation =
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() };

    let _ = WorthQueryDeclarationSignalCompatibility::<ExampleDomain, ExampleInput>::new(
        worth_query::facade::foundation::WorthQueryDeclarationPrimaryAuthorityFamily::RelationalTruth,
        worth_query::facade::foundation::WorthQueryDeclarationSignalExecutionFamily::RuntimeDerivedExecution,
        Vec::new(),
        envelope,
        "digest".to_string(),
        explanation,
    );
}
