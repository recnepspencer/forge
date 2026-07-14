use worth_query::facade::foundation::{WorthQueryCapabilityFamily, WorthQueryContinuationBindingInput, WorthQueryDeclarationAspectContract, WorthQueryDeclarationCanonicalEntry, WorthQueryDeclarationFamilyMarker, WorthQueryDeclarationInput, WorthQueryDeclarationLegalityContract, WorthQueryDeclarationRouteContract, WorthQueryDomainEntryMarker, WorthQueryNeighborhoodCapableGrouping, WorthQueryRelationalTruthAuthority, WorthQuerySignalCompatiblePosture};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ExampleDomain;

impl WorthQueryDomainEntryMarker for ExampleDomain {
    fn domain_key(&self) -> &'static str {
        "example.binding.domain"
    }

    fn display_name(&self) -> &'static str {
        "ExampleDomain"
    }

    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        &[]
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ExampleInput;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ExampleFamily;

impl WorthQueryDeclarationFamilyMarker<ExampleDomain> for ExampleFamily {
    type PrimaryAuthority = WorthQueryRelationalTruthAuthority;
    type SignalCompatibility = WorthQuerySignalCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "ExampleFamily"
    }

    fn aspect_contract() -> WorthQueryDeclarationAspectContract {
        WorthQueryDeclarationAspectContract::empty()
    }

    fn legality_contract() -> WorthQueryDeclarationLegalityContract {
        WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> WorthQueryDeclarationRouteContract {
        WorthQueryDeclarationRouteContract::relational_only()
    }
}

impl WorthQueryDeclarationInput<ExampleDomain> for ExampleInput {
    type Family = ExampleFamily;

    fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
        vec![]
    }
}

fn main() {
    let _ = WorthQueryContinuationBindingInput::<ExampleDomain, ExampleInput> {
        bridge_request: unsafe { std::mem::zeroed() },
        subject: unsafe { std::mem::zeroed() },
    };
}
