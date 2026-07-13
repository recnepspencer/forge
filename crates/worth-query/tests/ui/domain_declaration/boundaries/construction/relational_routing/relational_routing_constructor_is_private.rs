use worth_query::facade::foundation::{WorthQueryCapabilityFamily, WorthQueryDeclarationEnvelope, WorthQueryDeclarationInput, WorthQueryDeclarationRelationalAuthorityFamily, WorthQueryDeclarationRelationalBinding, WorthQueryDeclarationRelationalRouting, WorthQueryDeclarationRelationalRoutingClass, WorthQueryDeclarationRelationalRoutingExplanation, WorthQueryDeclarationRelationalTruthClaim, WorthQueryDomainEntryMarker};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ExampleDomain;

impl WorthQueryDomainEntryMarker for ExampleDomain {
    fn domain_key(&self) -> &'static str { "example.domain" }
    fn display_name(&self) -> &'static str { "ExampleDomain" }
    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] { &[] }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ExampleInput;

impl WorthQueryDeclarationInput<ExampleDomain> for ExampleInput {
    type Family = ExampleFamily;

    fn canonical_declaration_entries(
        &self,
    ) -> Vec<worth_query::facade::foundation::WorthQueryDeclarationCanonicalEntry> {
        Vec::new()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ExampleFamily;

impl worth_query::facade::foundation::WorthQueryDeclarationFamilyMarker<ExampleDomain> for ExampleFamily {
    type PrimaryAuthority = worth_query::facade::foundation::WorthQueryRelationalTruthAuthority;
    type SignalCompatibility = worth_query::facade::foundation::WorthQuerySignalCompatiblePosture;
    type GroupedPosture = worth_query::facade::foundation::WorthQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str { "example-family" }

    fn legality_contract() -> worth_query::facade::foundation::WorthQueryDeclarationLegalityContract {
        worth_query::facade::foundation::WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }
}

fn main() {
    let envelope: WorthQueryDeclarationEnvelope<ExampleDomain, ExampleInput> =
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
    let explanation: WorthQueryDeclarationRelationalRoutingExplanation =
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() };

    let _ = WorthQueryDeclarationRelationalRouting::new(
        WorthQueryDeclarationRelationalRoutingClass::ExclusiveRelationalTruth,
        WorthQueryDeclarationRelationalTruthClaim::AuthoritativeCurrentTruth,
        WorthQueryDeclarationRelationalAuthorityFamily::Runtime,
        WorthQueryDeclarationRelationalBinding::Runtime("worth_relational::facade::runtime"),
        envelope,
        "digest".to_string(),
        explanation,
    );
}
