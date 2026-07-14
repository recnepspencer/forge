use worth_query::facade::foundation::{WorthQueryApplicationFacade, WorthQueryCapabilityFamily, WorthQueryConfigSectionFamily, WorthQueryDeclarationCanonicalEntry, WorthQueryDeclarationFamilyMarker, WorthQueryDeclarationInput, WorthQueryDeclarationLegalityContract, WorthQueryDomainEntryMarker, WorthQueryDomainOperatingContext, WorthQueryNeighborhoodCapableGrouping, WorthQueryRelationalTruthAuthority, WorthQuerySignalCompatiblePosture};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ExampleDomain;

impl WorthQueryDomainEntryMarker for ExampleDomain {
    fn domain_key(&self) -> &'static str {
        "example.family.helper"
    }

    fn display_name(&self) -> &'static str {
        "ExampleDomain"
    }

    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        &[WorthQueryCapabilityFamily::QueryComposition]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ExampleWorld;

impl WorthQueryDomainOperatingContext<ExampleDomain> for ExampleWorld {
    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        &[]
    }

    fn required_config_sections(&self) -> &'static [WorthQueryConfigSectionFamily] {
        &[]
    }

    fn context_identity_digest(&self) -> String {
        "example-world".to_string()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct NonGeometryFamily;

impl WorthQueryDeclarationFamilyMarker<ExampleDomain> for NonGeometryFamily {
    type PrimaryAuthority = WorthQueryRelationalTruthAuthority;
    type SignalCompatibility = WorthQuerySignalCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "non-geometry-family"
    }

    fn legality_contract() -> WorthQueryDeclarationLegalityContract {
        WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct NonGeometryInput;

impl WorthQueryDeclarationInput<ExampleDomain> for NonGeometryInput {
    type Family = NonGeometryFamily;

    fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
        vec![WorthQueryDeclarationCanonicalEntry::text("id", "value")]
    }
}

fn main() {
    let handle = WorthQueryApplicationFacade::runtime_backed_default()
        .domain(ExampleDomain)
        .with_operating_context(ExampleWorld)
        .validate()
        .unwrap()
        .admit()
        .unwrap();
    let _ = handle
        .geometry_helpers()
        .progress_active_face_selection(NonGeometryInput);
}
