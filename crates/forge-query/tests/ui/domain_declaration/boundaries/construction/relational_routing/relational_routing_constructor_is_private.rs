use forge_query::facade::{
    ForgeQueryCapabilityFamily,
    ForgeQueryDeclarationEnvelope, ForgeQueryDeclarationInput,
    ForgeQueryDeclarationRelationalAuthorityFamily, ForgeQueryDeclarationRelationalBinding,
    ForgeQueryDeclarationRelationalRouting, ForgeQueryDeclarationRelationalRoutingClass,
    ForgeQueryDeclarationRelationalRoutingExplanation, ForgeQueryDeclarationRelationalTruthClaim,
    ForgeQueryDomainEntryMarker,
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

    fn canonical_declaration_entries(
        &self,
    ) -> Vec<forge_query::facade::ForgeQueryDeclarationCanonicalEntry> {
        Vec::new()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ExampleFamily;

impl forge_query::facade::ForgeQueryDeclarationFamilyMarker<ExampleDomain> for ExampleFamily {
    type PrimaryAuthority = forge_query::facade::ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = forge_query::facade::ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = forge_query::facade::ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str { "example-family" }

    fn legality_contract() -> forge_query::facade::ForgeQueryDeclarationLegalityContract {
        forge_query::facade::ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }
}

fn main() {
    let envelope: ForgeQueryDeclarationEnvelope<ExampleDomain, ExampleInput> =
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
    let explanation: ForgeQueryDeclarationRelationalRoutingExplanation =
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() };

    let _ = ForgeQueryDeclarationRelationalRouting::new(
        ForgeQueryDeclarationRelationalRoutingClass::ExclusiveRelationalTruth,
        ForgeQueryDeclarationRelationalTruthClaim::AuthoritativeCurrentTruth,
        ForgeQueryDeclarationRelationalAuthorityFamily::Runtime,
        ForgeQueryDeclarationRelationalBinding::Runtime("forge_relational::facade::runtime"),
        envelope,
        "digest".to_string(),
        explanation,
    );
}
