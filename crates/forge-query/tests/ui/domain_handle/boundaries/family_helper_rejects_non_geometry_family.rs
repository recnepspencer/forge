use forge_query::facade::{
    ForgeQueryApplicationFacade, ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily,
    ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationFamilyMarker,
    ForgeQueryDeclarationInput, ForgeQueryDeclarationLegalityContract, ForgeQueryDomainEntryMarker,
    ForgeQueryDomainOperatingContext, ForgeQueryNeighborhoodCapableGrouping,
    ForgeQueryRelationalTruthAuthority, ForgeQuerySignalCompatiblePosture,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ExampleDomain;

impl ForgeQueryDomainEntryMarker for ExampleDomain {
    fn domain_key(&self) -> &'static str {
        "example.family.helper"
    }

    fn display_name(&self) -> &'static str {
        "ExampleDomain"
    }

    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[ForgeQueryCapabilityFamily::QueryComposition]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ExampleWorld;

impl ForgeQueryDomainOperatingContext<ExampleDomain> for ExampleWorld {
    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[]
    }

    fn required_config_sections(&self) -> &'static [ForgeQueryConfigSectionFamily] {
        &[]
    }

    fn context_identity_digest(&self) -> String {
        "example-world".to_string()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct NonGeometryFamily;

impl ForgeQueryDeclarationFamilyMarker<ExampleDomain> for NonGeometryFamily {
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "non-geometry-family"
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct NonGeometryInput;

impl ForgeQueryDeclarationInput<ExampleDomain> for NonGeometryInput {
    type Family = NonGeometryFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![ForgeQueryDeclarationCanonicalEntry::text("id", "value")]
    }
}

fn main() {
    let handle = ForgeQueryApplicationFacade::runtime_backed_default()
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
