#![allow(invalid_value)]

use worth_query::facade::{
    WorthQueryCapabilityFamily, WorthQueryConfigSectionFamily,
    WorthQueryDeclarationCanonicalEntry, WorthQueryDeclarationFamilyMarker,
    WorthQueryDeclarationInput, WorthQueryDeclarationLegalityContract, WorthQueryDomainEntryMarker,
    WorthQueryDomainOperatingContext, WorthQueryNeighborhoodCapableGrouping,
    WorthQueryRelationalTruthAuthority, WorthQuerySignalCompatibilityOrchestrationChecked,
    WorthQuerySignalCompatibilityOrchestrationOutcome,
    WorthQuerySignalCompatibilityOrchestrationTranscript, WorthQuerySignalCompatiblePosture,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GeometryDomainEntry;

impl WorthQueryDomainEntryMarker for GeometryDomainEntry {
    fn domain_key(&self) -> &'static str {
        "example.geometry"
    }

    fn display_name(&self) -> &'static str {
        "GeometryDomainEntry"
    }

    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        &[]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GeometryOperatingContext;

impl WorthQueryDomainOperatingContext<GeometryDomainEntry> for GeometryOperatingContext {
    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        &[]
    }

    fn required_config_sections(&self) -> &'static [WorthQueryConfigSectionFamily] {
        &[]
    }

    fn context_identity_digest(&self) -> String {
        "geometry".to_string()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct GeometryFamily;

impl WorthQueryDeclarationFamilyMarker<GeometryDomainEntry> for GeometryFamily {
    type PrimaryAuthority = WorthQueryRelationalTruthAuthority;
    type SignalCompatibility = WorthQuerySignalCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "geometry-family"
    }

    fn legality_contract() -> WorthQueryDeclarationLegalityContract {
        WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct GeometryInput;

impl WorthQueryDeclarationInput<GeometryDomainEntry> for GeometryInput {
    type Family = GeometryFamily;

    fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
        vec![WorthQueryDeclarationCanonicalEntry::text("edge_ref", "edge-a")]
    }
}

fn main() {
    let outcome = WorthQuerySignalCompatibilityOrchestrationOutcome::<
        GeometryDomainEntry,
        GeometryInput,
    >::Unavailable(String::new());
    let _ = WorthQuerySignalCompatibilityOrchestrationChecked::<
        GeometryDomainEntry,
        GeometryInput,
    > {
        outcome,
        orchestration_digest: String::new(),
        linked_artifacts: unsafe { std::mem::zeroed() },
    };
    let outcome = WorthQuerySignalCompatibilityOrchestrationOutcome::<
        GeometryDomainEntry,
        GeometryInput,
    >::Unavailable(String::new());
    let _ = WorthQuerySignalCompatibilityOrchestrationTranscript::<
        GeometryDomainEntry,
        GeometryInput,
    > {
        request: unsafe { std::mem::zeroed() },
        outcome,
        witness_checks: vec![],
        narrowing_decisions: vec![],
        orchestration_digest: String::new(),
        linked_artifacts: unsafe { std::mem::zeroed() },
    };
}
